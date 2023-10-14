use std::ops::{Div, Sub};

use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub const NO_BOIDS: u16 = 500;
pub const BOID_SIZE: f32 = 10.;
pub const ALIGNMENT: f32 = 5.;
pub const COHESION: f32 = 1.;
pub const SEPARATION: f32 = 5.;
const PERCEPTION_RADIUS: f32 = 50.;
const MAX_FORCE: f32 = 0.5;
const MIN_VELOCITY: f32 = 1.5;
const MAX_VELOCITY: f32 = 5.;

#[derive(Resource)]
pub struct BoidSettings {
    pub alignment: f32,
    pub cohesion: f32,
    pub separation: f32,
}

impl Default for BoidSettings {
    fn default() -> Self {
        BoidSettings {
            alignment: ALIGNMENT,
            cohesion: COHESION,
            separation: SEPARATION,
        }
    }
}

pub const BOID_COLORS: [(f32, f32, f32); 9] = [
    (0.4, 0.361, 0.329),
    (0.49, 0.682, 0.639),
    (0.573, 0.514, 0.455),
    (0.537, 0.706, 0.51),
    (0.663, 0.714, 0.396),
    (0.831, 0.745, 0.596),
    (0.827, 0.525, 0.608),
    (0.918, 0.412, 0.384),
    (0.847, 0.651, 0.341),
];

#[derive(PartialEq, Copy, Clone, Component)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Boid {
    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        Self {
            position: Vec2::new(pos_x, pos_y),
            velocity: Vec2::new(random() - 0.5, random() - 0.5),
            acceleration: Vec2::new(random() - 0.5, random() - 0.5),
        }
    }

    pub fn update(&mut self) {
        self.acceleration = self.acceleration.clamp_length_max(MAX_FORCE);
        self.position += self.velocity;
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length(MIN_VELOCITY, MAX_VELOCITY);
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

pub fn get_random_color() -> Color {
    let mut rng = thread_rng();
    let color: [f32; 3] = BOID_COLORS[rng.gen_range(0..BOID_COLORS.len())].into();
    Color::from(color)
}

pub fn random() -> f32 {
    thread_rng().gen_range(0.0..=1.0)
}

pub fn random_range(min: f32, max: f32) -> f32 {
    thread_rng().gen_range(min..=max)
}
