// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Live stream membership on an open market-stream socket.
//!
//! Binance documents the same three control messages in Spot, USDⓈ-M and COIN-M
//! (Spot WebSocket Streams, "Live Subscribing/Unsubscribing to streams"; USDⓈ-M and
//! COIN-M WebSocket Market Streams, same title; verified 2026-10-10). Every product's
//! `Streams` delegates here so the wire shape and the per-socket stream cap are
//! enforced once.

use super::{Error, Socket};
use std::collections::{BTreeMap, BTreeSet};
use tokio::time::Instant;

/// "A single connection can listen to a maximum of 1024 streams" (Spot WebSocket
/// Limits; USDⓈ-M and COIN-M WebSocket Market Streams, Connect; verified 2026-10-10).
const STREAMS_PER_SOCKET: usize = 1024;

/// One documented live-membership control message on a market-stream socket.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum StreamControl {
    /// `SUBSCRIBE`: add streams to the open socket.
    Subscribe,
    /// `UNSUBSCRIBE`: remove streams from the open socket.
    Unsubscribe,
    /// `LIST_SUBSCRIPTIONS`: read the venue's current subscription list.
    ListSubscriptions,
}

impl StreamControl {
    /// Exact wire method name.
    #[must_use]
    pub fn method(self) -> &'static str {
        match self {
            Self::Subscribe => "SUBSCRIBE",
            Self::Unsubscribe => "UNSUBSCRIBE",
            Self::ListSubscriptions => "LIST_SUBSCRIPTIONS",
        }
    }

    /// What a lost answer proves: a membership change may have taken effect, a list did not.
    pub(crate) fn lost_outcome(self) -> super::Outcome {
        match self {
            Self::Subscribe | Self::Unsubscribe => super::Outcome::Unknown,
            Self::ListSubscriptions => super::Outcome::ReadFailed,
        }
    }

    /// Classify one correlated venue answer.
    ///
    /// The documented success is `result: null` for a membership change and an array
    /// of stream names for a list. A refusal carries a numeric `code` and `msg`, either
    /// at the top level as the stream documentation shows or inside an `error` object
    /// as Binance's WebSocket API envelopes it. Anything else is malformed and returns
    /// `None`, so the caller can report a continuity failure instead of a verdict.
    pub(crate) fn answer(
        self,
        value: &serde_json::Value,
    ) -> Option<Result<Option<Vec<String>>, Error>> {
        let refusal = value.get("code").map(|code| (code, value)).or_else(|| {
            value
                .get("error")
                .and_then(|error| error.get("code").map(|code| (code, error)))
        });
        if let Some((code, envelope)) = refusal {
            let code = code.as_i64()?;
            let message = match envelope.get("msg") {
                None => None,
                Some(msg) => Some(msg.as_str()?.to_owned()),
            };
            return Some(Err(Error::ControlRefused {
                operation: self.method(),
                code,
                message,
            }));
        }
        let result = value.get("result")?;
        match self {
            Self::Subscribe | Self::Unsubscribe => result.is_null().then_some(Ok(None)),
            Self::ListSubscriptions => result
                .as_array()?
                .iter()
                .map(|name| name.as_str().map(str::to_owned))
                .collect::<Option<Vec<_>>>()
                .map(|names| Ok(Some(names))),
        }
    }
}

/// The streams one market socket may be subscribed to, and how to decode each.
///
/// `members` is conservative: a subscription whose answer was lost stays counted
/// against the per-socket cap until an `UNSUBSCRIBE` is confirmed. `kinds` only
/// grows within a generation, so frames already in flight for a released stream
/// still decode.
#[derive(Debug, Default)]
pub(crate) struct Membership {
    kinds: BTreeMap<String, &'static str>,
    members: BTreeSet<String>,
}

