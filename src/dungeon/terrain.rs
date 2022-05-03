use bevy::prelude::*;

#[derive(Bundle)]
pub struct TerrainTileBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    tile: TerrainTile,
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
                    translation: at,
                    ..Default::default()
                },
                ..Default::default()
            },
            tile: TerrainTile {},
        }
    }
}

#[derive(Component)]
pub struct TerrainTile {}

pub fn terrain_rules_set() {}
