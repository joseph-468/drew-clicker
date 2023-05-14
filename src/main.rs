use bevy::prelude::*;
use bevy::window::*;
use std::time::Duration;

const WINDOW_TITLE: &str = "Drew Clicker";
const RESOLUTION_X: f32 = 1280.0;
const RESOLUTION_Y: f32 = 720.0;

const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(256.0), Val::Px(96.0)),
    ..Style::DEFAULT 
};

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
        .add_startup_system(setup_dps)
        .add_startup_system(spawn_buy_menu)
        .add_startup_system(spawn_camera)
        .add_system(calculate_purchases)
        .add_system(drew_click)
        .add_system(calculate_dps)
        .add_system(update_text)
        .run()
}

fn setup(mut commands: Commands, 
         window_query: Query<&Window, With<PrimaryWindow>>,
         asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // Spawn Drew & Player
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(300.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/drew-dribble.png"),
            ..default() 
        },
        Drew {},
    ));
    commands.spawn(Player {droodles: 0, dps: 1, click_strength: 10});

    // Spawn text
    commands.spawn((
        TextBundle::from_section(
            "Droodles: 0",
            TextStyle {
                font: asset_server.load("fonts/font.ttf"),
                font_size: 64.0,
                color: Color::WHITE,
            },     
       ) 
        .with_text_alignment(TextAlignment::Right)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(300.0),
                ..default()
            },
            ..default()
        }),
        MoneyText {}
    ));

    commands.spawn((
        TextBundle::from_section(
            "DPS: 0",
            TextStyle {
                font: asset_server.load("fonts/font.ttf"),
                font_size: 64.0,
                color: Color::WHITE,
            },
       )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        DPSText {}
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

fn drew_click(
    buttons: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Player>
    ) {
    let window = window_query.get_single().unwrap();
    let mut player = player_query.get_single_mut().unwrap();

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(_position) = window.cursor_position() {
            let pos = window.cursor_position().unwrap();
            if pos.x >= 100.0 && pos.x <= 500.0 && pos.y >= 150.0 && pos.y <= 550.0 {
                player.droodles += player.click_strength;
            }
        }
    }
} 

fn update_text(
        mut droodle_query: Query<&mut Text, (With<MoneyText>, Without<DPSText>)>,
        mut dps_query: Query<&mut Text, (With<DPSText>, Without<MoneyText>)>,
        player_query: Query<&Player>
        ) {
    let droodles = player_query.get_single().unwrap().droodles as f64 /  10.0;
    let dps = player_query.get_single().unwrap().dps as f64 / 10.0;
    for mut text in &mut droodle_query {
        text.sections[0].value = format!("Droodles: {:?}", droodles);
    }
    for mut text in &mut dps_query {
        text.sections[0].value = format!("DPS: {:?}", dps);
    }

}

fn setup_dps(
    mut commands: Commands,
    ) {
    commands.insert_resource(DPSTime {
        timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
    })
}

fn calculate_dps(
    mut player_query: Query<&mut Player>,
    mut dps_timer: ResMut<DPSTime>,
    time: Res<Time>,
    ) {
    let mut player = player_query.get_single_mut().unwrap();
    dps_timer.timer.tick(time.delta());
    if dps_timer.timer.finished() {
        player.droodles += player.dps;
    }
}

fn build_buy_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let buy_menu_entity = commands.spawn((NodeBundle {
        style: Style {
            position: UiRect {
                top: Val::Px(0.0),
                left: Val::Px(1024.0),
                ..default()
            },
            size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
            ..default()
        },
        background_color: Color::RED.into(),
        ..default()
    },
    BuyMenu {},
    ))
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: BUTTON_STYLE,
            background_color: Color::BLUE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                    "Slave: ",
                    get_text_style(&asset_server)),
                TextSection::new(
                    "10",
                    get_text_style(&asset_server)), 
                ]), 
            ));
        });
    }).id();

    buy_menu_entity
}

fn spawn_buy_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let buy_menu_entity = build_buy_menu(&mut commands, &asset_server);
}

fn calculate_purchases(
    mut button_query: Query<(&Interaction, &Children), Changed<Interaction>>,
    mut text_query: Query<&mut Text>
) {
    for (interaction, children) in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        println!("{:?}", text.sections[1].value);
        match *interaction {
            Interaction::Clicked => {println!("poop")},
            _ => {}
        }
    }
}

fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
    font: asset_server.load("fonts/font.ttf"),
    font_size: 40.0,
    color: Color::WHITE,
}}



#[derive(Component)]
struct Drew {}

#[derive(Component)]
struct Player {
    droodles: u128,
    dps: u128,
    click_strength: u128,
}

#[derive(Component)]
struct MoneyText {}

#[derive(Component)]
struct DPSText {}

#[derive(Component)]
struct BuyMenu {}

#[derive(Component)]
struct Slave {}

#[derive(Component)]
struct SlaveText {}

#[derive(Resource)]
struct DPSTime {timer: Timer}
