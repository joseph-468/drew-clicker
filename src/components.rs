use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Component, Serialize, Deserialize)]
pub struct Player {
    pub droodles: u128,
    pub dps: u128,
    pub click_strength: u128,
    pub autoclickers: [u128; 4],
    pub autoclicker_prices: [u128; 4],
    pub autoclicker_values: [u128; 4],
}

#[derive(Component)]
pub struct MoneyText {}

#[derive(Component)]
pub struct DPSText {}

#[derive(Component)]
pub struct BuyMenu {}

#[derive(Component)]
pub struct DroodleCoin {}
