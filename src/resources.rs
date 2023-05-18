use bevy::prelude::*;

#[derive(Resource)]
pub struct DPSTime {pub timer: Timer}

#[derive(Resource)]
pub struct CoinEffectTime {pub timer: Timer}

#[derive(Resource)]
pub struct SaveTime {pub timer: Timer}
