use bevy::{
    prelude::{App, Plugin, Query, Res, Transform, Update},
    time::Time,
};

use crate::boid::RigidBody;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (cohesion_system, separation_system, alignment_system),
        );
    }
}

pub fn cohesion_system(timer: Res<Time>, mut query: Query<(&mut Transform, &mut RigidBody)>) {
    for (mut transform, _) in &mut query {
        let direction = transform.local_x();
        transform.translation += direction * 10.0 * timer.delta_seconds();
    }
}

pub fn separation_system() {}

pub fn alignment_system() {}
