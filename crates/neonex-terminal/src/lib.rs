#![no_std]

use core::marker::PhantomData;

use bevy::prelude::*;
use ratatui::{Terminal, prelude::Backend};

#[derive(Deref, DerefMut)]
pub struct RatatuiContext<Ctx: TerminalContext<B>, B: Backend + 'static> {
    #[deref]
    pub context: Ctx,
    pub backend: PhantomData<B>,
}

impl<Ctx: TerminalContext<B>, B: Backend + 'static> Drop for RatatuiContext<Ctx, B> {
    fn drop(&mut self) {
        if let Err(err) = Ctx::restore() {
            error!("Failed to restore terminal: {}", err);
        }
    }
}

impl<Ctx: TerminalContext<B>, B: Backend + 'static> RatatuiContext<Ctx, B> {
    pub fn init(context: Ctx) -> Self {
        Self {
            context,
            backend: PhantomData,
        }
    }

    pub fn restore() -> Result<()> {
        Ctx::restore()
    }
}

/// Trait for types that implement lifecycle functions for initializing a terminal context and
/// restoring the terminal state after exiting. Implementors must also use their implementation of
/// the `configure_plugin_group()` function to add any systems, resources, events, etcetera
/// necessary for the functioning of its associated Ratatui backend or its particular
/// functionality.
pub trait TerminalContext<T: Backend + 'static>:
    Sized + core::ops::Deref<Target = Terminal<T>> + 'static
{
    /// Initialize the terminal context.
    fn init() -> Result<Self>;

    /// Restore the terminal to its normal state after exiting.
    fn restore() -> Result<()>;

    fn add_needed_plugins(app: &mut App);
}
