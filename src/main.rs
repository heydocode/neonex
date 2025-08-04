use bevy::app::AppExit;
use neonex_core::{ActivePlatform, NeoNexInstance};
use neonex_platform::{NeoNexConfig, NeoNexPlatform, NeoNexStartupConfig};

struct CustomizedNeoNex;

impl NeoNexConfig for CustomizedNeoNex {}

fn main() {
    let mut startup_config_set = ActivePlatform::retrieve_startup_config();
    let new_value = if startup_config_set.values.len() < 1 {
        false
    } else {
        if let NeoNexStartupConfig::NativeTerminal(n) = startup_config_set.values.last().unwrap() {
            !n
        }
        else {
            false
        }
    };
    println!("Retrieved: {:?}", startup_config_set);
    startup_config_set.values.push(NeoNexStartupConfig::NativeTerminal(new_value));
    ActivePlatform::update_startup_config(startup_config_set);
    let startup_config_set = ActivePlatform::retrieve_startup_config();
    println!("Retrieved: {:?}", startup_config_set);
    let path_or_key = ActivePlatform::retrieve_startup_config_key();
    println!("Path or key: {:?}", path_or_key);


    let mut instance: NeoNexInstance<CustomizedNeoNex> = NeoNexInstance::new();

    match &mut instance.run() {
        AppExit::Success => println!("NeoNex terminated successfully"),
        AppExit::Error(code) => eprintln!("NeoNex terminated with error code {}", code),
    };
}
