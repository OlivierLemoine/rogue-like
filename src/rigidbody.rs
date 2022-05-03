use crate::AppState;
use bevy::{core::FixedTimestep, prelude::*};
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Component, Default)]
pub struct RigidBody {
    pub speed: Vec3,
}
impl RigidBody {
    fn gravity(mut query: Query<(&mut Transform, &mut RigidBody)>) {
        for (mut tranform, mut rigidbody) in query.iter_mut() {
            rigidbody.speed.y -= 0.1;
            rigidbody.speed = rigidbody.speed.clamp(Vec3::splat(-10.), Vec3::splat(10.));
            tranform.translation += rigidbody.speed;
        }
    }

    pub fn system_set() -> SystemSet {
        SystemSet::on_update(AppState::RunningGame)
            .with_run_criteria(FixedTimestep::step(1. / 60.))
            .with_system(RigidBody::gravity)
    }
}
