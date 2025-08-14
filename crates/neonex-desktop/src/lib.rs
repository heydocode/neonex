use std::{
    env::temp_dir,
    fs::OpenOptions,
    io::{stdout, Read, Stdout, Write},
    path::{Path, PathBuf},
};

#[cfg(feature = "hybrid-contexts")]
use bevy::asset::uuid::serde;
use bevy::{
    DefaultPlugins,
    app::App,
    render::texture::ImagePlugin,
    utils::default,
    window::{Window, WindowPlugin},
};
use bevy::{
    ecs::error::BevyError,
    prelude::{Deref, DerefMut, PluginGroup},
};
use neonex_platform::{NeoNexConfig, NeoNexPlatform};
use neonex_shared::NeoNexStartupConfigSet;
use neonex_terminal::{RatatuiContext, TerminalContext};
#[cfg(not(any(feature = "softatui", feature = "crossterm")))]
use ratatui::backend::TestBackend;
use ratatui::Terminal;
#[cfg(any(feature = "crossterm", feature = "hybrid-contexts"))]
use ratatui::{
    crossterm::{
        ExecutableCommand, cursor,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::CrosstermBackend,
};
#[cfg(any(feature = "softatui", feature = "hybrid-contexts"))]
use soft_ratatui::SoftBackend;

#[cfg(any(feature = "crossterm", feature = "hybrid-contexts"))]
mod crossterm_plugins;
#[cfg(any(feature = "softatui", feature = "hybrid-contexts"))]
mod windowed_plugins;

pub struct DesktopPlatform;

impl NeoNexPlatform for DesktopPlatform {
    cfg_if::cfg_if! {if #[cfg(target_os = "windows")] {
        const PLATFORM: &'static str = "Windows";
    } else if #[cfg(target_os = "linux")] {
        const PLATFORM: &'static str = "Linux";
    } else if #[cfg(target_os = "macos")] {
        const PLATFORM: &'static str = "MacOS";
    } else {
        const PLATFORM: &'static str = "Desktop - Unknown OS";
    }}

    type StartupConfigRetrieveKeyType = PathBuf;

    fn retrieve_startup_config_key() -> Self::StartupConfigRetrieveKeyType {
        let mut path = temp_dir();
        path.push(Self::STARTUP_CONFIG_RANDOM_KEY.to_string());
        path
    }

    fn retrieve_startup_config() -> NeoNexStartupConfigSet {
        let path = Self::retrieve_startup_config_key();
        let mut options = OpenOptions::new();
        options.create(true).write(true).read(true);
        let mut file = options.open(path).expect("Unable to open temp file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Unable to read to string from temp file");
        if let Ok(output) = serde_json::from_str(&buf) {
            return output;
        } else {
            if buf.len() > 0 {
                file.set_len(0)
                    .expect("Unable to clear the file once the data has been corrupted");
            }
            return NeoNexStartupConfigSet::default();
        }
    }

    type UpdateResult = serde_json::Result<()>;

    fn update_startup_config(
        startup_config_set: neonex_shared::NeoNexStartupConfigSet,
    ) -> Self::UpdateResult {
        let path = Self::retrieve_startup_config_key();
        let mut options = OpenOptions::new();
        options.create(true).write(true);
        let mut file = options.open(path).expect("Unable to open temp file");
        file.write_all(serde_json::to_string(&startup_config_set)?.as_bytes())
            .expect("Unable to write to the temp file");
        Ok(())
    }

    fn setup_bevy<CONFIG: NeoNexConfig>(
        app: &mut App,
        startup_config_set: NeoNexStartupConfigSet,
    ) -> Result<(), BevyError> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "hybrid-contexts")] {
                let context_instance: either::Either<SoftatuiContext, CrosstermContext> =
                if CONFIG::DESKTOP_HYBRID_SOFTATUI {
                    either::Left(SoftatuiContext::init()?)
                }
                else {
                    either::Right(CrosstermContext::init()?)
                };
            }
            else if #[cfg(feature = "softatui")] {
                let instance = SoftatuiContext::init()?;
                SoftatuiContext::add_needed_plugins(app);
            } else if #[cfg(feature = "crossterm")] {
                let instance = CrosstermContext::init()?;
                CrosstermContext::add_needed_plugins(app);
            } else {
                let instance = MockContext::init()?;
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "hybrid-contexts")] {
                /*  instance: SoftatuiContext */
                if let either::Either::Left(instance) = context_instance {
                    app.insert_non_send_resource(RatatuiContext::init(instance));
                    SoftatuiContext::add_needed_plugins(app);
                }
                /*  instance: CrosstermContext */
                else if let either::Either::Right(instance) = context_instance {
                    app.insert_non_send_resource(RatatuiContext::init(instance));
                    CrosstermContext::add_needed_plugins(app);
                }
            } else {
                app.insert_non_send_resource(RatatuiContext::init(instance));
            }
        }

        Ok(())
    }

    #[cfg(feature = "crossterm")]
    type RatatuiContextBackend = CrosstermBackend<Stdout>;
    #[cfg(feature = "crossterm")]
    type RatatuiContextGenerics = CrosstermContext;

    #[cfg(feature = "softatui")]
    type RatatuiContextBackend = SoftBackend;
    #[cfg(feature = "softatui")]
    type RatatuiContextGenerics = SoftatuiContext;
    #[cfg(not(any(feature = "softatui", feature = "crossterm")))]
    type RatatuiContextBackend = TestBackend;
    #[cfg(not(any(feature = "softatui", feature = "crossterm")))]
    type RatatuiContextGenerics = MockContext;
}

