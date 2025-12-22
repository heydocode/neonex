#![no_std]

use core::marker::PhantomData;

use bevy::{
    app::{App, AppExit, PostStartup, Update},
    ecs::{
        error,
        resource::Resource,
        system::{NonSendMut, ResMut},
    },
    platform::prelude::{String, vec::Vec},
    prelude::{Deref, DerefMut},
};
use neonex_mockplatform::MockPlatform;
use neonex_platform::{NeoNexConfig, NeoNexPlatform};
use neonex_shared::{NeoNexStartupConfig, NeoNexStartupConfigSet};
use neonex_terminal::{RatatuiContext, TerminalContext};
use ratatui::prelude::Backend;

cfg_if::cfg_if! {
    if #[cfg(feature = "desktop")] {
        pub use neonex_desktop::DesktopPlatform as ActivePlatform;
    } else if #[cfg(feature = "mobile")] {
        pub use neonex_mobile::MobilePlatform as ActivePlatform;
    } else if #[cfg(feature = "web")] {
        pub use neonex_web::WebPlatform as ActivePlatform;
    }
    else if #[cfg(feature = "uefi")] {
        pub use neonex_uefi::UefiPlatform as ActivePlatform;
    }
    else if #[cfg(feature = "embedded")] {
        pub use neonex_embedded::EmbeddedPlatform as ActivePlatform;
    }
    else {
        pub use neonex_mockplatform::MockPlatform as ActivePlatform;
    }
}

pub struct DefaultNeoNexConfig;

impl NeoNexConfig for DefaultNeoNexConfig {
    type Platform = ActivePlatform;
}

///
pub struct NeoNexInstance<CONFIG: NeoNexConfig = DefaultNeoNexConfig> {
    pub _config: PhantomData<CONFIG>,
    pub app: App,
}

#[derive(Resource, Deref, DerefMut)]
pub struct SCSWrapper<CONFIG: NeoNexConfig = DefaultNeoNexConfig>(
    #[deref] pub NeoNexStartupConfigSet,
    pub PhantomData<CONFIG>,
);

impl<CONFIG: NeoNexConfig> From<NeoNexStartupConfigSet> for SCSWrapper<CONFIG> {
    fn from(value: NeoNexStartupConfigSet) -> Self {
        Self(value, PhantomData)
    }
}

impl<CONFIG: NeoNexConfig> Drop for SCSWrapper<CONFIG> {
    fn drop(&mut self) {
        // Update the NeoNexConfig before exiting bevy!
        let _ = CONFIG::Platform::update_startup_config(self.0.clone());
    }
}

impl NeoNexInstance<DefaultNeoNexConfig> {
    pub fn new() -> Self {
        let mut app = App::new();

        let startup_config_set = ActivePlatform::retrieve_startup_config();
        // Insert the resource into bevy_ECS in order to modify it, and save the modified one into bevy when needed.
        let resource: SCSWrapper = startup_config_set.clone().into();
        app.insert_resource(resource);
        app.add_systems(PostStartup, Self::add_scs);

        Self::setup_bevy(&mut app, startup_config_set);

        Self {
            _config: PhantomData::<DefaultNeoNexConfig>,
            app: app,
        }
    }
}

impl<CONFIG: NeoNexConfig> NeoNexInstance<CONFIG> {
    fn add_scs(mut scs: ResMut<SCSWrapper<CONFIG>>) {
        scs.values
            .insert(NeoNexStartupConfig::Bla(String::from("BlaBlabllalalaa")));
        scs.values.insert(NeoNexStartupConfig::Test1(19555));
    }

    /// Inits a NeoNex Instance from a config struct.
    /// A NeoNexConfig item should be present in the context, or specified manually:
    /// let mut instance: NeoNexInstance<CustomNeoNexConfig> = NeoNexInstance::new_with_config();
    pub fn new_with_config() -> Self {
        let mut app = App::new();

        let startup_config_set = ActivePlatform::retrieve_startup_config();
        // Insert the resource into bevy_ECS in order to modify it, and save the modified one into bevy when needed.
        let resource: SCSWrapper<CONFIG> = startup_config_set.clone().into();
        app.insert_resource(resource);
        app.add_systems(PostStartup, Self::add_scs);

        Self::setup_bevy(&mut app, startup_config_set);

        Self {
            _config: PhantomData::<CONFIG>,
            app: app,
        }
    }

    /// Isn't intended to be public: wrapper around internal bevy init
    fn setup_bevy(app: &mut App, startup_config_set: NeoNexStartupConfigSet) {
        CONFIG::Platform::setup_bevy::<CONFIG>(app, startup_config_set)
            .expect("Unable to setup platform_specific bevy");
    }

    /// Runs the NeoNex runtime: Launches bevy ECS, inits the window/terminal, etc.
    pub fn run(&mut self) -> AppExit {
        self.app.run()
    }
}

/// Startup Status Messages
pub struct SSM {
    messages: Vec<String>,
}

/// Allow for easy Ctx access within bevy ECS while being default
pub type DefaultRatatuiContext = RatatuiContext<
    <ActivePlatform as NeoNexPlatform>::RatatuiContextGenerics,
    <ActivePlatform as NeoNexPlatform>::RatatuiContextBackend,
>;
