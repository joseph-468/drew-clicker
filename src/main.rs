use bevy::prelude::*;
use bevy::window::*;
use std::time::Duration;

const WINDOW_TITLE: &str = "Drew Clicker";
const RESOLUTION_X: f32 = 1280.0;
const RESOLUTION_Y: f32 = 720.0;

const DEFAULT_PRICES: [u128; 2] = [100, 1000];
const AUTOCLICKER_VALUES: [u128; 2] = [1, 10];

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
        .add_startup_system(setup_timers)
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
    commands.spawn(Player {droodles: 0, dps: 0, click_strength: 10, auto_clickers: [0, 0], prices: [100, 1000]});

    // Spawn text
    commands.spawn((
        TextBundle::from_section(
            "Droodles: 0.0",
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
                top: Val::Px(16.0),
                left: Val::Px(450.0),
                ..default()
            },
            ..default()
        }),
        MoneyText {}
    ));

    commands.spawn((
        TextBundle::from_section(
            "DPS: 0.0",
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
                top: Val::Px(16.0),
                left: Val::Px(32.0),
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
    effect_query: Query<Entity, With<DroodleCoin>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut player_query: Query<&mut Player>,
    mut commands: Commands,
    mut sprite_query: Query<&mut Sprite, With<DroodleCoin>>, 
    mut coin_effect_timer: ResMut<CoinEffectTime>, 
    mut coin_despawn_timer: ResMut<CoinDespawnTime>,
    ) {
    let window = window_query.get_single().unwrap();
    let mut player = player_query.get_single_mut().unwrap(); 
    coin_effect_timer.timer.tick(time.delta());
    coin_despawn_timer.timer.tick(time.delta());

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(_position) = window.cursor_position() {
            let pos = window.cursor_position().unwrap();
            if pos.x >= 100.0 && pos.x <= 500.0 && pos.y >= 150.0 && pos.y <= 550.0 {
                player.droodles += player.click_strength;

                commands.spawn((SpriteBundle {
                    texture: asset_server.load("sprites/droodle.png"),
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0),

                    ..default()
                }, DroodleCoin {} ));
            }
        }
    }

    // Coin effect
    for mut sprite in sprite_query.iter_mut() {
        let a = sprite.color.a();
        if coin_effect_timer.timer.finished() {
            sprite.color.set_a(a-0.1);
        }   
    }  
    if coin_despawn_timer.timer.finished() {
        for effect in effect_query.iter() {
            commands.entity(effect).despawn();
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

fn setup_timers(
    mut commands: Commands,
    ) {
    commands.insert_resource(CoinEffectTime {timer: Timer::new(Duration::from_millis(750), TimerMode::Repeating)});
    commands.insert_resource(CoinDespawnTime {timer: Timer::new(Duration::from_secs(60), TimerMode::Repeating)}); // Destroy all effects every minute
    commands.insert_resource(DPSTime {
        timer: Timer::new(Duration::from_millis(1000), TimerMode::Repeating),
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

fn spawn_buy_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((NodeBundle {
        style: Style {
            gap: Size::new(Val::Px(0.0), Val::Px(8.0)),
            flex_direction: FlexDirection::Column,
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
                    "Slave $",
                    get_text_style(&asset_server)),
                TextSection::new(
                    "10",
                    get_text_style(&asset_server)), 
                ]), 
            ));
        });
        // poop
        parent.spawn(ButtonBundle {
            style: BUTTON_STYLE,
            background_color: Color::BLUE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                    "Farmers $",
                    get_text_style(&asset_server)),
                TextSection::new(
                    "100",
                    get_text_style(&asset_server)), 
                ]), 
            ));
        });
    });
}

fn calculate_purchases(
    mut player_query: Query<&mut Player>,
    mut button_query: Query<(&Interaction, &Children), Changed<Interaction>>,
    mut text_query: Query<&mut Text>
) {
    for (interaction, children) in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        let button_type =  &text.sections[0].value;
        match *interaction {
            Interaction::Clicked => {
                match button_type.as_str() {   
                "Slave $" => {purchase(0, &mut player_query, &mut text)},
                "Farmers $" => {purchase(1, &mut player_query, &mut text)},
                _ => {},
                }
            },
            _ => {}
        }
    }
}

fn purchase(index: usize, player_query: &mut Query<&mut Player>, text: &mut Text) {
    let mut player = player_query.get_single_mut().unwrap();
    let current_price = player.prices[index]; 
    if player.droodles >= current_price {
        player.droodles -= current_price;
        player.auto_clickers[index] += 1;
        player.prices[index] = calculate_price(DEFAULT_PRICES[index], player.auto_clickers[index]);
        player.dps += AUTOCLICKER_VALUES[index];
        text.sections[1].value = (player.prices[index]/10).to_string();
    }
}

fn calculate_price(base: u128, amount: u128) -> u128 { 
    let constant: f64 = 1.15;
    (base as f64 * constant.powf(amount as f64)) as u128 / 10 * 10
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
    auto_clickers: [u128; 2],
    prices: [u128; 2],
}

#[derive(Component)]
struct MoneyText {}

#[derive(Component)]
struct DPSText {}

#[derive(Component)]
struct BuyMenu {}

#[derive(Component)]
struct DroodleCoin {}

#[derive(Resource)]
struct DPSTime {timer: Timer}

#[derive(Resource)]
struct CoinEffectTime {timer: Timer}

#[derive(Resource)]
struct CoinDespawnTime {timer: Timer} // Very hacky
