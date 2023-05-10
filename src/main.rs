use bevy::prelude::*;
use bevy::window::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_startup_system(setup).add_startup_system(spawn_camera).add_system(drew_click).run()
}

fn setup(mut commands: Commands, 
         window_query: Query<&Window, With<PrimaryWindow>>,
         asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("drew-dribble.png"),
            ..default() 
        },
        Drew {},
    ));
}

fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
    ) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn drew_click(buttons: Res<Input<MouseButton>>,
              drew_query: Query<&Drew>,
              player_query: Query<&Player>,) {
if buttons.just_pressed(MouseButton::Left) {
    println!("press");
    }
} 

#[derive(Component)]
struct Drew {}

#[derive(Component)]
struct Player {
    money: u128
}
