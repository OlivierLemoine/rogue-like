use bevy::{core::FixedTimestep, prelude::*};
use bevy_inspector_egui::Inspectable;

#[derive(Bundle)]
pub struct MonsterBundle {
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    player: Monster,
}
impl MonsterBundle {
    pub fn new(atlas: Handle<TextureAtlas>) -> Self {
        MonsterBundle {
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: atlas,
                ..Default::default()
            },
            player: Monster {},
        }
    }
}

#[derive(Inspectable, Component)]
pub struct Monster {}
impl Monster {
    pub fn system_set() -> SystemSet {
        SystemSet::new().with_run_criteria(FixedTimestep::step(1. / 60.))
    }
}
