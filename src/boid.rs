use std::ops::{Div, Sub};

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::BOID_COLORS;

const PERCEPTION_RADIUS: f32 = 50.;

#[derive(PartialEq, Copy, Clone, Component)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    max_force: f32,
    max_velocity: f32,
    min_velocity: f32,
}

#[derive(Component)]
pub struct RigidBody {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub max_force: f32,
    pub min_velocity: f32,
    pub max_velocity: f32,
}

impl Boid {
    pub fn update(&mut self) {
        self.acceleration = self.acceleration.clamp_length_max(self.max_force);
        self.position += self.velocity;
        self.velocity += self.acceleration;
        self.velocity = self
            .velocity
            .clamp_length(self.min_velocity, self.max_velocity);
        self.acceleration = Vec2::ZERO;
    }

    pub fn local_boids<'a>(&self, boids: &'a [Boid]) -> Vec<&'a Boid> {
        boids
            .iter()
            .filter(|boid| {
                *boid != self && self.position.distance(boid.position) < PERCEPTION_RADIUS
            })
            .collect()
    }

    pub fn contain(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        if self.position.x > right {
            self.position.x = left;
        } else if self.position.x < left {
            self.position.x = right;
        }
        if self.position.y > top {
            self.position.y = bottom;
        } else if self.position.y < bottom {
            self.position.y = top;
        }
    }

    pub fn alignment(&self, local_boids: &[&Boid]) -> Vec2 {
        let len = local_boids.len();
        if len == 0 {
            return Vec2::ZERO;
        }
        local_boids
            .iter()
            .fold(Vec2::ZERO, |sum, boid| sum + boid.velocity)
            .div(len as f32)
            .sub(self.velocity)
    }

    pub fn cohesion(&self, local_boids: &[&Boid]) -> Vec2 {
        let len = local_boids.len();
        if len == 0 {
            return Vec2::ZERO;
        }
        local_boids
            .iter()
            .fold(Vec2::ZERO, |sum, boid| sum + boid.position)
            .div(len as f32)
            .sub(self.position)
    }

    pub fn separation(&self, local_boids: &[&Boid]) -> Vec2 {
        let len = local_boids.len();
        if len == 0 {
            return Vec2::ZERO;
        }
        local_boids.iter().fold(Vec2::ZERO, |sum, boid| {
            let distance = self.position.distance(boid.position);
            let inverse_magnitude = if distance != 0. { distance } else { f32::MIN };

            sum - boid.position.sub(self.position).div(inverse_magnitude)
        })
    }
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
