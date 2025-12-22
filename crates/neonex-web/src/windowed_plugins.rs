use crate::SoftatuiContext;
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::WindowResized,
};
use neonex_terminal::RatatuiContext as RatatuiContextGeneric;
use soft_ratatui::SoftBackend;

type RatatuiContext = RatatuiContextGeneric<SoftatuiContext, SoftBackend>;

/// A plugin that, rather than drawing to a terminal buffer, uses software rendering to build a 2D
/// texture from the ratatui buffer, and displays the result in a window.
pub struct WindowedPlugin;

impl Plugin for WindowedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, terminal_render_setup)
            .add_systems(PreUpdate, handle_resize_events)
            .add_systems(Update, render_terminal_to_handle);
    }
}

#[derive(Resource)]
struct TerminalRender(Handle<Image>);

/// A startup system that sets up the terminal
pub fn terminal_render_setup(
    mut commands: Commands,
    softatui: NonSendMut<RatatuiContext>,
    mut images: ResMut<Assets<Image>>,
) -> Result {
    commands.spawn(Camera2d);
    // Create an image that we are going to draw into
    let width = softatui.backend().get_pixmap_width() as u32;
    let height = softatui.backend().get_pixmap_height() as u32;
    let data = softatui.backend().get_pixmap_data_as_rgba();

    let image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        // RECHECK if only RENDER_WORLD suffices
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    let handle = images.add(image);
    commands.spawn((
        ImageNode::new(handle.clone()),
        Node {
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..default()
        },
    ));

    commands.insert_resource(TerminalRender(handle));

    Ok(())
}

/// System that updates the terminal texture each frame
fn render_terminal_to_handle(
    softatui: NonSendMut<RatatuiContext>,
    mut images: ResMut<Assets<Image>>,
    my_handle: Res<TerminalRender>,
) {
    let width = softatui.backend().get_pixmap_width() as u32;
    let height = softatui.backend().get_pixmap_height() as u32;
    // NOTE: Even though retrieving RGB instead of RGBA would require further conversion,
    //  as the function provides a reference, we don't need to reallocate the data, which
    //  leads to a significant performance win (even if including the conversion!)
    //
    //  It would be great if `soft_ratatui` provided a feature to store its data in RGBA
    //  directly, and/or then provide a function to get RGBA data REFERENCE, not OWNERSHIP.
    //  Obviously, if soft stored initially RGBA, it's better because it doesn't need to do
    //  the conversion on its side.
    // TODO Contribute to soft_ratatui to bump its ratatui version to 0.30.0, and provide a
    //  feature to store its pixel data in RGBA, then if feature disabled store it in RGB.
    let rgb_data = softatui.backend().get_pixmap_data();

    let image = images.get_mut(&my_handle.0).expect("Image not found");

    if image.texture_descriptor.size.width != width
        || image.texture_descriptor.size.height != height
    {
        image.resize(Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: 1,
        });

        image.data.as_mut().unwrap().resize((width * height * 4) as usize, 255);
    }

    // NOTE: If compiling in release mode, this for loop gets incredibly performant (-50% of overall frame time)
    //  "overall frame time" means that the entire app got a 50% frame time reduce after applying this method of updating the rendered texture
    //  As of 08.15.2025, frame time went from 0.00919s (before optimization) to 0.00497s (after optimization).
    //  Frametime calculated on an average frametime from 2000 frames.
    // ALTERNATIVE: Conversion from RGB to RGBA can and should be done on the GPU side.
    //  Wins from this may be considerable, but out of scope at the moment, as it's better
    //  to contribute to soft_ratatui to have in their workspace a new feature "rgba", to
    //  store its pixel data into the RGBA format directly.
    //  I think that I'll make features `rgb` (default) and `rgba` (non-default, useful for
    //  `bevy_ratatui`, and this project, which takes the bevy_ratatui code while fine-tuning it
    //  for its own needs).
    for (rgb, rgba) in rgb_data.chunks_exact(3).zip(image.data.as_mut().unwrap().chunks_exact_mut(4)) {
        rgba[0] = rgb[0];
        rgba[1] = rgb[1];
        rgba[2] = rgb[2];
        rgba[3] = 255;
    }
}

/// System that reacts to window resize
fn handle_resize_events(
    mut resize_reader: EventReader<WindowResized>,
    mut softatui: NonSendMut<RatatuiContext>,
) {
    for event in resize_reader.read() {
        let cur_pix_width = softatui.backend().char_width;
        let cur_pix_height = softatui.backend().char_height;
        let av_wid = (event.width / cur_pix_width as f32) as u16;
        let av_hei = (event.height / cur_pix_height as f32) as u16;
        softatui.backend_mut().resize(av_wid, av_hei);
    }
}
