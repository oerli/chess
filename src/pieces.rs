use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // current position
    pub x: u8,
    pub y: u8
}

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_pieces.system()).add_system(move_pieces.system());
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

    let white_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0.2, 0.2, 0.2).into());

    spawn_rook(commands, white_material.clone(), PieceColor::White, rook_handle.clone(), (0, 0));
    spawn_knight(commands, white_material.clone(), PieceColor::White, knight_1_handle.clone(), knight_2_handle.clone(), (0, 1));
    spawn_bishop(commands, white_material.clone(), PieceColor::White, bishop_handle.clone(), (0, 2));
    spawn_queen(commands, white_material.clone(), PieceColor::White, queen_handle.clone(), (0, 3));
    spawn_king(commands, white_material.clone(), PieceColor::White, king_handle.clone(), king_cross_handle.clone(), (0, 4));
    spawn_bishop(commands, white_material.clone(), PieceColor::White, bishop_handle.clone(), (0, 5));
    spawn_knight(commands, white_material.clone(), PieceColor::White, knight_1_handle.clone(), knight_2_handle.clone(), (0, 6));
    spawn_rook(commands, white_material.clone(), PieceColor::White, rook_handle.clone(), (0, 7));
    
    for i in 0..8 {
        spawn_pawn(commands, white_material.clone(), PieceColor::White, pawn_handle.clone(), (1, i));
    }

    spawn_rook(commands, black_material.clone(), PieceColor::Black, rook_handle.clone(), (7, 0));
    spawn_knight(commands, black_material.clone(), PieceColor::Black, knight_1_handle.clone(), knight_2_handle.clone(), (7, 1));
    spawn_bishop(commands, black_material.clone(), PieceColor::Black, bishop_handle.clone(), (7, 2));
    spawn_queen(commands, black_material.clone(), PieceColor::Black, queen_handle.clone(), (7, 3));
    spawn_king(commands, black_material.clone(), PieceColor::Black, king_handle.clone(), king_cross_handle.clone(), (7, 4));
    spawn_bishop(commands, black_material.clone(), PieceColor::Black, bishop_handle.clone(), (7, 5));
    spawn_knight(commands, black_material.clone(), PieceColor::Black, knight_1_handle.clone(), knight_2_handle.clone(), (7, 6));
    spawn_rook(commands, black_material.clone(), PieceColor::Black, rook_handle.clone(), (7, 7));
    
    for i in 0..8 {
        spawn_pawn(commands, black_material.clone(), PieceColor::Black, pawn_handle.clone(), (6, i));
    }

}

fn spawn_king(commands: &mut Commands, material: Handle<StandardMaterial>, piece_color: PieceColor, mesh: Handle<Mesh>, mesh_cross: Handle<Mesh>, position: (u8, u8)) {
    // spwan parent entity
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32)),
        ..Default::default()
    })
    .with(Piece {
        color: piece_color,
        piece_type: PieceType::King,
        x: position.0,
        y: position.1
    })
    // add children to the parent
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
        parent.spawn(PbrBundle {
            mesh: mesh_cross,
            material,
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
    });
}

fn spawn_knight(commands: &mut Commands, material: Handle<StandardMaterial>, piece_color: PieceColor, mesh_1: Handle<Mesh>, mesh_2: Handle<Mesh>, position: (u8, u8)) {
    // spwan parent entity
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32)),
        ..Default::default()
    })
    .with(Piece {
        color: piece_color,
        piece_type: PieceType::Knight,
        x: position.0,
        y: position.1
    })
    // add children to the parent
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: mesh_1,
            material: material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
        parent.spawn(PbrBundle {
            mesh: mesh_2,
            material,
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
    });
}

fn spawn_queen(commands: &mut Commands, material: Handle<StandardMaterial>, piece_color: PieceColor, mesh: Handle<Mesh>, position: (u8, u8)) {
    // spwan parent entity
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32)),
        ..Default::default()
    })
    .with(Piece {
        color: piece_color,
        piece_type: PieceType::Queen,
        x: position.0,
        y: position.1
    })
    // add children to the parent
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
    });
}

fn spawn_bishop(commands: &mut Commands, material: Handle<StandardMaterial>, piece_color: PieceColor, mesh: Handle<Mesh>, position: (u8, u8)) {
    // spwan parent entity
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32)),
        ..Default::default()
    })
    .with(Piece {
        color: piece_color,
        piece_type: PieceType::Bishop,
        x: position.0,
        y: position.1
    })
    // add children to the parent
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
    });
}

fn spawn_rook(commands: &mut Commands, material: Handle<StandardMaterial>, piece_color: PieceColor, mesh: Handle<Mesh>, position: (u8, u8)) {
    // spwan parent entity
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32)),
        ..Default::default()
    })
    .with(Piece {
        color: piece_color,
        piece_type: PieceType::Rook,
        x: position.0,
        y: position.1
    })
    // add children to the parent
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
    });
}

fn spawn_pawn(commands: &mut Commands, material: Handle<StandardMaterial>, piece_color: PieceColor, mesh: Handle<Mesh>, position: (u8, u8)) {
    // spwan parent entity
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(Vec3::new(position.0 as f32, 0., position.1 as f32)),
        ..Default::default()
    })
    .with(Piece {
        color: piece_color,
        piece_type: PieceType::Pawn,
        x: position.0,
        y: position.1
    })
    // add children to the parent
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
    });
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        // get the direction to move in
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;
    
        // only move if the piece isn't already there
        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}