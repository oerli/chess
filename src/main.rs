use bevy::prelude::*;
use bevy_mod_picking::*;

mod pieces;
use pieces::*;

mod board;
use board::*;

fn main() {
    App::build().add_resource(Msaa {samples: 4})
    .add_resource(WindowDescriptor {title: "Chess".to_string(), width: 1200., height: 800., ..Default::default()})
    .add_plugins(DefaultPlugins)
    .add_plugin(PickingPlugin)
    .add_plugin(BoardPlugin)
    .add_plugin(PiecesPlugin)
    .add_startup_system(setup.system())
    .run();
}

fn setup(commands: &mut Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(), Vec3::new(-7., 20., 4.))),
        ..Default::default()
    }).with(PickSource::default())
    // light
    .spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4., 8., 4.)),
        ..Default::default()
    });
}
