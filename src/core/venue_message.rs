// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use zeroize::Zeroizing;

/// Exact venue explanation retained for explicit inspection after a refusal.
///
/// Venue text is untrusted and can contain secrets or signed URLs. Formatting an
/// error redacts it; only `expose_sensitive_text` reveals the original text.
#[derive(Clone, PartialEq, Eq)]
pub struct VenueMessage(Zeroizing<String>);

impl VenueMessage {
    pub(crate) fn new(value: &str) -> Self {
        Self(Zeroizing::new(value.to_owned()))
    }

    /// Explicitly access the venue's exact message. Callers own its use and must
    /// not automatically log or persist this potentially sensitive text.
    #[must_use]
    pub fn expose_sensitive_text(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for VenueMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}
