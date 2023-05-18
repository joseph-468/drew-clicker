use bevy::prelude::*;
use bevy::window::*;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use rand::{self, Rng};

const WINDOW_TITLE: &str = "Drew Clicker";
const RESOLUTION_X: f32 = 1280.0;
const RESOLUTION_Y: f32 = 720.0;

const EXPONENT_THRESHOLD: u128 = 1000000000;
const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Start,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(896.0), Val::Px(96.0)),
    padding: UiRect {left: Val::Px(16.0), top: Val::Px(0.0), bottom: Val::Px(0.0), right: Val::Px(0.0)},
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
    })).insert_resource(PkvStore::new("joseph468", "Drew Clicker"))
        .add_startup_system(setup) 
        .add_startup_system(setup_timers)
        .add_startup_system(spawn_buy_menu)
        .add_startup_system(spawn_camera)
        .add_system(calculate_purchases)
        .add_system(drew_click)
        .add_system(calculate_dps)
        .add_system(save)
        .add_system(update_text)
        .run()
}

fn setup(mut commands: Commands,  
         window_query: Query<&Window, With<PrimaryWindow>>,
         asset_server: Res<AssetServer>,
         audio: Res<Audio>,
         mut pkv: ResMut<PkvStore>,
) { 
    let window = window_query.get_single().unwrap();
    // Play background music
    audio.play_with_settings(
        asset_server.load("sounds/funkytown.ogg"),
        PlaybackSettings::LOOP.with_volume(1.0),
    );
    // Spawn Drew & Player
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(300.0, window.height() / 2.0, -1.0),
            texture: asset_server.load("sprites/drew-dribble.png"),
            ..default() 
        },
        Drew {},
    )); 

    if let Ok(player) = pkv.get::<Player>("Player") {
        commands.spawn(player);
    } else {
        let player = Player {droodles: 0, dps: 0, click_strength: 10,
            autoclickers: [0, 0, 0, 0],
            autoclicker_prices: [100, 1000, 10000, 100000],
            autoclicker_values: [1, 10, 100, 1000]};
        pkv.set("Player", &player).expect("Couldn't save player");
        commands.spawn(player); 
    }

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
                left: Val::Px(32.0),
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
                font_size: 52.0,
                color: Color::WHITE,
            },
       )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(80.0),
                left: Val::Px(32.0),
                ..default()
            },
            ..default()
        }),
        DPSText {}
    ));

}

fn save(mut pkv: ResMut<PkvStore>, player_query: Query<&Player>, time: Res<Time>, mut save_time: ResMut<SaveTime>) {
    save_time.timer.tick(time.delta());
    let player = player_query.get_single().unwrap();
    if save_time.timer.finished() {
        pkv.set("Player", &player).expect("Couldn't save player");
    }
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
    audio: Res<Audio>,
    mut player_query: Query<&mut Player>,
    mut commands: Commands,
    mut sprite_query: Query<&mut Sprite, With<DroodleCoin>>, 
    mut coin_effect_timer: ResMut<CoinEffectTime>, 
    ) {
    coin_effect_timer.timer.tick(time.delta());
    let window = window_query.get_single().unwrap();
    let mut player = player_query.get_single_mut().unwrap();  

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(_position) = window.cursor_position() {
            let pos = window.cursor_position().unwrap();
            if pos.x >= 100.0 && pos.x <= 500.0 && pos.y >= 150.0 && pos.y <= 550.0 { 
                let mut rng = rand::thread_rng();
                let toasty: u8 = rng.gen_range(0..50);
                if toasty == 0 {
                    // 5x the regular amount of droodles
                    audio.play_with_settings(asset_server.load("sounds/toasty.ogg"), PlaybackSettings::ONCE.with_volume(3.0));

                    if player.click_strength * player.dps > 1000 {
                        player.droodles += player.click_strength * player.dps;
                    } else {
                        player.droodles += 1000;
                    }

                    for _ in 0..8 {
                        let rand_x: f32 = rng.gen_range(164.0..434.0);
                        let rand_y: f32 = rng.gen_range(216.0..494.0);
                        
                        commands.spawn((SpriteBundle {
                            texture: asset_server.load("sprites/droodle.png"),
                            transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                            ..default()
                        }, DroodleCoin {} ));
                    }
                }
                else {
                    // Regular click
                    audio.play(asset_server.load("sounds/click.ogg"));
                    player.droodles += player.click_strength;
                    
                    let rand_x: f32 = rng.gen_range(-16.0..16.0);
                    let rand_y: f32 = rng.gen_range(-16.0..16.0);
                    commands.spawn((SpriteBundle {
                        texture: asset_server.load("sprites/droodle.png"),
                        transform: Transform::from_xyz(pos.x + rand_x, pos.y + rand_y, 0.0),

                        ..default()
                    }, DroodleCoin {} ));
                }
            }
        }
    }
    let mut effects = Vec::new();
    for effect in effect_query.iter() {
        effects.push(effect);
    }

    // Coin effect
    for (i, mut sprite) in sprite_query.iter_mut().enumerate() {
        let a = sprite.color.a();
        if coin_effect_timer.timer.finished() {
            if a <= 0.0 {
                commands.entity(effects[i]).despawn();
            }
            else {
                sprite.color.set_a(a-0.1);
            }
        }   
    }   
}

