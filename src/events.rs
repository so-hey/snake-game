use bevy::prelude::*;

#[derive(Event)]
pub struct GrowthEvent {
    pub snake: Entity,
}

#[derive(Event)]
pub struct GameOverEvent;
