#![feature(uefi_std)]

use agnostic_logic::DemoPlugin;
use bevy::app::Update;
use bevy::ecs::error::BevyError;
use bevy::ecs::system::{Local, NonSendMut};
use neonex_core::{ActivePlatform, DefaultNeoNexConfig, NeoNexInstance};
use neonex_platform::{NeoNexConfig, NeoNexPlatform};
use neonex_core::DefaultRatatuiContext;
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use std::os::uefi as uefi_std;
use uefi::runtime::ResetType;
use uefi::{Handle, Status, boot};

/// Performs the necessary setup code for the `uefi` crate.
fn setup_uefi_crate() {
    let st = uefi_std::env::system_table();
    let ih = uefi_std::env::image_handle();

    // Mandatory setup code for `uefi` crate.
    unsafe {
        uefi::table::set_system_table(st.as_ptr().cast());

        let ih = Handle::from_ptr(ih.as_ptr().cast()).unwrap();
        uefi::boot::set_image_handle(ih);
    }
}

fn setup_panic_handler() {
    std::panic::set_hook(Box::new(|info| {
        if let Some(location) = info.location() {
            println!("Panic \"{}\" at {}:{}", info.payload().downcast_ref::<&str>().unwrap_or(&"<no msg>"), location.file(), location.line());
        } else {
            println!("Panic occurred but no location information available.");
        }
    }));
}

fn main() {
    setup_panic_handler();
    setup_uefi_crate();

    let mut instance: NeoNexInstance = NeoNexInstance::new();
    // DemoPlugin - a ratatui set of animated widgets setup
    instance.app.add_plugins(DemoPlugin);
    instance.run();

    boot::stall(10_000_000);
    uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}