use bevy::prelude::*;
use bevy::window::*;
use bevy_pkv::PkvStore;
use rand::{self, Rng};
use std::time::Duration;

use crate::components::*;
use crate::resources::*;

const EXPONENT_THRESHOLD: u128 = 1000000000;
const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Start,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(896.0), Val::Px(96.0)),
    padding: UiRect {
        left: Val::Px(16.0),
        top: Val::Px(0.0),
        bottom: Val::Px(0.0),
        right: Val::Px(0.0),
    },
    ..Style::DEFAULT
};

pub fn setup(
    mut commands: Commands,
    mut pkv: ResMut<PkvStore>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Play looping background music
    let window = window_query.get_single().unwrap();
    audio.play_with_settings(
        asset_server.load("sounds/funkytown.ogg"),
        PlaybackSettings::LOOP.with_volume(1.0),
    );

    // Spawn image of Drew
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(300.0, window.height() / 2.0, -1.0),
        texture: asset_server.load("sprites/drew-dribble.png"),
        ..default()
    });

    // Load or create player and then spawn player
    if let Ok(player) = pkv.get::<Player>("Player") {
        commands.spawn(player);
    } else {
        let player = Player {
            droodles: 0,
            dps: 0,
            click_strength: 10,
            autoclickers: [0, 0, 0, 0],
            autoclicker_prices: [100, 1000, 10000, 100000],
            autoclicker_values: [1, 10, 100, 1000],
        };
        pkv.set("Player", &player).expect("Couldn't save player");
        commands.spawn(player);
    }

    // Spawn Droodles text
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
        MoneyText {},
    ));

    // Spawn DPS text
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
        DPSText {},
    ));
}

pub fn setup_timers(mut commands: Commands) {
    // Timers used in other systems
    commands.insert_resource(SaveTime {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    });
    commands.insert_resource(DPSTime {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    });
    commands.insert_resource(CoinEffectTime {
        timer: Timer::new(Duration::from_millis(40), TimerMode::Repeating),
    });
}

