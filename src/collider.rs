use crate::{AppState, Player, RigidBody};
use bevy::{
    core::FixedTimestep,
    math::Vec3Swizzles,
    prelude::*,
    sprite::collide_aabb::{self, Collision},
};
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Component, Default)]
pub struct Collider {
    pub size: Vec2,
}
impl Collider {
    pub fn collide(
        mut query: Query<(&mut Transform, &Collider, &mut RigidBody), With<Player>>,
        query_2: Query<(&Transform, &Collider), Without<Player>>,
    ) {
        for (mut transform, collider, mut rigidbody) in query.iter_mut() {
            for (other_transform, other_collider) in query_2.iter() {
                if let Some(c) = collide_aabb::collide(
                    transform.translation,
                    collider.size,
                    other_transform.translation,
                    other_collider.size,
                ) {
                    match c {
                        Collision::Top => {
                            rigidbody.speed.y = 0.;
                            transform.translation.y -=
                                (other_transform.translation.y - transform.translation.y).abs()
                                    + 0.1;
                        }
                        Collision::Bottom => {
                            rigidbody.speed.y = 0.;
                            transform.translation.y +=
                                (other_transform.translation.y - transform.translation.y).abs()
                                    + 0.1;
                        }
                        Collision::Left => {
                            rigidbody.speed.x = 0.;
                            transform.translation.x -=
                                (other_transform.translation.x - transform.translation.x).abs()
                                    + 0.1;
                        }
                        Collision::Right => {
                            rigidbody.speed.x = 0.;
                            transform.translation.x +=
                                (other_transform.translation.x - transform.translation.x).abs()
                                    + 0.1;
                        }
                        Collision::Inside => {}
                    }
                }
            }
        }
    }

    pub fn system_set() -> SystemSet {
        SystemSet::on_update(AppState::RunningGame)
            .with_run_criteria(FixedTimestep::step(1. / 8.))
            .with_system(Collider::collide)
    }
}
