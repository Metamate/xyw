use std::ops::{Div, Sub};

use bevy::{
    prelude::{App, Plugin, Query, Res, Transform, Update, Vec3, With},
    time::Time,
    window::Window,
};

use crate::boid::RigidBody;

const PERCEPTION_RADIUS: f32 = 50.;

const ALIGNMENT: f32 = 10.;
const COHESION: f32 = 1.;
const SEPARATION: f32 = 10.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                alignment_system,
                cohesion_system,
                separation_system,
                move_system,
                contain_system,
            ),
        );
    }
}

pub fn cohesion_system(timer: Res<Time>, mut query: Query<(&Transform, &mut RigidBody)>) {
    let boids: Vec<(Transform, RigidBody)> = query.iter().map(|(t, rb)| (*t, *rb)).collect();

    for (transform, mut rigidbody) in query.iter_mut() {
        let local_boids: Vec<&(Transform, RigidBody)> = boids
            .iter()
            .filter(|(t, _)| t.translation.distance(transform.translation) < PERCEPTION_RADIUS)
            .collect();

        let cohesion: Vec3 = local_boids
            .iter()
            .fold(Vec3::ZERO, |sum, (t, _)| sum + t.translation)
            .div(local_boids.len() as f32)
            .sub(transform.translation);

        rigidbody.acceleration += cohesion * COHESION * timer.delta_seconds();
    }
}

pub fn separation_system(timer: Res<Time>, mut query: Query<(&Transform, &mut RigidBody)>) {
    let boids: Vec<(Transform, RigidBody)> = query.iter().map(|(t, rb)| (*t, *rb)).collect();

    for (transform, mut rigidbody) in query.iter_mut() {
        let local_boids: Vec<&(Transform, RigidBody)> = boids
            .iter()
            .filter(|(t, _)| t.translation.distance(transform.translation) < PERCEPTION_RADIUS)
            .collect();

        let separation: Vec3 = local_boids.iter().fold(Vec3::ZERO, |sum, (t, _)| {
            let distance = transform.translation.distance(t.translation);
            let inverse_magnitude = if distance != 0. { distance } else { f32::MIN };

            sum - t
                .translation
                .sub(transform.translation)
                .div(inverse_magnitude)
        });
        rigidbody.acceleration += separation * SEPARATION * timer.delta_seconds();
    }
}

pub fn alignment_system(timer: Res<Time>, mut query: Query<(&Transform, &mut RigidBody)>) {
    let boids: Vec<(Transform, RigidBody)> = query.iter().map(|(t, rb)| (*t, *rb)).collect();

    for (transform, mut rigidbody) in &mut query {
        let local_boids: Vec<&(Transform, RigidBody)> = boids
            .iter()
            .filter(|(t, _)| t.translation.distance(transform.translation) < PERCEPTION_RADIUS)
            .collect();

        let alignment: Vec3 = local_boids
            .iter()
            .filter(|(t, _)| t.translation.distance(transform.translation) < PERCEPTION_RADIUS)
            .fold(Vec3::ZERO, |sum, (_, rb)| sum + rb.velocity)
            .div(local_boids.len() as f32)
            .sub(rigidbody.velocity);

        rigidbody.acceleration += alignment * ALIGNMENT * timer.delta_seconds();
    }
}

pub fn move_system(mut query: Query<(&mut Transform, &mut RigidBody)>) {
    for (mut transform, mut rigidbody) in &mut query {
        rigidbody.acceleration = rigidbody.acceleration.clamp_length_max(rigidbody.max_force);
        transform.translation += rigidbody.velocity;

        let acceleration = rigidbody.acceleration;

        rigidbody.velocity += acceleration;
        rigidbody.velocity = rigidbody
            .velocity
            .clamp_length(rigidbody.min_velocity, rigidbody.max_velocity);
        rigidbody.acceleration = Vec3::ZERO;
    }
}

pub fn contain_system(mut query: Query<&mut Transform, With<RigidBody>>, windows: Query<&Window>) {
    let window = windows.single();

    for mut transform in query.iter_mut() {
        if transform.translation.x > window.resolution.width() / 2. {
            transform.translation.x = -window.resolution.width() / 2.;
        } else if transform.translation.x < -window.resolution.width() / 2. {
            transform.translation.x = window.resolution.width() / 2.;
        }
        if transform.translation.y > window.resolution.height() / 2. {
            transform.translation.y = -window.resolution.height() / 2.;
        } else if transform.translation.y < -window.resolution.height() / 2. {
            transform.translation.y = window.resolution.height() / 2.;
        }
    }
}
