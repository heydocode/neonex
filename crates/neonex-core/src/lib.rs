use std::{io::Result, marker::PhantomData};

use bevy::app::{App, AppExit};
use neonex_platform::{NeoNexPlatform, NeoNexStartupConfigSet};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "desktop")] {
        use neonex_desktop::DesktopPlatform as ActivePlatform;
    } else if #[cfg(feature = "mobile")] {
        use neonex_mobile::MobilePlatform as ActivePlatform;
    } else if #[cfg(feature = "web")] {
        use neonex_web::WebPlatform as ActivePlatform;
    }
    else {
        compile_error!("One of the three main platform features must be set!");
    }
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
pub trait NeoNexConfig: Sized {
    const NAME: &'static str = "NeoNex";
}

///
pub struct NeoNexInstance<CONFIG: NeoNexConfig> {
    _marker: PhantomData<CONFIG>,
    app: App,
}

impl<CONFIG: NeoNexConfig> NeoNexInstance<CONFIG> {
    /// Inits a NeoNex Instance from a config struct.
    pub fn new() -> Self {
        let mut app = App::new();

        let startup_config_set = <ActivePlatform as NeoNexPlatform>::retrieve_startup_config();
        // Insert the resource into bevy_ECS in order to modify it, and save the modified one into bevy when needed.
        app.insert_resource(startup_config_set.clone());

        <ActivePlatform as NeoNexPlatform>::setup_bevy(&mut app, startup_config_set);

        Self {
            _marker: PhantomData::<CONFIG>,
            app: app,
        }
    }

    /// Runs the NeoNex runtime: Launches bevy ECS, inits the window/terminal, etc.
    ///
    /// NOTE: accepting a direct ownership of the instance allows to concisely indicate
    //  that after the run function, nothing in the instance should be accessed
    pub fn run(mut self) -> AppExit {
        self.app.run()
    }
}

pub struct DefaultNeoNexConfig;

impl NeoNexConfig for DefaultNeoNexConfig {}
