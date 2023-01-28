use bevy::prelude::*;

#[derive(Default, PartialEq, Copy, Clone)]
pub enum CellState {
    #[default]
    Empty, 
    Wall, 
    Box,
    BoxOnGoal,
    Player,
    PlayerOnGoal,
    Goal,
}

impl CellState {
    pub fn remove_box(&self) -> Self {
        match self {
            CellState::Box => CellState::Empty,
            CellState::BoxOnGoal => CellState::Goal,
            _ => self.clone()
        }
    }

    pub fn add_box(&self) -> Self {
        match self {
            CellState::Empty => CellState::Box,
            CellState::Goal => CellState::BoxOnGoal,
            _ => self.clone()
        }
    }

    pub fn from_str(s: &str) -> CellState {
        match s {
            " " => CellState::Empty,
            "#" => CellState::Wall,
            "x" => CellState::Box,
            "X" => CellState::BoxOnGoal,
            "s" => CellState::Player,
            "S" => CellState::PlayerOnGoal,
            "." => CellState::Goal,
            _ => panic!("Invalid cell state: {}", s),
        }
    }

    pub fn tile_index(&self) -> usize {
        match self {
            CellState::Empty => 88,
            CellState::Wall => 98,
            CellState::Box => 1,
            CellState::BoxOnGoal => 39,
            CellState::Player => 65,
            CellState::PlayerOnGoal => 39,
            CellState::Goal => 39,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Cell {
    pub state: CellState,
    pub entity: Option<Entity>,
}

impl Cell {
    pub fn from_str(s: &str) -> Cell {
        Cell {
            state: CellState::from_str(s),
            entity: None,
        }
    }

    pub fn tile_index(&self) -> usize {
        self.state.tile_index()
    }
}
