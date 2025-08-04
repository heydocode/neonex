use std::{env::temp_dir, path::PathBuf};

use bevy::app::App;
use neonex_platform::{NeoNexConfig, NeoNexPlatform, NeoNexStartupConfigSet};

pub struct MockPlatform;

impl NeoNexPlatform for MockPlatform {
    const PLATFORM: &'static str = "MockPlatform - Passing Compilation Only";

    type StartupConfigRetrieveKeyType = PathBuf;

    const ADVICE_NATIVE_TERMINAL: bool = true;

    fn setup_bevy<CONFIG: NeoNexConfig>(
        app: &mut App,
        startup_config_set: neonex_platform::NeoNexStartupConfigSet,
    ) {
        panic!("MockPlatform - Not intended as a runtime platform!");
    }

    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType {
        panic!("MockPlatform - Not intended as a runtime platform!");
    }

    fn retrieve_startup_config() -> NeoNexStartupConfigSet {
        panic!("MockPlatform - Not intended as a runtime platform!");
    }

    type UpdateResult = ();

    fn update_startup_config(
        startup_config_set: neonex_platform::NeoNexStartupConfigSet,
    ) -> Self::UpdateResult {
        panic!("MockPlatform - Not intended as a runtime platform!");
    }
}
