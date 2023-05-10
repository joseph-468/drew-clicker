use bevy::prelude::*;
use bevy::window::*;

const WINDOW_TITLE: &str = "Drew Clicker";
const RESOLUTION_X: f32 = 1200.0;
const RESOLUTION_Y: f32 = 700.0;

fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(RESOLUTION_X, RESOLUTION_Y),
            title: String::from(WINDOW_TITLE),
            ..default()
        }),
        ..default()
    }))
        .add_startup_system(setup)
        .add_startup_system(spawn_camera)
        .add_system(drew_click)
        .add_system(update_text)
        .run()
}

fn setup(mut commands: Commands, 
         window_query: Query<&Window, With<PrimaryWindow>>,
         asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(300.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/drew-dribble.png"),
            ..default() 
        },
        Drew {},
    ));
    commands.spawn(Player {money: 0});
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Droodles: 0",
            TextStyle {
                font: asset_server.load("fonts/font.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        MoneyText {}
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
              window_query: Query<&Window, With<PrimaryWindow>>,
              mut player_query: Query<&mut Player>,) {

    let window = window_query.get_single().unwrap();
    let mut player = player_query.get_single_mut().unwrap();

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(_position) = window.cursor_position() {
            let pos = window.cursor_position().unwrap();
            if pos.x >= 100.0 && pos.x <= 500.0 && pos.y >= 150.0 && pos.y <= 550.0 {
                player.money += 1;
            }
        }
    }
} 

fn update_text(mut query: Query<&mut Text, With<MoneyText>>, player_query: Query<&Player>) {
    let money = player_query.get_single().unwrap().money;
    for mut text in &mut query {
        text.sections[0].value = format!("Droodles: {money}");
    }
}

#[derive(Component)]
struct Drew {}

#[derive(Component)]
struct Player {
    money: u128
}

#[derive(Component)]
struct MoneyText {}
