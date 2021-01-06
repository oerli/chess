use bevy::prelude::*;

mod pieces;
use pieces::*;

fn main() {
    App::build().add_resource(Msaa {samples: 4})
    .add_resource(WindowDescriptor {title: "Chess".to_string(), width: 1200., height: 800., ..Default::default()})
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_startup_system(create_board.system())
    .add_startup_system(create_pieces.system())
    .run();
}

fn setup(commands: &mut Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(), Vec3::new(-7., 20., 4.))),
        ..Default::default()
    })
    // light
    .spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4., 8., 4.)),
        ..Default::default()
    });
}

fn create_board(commands: &mut Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane {size: 1.}));
    let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

    // spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: if (i+j+1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            });
        }
    }
}

fn create_pieces(commands: &mut Commands, asset_server: ResMut<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let king_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    spawn_rook(commands, white_material.clone(), rook_handle.clone(), Vec3::new(0., 0., 0.));
    spawn_knight(commands, white_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(0., 0., 1.));
    spawn_bishop(commands, white_material.clone(), bishop_handle.clone(), Vec3::new(0., 0., 2.));
    spawn_queen(commands, white_material.clone(), queen_handle.clone(), Vec3::new(0., 0., 3.));
    spawn_king(commands, white_material.clone(), king_handle.clone(), king_cross_handle.clone(), Vec3::new(0., 0., 4.));
    spawn_bishop(commands, white_material.clone(), bishop_handle.clone(), Vec3::new(0., 0., 5.));
    spawn_knight(commands, white_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(0., 0., 6.));
    spawn_rook(commands, white_material.clone(), rook_handle.clone(), Vec3::new(0., 0., 7.));
    
    for i in 0..8 {
        spawn_pawn(commands, white_material.clone(), pawn_handle.clone(), Vec3::new(1., 0., i as f32));
    }

    spawn_rook(commands, black_material.clone(), rook_handle.clone(), Vec3::new(7., 0., 0.));
    spawn_knight(commands, black_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(7., 0., 1.));
    spawn_bishop(commands, black_material.clone(), bishop_handle.clone(), Vec3::new(7., 0., 2.));
    spawn_queen(commands, black_material.clone(), queen_handle.clone(), Vec3::new(7., 0., 3.));
    spawn_king(commands, black_material.clone(), king_handle.clone(), king_cross_handle.clone(), Vec3::new(7., 0., 4.));
    spawn_bishop(commands, black_material.clone(), bishop_handle.clone(), Vec3::new(7., 0., 5.));
    spawn_knight(commands, black_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(7., 0., 6.));
    spawn_rook(commands, black_material.clone(), rook_handle.clone(), Vec3::new(7., 0., 7.));
    
    for i in 0..8 {
        spawn_pawn(commands, black_material.clone(), pawn_handle.clone(), Vec3::new(6., 0., i as f32));
    }

}