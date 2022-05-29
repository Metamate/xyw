mod boid;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use boid::{random, random_range, Boid};

const NO_BOIDS: u16 = 100;

const ALIGNMENT: f32 = 1.;
const COHESION: f32 = 0.05;
const SEPARATION: f32 = 1.;

const WINDOW_HEIGHT: f32 = 900.;
const WINDOW_WIDTH: f32 = 1600.;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.95, 0.95, 0.8);

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
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BoidPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Box::default())).into(),
        transform: Transform::from_xyz(0., 0., 1.).with_scale(Vec3::new(20., 10., 1.)),
        material: materials.add(ColorMaterial::from(Color::GREEN)),
        ..default()
    });

    for _ in 0..NO_BOIDS {
        let boid = Boid::new(
            random_range(-WINDOW_WIDTH / 2., WINDOW_WIDTH / 2.),
            random_range(-WINDOW_HEIGHT / 2., WINDOW_HEIGHT / 2.),
            5.,
            5.,
            (random(), random(), random()),
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
                -WINDOW_WIDTH / 2.,
                WINDOW_WIDTH / 2.,
                -WINDOW_HEIGHT / 2.,
                WINDOW_HEIGHT / 2.,
            );
            transform.translation.y = boid.position.y;
            transform.translation.x = boid.position.x;
        }
    }
}
