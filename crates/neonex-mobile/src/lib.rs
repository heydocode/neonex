use std::{
    env::temp_dir,
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use bevy::{
    app::App, render::texture::ImagePlugin, utils::default, window::{MonitorSelection, Window, WindowMode, WindowPlugin}, DefaultPlugins
};
use bevy::{
    ecs::error::BevyError,
    prelude::{Deref, DerefMut, PluginGroup},
};
use neonex_platform::{NeoNexConfig, NeoNexPlatform,};
use neonex_shared::NeoNexStartupConfigSet;
use neonex_terminal::{RatatuiContext, TerminalContext};
use ratatui::Terminal;
use soft_ratatui::SoftBackend;

mod windowed_plugins;

pub struct MobilePlatform;

impl NeoNexPlatform for MobilePlatform {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "android")] {
            const PLATFORM: &'static str = "Android";
        } else if #[cfg(target_os = "ios")] {
            const PLATFORM: &'static str = "iOS";
        } else {
            const PLATFORM: &'static str = "Mobile - Unknown OS";
        }
    }

    type StartupConfigRetrieveKeyType = PathBuf;

    fn setup_bevy<CONFIG: NeoNexConfig>(
        app: &mut App,
        startup_config_set: NeoNexStartupConfigSet,
    ) -> Result<(), BevyError> {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: CONFIG::WINDOW_NAME.to_string(),
                resizable: false,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                // on iOS, gestures must be enabled.
                // This doesn't work on Android
                #[cfg(target_os = "ios")]
                recognize_rotation_gesture: true,
                // Only has an effect on iOS
                #[cfg(target_os = "ios")]
                prefers_home_indicator_hidden: true,
                // Only has an effect on iOS
                #[cfg(target_os = "ios")]
                prefers_status_bar_hidden: true,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()));
        Ok(())
    }

    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType {
        let mut path = temp_dir();
        path.push(Self::STARTUP_CONFIG_RANDOM_KEY.to_string());
        path
    }

    fn retrieve_startup_config() -> NeoNexStartupConfigSet {
        let path = Self::retrieve_startup_config_key();
        let mut options = OpenOptions::new();
        options.create(true).write(true).read(true);
        let mut file = options.open(path).expect("Unable to open temp file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Unable to read to string from temp file");
        if let Ok(output) = serde_json::from_str(&buf) {
            return output;
        } else {
            if buf.len() > 0 {
                file.set_len(0)
                    .expect("Unable to clear the file once the data has been corrupted");
            }
            return NeoNexStartupConfigSet::default();
        }
    }

    type UpdateResult = serde_json::Result<()>;

    fn update_startup_config(
        startup_config_set: NeoNexStartupConfigSet,
    ) -> serde_json::Result<()> {
        let path = Self::retrieve_startup_config_key();
        let mut options = OpenOptions::new();
        options.create(true).write(true);
        let mut file = options.open(path).expect("Unable to open temp file");
        file.write_all(serde_json::to_string(&startup_config_set)?.as_bytes())
            .expect("Unable to write to the temp file");
        Ok(())
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
        app.insert_non_send_resource(RatatuiContext::init(SoftatuiContext::init().expect("Unable to init Mobile Softatui Context")));
    }
}
