#![cfg_attr(feature = "uefi", uefi_std)]

use bevy::app::Update;
use bevy::ecs::error::BevyError;
use bevy::ecs::system::{Local, NonSendMut};
use neonex_core::{ActivePlatform, DefaultNeoNexConfig, NeoNexInstance};
#[cfg(feature = "desktop-hybrid-contexts")]
use neonex_desktop::SoftatuiDesktop;
use neonex_platform::{NeoNexConfig, NeoNexPlatform};
use neonex_terminal::RatatuiContext;
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
#[cfg(feature = "uefi")]
use std::os::uefi as uefi_std;
use std::time::Duration;
#[cfg(feature = "uefi")]
use uefi::proto::console::gop::GraphicsOutput;
#[cfg(feature = "uefi")]
use uefi::runtime::ResetType;
#[cfg(feature = "uefi")]
use uefi::{Handle, Status, boot};

#[cfg(feature = "uefi")]
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

#[cfg(feature = "uefi")]
fn main() {
    println!("Hello World from uefi_std");
    setup_uefi_crate();
    println!("UEFI-Version is {}", uefi::system::uefi_revision());

    let mut instance: NeoNexInstance<DefaultNeoNexConfig, ActivePlatform> = NeoNexInstance::new();
    instance.app.add_systems(Update, tui);
    instance.run();

    boot::stall(10_000_000);
    uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}

#[cfg(not(feature = "uefi"))]
fn main() {

    let mut instance: NeoNexInstance<DefaultNeoNexConfig> = NeoNexInstance::new();
    instance.app.add_systems(Update, tui);
    // CAUTION Custom runner for bench purposes only.
    // instance.app.set_runner(|mut app| {
    //     for _ in 0..1_000 {
    //         app.update();
    //         if let Some(exit) = app.should_exit() {
    //             return exit;
    //         }
    //     }
    //     return bevy::app::AppExit::Success;
    // });
    instance.run();
}

// mod std_init;

// fn main() {
//     std_init::init();
// }

// #![no_std]
// #![no_main]

// use bevy::prelude::*;
// use core::panic::PanicInfo;
// use neonex_core::{ActivePlatform, DefaultNeoNexConfig, NeoNexInstance};
// use neonex_platform::NeoNexPlatform;
// use neonex_terminal::RatatuiContext;
// use uefi::{allocator::Allocator, prelude::*, println};

// // 1. Set the UEFI allocator as global
// #[global_allocator]
// static ALLOCATOR: Allocator = Allocator;

// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     if let Some(location) = info.location() {
//         uefi::println!("at {}:{}\r\n", location.file(), location.line());
//     }

//     println!("Panic message: {}", info.message());
//     println!("As no_std panic handlers must have ! signature, looping no-op...");

//     loop {}
// }

// #[entry]
// fn main() -> Status {
//     // 3. Initialize allocator & logger
//     uefi::helpers::init().unwrap();

//     uefi::println!("Hello world with allocator!");

//     let mut instance: NeoNexInstance<DefaultNeoNexConfig, ActivePlatform> = NeoNexInstance::new();
//     instance.app.add_systems(Update, tui);
//     instance.run();

//     Status::SUCCESS
// }

fn tui(
    mut context: NonSendMut<
        RatatuiContext<
            <ActivePlatform as NeoNexPlatform>::RatatuiContextGenerics,
            <ActivePlatform as NeoNexPlatform>::RatatuiContextBackend,
        >,
    >,
) -> core::result::Result<(), BevyError> {
    context.draw(|frame| {
        let area = frame.area();

        let text = ratatui::text::Line::from(frame.count().to_string());
        let widget = ratatui::widgets::Paragraph::new(text);

        frame.render_widget(widget, area);
    })?;

    Ok(())
}
