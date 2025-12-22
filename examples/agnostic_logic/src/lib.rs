// #![no_std]

use bevy::prelude::*;
use neonex_core::ActivePlatform;
use neonex_core::DefaultRatatuiContext;
use neonex_platform::NeoNexPlatform;
use ratatui::{layout::Flex, prelude::*, widgets::{Block, Clear, Paragraph, Wrap}};

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tui);
    }
}

fn tui(
    mut context: NonSendMut<
        DefaultRatatuiContext
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