#[cfg(feature = "hybrid-contexts")]
pub struct SoftatuiDesktop;

#[cfg(feature = "hybrid-contexts")]
impl NeoNexPlatform for SoftatuiDesktop {
    cfg_if::cfg_if! {if #[cfg(target_os = "windows")] {
        const PLATFORM: &'static str = "Windows";
    } else if #[cfg(target_os = "linux")] {
        const PLATFORM: &'static str = "Linux";
    } else if #[cfg(target_os = "macos")] {
        const PLATFORM: &'static str = "MacOS";
    } else {
        const PLATFORM: &'static str = "Desktop - Unknown OS";
    }}

    fn retrieve_startup_config_key() -> PathBuf {
        let mut path = temp_dir();
        path.push(Self::STARTUP_CONFIG_RANDOM_KEY.to_string());
        path
    }

    fn retrieve_startup_config() -> NeoNexStartupConfigSet {
        let path: PathBuf = Self::retrieve_startup_config_key();
        let mut options = OpenOptions::new();
        options.create(true).write(true).read(true);
        let mut file = options.open(path).expect("Unable to open temp file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Unable to read to string from temp file");
        if let Ok(output) = serde_json::from_str(&buf) {
            return output;
        } else {
            if buf.len() > 0 {
                file.set_len(0)
                    .expect("Unable to clear the file once the data has been corrupted");
            }
            return NeoNexStartupConfigSet::default();
        }
    }

    fn update_startup_config(
        startup_config_set: neonex_shared::NeoNexStartupConfigSet,
    ) -> serde_json::Result<()> {
        let path: PathBuf = Self::retrieve_startup_config_key();
        let mut options = OpenOptions::new();
        options.create(true).write(true);
        let mut file = options.open(path).expect("Unable to open temp file");
        file.write_all(serde_json::to_string(&startup_config_set)?.as_bytes())
            .expect("Unable to write to the temp file");
        Ok(())
    }

    fn setup_bevy<CONFIG: NeoNexConfig>(
        app: &mut App,
        startup_config_set: NeoNexStartupConfigSet,
    ) -> Result<(), BevyError> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "hybrid-contexts")] {
                let context_instance: either::Either<SoftatuiContext, CrosstermContext> =
                if CONFIG::DESKTOP_HYBRID_SOFTATUI {
                    either::Left(SoftatuiContext::init()?)
                }
                else {
                    either::Right(CrosstermContext::init()?)
                };
            }
            else if #[cfg(feature = "softatui")] {
                let instance = SoftatuiContext::init()?;
                SoftatuiContext::add_needed_plugins(app);
            } else if #[cfg(feature = "crossterm")] {
                let instance = CrosstermContext::init()?;
                CrosstermContext::add_needed_plugins(app);
            } else {
                let instance = MockContext::init()?;
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "hybrid-contexts")] {
                /*  instance: SoftatuiContext */
                if let either::Either::Left(instance) = context_instance {
                    app.insert_non_send_resource(RatatuiContext::init(instance));
                    SoftatuiContext::add_needed_plugins(app);
                }
                /*  instance: CrosstermContext */
                else if let either::Either::Right(instance) = context_instance {
                    app.insert_non_send_resource(RatatuiContext::init(instance));
                    CrosstermContext::add_needed_plugins(app);
                }
            } else {
                app.insert_non_send_resource(RatatuiContext::init(instance));
            }
        }

        Ok(())
    }
    
    type RatatuiContextBackend = SoftBackend;
    
    type RatatuiContextGenerics = SoftatuiContext;
    
    type StartupConfigRetrieveKeyType = PathBuf;
    
    type UpdateResult = serde_json::Result<()>;
}

