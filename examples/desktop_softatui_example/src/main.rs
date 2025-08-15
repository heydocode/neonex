use neonex_core::{ActivePlatform, DefaultNeoNexConfig, NeoNexInstance};
use bevy::prelude::*;
use neonex_platform::NeoNexPlatform;
use neonex_terminal::RatatuiContext;

fn main() {
    let mut instance: NeoNexInstance<DefaultNeoNexConfig> = NeoNexInstance::new();
    instance.app.add_systems(Update, tui);
    instance.run();
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
