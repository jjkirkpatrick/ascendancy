//! Connects all of the TEMPLATE game logic.
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
// Often exceeded by queries
#![allow(clippy::type_complexity)]

/// Enum iterator module
pub mod enum_iter;
/// Faction module
pub mod faction;
/// Graphics module
pub mod graphics;
/// Player interactions module
pub mod player_interactions;
/// Solar system module
pub mod solar_system;
/// Units module
pub mod units;
/// world generation module
pub mod world_gen;
