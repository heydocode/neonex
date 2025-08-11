use bevy::{
    DefaultPlugins,
    app::App,
    ecs::error::BevyError,
    prelude::{Deref, DerefMut},
    render::texture::ImagePlugin,
    utils::default,
    window::{Window, WindowPlugin},
};
use bevy::{
    app::{ScheduleRunnerPlugin, TaskPoolPlugin},
    prelude::PluginGroup,
    time::TimePlugin,
};
use neonex_platform::{NeoNexConfig, NeoNexPlatform};
use neonex_shared::NeoNexStartupConfigSet;
use neonex_terminal::TerminalContext;
use ratatui::Terminal;

use crate::ratatui_uefi::UefiOutputBackend;
mod ratatui_uefi;
mod terminput_uefi;

pub struct UefiPlatform;

impl NeoNexPlatform for UefiPlatform {
    const PLATFORM: &'static str = "UEFI";

    type StartupConfigRetrieveKeyType = ();

    fn setup_bevy<CONFIG: NeoNexConfig>(
        app: &mut App,
        startup_config_set: neonex_shared::NeoNexStartupConfigSet
    ) -> Result<(), BevyError> {
        app.add_plugins((
            TaskPoolPlugin::default(),
            TimePlugin::default(),
            ScheduleRunnerPlugin::default(),
        ));
        Ok(())
    }

    // UEFI Doesn't support startup config! No filesystem... Maybe later..? :)
    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType {}
    fn retrieve_startup_config() -> NeoNexStartupConfigSet {
        NeoNexStartupConfigSet::default()
    }
    type UpdateResult = ();
    fn update_startup_config(
        startup_config_set: NeoNexStartupConfigSet,
    ) -> Self::UpdateResult {
    }

    type RatatuiContextBackend = UefiOutputBackend;
    type RatatuiContextGenerics = UefiTerminalContext;
}

#[derive(Deref, DerefMut)]
pub struct UefiTerminalContext(Terminal<UefiOutputBackend>);

impl TerminalContext<UefiOutputBackend> for UefiTerminalContext {
    fn init() -> bevy::ecs::error::Result<Self> {
        todo!()
    }

    fn restore() -> bevy::ecs::error::Result<()> {
        todo!()
    }

    fn add_needed_plugins(app: &mut App) {
        todo!()
    }
}