fn update_text(
        mut droodle_query: Query<&mut Text, (With<MoneyText>, Without<DPSText>)>,
        mut dps_query: Query<&mut Text, (With<DPSText>, Without<MoneyText>)>,
        player_query: Query<&Player>
        ) {
    let player = player_query.get_single().unwrap();
    let droodles = player.droodles as f64 / 10.0;
    let dps = player.dps as f64 / 10.0;
 
    for mut text in &mut droodle_query {
        if player.droodles >= EXPONENT_THRESHOLD {
            text.sections[0].value = format!("Droodles: {:.6e}", droodles);
        }
        else {
            text.sections[0].value = format!("Droodles: {:?}", droodles);
        }
    }
    for mut text in &mut dps_query {
        if player.dps >= EXPONENT_THRESHOLD {
            text.sections[0].value = format!("DPS: {:.6e}", dps);
        }
        else {
            text.sections[0].value = format!("DPS: {:?}", dps);
        }
    }

}

fn setup_timers(
    mut commands: Commands,
    ) {
    commands.insert_resource(CoinEffectTime {timer: Timer::new(Duration::from_millis(75), TimerMode::Repeating)});
    commands.insert_resource(SaveTime {timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating)});
    commands.insert_resource(DPSTime {timer: Timer::new(Duration::from_millis(1000), TimerMode::Repeating)})
}

fn calculate_dps(
    mut player_query: Query<&mut Player>,
    mut dps_timer: ResMut<DPSTime>,
    time: Res<Time>,
    ) {
    dps_timer.timer.tick(time.delta());
    let mut player = player_query.get_single_mut().unwrap();
    
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
                left: Val::Px(896.0),
                ..default()
            },
            size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
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
                    "Pirate $",
                    get_text_style(&asset_server)),
                TextSection::new(
                    "10",
                    get_text_style(&asset_server)),
                TextSection::new(
                    "\nOwned: 0",
                    get_text_style2(&asset_server)),
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
                    "Camel $",
                    get_text_style(&asset_server)),
                TextSection::new(
                    "100",
                    get_text_style(&asset_server)), 
                TextSection::new(
                    "\nOwned: 0",
                    get_text_style2(&asset_server)), 
                ]), 
            ));
        });
        parent.spawn(ButtonBundle {
            style: BUTTON_STYLE,
            background_color: Color::BLUE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                    "Communist $",
                    get_text_style(&asset_server)),
                TextSection::new(
                    "1000",
                    get_text_style(&asset_server)),
                TextSection::new(
                    "\nOwned: 0",
                    get_text_style2(&asset_server)),
                ]), 
            ));
        });
        parent.spawn(ButtonBundle {
            style: BUTTON_STYLE,
            background_color: Color::BLUE.into(), 
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                    "Femboy $",
                    get_text_style(&asset_server)),
                TextSection::new(
                    "10000",
                    get_text_style(&asset_server)), 
                TextSection::new(
                    "\nOwned: 0",
                    get_text_style2(&asset_server)), 
                ]), 
            ));
        });
    });
}

fn calculate_purchases(
    mut player_query: Query<&mut Player>,
    mut button_query: Query<(&Interaction, &Children), Changed<Interaction>>,
    mut text_query: Query<&mut Text>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (interaction, children) in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        let button_type =  &text.sections[0].value;
        let mut purchased = false;
        match *interaction {
            Interaction::Clicked => {
                match button_type.as_str() {   
                "Pirate $" => {purchased = purchase(0, &mut player_query, &mut text);},
                "Camel $" => {purchased = purchase(1, &mut player_query, &mut text);},
                "Communist $" => {purchased = purchase(2, &mut player_query, &mut text);},
                "Femboy $" => {purchased = purchase(3, &mut player_query, &mut text);},
                _ => {},
                }
            },
            _ => {}
        }
        if purchased {
            audio.play_with_settings(asset_server.load("sounds/purchase.ogg"), PlaybackSettings::ONCE.with_volume(2.5).with_speed(1.25));
        }
    }
}

fn purchase(index: usize, player_query: &mut Query<&mut Player>, text: &mut Text) -> bool {
    let mut player = player_query.get_single_mut().unwrap();
    let mut current_price = calculate_price(player.autoclicker_prices[index], player.autoclickers[index]);
    if player.droodles >= current_price {
        player.droodles -= current_price;
        player.autoclickers[index] += 1;
        player.dps += player.autoclicker_values[index];
        current_price = calculate_price(player.autoclicker_prices[index], player.autoclickers[index]) / 10;

        if current_price >= EXPONENT_THRESHOLD {
            text.sections[1].value = format!("{:.3e}", current_price);
        }
        else { 
            text.sections[1].value = format!("{:?}", current_price);
        } 
        text.sections[2].value = format!("\nOwned: {}", player.autoclickers[index]);
        return true;
    }
    false
}

fn calculate_price(base: u128, amount: u128) -> u128 { 
    let constant: f64 = 1.15;
    (base as f64 / 10.0 * constant.powf(amount as f64)) as u128 * 10
}

fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
    font: asset_server.load("fonts/font.ttf"),
    font_size: 40.0,
    color: Color::WHITE,
}}

fn get_text_style2(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
    font: asset_server.load("fonts/font.ttf"),
    font_size: 30.0,
    color: Color::SILVER,
    }
}

#[derive(Component)]
struct Drew {}

#[derive(Component, Serialize, Deserialize, Debug)]
struct Player {
    droodles: u128,
    dps: u128,
    click_strength: u128,
    autoclickers: [u128; 4],
    autoclicker_prices: [u128; 4],
    autoclicker_values: [u128; 4],
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
struct SaveTime {timer: Timer}
