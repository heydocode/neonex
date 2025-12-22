use agnostic_logic::DemoPlugin;
use neonex_core::{ActivePlatform, DefaultNeoNexConfig, NeoNexInstance};
use bevy::prelude::*;
use neonex_platform::NeoNexPlatform;
use neonex_terminal::RatatuiContext;

#[bevy_main]
fn main() {
    let mut instance: NeoNexInstance = NeoNexInstance::new();
    // DemoPlugin - a ratatui set of animated widgets setup
    instance.app.add_plugins(DemoPlugin);
    instance.run();
}