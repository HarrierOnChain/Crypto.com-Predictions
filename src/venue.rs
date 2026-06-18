//! Crypto.com Predictions venue metadata.
//!
//! The execution core, risk layer, and strategy implementations live in the
//! shared engine crate. This module just describes the venue this binary targets.

/// Display name of this venue.
pub const NAME: &str = "Crypto.com Predictions";

/// Venue category.
pub const VENUE_TYPE: &str = "Crypto-integrated";

/// Strategies this venue runs on the shared engine.
pub const STRATEGIES: &[&str] = &[
    "BTC 5m / 15m / 1hr Arbitrage",
    "Direction Hunting",
];