pub fn spawn_buy_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Player>,
) {
    // Spawn column containing all buttons
    let player = player_query.get_single().unwrap();
    commands
        .spawn((NodeBundle {
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
        },))
        // Pirate button
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: Color::BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_sections([
                        TextSection::new("Pirate $", get_text_style(&asset_server)),
                        TextSection::new(
                            format!(
                                "{}",
                                calculate_price(
                                    player.autoclicker_prices[0],
                                    player.autoclickers[0]
                                ) / 10
                            ),
                            get_text_style(&asset_server),
                        ),
                        TextSection::new(
                            format!("\nOwned: {}", player.autoclickers[0]),
                            get_text_style2(&asset_server),
                        ),
                    ]),));
                });
            // Camel button
            parent
                .spawn(ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: Color::BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_sections([
                        TextSection::new("Camel $", get_text_style(&asset_server)),
                        TextSection::new(
                            format!(
                                "{}",
                                calculate_price(
                                    player.autoclicker_prices[1],
                                    player.autoclickers[1]
                                ) / 10
                            ),
                            get_text_style(&asset_server),
                        ),
                        TextSection::new(
                            format!("\nOwned: {}", player.autoclickers[1]),
                            get_text_style2(&asset_server),
                        ),
                    ]),));
                });
            // Communist button
            parent
                .spawn(ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: Color::BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_sections([
                        TextSection::new("Communist $", get_text_style(&asset_server)),
                        TextSection::new(
                            format!(
                                "{}",
                                calculate_price(
                                    player.autoclicker_prices[2],
                                    player.autoclickers[2]
                                ) / 10
                            ),
                            get_text_style(&asset_server),
                        ),
                        TextSection::new(
                            format!("\nOwned: {}", player.autoclickers[2]),
                            get_text_style2(&asset_server),
                        ),
                    ]),));
                });
            // Femboy button
            parent
                .spawn(ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: Color::BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_sections([
                        TextSection::new("Femboy $", get_text_style(&asset_server)),
                        TextSection::new(
                            format!(
                                "{}",
                                calculate_price(
                                    player.autoclicker_prices[3],
                                    player.autoclickers[3]
                                ) / 10
                            ),
                            get_text_style(&asset_server),
                        ),
                        TextSection::new(
                            format!("\nOwned: {}", player.autoclickers[3]),
                            get_text_style2(&asset_server),
                        ),
                    ]),));
                });
        });
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    // Spawn 2D camera
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
pub fn drew_click(
    mut commands: Commands,
    mut coin_effect_timer: ResMut<CoinEffectTime>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    time: Res<Time>,
    buttons: Res<Input<MouseButton>>,
    mut player_query: Query<&mut Player>,
    mut sprite_query: Query<&mut Sprite, With<DroodleCoin>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    effect_query: Query<Entity, With<DroodleCoin>>,
) {
    // Boring initalisation stuff
    coin_effect_timer.timer.tick(time.delta());
    let window = window_query.get_single().unwrap();
    let mut player = player_query.get_single_mut().unwrap();

    // If clicked and cursor is within Drew's face
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(_position) = window.cursor_position() {
            let pos = window.cursor_position().unwrap();
            if pos.x >= 100.0 && pos.x <= 500.0 && pos.y >= 150.0 && pos.y <= 550.0 {
                // 1 / 50 chance of activitating otherwise regular click
                let mut rng = rand::thread_rng();
                let toasty: u8 = rng.gen_range(0..50);
                if toasty == 0 {
                    // Earn DPS * 10 or 100 minimum Droodles
                    if player.click_strength * player.dps > 1000 {
                        player.droodles += player.click_strength * player.dps;
                    } else {
                        player.droodles += 1000;
                    }
                    // Play toasty sound effect
                    audio.play_with_settings(
                        asset_server.load("sounds/toasty.ogg"),
                        PlaybackSettings::ONCE.with_volume(3.0),
                    );
                    // Spawn in 8 coin effects randomly
                    for _ in 0..8 {
                        let rand_x: f32 = rng.gen_range(164.0..434.0);
                        let rand_y: f32 = rng.gen_range(216.0..494.0);
                        commands.spawn((
                            SpriteBundle {
                                texture: asset_server.load("sprites/droodle.png"),
                                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                                ..default()
                            },
                            DroodleCoin {},
                        ));
                    }
                } else {
                    // Play sound and increment Droodles
                    audio.play(asset_server.load("sounds/click.ogg"));
                    player.droodles += player.click_strength;
                    // Spawn coin effect within 16 px each direction of cursor
                    let rand_x: f32 = rng.gen_range(-16.0..16.0);
                    let rand_y: f32 = rng.gen_range(-16.0..16.0);
                    commands.spawn((
                        SpriteBundle {
                            texture: asset_server.load("sprites/droodle.png"),
                            transform: Transform::from_xyz(pos.x + rand_x, pos.y + rand_y, 0.0),
                            ..default()
                        },
                        DroodleCoin {},
                    ));
                }
            }
        }
    }
    // Create vector with all existing coin effects in
    let mut effects = Vec::new();
    for effect in effect_query.iter() {
        effects.push(effect);
    }

    // Decrease transparency every 40ms (.8 seconds total time)
    if coin_effect_timer.timer.finished() {
        for (i, mut sprite) in sprite_query.iter_mut().enumerate() {
            let a = sprite.color.a();
            // Despawn if fully transparent
            if a <= 0.0 {
                commands.entity(effects[i]).despawn();
            } else {
                // Decrease transparency
                sprite.color.set_a(a - 0.05);
            }
        }
    }
}

pub fn calculate_dps(
    mut dps_timer: ResMut<DPSTime>,
    time: Res<Time>,
    mut player_query: Query<&mut Player>,
) {
    // Increment droodles by DPS every second
    dps_timer.timer.tick(time.delta());
    let mut player = player_query.get_single_mut().unwrap();
    if dps_timer.timer.finished() {
        player.droodles += player.dps;
    }
}

