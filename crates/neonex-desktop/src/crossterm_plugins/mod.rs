use bevy::app::Plugin;

use crate::crossterm_plugins::{error::ErrorPlugin, event::EventPlugin, kitty::KittyPlugin, mouse::MousePlugin};

mod mouse;
mod kitty;
mod event;
mod error;

pub struct CrosstermPlugins;

impl Plugin for CrosstermPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            ErrorPlugin,
            EventPlugin::default(),
            KittyPlugin,
            MousePlugin
        ));
    }
}