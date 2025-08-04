use std::marker::PhantomData;

use bevy::app::{App, AppExit};
use neonex_platform::{NeoNexConfig, NeoNexPlatform, NeoNexStartupConfigSet};

cfg_if::cfg_if! {
    if #[cfg(feature = "desktop")] {
        pub use neonex_desktop::DesktopPlatform as ActivePlatform;
    } else if #[cfg(feature = "mobile")] {
        pub use neonex_mobile::MobilePlatform as ActivePlatform;
    } else if #[cfg(feature = "web")] {
        pub use neonex_web::WebPlatform as ActivePlatform;
    }
    else {
        pub use neonex_mockplatform::MockPlatform as ActivePlatform;
    }
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

        let startup_config_set = ActivePlatform::retrieve_startup_config();
        // Insert the resource into bevy_ECS in order to modify it, and save the modified one into bevy when needed.
        app.insert_resource(startup_config_set.clone());

        Self::setup_bevy(&mut app, startup_config_set);

        Self {
            _marker: PhantomData::<CONFIG>,
            app: app,
        }
    }

    /// Isn't intended to be public: wrapper around internal bevy init
    fn setup_bevy(app: &mut App, startup_config_set: NeoNexStartupConfigSet) {
        ActivePlatform::setup_bevy::<CONFIG>(app, startup_config_set);
    }

    /// Runs the NeoNex runtime: Launches bevy ECS, inits the window/terminal, etc.
    pub fn run(&mut self) -> AppExit {
        self.app.run()
    }
}
