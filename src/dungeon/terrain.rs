use crate::Collider;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct TerrainTileBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    tile: TerrainTile,
    collider: Collider,
}
impl TerrainTileBundle {
    pub fn new(atlas_handle: Handle<TextureAtlas>, atlas_idx: usize, at: Vec3) -> Self {
        TerrainTileBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: atlas_handle,
                sprite: TextureAtlasSprite {
                    index: atlas_idx,
                    ..Default::default()
                },
                transform: Transform {
                    translation: at * 3.,
                    scale: Vec3::splat(3.),
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider {size: Vec2::splat(16. * 3.)},
            ..Default::default()
        }
    }
}

#[derive(Component, Default)]
pub struct TerrainTile {}

pub fn terrain_rules_set() {}
