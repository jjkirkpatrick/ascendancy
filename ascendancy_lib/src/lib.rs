//! Connects all of the TEMPLATE game logic.
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
// Often exceeded by queries
#![allow(clippy::type_complexity)]

use bevy::prelude::*;

/// Ui
pub mod Ui;
/// Units module
pub mod agent;
/// Asset management module
pub mod asset_management;
/// Enum iterator module
pub mod enum_iter;
/// Faction module
pub mod faction;
/// Graphics module
pub mod graphics;
/// Items module
pub mod items;
///Asset loading
pub mod loading;
/// Menu manager
pub mod menu;
/// Player interactions module
pub mod player_interactions;
/// Solar system module
pub mod solar_system;
/// structures
pub mod structures;
/// world generation module
pub mod world_gen;
/// define game states
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    /// During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    /// Here the menu is drawn and waiting for player interaction
    Menu,
    ///World generation states
    WorldGenPreGenerate,
    /// The world is being generated.
    WorldGenerating,
    /// The world is being simulated to let it stabilize.
    WorldGenBurningIn,
    /// The world has been generated.
    WorldGenPostGenerate,

    /// During this State the actual game logic is executed
    Playing,
}
