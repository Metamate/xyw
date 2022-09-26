mod boid;
mod input;

use bevy::{ecs::event::Events, prelude::*, sprite::MaterialMesh2dBundle, window::WindowResized};
use boid::{get_random_color, random_range, Boid};
use input::InputPlugin;

const NO_BOIDS: u16 = 100;

const ALIGNMENT: f32 = 1.;
const COHESION: f32 = 0.05;
const SEPARATION: f32 = 1.;

#[derive(Component)]
pub struct MainCamera;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.161, 0.157, 0.157);
pub const BOID_COLORS: [(f32, f32, f32); 5] = [
    (0.4, 0.361, 0.329),
    (0.49, 0.682, 0.639),
    (0.573, 0.514, 0.455),
    (0.49, 0.682, 0.639),
    (0.537, 0.706, 0.51), //next: brightgreen
];

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer(Timer::from_seconds(0.01, true)))
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_startup_system(setup)
            .add_system(update_boids);
    }
}

struct GameTimer(Timer);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Superboids".to_string(),
            width: 1280.,
            height: 720.,
            ..default()
        })
        .add_system(window_resize)
        .add_plugins(DefaultPlugins)
        .add_plugin(BoidPlugin)
        .add_plugin(InputPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Res<WindowDescriptor>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    for _ in 0..NO_BOIDS {
        let boid = Boid::new(
            random_range(-window.width / 2., window.width / 2.),
            random_range(-window.height / 2., window.height / 2.),
            5.,
            5.,
            get_random_color(),
        );

        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere::default())).into(),
                transform: Transform::from_xyz(boid.position.x, boid.position.y, 1.)
                    .with_scale(Vec3::new(boid.width, boid.height, 1.)),
                material: materials.add(ColorMaterial::from(boid.color)),
                ..default()
            })
            .insert(boid);
    }
}

fn update_boids(
    time: Res<Time>,
    window: Res<WindowDescriptor>,
    mut timer: ResMut<GameTimer>,
    mut query: Query<(&mut Boid, &mut Transform)>,
) {
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
                -window.width / 2.,
                window.width / 2.,
                -window.height / 2.,
                window.height / 2.,
            );
            transform.translation.y = boid.position.y;
            transform.translation.x = boid.position.x;
        }
    }
}

fn window_resize(resize_event: Res<Events<WindowResized>>, mut window: ResMut<WindowDescriptor>) {
    let mut event_reader = resize_event.get_reader();
    for event in event_reader.iter(&resize_event) {
        window.width = event.width.try_into().unwrap();
        window.height = event.height.try_into().unwrap();
    }
}
