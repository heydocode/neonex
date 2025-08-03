use bevy::{app::App, ecs::resource::Resource};
use neonex_macros::generate_32char_seed;
use serde::{Serialize, Deserialize};

/// Startup Status Messages
pub struct SSM {
    messages: Vec<String>,
}

/// Platform-specific data, that make cross-platform
pub trait NeoNexPlatform {
    /// The name of the current platform the trait has been implemented for.
    const PLATFORM: &'static str;

    /// Can be a type to define key to access the Startup Config on the web, and can be a type to define a path to access it on desktop/mobile
    type StartupConfigRetrieveKeyType;

    const STARTUP_CONFIG_RANDOM_SEED: &'static str = concat!(generate_32char_seed!(), "-neonex-startup-config.json");

    /// Advices what kind of terminal to init:
    /// - ADVICE_NATIVE_TERMINAL: true => If the binary hasn't been executed with arguments that require the app to init a virtual terminal, a native terminal tries to init, on failure, the app restarts and inits virtual terminal (via launching the binary again but with an argument to init a virtual terminal).
    /// - ADVICE_NATIVE_TERMINAL: false => A window inits, with a virtual terminal in it.
    ///
    /// This advice is platform-specific, as only desktop can init in a native terminal.
    /// Also, via arguments, the user can define to init a virtual, or a native terminal.
    /// If the constant value is false and an argument requires NeoNex to init a native terminal, the app inits a virtual terminal
    /// and notices the user on startup that his requirement haven't been executed, with a descriptive message (Startup Status Messages - SSM).
    const ADVICE_NATIVE_TERMINAL: bool;

    /// Required setup:
    /// - Init the terminal (whether native or virtual)
    /// - Inject all stuff needed for other platform-specific stuff. Example:
    ///     on desktop/mobile, inject a resource to know where
    fn setup_bevy(app: &mut App, startup_config_set: NeoNexStartupConfigSet);
    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType;
    /// Retrieve a startup config (if exists). If doesn't exist, it outputs None.
    fn retrieve_startup_config() -> NeoNexStartupConfigSet;
    /// Update the startup config at a specified location.
    fn update_startup_config(sc: NeoNexStartupConfigSet) -> serde_json::Result<()>;
}

/// At launch, before that NeoNex starts its instance, it retrieves a Startup Config,
/// located differently in each platform (Desktop, Mobile, Web).
///
/// Following this Startup Config, the app can be customized even more, while remaining
/// only one binary, and not requiring a reboot.
///
/// On Desktop and Mobile, this would be saved in a persistent temp file.
/// On Web, this would be saved in a localStorage location, that can be accessed with a key from Rust
/// (and js if you want for example to do a launcher in HTML/CSS/JS that launches NeoNex with a startup config).
#[derive(Serialize, Deserialize, Resource, Clone)]
pub struct NeoNexStartupConfigSet {
    pub values: Vec<NeoNexStartupConfig>,
}

/// Variants of this enum may have different parameters, and types
#[derive(Serialize, Deserialize, Clone)]
pub enum NeoNexStartupConfig {
}