#[cfg(feature = "hybrid-contexts")]
pub struct CrosstermDesktop;

#[cfg(feature = "hybrid-contexts")]
impl NeoNexPlatform for CrosstermDesktop {
    cfg_if::cfg_if! {if #[cfg(target_os = "windows")] {
        const PLATFORM: &'static str = "Windows";
    } else if #[cfg(target_os = "linux")] {
        const PLATFORM: &'static str = "Linux";
    } else if #[cfg(target_os = "macos")] {
        const PLATFORM: &'static str = "MacOS";
    } else {
        const PLATFORM: &'static str = "Desktop - Unknown OS";
    }}

    fn retrieve_startup_config_key() -> PathBuf {
        let mut path = temp_dir();
        path.push(Self::STARTUP_CONFIG_RANDOM_KEY.to_string());
        path
    }

    fn retrieve_startup_config() -> NeoNexStartupConfigSet {
        let path: PathBuf = Self::retrieve_startup_config_key();
        let mut options = OpenOptions::new();
        options.create(true).write(true).read(true);
        let mut file = options.open(path).expect("Unable to open temp file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Unable to read to string from temp file");
        if let Ok(output) = serde_json::from_str(&buf) {
            return output;
        } else {
            if buf.len() > 0 {
                file.set_len(0)
                    .expect("Unable to clear the file once the data has been corrupted");
            }
            return NeoNexStartupConfigSet::default();
        }
    }

    fn update_startup_config(
        startup_config_set: neonex_shared::NeoNexStartupConfigSet,
    ) -> serde_json::Result<()> {
        let path: PathBuf = Self::retrieve_startup_config_key();
        let mut options = OpenOptions::new();
        options.create(true).write(true);
        let mut file = options.open(path).expect("Unable to open temp file");
        file.write_all(serde_json::to_string(&startup_config_set)?.as_bytes())
            .expect("Unable to write to the temp file");
        Ok(())
    }

    fn setup_bevy<CONFIG: NeoNexConfig>(
        app: &mut App,
        startup_config_set: NeoNexStartupConfigSet,
    ) -> Result<(), BevyError> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "hybrid-contexts")] {
                let context_instance: either::Either<SoftatuiContext, CrosstermContext> =
                if CONFIG::DESKTOP_HYBRID_SOFTATUI {
                    either::Left(SoftatuiContext::init()?)
                }
                else {
                    either::Right(CrosstermContext::init()?)
                };
            }
            else if #[cfg(feature = "softatui")] {
                let instance = SoftatuiContext::init()?;
                SoftatuiContext::add_needed_plugins(app);
            } else if #[cfg(feature = "crossterm")] {
                let instance = CrosstermContext::init()?;
                CrosstermContext::add_needed_plugins(app);
            } else {
                let instance = MockContext::init()?;
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "hybrid-contexts")] {
                /*  instance: SoftatuiContext */
                if let either::Either::Left(instance) = context_instance {
                    app.insert_non_send_resource(RatatuiContext::init(instance));
                    SoftatuiContext::add_needed_plugins(app);
                }
                /*  instance: CrosstermContext */
                else if let either::Either::Right(instance) = context_instance {
                    app.insert_non_send_resource(RatatuiContext::init(instance));
                    CrosstermContext::add_needed_plugins(app);
                }
            } else {
                app.insert_non_send_resource(RatatuiContext::init(instance));
            }
        }

        Ok(())
    }
    
    type RatatuiContextBackend = CrosstermBackend<Stdout>;
    
    type RatatuiContextGenerics = CrosstermContext;
    
    type StartupConfigRetrieveKeyType = PathBuf;
    
    type UpdateResult = serde_json::Result<()>;
}

