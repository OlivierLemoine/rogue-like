use bevy::{core::FixedTimestep, prelude::*};
use bevy_inspector_egui::Inspectable;

use crate::AppState;

#[derive(Inspectable, Component)]
pub struct Animator {
    animations: Vec<Vec<usize>>,
    curr_animation: usize,
    curr_sprite: usize,
    flip_x: bool,
}
impl Animator {
    pub fn new(animations: Vec<Vec<usize>>) -> Self {
        Animator {
            animations,
            curr_animation: 0,
            curr_sprite: 0,
            flip_x: false,
        }
    }

    pub fn animation_is(&self, anim: usize, flip_x: bool) -> bool {
        self.curr_animation == anim && self.flip_x == flip_x
    }

    pub fn change_animation(&mut self, anim: usize, flip_x: bool) {
        if !self.animation_is(anim, flip_x) {
            self.curr_animation = anim;
            self.curr_sprite = 0;
            self.flip_x = flip_x;
        }
    }

    pub fn animate(mut query: Query<(&mut Animator, &mut TextureAtlasSprite)>) {
        for (mut animator, mut sprite) in query.iter_mut() {
            animator.curr_sprite =
                (animator.curr_sprite + 1) % animator.animations[animator.curr_animation].len();
            sprite.index = animator.curr_sprite;
            sprite.flip_x = animator.flip_x;
        }
    }

    pub fn system_set() -> SystemSet {
        SystemSet::on_update(AppState::RunningGame)
            .with_run_criteria(FixedTimestep::step(1. / 8.))
            .with_system(Animator::animate)
    }
}