impl Membership {
    /// Membership requested in the connection URL.
    pub fn connect(streams: &[(&str, &'static str)]) -> Result<Self, Error> {
        let mut membership = Self::default();
        if !streams.is_empty() {
            membership.reserve(streams)?;
        }
        Ok(membership)
    }

    /// Payload kind of a stream this socket has subscribed in this generation.
    pub fn kind(&self, name: &str) -> Option<&'static str> {
        self.kinds.get(name).copied()
    }

    /// Validate and record a subscription before it is sent; returns the names
    /// this call newly counted, so a definitive refusal can return them.
    fn reserve(&mut self, streams: &[(&str, &'static str)]) -> Result<Vec<String>, Error> {
        if streams.is_empty() {
            return Err(Error::Validation("empty subscription set"));
        }
        let mut names = BTreeSet::new();
        if !streams.iter().all(|(name, _)| names.insert(*name)) {
            return Err(Error::Validation("duplicate streams"));
        }
        let added: Vec<String> = names
            .into_iter()
            .filter(|name| !self.members.contains(*name))
            .map(str::to_owned)
            .collect();
        if self.members.len().saturating_add(added.len()) > STREAMS_PER_SOCKET {
            return Err(Error::Validation("documented 1024 streams per socket"));
        }
        for (name, kind) in streams {
            self.kinds.insert((*name).to_owned(), kind);
        }
        self.members.extend(added.iter().cloned());
        Ok(added)
    }

    fn release_check(&self, names: &[&str]) -> Result<(), Error> {
        if names.is_empty() {
            return Err(Error::Validation("empty subscription set"));
        }
        let mut seen = BTreeSet::new();
        if !names.iter().all(|name| seen.insert(*name)) {
            return Err(Error::Validation("duplicate streams"));
        }
        if !names.iter().all(|name| self.members.contains(*name)) {
            return Err(Error::Validation("stream not subscribed on this socket"));
        }
        Ok(())
    }
}

/// Send one `SUBSCRIBE` for `streams` and record the membership it adds.
pub(crate) async fn subscribe(
    socket: &Socket,
    membership: &mut Membership,
    streams: &[(&str, &'static str)],
    deadline: Instant,
) -> Result<(), Error> {
    let added = membership.reserve(streams)?;
    let names = streams.iter().map(|(name, _)| (*name).to_owned()).collect();
    match socket
        .control(StreamControl::Subscribe, names, deadline)
        .await
    {
        Ok(_) => Ok(()),
        Err(error) => {
            // Only proof that nothing was subscribed returns the reservation;
            // an unknown outcome stays counted against the cap.
            if matches!(
                error.outcome(),
                Some(super::Outcome::NotSent | super::Outcome::Rejected)
            ) {
                for name in &added {
                    membership.members.remove(name);
                }
            }
            Err(error)
        }
    }
}

/// Send one `UNSUBSCRIBE` for `names`, releasing them once the venue confirms.
pub(crate) async fn unsubscribe(
    socket: &Socket,
    membership: &mut Membership,
    names: &[&str],
    deadline: Instant,
) -> Result<(), Error> {
    membership.release_check(names)?;
    let wire = names.iter().map(|name| (*name).to_owned()).collect();
    socket
        .control(StreamControl::Unsubscribe, wire, deadline)
        .await?;
    for name in names {
        membership.members.remove(*name);
    }
    Ok(())
}

/// Send one `LIST_SUBSCRIPTIONS` and return the venue's answer unchanged.
pub(crate) async fn list(socket: &Socket, deadline: Instant) -> Result<Vec<String>, Error> {
    socket
        .control(StreamControl::ListSubscriptions, Vec::new(), deadline)
        .await?
        .ok_or(Error::Gap("stream control answer shape"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn refusal_codes_are_read_from_the_documented_and_the_enveloped_shape() {
        for value in [
            json!({"code":2,"msg":"Invalid request: too many parameters","id":1}),
            json!({"error":{"code":2,"msg":"Invalid request: too many parameters"},"id":1}),
        ] {
            assert!(matches!(
                StreamControl::Subscribe.answer(&value),
                Some(Err(Error::ControlRefused { operation: "SUBSCRIBE", code: 2, message: Some(m) }))
                    if m == "Invalid request: too many parameters"
            ));
        }
    }

    #[test]
    fn an_answer_that_contradicts_its_method_is_malformed() {
        assert!(
            StreamControl::Subscribe
                .answer(&json!({"result":["x"],"id":1}))
                .is_none()
        );
        assert!(
            StreamControl::ListSubscriptions
                .answer(&json!({"result":null,"id":1}))
                .is_none()
        );
        assert!(
            StreamControl::Unsubscribe
                .answer(&json!({"code":"2","id":1}))
                .is_none()
        );
        assert!(
            StreamControl::Unsubscribe
                .answer(&json!({"id":1}))
                .is_none()
        );
    }

    #[test]
    fn reserving_counts_only_new_names_and_release_requires_membership() {
        let mut membership = Membership::connect(&[("a@aggTrade", "k")]).unwrap();
        let added = membership
            .reserve(&[("a@aggTrade", "k"), ("b@aggTrade", "k")])
            .unwrap();
        assert_eq!(added, vec!["b@aggTrade".to_owned()]);
        assert!(
            membership
                .release_check(&["a@aggTrade", "b@aggTrade"])
                .is_ok()
        );
        assert!(matches!(
            membership.release_check(&["c@aggTrade"]),
            Err(Error::Validation(_))
        ));
    }
}
