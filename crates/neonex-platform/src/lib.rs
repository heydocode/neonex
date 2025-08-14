#![no_std]

use bevy::app::{App, AppExit};
use bevy::ecs::error::BevyError;
use bevy::ecs::resource::Resource;
use bevy::platform::collections::HashSet;
use bevy::platform::prelude::String;
use bevy::platform::prelude::vec::Vec;
use neonex_macros::generate_32char_seed;
use neonex_shared::NeoNexStartupConfigSet;
use neonex_terminal::TerminalContext;
use ratatui::prelude::Backend;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use core::hash::{Hash, Hasher};

/// Platform-specific data, that make cross-platform
pub trait NeoNexPlatform {
    /// The name of the current platform the trait has been implemented for.
    const PLATFORM: &'static str;

    type RatatuiContextBackend: Backend + 'static;
    type RatatuiContextGenerics: TerminalContext<Self::RatatuiContextBackend>;

    /// Can be a type to define key to access the Startup Config on the web, and can be a type to define a path to access it on desktop/mobile
    type StartupConfigRetrieveKeyType;

    const STARTUP_CONFIG_RANDOM_KEY: &'static str =
        concat!(generate_32char_seed!(), "-neonex-startup-config.json");

    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType;
    /// Retrieve a startup config (if exists). If doesn't exist, it outputs None.
    fn retrieve_startup_config() -> NeoNexStartupConfigSet;
    type UpdateResult;
    /// Update the startup config at a specified location.
    fn update_startup_config(sc: NeoNexStartupConfigSet) -> Self::UpdateResult;
    fn setup_bevy<CONFIG: NeoNexConfig>(
        app: &mut App,
        startup_config_set: NeoNexStartupConfigSet,
    ) -> core::result::Result<(), BevyError>;
}

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
pub trait NeoNexConfig: Sized + Send + Sync + 'static {
    /// Allow to implement a custom platform within NeoNex
    type Platform: NeoNexPlatform;
    #[cfg(feature = "desktop-hybrid-contexts")]
    const DESKTOP_HYBRID_SOFTATUI: bool = true;
    const WINDOW_NAME: &'static str = "NeoNex";
    const NAME: &'static str = "NeoNex";
    const DEFAULT_BACKGROUND_COLOR: Color = Color::Black;
    const DEFAULT_FOREGROUND_COLOR: Color = Color::White;
}
