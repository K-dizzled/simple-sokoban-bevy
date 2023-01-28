use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.96, 0.87, 0.7);
pub const VIEWPORT_WIDTH: usize = 1280;
pub const VIEWPORT_HEIGHT: usize = 720;
pub const RESOLUTION: f32 = VIEWPORT_WIDTH as f32 / VIEWPORT_HEIGHT as f32;

pub const TILE_SIZE: f32 = 0.15;

mod player;
mod tileset;
mod game;
mod cell;
mod utils;
mod movement;

use game::{Game, GamePlugin};
use tileset::TileSetPlugin;
use movement::MovementPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: VIEWPORT_WIDTH as f32,
                height: VIEWPORT_HEIGHT as f32,
                title: "UV Debug".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(setup)
        .add_plugin(TileSetPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MovementPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn setup(
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(VIEWPORT_WIDTH as f32, VIEWPORT_HEIGHT as f32);
}

fn spawn_camera(
    mut commands: Commands, 
    game: Res<Game>
) {
    let i = game.board_size_i as f32;
    let j = game.board_size_j as f32;

    let i_mid = TILE_SIZE * i / 2.0; 
    let j_mid = TILE_SIZE * j / 2.0;

    let scale_factor = 1.2;

    let mut camera = Camera2dBundle {
        transform: Transform::from_xyz(
            (i - 1.0) * TILE_SIZE / 2.0, 
            (-j + 1.0) * TILE_SIZE / 2.0, 
            100.0
        ),
        ..Default::default()
    };

    let mut scale = if i > j { i_mid } else { j_mid };
    scale *= scale_factor;

    camera.projection.top = scale;
    camera.projection.bottom = -scale;

    camera.projection.right = scale * RESOLUTION;
    camera.projection.left = -scale * RESOLUTION;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn(camera);
}
