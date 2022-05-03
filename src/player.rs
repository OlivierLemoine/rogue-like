use crate::{Animator, AppState};
use bevy::{core::FixedTimestep, prelude::*};
use bevy_inspector_egui::Inspectable;

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    player: Player,
    animator: Animator,
}
impl PlayerBundle {
    pub fn new(atlas: Handle<TextureAtlas>) -> Self {
        PlayerBundle {
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: atlas,
                ..Default::default()
            },
            player: Player {},
            animator: Animator::new(vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]]),
        }
    }
}

#[derive(Inspectable, Component)]
pub struct Player {}
impl Player {
    pub fn move_player(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(&mut Transform, &mut Animator), With<Player>>,
    ) {
        if query.is_empty() {
            return;
        }

        let (mut transform, mut animator) = query.single_mut();

        let mut has_moved = false;

        if keyboard_input.pressed(KeyCode::D) {
            has_moved = true;
            transform.translation.x += 1.;
            animator.change_animation(2, false);
        }

        if keyboard_input.pressed(KeyCode::A) {
            has_moved = true;
            transform.translation.x -= 1.;
            animator.change_animation(2, true);
        }

        if !has_moved {
            animator.change_animation(1, false);
        }
    }

    pub fn system_set() -> SystemSet {
        SystemSet::on_update(AppState::RunningGame)
            .with_run_criteria(FixedTimestep::step(1. / 60.))
            .with_system(Player::move_player)
    }
}
