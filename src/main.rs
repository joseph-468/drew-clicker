use bevy::prelude::*;
use bevy::window::*;
use bevy_pkv::PkvStore;

use systems::*;

pub mod components;
pub mod resources;
mod systems;

const WINDOW_TITLE: &str = "Drew Clicker";
const RESOLUTION_X: f32 = 1280.0;
const RESOLUTION_Y: f32 = 720.0;

fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(RESOLUTION_X, RESOLUTION_Y),
            title: String::from(WINDOW_TITLE),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(PkvStore::new("joseph468", "Drew Clicker"))
    .add_startup_system(setup.in_base_set(StartupSet::PreStartup))
    .add_startup_system(setup_timers.in_base_set(StartupSet::Startup))
    .add_startup_system(spawn_buy_menu.in_base_set(StartupSet::Startup))
    .add_startup_system(spawn_camera.in_base_set(StartupSet::PostStartup))
    .add_system(drew_click.before(calculate_dps))
    .add_system(calculate_dps.before(calculate_purchases)) 
    .add_system(calculate_purchases.before(update_text))  
    .add_system(update_text.before(save))
    .add_system(save)
    .run()
}

