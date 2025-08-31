#![no_std]

use core::hash::Hasher;

use bevy::{ecs::resource::Resource, platform::{collections::HashSet, prelude::String}};
use serde::{Deserialize, Serialize};
use core::hash::Hash;

/// At launch, before that NeoNex starts its instance, it retrieves a Startup Config,
/// located differently in each platform (Desktop, Mobile, Web).
///
/// Following this Startup Config, the app can be customized even more, while remaining
/// only one binary, and not requiring a reboot.
///
/// On Desktop and Mobile, this would be saved in a persistent temp file.
/// On Web, this would be saved in a localStorage location, that can be accessed with a key from Rust
/// (and js if you want for example to do a launcher in HTML/CSS/JS that launches NeoNex with a startup config).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct NeoNexStartupConfigSet {
    pub values: HashSet<NeoNexStartupConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NeoNexStartupConfig {
    NativeTerminal(bool),
    Test1(u16),
    Bla(String),
}


impl PartialEq for NeoNexStartupConfig {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
impl Eq for NeoNexStartupConfig {}

impl Hash for NeoNexStartupConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}