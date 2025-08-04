use bevy::{app::{App, PluginGroup, Update}, ecs::system::Res, log::{info, warn}, utils::default, window::{Window, WindowPlugin}, DefaultPlugins};
use gloo_storage::{LocalStorage, Storage};
use neonex_platform::{NeoNexConfig, NeoNexPlatform, NeoNexStartupConfigSet};
use serde_json::Error;

pub struct WebPlatform;

fn debug_sc(sc: Res<NeoNexStartupConfigSet>) {
    info!("{:?}", sc);
    let path = WebPlatform::retrieve_startup_config_key();
    warn!("{:?}", path);
}

impl NeoNexPlatform for WebPlatform {
    cfg_if::cfg_if! {
        if #[cfg(target_family = "wasm")] {
            const PLATFORM: &'static str = "WASM";
        } else {
            const PLATFORM: &'static str = "Web - not WASM";
        }
    }

    type StartupConfigRetrieveKeyType = &'static str;

    const ADVICE_NATIVE_TERMINAL: bool = false;

    fn setup_bevy<CONFIG: NeoNexConfig>(
        app: &mut App,
        startup_config_set: neonex_platform::NeoNexStartupConfigSet,
    ) {
        let sc = startup_config_set.clone();
        app.add_systems(Update, debug_sc);
        app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
           primary_window: Some(Window {
               title: CONFIG::WINDOW_NAME.to_string(),
               canvas: None,
               fit_canvas_to_parent: true,
               prevent_default_event_handling: false,
               visible: true,
               ..default()
           }),
           ..default()
           })
    );
    }

    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType {
        Self::STARTUP_CONFIG_RANDOM_KEY
    }

    fn retrieve_startup_config() -> neonex_platform::NeoNexStartupConfigSet {
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

    fn update_startup_config(
        sc: neonex_platform::NeoNexStartupConfigSet,
    ) -> Self::UpdateResult {
        let key = Self::retrieve_startup_config_key();
        LocalStorage::set(key, &sc)
    }
}
