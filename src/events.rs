use bevy::prelude::*;

use crate::components::Position;

#[derive(Event)]
pub struct GrowthEvent {
    pub snake: Entity,
    pub food: (Position, Entity),
}

#[derive(Event)]
pub struct EnemyDieEvent {
    pub enemy: Entity,
}
