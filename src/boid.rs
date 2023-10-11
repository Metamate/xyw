use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::BOID_COLORS;

#[derive(Component, Copy, Clone)]
pub struct RigidBody {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub max_force: f32,
    pub min_velocity: f32,
    pub max_velocity: f32,
}

// HELPER FUNCTIONS

pub fn get_random_color() -> [f32; 3] {
    let mut rng = thread_rng();
    BOID_COLORS[rng.gen_range(0..BOID_COLORS.len())].into()
}

pub fn random() -> f32 {
    thread_rng().gen_range(0.0..=1.0)
}

pub fn random_range(min: f32, max: f32) -> f32 {
    thread_rng().gen_range(min..=max)
}