pub fn calculate_purchases(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut player_query: Query<&mut Player>,
    mut button_query: Query<(&Interaction, &Children), Changed<Interaction>>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children) in &mut button_query {
        // Get text from clicked button
        let mut text = text_query.get_mut(children[0]).unwrap();
        let mut purchased = false;
        let button_type = &text.sections[0].value;
        // Match text to autoclicker types and buy it if it matches
        match *interaction {
            Interaction::Clicked => match button_type.as_str() {
                "Pirate $" => {
                    purchased = purchase(0, &mut player_query, &mut text);
                }
                "Camel $" => {
                    purchased = purchase(1, &mut player_query, &mut text);
                }
                "Communist $" => {
                    purchased = purchase(2, &mut player_query, &mut text);
                }
                "Femboy $" => {
                    purchased = purchase(3, &mut player_query, &mut text);
                }
                _ => {}
            },
            _ => {}
        }
        // Play sped up money sound if something was purchased
        if purchased {
            audio.play_with_settings(
                asset_server.load("sounds/purchase.ogg"),
                PlaybackSettings::ONCE.with_volume(2.5).with_speed(1.25),
            );
        }
    }
}

pub fn update_text(
    mut droodle_query: Query<&mut Text, (With<MoneyText>, Without<DPSText>)>,
    mut dps_query: Query<&mut Text, (With<DPSText>, Without<MoneyText>)>,
    player_query: Query<&Player>,
) {
    // Convert Droodles and DPS to formatted values
    let player = player_query.get_single().unwrap();
    let droodles = player.droodles as f64 / 10.0;
    let dps = player.dps as f64 / 10.0;

    // Format Droodles text as decimal or scientific notation value if big enough
    for mut text in &mut droodle_query {
        if player.droodles >= EXPONENT_THRESHOLD {
            text.sections[0].value = format!("Droodles: {:.6e}", droodles);
        } else {
            text.sections[0].value = format!("Droodles: {}", droodles);
        }
    }

    // Format DPS text as decimal or scientific notation value if big enough
    for mut text in &mut dps_query {
        if player.dps >= EXPONENT_THRESHOLD {
            text.sections[0].value = format!("DPS: {:.6e}", dps);
        } else {
            text.sections[0].value = format!("DPS: {}", dps);
        }
    }
}

pub fn save(
    mut pkv: ResMut<PkvStore>,
    mut save_time: ResMut<SaveTime>,
    time: Res<Time>,
    player_query: Query<&Player>,
) {
    // Save every second
    save_time.timer.tick(time.delta());
    let player = player_query.get_single().unwrap();
    if save_time.timer.finished() {
        pkv.set("Player", &player).expect("Couldn't save player");
    }
}

pub fn purchase(index: usize, player_query: &mut Query<&mut Player>, text: &mut Text) -> bool {
    // Get current price of autoclicker
    let mut player = player_query.get_single_mut().unwrap();
    let mut current_price =
        calculate_price(player.autoclicker_prices[index], player.autoclickers[index]);
    // Handle purchase if successful
    if player.droodles >= current_price {
        // Remove droodles, increment autoclicker count, and add to DPS value
        player.droodles -= current_price;
        player.autoclickers[index] += 1;
        player.dps += player.autoclicker_values[index];
        // Calculate new price
        current_price =
            calculate_price(player.autoclicker_prices[index], player.autoclickers[index]) / 10;
        // Update button text and format it in scientific notation if amount is too big
        if current_price >= EXPONENT_THRESHOLD {
            text.sections[1].value = format!("{:.3e}", current_price);
        } else {
            text.sections[1].value = format!("{:?}", current_price);
        }
        text.sections[2].value = format!("\nOwned: {}", player.autoclickers[index]);
        return true;
    }
    false
}

// Helper function to calculate price (stolen from cookie clicker)
pub fn calculate_price(base: u128, amount: u128) -> u128 {
    let constant: f64 = 1.15;
    (base as f64 / 10.0 * constant.powf(amount as f64)) as u128 * 10
}

// Simple way to get text style for button sections 1 & 2
pub fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/font.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    }
}

// Simple way to get text style for button section 3
pub fn get_text_style2(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/font.ttf"),
        font_size: 30.0,
        color: Color::SILVER,
    }
}
