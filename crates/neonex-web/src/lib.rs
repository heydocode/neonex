use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup, Update},
    ecs::{error::BevyError, system::Res},
    log::{info, warn},
    prelude::{Deref, DerefMut},
    utils::default,
    window::{Window, WindowPlugin},
};
use gloo_storage::{LocalStorage, Storage};
use neonex_platform::{NeoNexConfig, NeoNexPlatform};
use neonex_shared::NeoNexStartupConfigSet;
use neonex_terminal::TerminalContext;
use ratatui::Terminal;
use serde_json::Error;
use soft_ratatui::SoftBackend;

pub struct WebPlatform;

impl NeoNexPlatform for WebPlatform {
    cfg_if::cfg_if! {
        if #[cfg(target_family = "wasm")] {
            const PLATFORM: &'static str = "WASM";
        } else {
            const PLATFORM: &'static str = "Web - not WASM";
        }
    }

    type StartupConfigRetrieveKeyType = &'static str;

    fn setup_bevy<CONFIG: NeoNexConfig>(
        app: &mut App,
        startup_config_set: neonex_shared::NeoNexStartupConfigSet,
    ) -> Result<(), BevyError> {
        let sc = startup_config_set.clone();
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: CONFIG::WINDOW_NAME.to_string(),
                canvas: None,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                visible: true,
                ..default()
            }),
            ..default()
        }));
        Ok(())
    }

    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType {
        Self::STARTUP_CONFIG_RANDOM_KEY
    }

    fn retrieve_startup_config() -> NeoNexStartupConfigSet {
        let key = Self::retrieve_startup_config_key();
        if let Ok(sc) = LocalStorage::get(key) {
            return sc;
        } else {
            let default = NeoNexStartupConfigSet::default();
            Self::update_startup_config(default.clone()).expect("Unable to update the startup config LocalStorage to create or correct the corrupted data");
            return default;
        }
    }

    type UpdateResult = gloo_storage::Result<()>;

    fn update_startup_config(sc: neonex_shared::NeoNexStartupConfigSet) -> Self::UpdateResult {
        let key = Self::retrieve_startup_config_key();
        LocalStorage::set(key, &sc)
    }

    type RatatuiContextBackend = SoftBackend;

    type RatatuiContextGenerics = SoftatuiContext;
}

/// Ratatui context that will set up a window and render the ratatui buffer using a 2D texture,
/// instead of drawing to a terminal buffer.
#[derive(Deref, DerefMut)]
pub struct SoftatuiContext(Terminal<SoftBackend>);

impl TerminalContext<SoftBackend> for SoftatuiContext {
    fn init() -> Result<SoftatuiContext, BevyError> {
        let backend = SoftBackend::new_with_system_fonts(15, 15, 16);
        let terminal = Terminal::new(backend)?;
        Ok(Self(terminal))
    }

    fn restore() -> Result<(), BevyError> {
        Ok(())
    }

    fn add_needed_plugins(app: &mut App) {
        // TODO
    }
}
