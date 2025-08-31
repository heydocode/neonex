use bevy::{
    app::App,
    ecs::error::BevyError,
    prelude::{Deref, DerefMut},
};
use neonex_platform::{NeoNexConfig, NeoNexPlatform};
use neonex_shared::NeoNexStartupConfigSet;
use neonex_terminal::{RatatuiContext, TerminalContext};
use ratatui::Terminal;
use uefi::proto::console;

use crate::ratatui_uefi::UefiOutputBackend;
mod ratatui_uefi;
mod terminput_uefi;

pub struct UefiPlatform;

impl NeoNexPlatform for UefiPlatform {
    const PLATFORM: &'static str = "UEFI";

    type StartupConfigRetrieveKeyType = ();

    fn setup_bevy<CONFIG: NeoNexConfig>(
        app: &mut App,
        startup_config_set: neonex_shared::NeoNexStartupConfigSet,
    ) -> Result<(), BevyError> {
        app.set_runner(|mut app| {
            loop {
                app.update();
                if let Some(exit) = app.should_exit() {
                    return exit;
                }
            }
        });
        app.insert_non_send_resource(RatatuiContext::init(Self::RatatuiContextGenerics::init()?));
        Ok(())
    }

    // CAUTION UEFI Doesn't support startup config! No filesystem... Maybe later..? :)
    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType {}
    fn retrieve_startup_config() -> NeoNexStartupConfigSet {
        NeoNexStartupConfigSet::default()
    }
    type UpdateResult = ();
    fn update_startup_config(startup_config_set: NeoNexStartupConfigSet) -> Self::UpdateResult {}

    type RatatuiContextBackend = UefiOutputBackend;
    type RatatuiContextGenerics = UefiTerminalContext;
}

#[derive(Deref, DerefMut)]
pub struct UefiTerminalContext(Terminal<UefiOutputBackend>);

impl TerminalContext<UefiOutputBackend> for UefiTerminalContext {
    fn init() -> bevy::ecs::error::Result<Self> {
        let output_handle = uefi::boot::get_handle_for_protocol::<console::text::Output>()?;
        let output = uefi::boot::open_protocol_exclusive::<console::text::Output>(output_handle)?;
        let backend = UefiOutputBackend::new(output);
        let mut terminal = Terminal::new(backend)?;
        if let Err(e) = terminal.clear() {}
        Ok(Self(terminal))
    }

    fn restore() -> bevy::ecs::error::Result<()> {
        Ok(())
    }

    fn add_needed_plugins(app: &mut App) {}
}
