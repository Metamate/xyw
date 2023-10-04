mod boid;
mod input;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use boid::{get_random_color, random_range, Boid};
use input::InputPlugin;

const NO_BOIDS: u16 = 100;

const ALIGNMENT: f32 = 1.;
const COHESION: f32 = 0.05;
const SEPARATION: f32 = 1.;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.161, 0.157, 0.157);
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

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer(Timer::from_seconds(0.01, TimerMode::Repeating)))
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_systems(Startup, setup)
            .add_systems(Update, update_boids);
    }
}

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn main() {
    App::new()
        //.add_systems(Update, on_resize_system)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Superboids".to_string(),
                    resolution: (1280., 720.).into(),
                    ..default()
                }),
                ..default()
            }),
            BoidPlugin,
            InputPlugin,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.single();

    for _ in 0..NO_BOIDS {
        let boid = spawn_random_boid(
            -window.resolution.width() / 2.,
            window.resolution.width() / 2.,
            -window.resolution.height() / 2.,
            window.resolution.height() / 2.,
        );

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                transform: Transform::from_xyz(boid.position.x, boid.position.y, 1.)
                    .with_scale(Vec3::new(boid.width, boid.height, 1.)),
                material: materials.add(ColorMaterial::from(boid.color)),
                ..default()
            })
            .insert(boid);
    }
}

fn spawn_random_boid(left: f32, right: f32, bottom: f32, top: f32) -> Boid {
    Boid::new(
        random_range(left, right),
        random_range(bottom, top),
        10.,
        10.,
        get_random_color(),
    )
}

fn update_boids(
    time: Res<Time>,
    windows: Query<&Window>,
    mut timer: ResMut<GameTimer>,
    mut query: Query<(&mut Boid, &mut Transform)>,
) {
    let window = windows.single();

    let boids: Vec<Boid> = query.iter().map(|(b, _)| *b).collect();

    if timer.0.tick(time.delta()).just_finished() {
        for (mut boid, mut transform) in query.iter_mut() {
            let local_boids = boid.local_boids(&boids);
            let alignment = boid.alignment(&local_boids);
            let cohesion = boid.cohesion(&local_boids);
            let separation = boid.separation(&local_boids);

            boid.acceleration +=
                alignment * ALIGNMENT + cohesion * COHESION + separation * SEPARATION;

            boid.update();
            boid.contain(
                -window.resolution.width() / 2.,
                window.resolution.width() / 2.,
                -window.resolution.height() / 2.,
                window.resolution.height() / 2.,
            );
            transform.translation.y = boid.position.y;
            transform.translation.x = boid.position.x;
        }
    }
}
