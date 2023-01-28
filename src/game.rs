use bevy::{prelude::*};

use crate::{
    cell::*, 
    tileset::*, 
    player::*,
    utils::*,
};

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Resource)]
pub struct Game {
    pub board: Vec<Vec<Cell>>,
    pub board_size_i: usize,
    pub board_size_j: usize,
    pub player: Player,
}

impl Game {
    pub fn create_simple() -> Self {
        let file = File::open("assets/maps/map.txt").expect("Invalid path to map");
        let reader = BufReader::new(file);  
        let mut board = Vec::new();
        let mut player = Player::default();

        for (i, line) in reader.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.unwrap().chars().enumerate() {
                let cell = Cell::from_str(&c.to_string());
                if cell.state == CellState::Player {
                    player = Player::from(i, j);
                }
                row.push(cell);
            }
            board.push(row);
        }

        Game {
            board_size_i: board[0].len(),
            board_size_j: board.len(),
            board: board,
            player: player,
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Game::create_simple())
            .add_startup_system(spawn_map)
            .add_startup_system(spawn_player);
    }
}

fn spawn_player(
    mut game: ResMut<Game>,
    mut commands: Commands,
    tileset: Res<TileSet>
) {
    let transform = xy_to_ij(game.player.point.y, game.player.point.x, 3.0);

    game.player.entity = Some(
        spawn_sprite(
            &mut commands, 
            &tileset, 
            CellState::Player.tile_index(), 
            transform
        )
    );
}

fn spawn_map(
    mut map: ResMut<Game>, 
    mut commands: Commands,
    tileset: Res<TileSet>
) {
    for (i, row) in map.board.clone().iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let transform = xy_to_ij(j, i, 0.0);

            // Spawn grass tile
            spawn_sprite(
                &mut commands, 
                &tileset, 
                CellState::Empty.tile_index(), 
                transform
            );

            // Draw goals
            match cell.state {
                CellState::BoxOnGoal | CellState::PlayerOnGoal | CellState::Goal | CellState::Wall => {
                    spawn_sprite(
                        &mut commands, 
                        &tileset, 
                        cell.tile_index(), 
                        transform + Vec3::new(0.0, 0.0, 1.0)
                    );
                },
                _ => (),
            }

            // Draw boxes
            match cell.state {
                CellState::Box | CellState::BoxOnGoal => {
                    let entity = spawn_sprite(
                        &mut commands, 
                        &tileset, 
                        CellState::Box.tile_index(), 
                        transform + Vec3::new(0.0, 0.0, 2.0)
                    );

                    map.board[i][j].entity = Some(entity);             
                },
                _ => (),
            }
        }
    }
}