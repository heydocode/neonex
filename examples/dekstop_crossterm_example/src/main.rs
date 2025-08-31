use agnostic_logic::DemoPlugin;
use bevy::{app::Update, ecs::{error::BevyError, system::NonSendMut}};
use neonex_core::{ActivePlatform, DefaultNeoNexConfig, NeoNexInstance};
use neonex_platform::NeoNexPlatform;
use neonex_terminal::RatatuiContext;
use ratatui::{layout::{Constraint, Flex, Layout, Rect}, style::Stylize, widgets::{Block, Clear, Paragraph, Wrap}, Frame};

fn main() {
    let mut instance: NeoNexInstance = NeoNexInstance::new();
    // DemoPlugin - a ratatui set of animated widgets setup
    instance.app.add_plugins(DemoPlugin);
    instance.run();
}

