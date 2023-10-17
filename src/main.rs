mod boid;
mod input;
mod ui;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use boid::{get_random_color, BOID_SIZE, NO_BOIDS, BoidPlugin, spawn_random_boid};
use input::InputPlugin;
use ui::UiPlugin;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.161, 0.157, 0.157);


#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn main() {
    App::new()
    .insert_resource(GameTimer(Timer::from_seconds(0.01, TimerMode::Repeating)))
    .insert_resource(ClearColor(BACKGROUND_COLOR))
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
            UiPlugin,
        )).add_systems(Startup, setup)
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
                    .with_scale(Vec3::new(BOID_SIZE, BOID_SIZE, 1.)),
                material: materials.add(ColorMaterial::from(get_random_color())),
                ..default()
            })
            .insert(boid);
    }
}