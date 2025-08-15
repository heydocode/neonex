#![feature(uefi_std)]

use bevy::app::Update;
use bevy::ecs::error::BevyError;
use bevy::ecs::system::{Local, NonSendMut};
use neonex_core::{ActivePlatform, DefaultNeoNexConfig, NeoNexInstance};
use neonex_platform::{NeoNexConfig, NeoNexPlatform};
use neonex_terminal::RatatuiContext;
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

fn main() {
    setup_uefi_crate();

    let mut instance: NeoNexInstance<DefaultNeoNexConfig> = NeoNexInstance::new();
    instance.app.add_systems(Update, tui);
    instance.run();

    boot::stall(10_000_000);
    uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}

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
