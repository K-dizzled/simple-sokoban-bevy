use crate::TILE_SIZE;

use bevy::{prelude::*};

pub fn xy_to_ij(x: usize, y: usize, z: f32) -> Vec3 {
    Vec3::new(
        (x as f32) * TILE_SIZE,
        -(y as f32) * TILE_SIZE,
        z,
    )
}

#[derive(Copy, Clone)]
pub enum Direction {
    Backward,
    Forward,
    Up,
    Down,
}

impl Direction {
    pub fn from_keycode(keycode: KeyCode) -> Option<Self> {
        match keycode {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Backward),
            KeyCode::Right => Some(Direction::Forward),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn get_point_in_direction(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Backward => {
                if self.y != 0 {
                    Some(Point {
                            y: self.y - 1,
                            ..*self
                        })
                } else { None }
            },
            Direction::Forward => Some(Point {
                y: self.y + 1,
                ..*self
            }),
            Direction::Up => {
                if self.x != 0 {
                    Some(Point {
                        x: self.x - 1,
                        ..*self
                    })
                } else { None }
            }, 
            Direction::Down => Some(Point {
                x: self.x + 1,
                ..*self
            }),
        }
    }
}