use bevy::{prelude::*};
use crate::utils::*;

#[derive(Default)]
pub struct Player {
    pub entity: Option<Entity>,
    pub point: Point,
    pub move_cooldown: Timer,
}

impl Player {
    pub fn from(
        i: usize, 
        j: usize, 
    ) -> Self {
        Player {
            entity: None,
            point: Point { x: i, y: j },
            move_cooldown: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}