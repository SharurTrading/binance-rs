// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Pure Options depth bootstrap using snapshot IDs and the `pu` continuity chain.
//!
//! Source: <https://developers.binance.com/en/docs/products/derivatives-trading-options/websocket-market-streams/How-to-manage-a-local-order-book-correctly>.
//! A finite REST snapshot is partial. This helper never invents unseen depth.

use super::Symbol;
use super::{
    rest_models::OrderBookResponse, stream_models::DiffBookDepthStreamsEvent, wire::PriceLevel,
};
use crate::{Decimal, Error};
use std::collections::{BTreeMap, VecDeque};

/// Proven lifecycle of a local depth mirror.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BookState {
    /// Updates are retained until a snapshot and a bridging update exist.
    AwaitingSnapshot,
    /// Snapshot/update ID continuity is established, within finite snapshot depth.
    Ready,
    /// A real gap disproved continuity; establish a new snapshot before viewing.
    Gap,
}

/// An observed, finite-depth book. A caller is the single writer; no I/O or tasks.
#[derive(Clone)]
pub struct DepthBook {
    symbol: Symbol,
    generation: u64,
    state: BookState,
    bids: BTreeMap<Decimal, Decimal>,
    asks: BTreeMap<Decimal, Decimal>,
    pending: VecDeque<DiffBookDepthStreamsEvent>,
    last: i64,
    bridging: bool,
    have_snapshot: bool,
}
impl DepthBook {
    /// Begin collecting updates from one symbol and socket generation.
    #[must_use]
    pub fn new(symbol: Symbol, generation: u64) -> Self {
        Self {
            symbol,
            generation,
            state: BookState::AwaitingSnapshot,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            pending: VecDeque::new(),
            last: 0,
            bridging: true,
            have_snapshot: false,
        }
    }
    /// Current continuity evidence, independent of event age.
    #[must_use]
    pub fn state(&self) -> BookState {
        self.state
    }
    /// Number of accepted updates buffered while awaiting a snapshot bridge.
    /// Failed installations preserve this evidence for subsequent venue snapshots.
    #[must_use]
    pub fn pending_updates(&self) -> usize {
        self.pending.len()
    }
    /// This mirror cannot claim complete venue depth beyond its finite snapshot.
    #[must_use]
    pub fn is_partial(&self) -> bool {
        true
    }
    /// Last applied update ID, only when continuity is established.
    #[must_use]
    pub fn last_update_id(&self) -> Option<i64> {
        if self.state == BookState::Ready {
            Some(self.last)
        } else {
            None
        }
    }
    /// Retain an accepted update without a bootstrap capacity cutoff.
    ///
    /// # Errors
    /// Refuses a different generation/symbol, malformed IDs, or a broken `pu` chain.
    /// A failure marks this mirror unproven; it never exposes partial success as ready.
    pub fn update(
        &mut self,
        generation: u64,
        event: DiffBookDepthStreamsEvent,
    ) -> Result<(), Error> {
        if generation != self.generation || event.s.as_str() != self.symbol.as_str() {
            self.state = BookState::Gap;
            return Err(Error::Gap("depth generation or symbol mismatch"));
        }
        if self.state == BookState::Gap {
            return Err(Error::Gap("depth mirror requires a new snapshot"));
        }
        if !self.have_snapshot {
            self.pending.push_back(event);
            return Ok(());
        }
        self.apply(&event)
    }
    /// Install a finite snapshot and apply buffered updates in accepted source order.
    ///
    /// # Errors
    /// Refuses malformed snapshot/update evidence or a gap. The snapshot alone
    /// does not make the mirror Ready: a bridging stream update is required.
    pub fn snapshot(&mut self, snapshot: &OrderBookResponse) -> Result<(), Error> {
        let mut staging = self.clone();
        match staging.install(snapshot) {
            Ok(()) => {
                *self = staging;
                Ok(())
            }
            Err(error) => {
                self.state = BookState::Gap;
                Err(error)
            }
        }
    }

    fn install(&mut self, snapshot: &OrderBookResponse) -> Result<(), Error> {
        self.last = snapshot.last_update_id;
        if self.last < 0 {
            return Err(Error::Gap("negative snapshot ID"));
        }
        self.bids.clear();
        self.asks.clear();
        self.bridging = true;
        self.have_snapshot = true;
        apply_levels(&mut self.bids, &snapshot.bids)?;
        apply_levels(&mut self.asks, &snapshot.asks)?;
        while let Some(event) = self.pending.pop_front() {
            self.apply(&event)?;
        }
        self.state = if self.bridging {
            BookState::AwaitingSnapshot
        } else {
            BookState::Ready
        };
        Ok(())
    }
    fn apply(&mut self, event: &DiffBookDepthStreamsEvent) -> Result<(), Error> {
        let result = self.apply_inner(event);
        if result.is_err() {
            self.state = BookState::Gap;
        }
        result
    }
    fn apply_inner(&mut self, event: &DiffBookDepthStreamsEvent) -> Result<(), Error> {
        let first = event.upper_u;
        let last = event.u;
        let previous = event.pu;
        if first < 0 || last < first || previous < 0 {
            return Err(Error::Gap("invalid depth update IDs"));
        }
        if last < self.last || (!self.bridging && last == self.last) {
            return Ok(());
        }
        if self.bridging {
            if first > self.last || last < self.last {
                return Err(Error::Gap("snapshot not bridged by buffered updates"));
            }
        } else if previous != self.last {
            return Err(Error::Gap("depth previous-update discontinuity"));
        }
        validate_levels(&event.b)?;
        validate_levels(&event.a)?;
        apply_levels(&mut self.bids, &event.b)?;
        apply_levels(&mut self.asks, &event.a)?;
        self.last = last;
        self.bridging = false;
        self.state = BookState::Ready;
        Ok(())
    }
    /// Observed bids, ordered from low to high; iterate in reverse for best bid first.
    ///
    /// # Errors
    /// Refuses a mirror whose snapshot/update continuity is unproven.
    pub fn bids(&self) -> Result<&BTreeMap<Decimal, Decimal>, Error> {
        if self.state != BookState::Ready {
            return Err(Error::Gap("unproven depth mirror"));
        }
        Ok(&self.bids)
    }
    /// Observed asks, ordered from best to worst.
    ///
    /// # Errors
    /// Refuses a mirror whose snapshot/update continuity is unproven.
    pub fn asks(&self) -> Result<&BTreeMap<Decimal, Decimal>, Error> {
        if self.state != BookState::Ready {
            return Err(Error::Gap("unproven depth mirror"));
        }
        Ok(&self.asks)
    }
}
fn apply_levels(
    levels: &mut BTreeMap<Decimal, Decimal>,
    updates: &[PriceLevel],
) -> Result<(), Error> {
    validate_levels(updates)?;
    for level in updates {
        if level.quantity == Decimal::ZERO {
            levels.remove(&level.price);
        } else {
            levels.insert(level.price, level.quantity);
        }
    }
    Ok(())
}

fn validate_levels(updates: &[PriceLevel]) -> Result<(), Error> {
    if updates
        .iter()
        .any(|l| l.price <= Decimal::ZERO || l.quantity < Decimal::ZERO)
    {
        return Err(Error::Gap("invalid Options depth magnitude"));
    }
    Ok(())
}
