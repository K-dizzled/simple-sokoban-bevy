use bevy::{
    prelude::*,
};

use crate::{
    TILE_SIZE,
    cell::CellState,
    game::*,
    utils::*,
    utils::Direction,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_player);
    }
}

fn move_player(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if game.player.move_cooldown.tick(time.delta()).finished() {
        let current = &game.player.point;
        let keycodes = vec![KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right];
        let mut direct: Option<Direction> = None;

        fn process_input(
            input: KeyCode, keyboard_input: &Res<Input<KeyCode>>, 
            cur: &Point, direct: &mut Option<Direction>
        ) -> Option<Point> {
            if keyboard_input.pressed(input) {
                if let Some(direction) = Direction::from_keycode(input) {
                    *direct = Some(direction);
                    return cur.get_point_in_direction(direction)
                } 
            }
            None
        } 

        let target = keycodes.iter().fold(None, |acc, keycode| {
            if let Some(point) = process_input(*keycode, &keyboard_input, current, &mut direct) {
                return Some(point)
            }
            acc
        });

        if let (Some(target), Some(direct)) = (target, direct) {
            match game.board[target.x][target.y].state {
                CellState::Wall => (),
                CellState::Box | CellState::BoxOnGoal => {
                    if let Some(point) = target.get_point_in_direction(direct) {
                        if game.board[point.x][point.y].state == CellState::Empty || game.board[point.x][point.y].state == CellState::Goal {
                            game.board[point.x][point.y].entity = game.board[target.x][target.y].entity; 
                            game.board[point.x][point.y].state = game.board[point.x][point.y].state.add_box();
                            game.board[target.x][target.y].entity = None;
                            game.board[target.x][target.y].state = game.board[target.x][target.y].state.remove_box();

                            *transforms.get_mut(game.board[point.x][point.y].entity.unwrap()).unwrap() = Transform {
                                translation: xy_to_ij(point.y, point.x, 3.0),
                                scale: Vec3::splat(TILE_SIZE),
                                ..default()
                            };

                            game.player.point = target;
                            game.player.move_cooldown.reset();
                            *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
                                translation: xy_to_ij(game.player.point.y, game.player.point.x, 3.0),
                                scale: Vec3::splat(TILE_SIZE),
                                ..default()
                            };
                        }
                    }
                } 
                
                _ => {
                    game.player.point = target;
                    game.player.move_cooldown.reset();
                    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
                        translation: xy_to_ij(game.player.point.y, game.player.point.x, 3.0),
                        scale: Vec3::splat(TILE_SIZE),
                        ..default()
                    };
                }
            }
        }
    }
}