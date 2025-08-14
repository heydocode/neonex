use bevy::{app::{AppExit, PostStartup, Update}, ecs::{error::BevyError, system::{Local, NonSendMut, Res, ResMut}}};
use neonex_core::{ActivePlatform, NeoNexInstance};
use neonex_platform::{NeoNexConfig, NeoNexPlatform};
use neonex_shared::NeoNexStartupConfig;
use neonex_terminal::RatatuiContext;
use ratatui::text::Line;

struct CustomizedNeoNex;

impl NeoNexConfig for CustomizedNeoNex {
    type Platform = ActivePlatform;
}

pub fn init() {
    let mut startup_config_set = ActivePlatform::retrieve_startup_config();
    let new_value = if startup_config_set.values.len() < 1 {
        false
    } else {
        if let NeoNexStartupConfig::NativeTerminal(n) = startup_config_set.values.clone().into_iter().last().unwrap() {
            !n
        } else {
            false
        }
    };
    println!("Retrieved: {:?}", startup_config_set);
    startup_config_set
        .values
        .insert(NeoNexStartupConfig::NativeTerminal(new_value));
    ActivePlatform::update_startup_config(startup_config_set);
    let startup_config_set = ActivePlatform::retrieve_startup_config();
    println!("Retrieved: {:?}", startup_config_set);
    let path_or_key = ActivePlatform::retrieve_startup_config_key();
    println!("Path or key: {:?}", path_or_key);

    let mut instance: NeoNexInstance<CustomizedNeoNex, ActivePlatform> = NeoNexInstance::new();

    println!("{}", ActivePlatform::PLATFORM);

    instance.app.add_systems(Update, ui_system);

    match &mut instance.run() {
        AppExit::Success => println!("NeoNex terminated successfully"),
        AppExit::Error(code) => eprintln!("NeoNex terminated with error code {}", code),
    };
}

fn ui_system(
    mut context: NonSendMut<RatatuiContext<<ActivePlatform as NeoNexPlatform>::RatatuiContextGenerics, <ActivePlatform as NeoNexPlatform>::RatatuiContextBackend>>, mut iter: Local<u64>) -> Result<(), BevyError> {
    context.draw(|frame| {
        let area = frame.area();
        frame.render_widget(Line::from(iter.to_string()), area);
    })?;
    *iter += 1;

    Ok(())
}