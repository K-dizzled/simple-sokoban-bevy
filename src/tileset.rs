use bevy::prelude::*;
use crate::TILE_SIZE;

pub struct TileSetPlugin;

#[derive(Resource)]
pub struct TileSet(pub Handle<TextureAtlas>);

impl Plugin for TileSetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_tiles);
    }
}

pub fn spawn_sprite(
    commands: &mut Commands,
    tile_map: &TileSet,
    index: usize,
    translation: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::splat(1.0));

    commands
        .spawn(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: tile_map.0.clone(),
            transform: Transform {
                translation: translation,
                scale: Vec3::splat(TILE_SIZE),
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

fn load_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_image = asset_server.load("sprites/sokoban_tilesheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_image,
        Vec2::new(64.0, 64.0),
        13,
        8,
        None,
        Some(Vec2::new(0.0, 0.0)),
    );

    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(TileSet(atlas_handle));
}