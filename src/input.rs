use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_lyon::plugin::ShapePlugin;

use crate::{
    boid::{get_random_color, Boid},
    MainCamera,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .add_plugin(ShapePlugin)
            .add_system(mouse_button_input);
    }
}

pub fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if buttons.pressed(MouseButton::Left) {
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so query::single() is OK
        let (camera, camera_transform) = q_camera.single();

        let window = windows.get_primary().unwrap();

        if let Some(screen_pos) = window.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            let boid = Boid::new(world_pos.x, world_pos.y, 5., 5., get_random_color());

            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere::default())).into(),
                    transform: Transform::from_xyz(boid.position.x, boid.position.y, 1.)
                        .with_scale(Vec3::new(boid.width, boid.height, 1.)),
                    material: materials.add(ColorMaterial::from(boid.color)),
                    ..default()
                })
                .insert(boid);
            // commands.spawn_bundle(MaterialMesh2dBundle {
            //     mesh: meshes.add(Mesh::from(shape::Icosphere::default())).into(),
            //     transform: Transform::from_xyz(world_pos.x, world_pos.y, 1.)
            //         .with_scale(Vec3::new(5., 5., 1.)),
            //     material: materials.add(ColorMaterial::from(Color::RED)),
            //     ..default()
            // });
        }
    }
}
