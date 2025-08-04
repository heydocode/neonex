use std::collections::HashSet;
use std::{io::Result, marker::PhantomData};

use bevy::app::{App, AppExit};
use bevy::ecs::resource::Resource;
use neonex_macros::generate_32char_seed;
use serde::{Deserialize, Serialize};

/// `NeoNexConfig` is a trait, containing all of the NeoNex static customizations.
/// A static customization stated above means "constant", "function", "type".
///
///
/// This trait has, for each of its items, a default implementation, so that
/// the final use would be able to customize only certain items, while keeping
/// the default implementation for the rest of the items, by keeping the user-side
/// code sane.
///
///
/// The huge benefits for NeoNex of storing static Rust items into NeoNexConfig are many:
///
///
/// - Customize NeoNex in a user workspace. This is possible by following these steps:
///
///     - Define an union struct (you can define one with fields, but with the only purpose
///         to store NeoNex required customizable items, this is useless, but again, if you
///         repurpose the struct for other tasks too, you're good to go).
///         ```rust
///         struct ExampleCustomizations;
///         ```
///
///     - Implement the trait NeoNexConfig for the struct we have defined, by customizing
///         the items we want to (in our example, we'll change the name of the assistant):
///         ```rust
///         impl NeoNexConfig for ExampleCustomizations {
///             const NAME: &'static str = "Bob";
///         }
///         ```
///
///     - Inject the struct that stores our NAME customization into NeoNex:
///         ```rust
///         let mut instance = NeoNexInstance::new_from(ExampleCustomizations);
///         // Match the exit state to indicate if NeoNex terminated because of an error or not
///         // This match is purely optional, you can write `instance.run();` instead, by ignoring
///         // the AppExit value that the function returns.
///         match instance.run() {
///             Success => println!("NeoNex has been terminated without errors"),
///             Error(e) => println!("NeoNex has been terminated with an error: {:?}", e)
///         }
///         ```
///
///
/// - Let the user to either ignore, or customize **some** items. Indeed, thank to the fact
///     that this trait implements a default implementation for all ts items, the user can
///     define a custom implementation of some items, while letting all other items with its
///     default NeoNex implementation. Obviously, users can still customize every single item
///     of the trait!
pub trait NeoNexConfig: Sized {
    const WINDOW_NAME: &'static str = "NeoNex";
    const NAME: &'static str = "NeoNex";
}

pub struct DefaultNeoNexConfig;

impl NeoNexConfig for DefaultNeoNexConfig {}

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

    const STARTUP_CONFIG_RANDOM_KEY: &'static str =
        concat!(generate_32char_seed!(), "-neonex-startup-config.json");

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
    fn setup_bevy<CONFIG: NeoNexConfig>(app: &mut App, startup_config_set: NeoNexStartupConfigSet);
    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType;
    /// Retrieve a startup config (if exists). If doesn't exist, it outputs None.
    fn retrieve_startup_config() -> NeoNexStartupConfigSet;
    type UpdateResult;
    /// Update the startup config at a specified location.
    fn update_startup_config(sc: NeoNexStartupConfigSet) -> Self::UpdateResult;
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
#[derive(Debug, Serialize, Deserialize, Resource, Clone, PartialEq, Eq, Hash, Default)]
pub struct NeoNexStartupConfigSet {
    pub values: Vec<NeoNexStartupConfig>,
}

/// Variants of this enum may have different parameters, and types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum NeoNexStartupConfig {
    NativeTerminal(bool)
}
