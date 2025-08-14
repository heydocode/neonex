#![feature(uefi_std)]

use bevy::ecs::{error::BevyError, system::NonSendMut};
use neonex_core::{ActivePlatform, DefaultNeoNexConfig, NeoNexInstance};
use neonex_platform::NeoNexPlatform;
use neonex_terminal::RatatuiContext;

/// Performs the necessary setup code for the `uefi` crate.
fn setup_uefi_crate() {
    let system_table = std::os::uefi::env::system_table();
    let image_handle = std::os::uefi::env::image_handle();

    // Mandatory setup code for `uefi` crate.
    unsafe {
        uefi::table::set_system_table(system_table.as_ptr().cast());

        let ih = uefi::Handle::from_ptr(image_handle.as_ptr().cast()).unwrap();
        uefi::boot::set_image_handle(ih);
    }
}

fn setup_panic_handler() {
    std::panic::set_hook(Box::new(|info: &std::panic::PanicHookInfo<'_>| {
        if let Some(location) = info.location() {
            println!("Panic at {}:{}", location.file(), location.line());
        } else {
            println!("Panic occurred but no location information available.");
        }
    }));
}

fn main() {
    // Basic required setup.
    setup_panic_handler();
    setup_uefi_crate();

    let mut instance: NeoNexInstance<DefaultNeoNexConfig, ActivePlatform> = NeoNexInstance::new();
    instance.app.add_systems(bevy::app::Update, tui);
    instance.run();
}

fn tui(
    mut context: NonSendMut<
        RatatuiContext<
            <ActivePlatform as NeoNexPlatform>::RatatuiContextGenerics,
            <ActivePlatform as NeoNexPlatform>::RatatuiContextBackend,
        >,
    >,
) -> Result<(), BevyError> {
    context.draw(|frame| {
        let area = frame.area();

        let text = ratatui::text::Line::from(frame.count().to_string());
        let widget = ratatui::widgets::Paragraph::new(text);

        frame.render_widget(widget, area);
    })?;

    Ok(())
}
