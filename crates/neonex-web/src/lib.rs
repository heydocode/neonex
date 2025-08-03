use bevy::app::App;
use neonex_platform::NeoNexPlatform;

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

    const ADVICE_NATIVE_TERMINAL: bool = false;

    fn setup_bevy(app: &mut App, startup_config_set: neonex_platform::NeoNexStartupConfigSet) {
        // TODO
    }

    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType {
        // TODO Use [gloo_storage](https://docs.rs/gloo-storage/latest/gloo_storage/)
        //  for web startup configs!
    }

    fn retrieve_startup_config() -> neonex_platform::NeoNexStartupConfigSet {
        todo!()
    }

    fn update_startup_config(sc: neonex_platform::NeoNexStartupConfigSet) -> serde_json::Result<()> {
        todo!()
    }
}