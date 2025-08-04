use std::{
    env::temp_dir,
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use bevy::prelude::PluginGroup;
use bevy::{
    DefaultPlugins,
    app::App,
    render::texture::ImagePlugin,
    utils::default,
    window::{Window, WindowPlugin},
};
use neonex_platform::{NeoNexConfig, NeoNexPlatform, NeoNexStartupConfigSet};

pub struct DesktopPlatform;

impl NeoNexPlatform for DesktopPlatform {
    cfg_if::cfg_if! {if #[cfg(target_os = "windows")] {
        const PLATFORM: &'static str = "Windows";
    } else if #[cfg(target_os = "linux")] {
        const PLATFORM: &'static str = "Linux";
    } else if #[cfg(target_os = "macos")] {
        const PLATFORM: &'static str = "MacOS";
    } else {
        const PLATFORM: &'static str = "Desktop - Unknown OS";
    }}

    type StartupConfigRetrieveKeyType = PathBuf;

    const ADVICE_NATIVE_TERMINAL: bool = true;

    fn setup_bevy<CONFIG: NeoNexConfig>(
        app: &mut App,
        startup_config_set: neonex_platform::NeoNexStartupConfigSet,
    ) {
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: CONFIG::WINDOW_NAME.to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        );
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
                file.set_len(0).expect("Unable to clear the file once the data has been corrupted");
            }
            return NeoNexStartupConfigSet::default();
        }
    }

    type UpdateResult = serde_json::Result<()>;

    fn update_startup_config(
        startup_config_set: neonex_platform::NeoNexStartupConfigSet,
    ) -> Self::UpdateResult {
        let path = Self::retrieve_startup_config_key();
        let mut options = OpenOptions::new();
        options.create(true).write(true);
        let mut file = options.open(path).expect("Unable to open temp file");
        file.write_all(serde_json::to_string(&startup_config_set)?.as_bytes())
            .expect("Unable to write to the temp file");
        Ok(())
    }
}
