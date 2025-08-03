use std::{
    env::temp_dir,
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use bevy::app::App;
use neonex_platform::{NeoNexPlatform, NeoNexStartupConfigSet};

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

    const ADVICE_NATIVE_TERMINAL: bool = false;

    fn setup_bevy(app: &mut App, startup_config_set: neonex_platform::NeoNexStartupConfigSet) {
        // TODO
    }

    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType {
        let mut path = temp_dir();
        path.push(Self::STARTUP_CONFIG_RANDOM_SEED.to_string());
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
            return NeoNexStartupConfigSet { values: Vec::new() };
        }
    }

    fn update_startup_config(
        startup_config_set: neonex_platform::NeoNexStartupConfigSet,
    ) -> serde_json::Result<()> {
        let path = Self::retrieve_startup_config_key();
        let mut options = OpenOptions::new();
        options.create(true).write(true);
        let mut file = options.open(path).expect("Unable to open temp file");
        file.write_all(serde_json::to_string(&startup_config_set)?.as_bytes())
            .expect("Unable to write to the temp file");
        Ok(())
    }
}
