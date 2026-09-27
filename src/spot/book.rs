// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Pure Spot depth bootstrap using snapshot IDs and the `U`/`u` sequence.
//!
//! Source: Binance's “How to manage a local order book correctly” documentation.
//! A finite REST snapshot is partial. This helper never invents unseen depth.

use super::{rest_models::DepthResponse, stream_models::DiffBookDepthEvent, wire::PriceLevel};
use crate::{Decimal, Error, Symbol};
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
pub struct DepthBook {
    symbol: Symbol,
    generation: u64,
    state: BookState,
    bids: BTreeMap<Decimal, Decimal>,
    asks: BTreeMap<Decimal, Decimal>,
    pending: VecDeque<DiffBookDepthEvent>,
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
    /// Refuses a different generation/symbol, malformed IDs, or a real update-ID gap.
    /// A failure marks this mirror unproven; it never exposes partial success as ready.
    pub fn update(&mut self, generation: u64, event: DiffBookDepthEvent) -> Result<(), Error> {
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
    pub fn snapshot(&mut self, snapshot: &DepthResponse) -> Result<(), Error> {
        let result = self.install(snapshot);
        if result.is_err() {
            self.state = BookState::Gap;
        }
        result
    }
    fn install(&mut self, snapshot: &DepthResponse) -> Result<(), Error> {
        self.last = snapshot
            .last_update_id
            .ok_or(Error::Gap("snapshot update ID"))?;
        if self.last < 0 {
            return Err(Error::Gap("negative snapshot ID"));
        }
        self.bids.clear();
        self.asks.clear();
        self.bridging = true;
        self.have_snapshot = true;
        apply_levels(
            &mut self.bids,
            snapshot
                .bids
                .as_deref()
                .ok_or(Error::Gap("snapshot bids"))?,
        )?;
        apply_levels(
            &mut self.asks,
            snapshot
                .asks
                .as_deref()
                .ok_or(Error::Gap("snapshot asks"))?,
        )?;
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
    fn apply(&mut self, event: &DiffBookDepthEvent) -> Result<(), Error> {
        let result = self.apply_inner(event);
        if result.is_err() {
            self.state = BookState::Gap;
        }
        result
    }
    fn apply_inner(&mut self, event: &DiffBookDepthEvent) -> Result<(), Error> {
        let first = event.upper_u;
        let last = event.u;
        if first < 0 || last < first {
            return Err(Error::Gap("invalid depth update IDs"));
        }
        if last <= self.last {
            return Ok(());
        }
        let next = self
            .last
            .checked_add(1)
            .ok_or(Error::Gap("depth update ID overflow"))?;
        if first > next {
            return Err(Error::Gap("Spot depth update discontinuity"));
        }
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
    for level in updates {
        if level.price <= Decimal::ZERO || level.quantity < Decimal::ZERO {
            return Err(Error::Gap("invalid depth magnitude"));
        }
        if level.quantity == Decimal::ZERO {
            levels.remove(&level.price);
        } else {
            levels.insert(level.price, level.quantity);
        }
    }
    Ok(())
}
