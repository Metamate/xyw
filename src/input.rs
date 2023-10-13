use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::boid::{get_random_color, Boid, BOID_SIZE};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Sample4)
            .add_systems(Update, mouse_button_input);
    }
}

pub fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if buttons.pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();

        let window = windows.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            let boid = Boid::new(world_position.x, world_position.y);

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
}