#[cfg(any(feature = "softatui", feature = "hybrid-contexts"))]
/// Ratatui context that will set up a window and render the ratatui buffer using a 2D texture,
/// instead of drawing to a terminal buffer.
#[derive(Deref, DerefMut)]
pub struct SoftatuiContext(Terminal<SoftBackend>);

#[cfg(any(feature = "softatui", feature = "hybrid-contexts"))]
impl TerminalContext<SoftBackend> for SoftatuiContext {
    fn init() -> Result<SoftatuiContext, BevyError> {
        let backend = SoftBackend::new_with_system_fonts(15, 15, 16);
        let terminal = Terminal::new(backend)?;
        Ok(Self(terminal))
    }

    fn restore() -> Result<(), BevyError> {
        Ok(())
    }

    fn add_needed_plugins(app: &mut App) {
        use std::time::Duration;

        use bevy::app::ScheduleRunnerPlugin;

        app.add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            windowed_plugins::WindowedPlugin,
        ));
    }
}

#[cfg(any(feature = "crossterm", feature = "hybrid-contexts"))]
#[derive(Deref, DerefMut)]
pub struct CrosstermContext(Terminal<CrosstermBackend<Stdout>>);

#[cfg(any(feature = "crossterm", feature = "hybrid-contexts"))]
impl TerminalContext<CrosstermBackend<Stdout>> for CrosstermContext {
    fn init() -> Result<Self, BevyError> {
        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self(terminal))
    }

    fn restore() -> Result<(), BevyError> {
        let mut stdout = stdout();
        stdout
            .execute(LeaveAlternateScreen)?
            .execute(cursor::Show)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn add_needed_plugins(app: &mut App) {
        // TODO
    }
}

#[cfg(not(any(feature = "softatui", feature = "crossterm")))]
#[derive(Deref, DerefMut)]
pub struct MockContext(pub Terminal<TestBackend>);

#[cfg(not(any(feature = "softatui", feature = "crossterm")))]
impl TerminalContext<TestBackend> for MockContext {
    fn init() -> bevy::ecs::error::Result<Self> {
        panic!(
            "In order to use hybrid-contexts feature on Desktop, make your own platform implementation while specifying Crossterm or Softatui backends!"
        );
    }

    fn restore() -> bevy::ecs::error::Result<()> {
        panic!(
            "In order to use hybrid-contexts feature on Desktop, make your own platform implementation while specifying Crossterm or Softatui backends!"
        );
    }

    fn add_needed_plugins(app: &mut bevy::app::App) {
        panic!(
            "In order to use hybrid-contexts feature on Desktop, make your own platform implementation while specifying Crossterm or Softatui backends!"
        );
    }
}
