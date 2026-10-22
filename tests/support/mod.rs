// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

mod fixtures;

// Glob because each contract binary uses a different subset; an explicit list
// would be flagged unused in the binaries that do not name every item.
pub use fixtures::*;
