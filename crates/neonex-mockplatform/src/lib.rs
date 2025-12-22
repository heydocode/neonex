use bevy::prelude::{Deref, DerefMut};
use neonex_platform::NeoNexPlatform;
use neonex_shared::NeoNexStartupConfigSet;
use neonex_terminal::TerminalContext;
use ratatui::{backend::TestBackend, prelude::Backend, Terminal};

pub struct MockPlatform;

impl NeoNexPlatform for MockPlatform {
    const PLATFORM: &'static str = "MockPlatform // Should never be chosen at runtime";

    type RatatuiContextBackend = TestBackend;

    type RatatuiContextGenerics = MockContext;

    type StartupConfigRetrieveKeyType = ();

    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType {
        panic!("MockPlatform should never be chosen at runtime! Its only purpose is to provide a full Rust-Analyzer support within NeoNex");
    }

    fn retrieve_startup_config() -> NeoNexStartupConfigSet {
        panic!("MockPlatform should never be chosen at runtime! Its only purpose is to provide a full Rust-Analyzer support within NeoNex");
    }

    type UpdateResult = ();

    fn update_startup_config(sc: NeoNexStartupConfigSet) -> Self::UpdateResult {
        panic!("MockPlatform should never be chosen at runtime! Its only purpose is to provide a full Rust-Analyzer support within NeoNex");
    }

    fn setup_bevy<CONFIG: neonex_platform::NeoNexConfig>(
        app: &mut bevy::app::App,
        startup_config_set: NeoNexStartupConfigSet,
    ) -> core::result::Result<(), bevy::ecs::error::BevyError> {
        panic!("MockPlatform should never be chosen at runtime! Its only purpose is to provide a full Rust-Analyzer support within NeoNex");
    }
}

#[derive(Deref, DerefMut)]
pub struct MockContext(pub Terminal<TestBackend>);

impl TerminalContext<TestBackend> for MockContext {
    fn init() -> bevy::ecs::error::Result<Self> {
        panic!("MockPlatform should never be chosen at runtime! Its only purpose is to provide a full Rust-Analyzer support within NeoNex");
    }

    fn restore() -> bevy::ecs::error::Result<()> {
        panic!("MockPlatform should never be chosen at runtime! Its only purpose is to provide a full Rust-Analyzer support within NeoNex");
    }

    fn add_needed_plugins(app: &mut bevy::app::App) {
        panic!("MockPlatform should never be chosen at runtime! Its only purpose is to provide a full Rust-Analyzer support within NeoNex");
    }
}