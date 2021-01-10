use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_mod_picking::*;

use crate::pieces::*;

pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

pub struct PlayerTurn(pub PieceColor);

impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColor::White)
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>().init_resource::<SelectedPiece>().init_resource::<PlayerTurn>().add_startup_system(create_board.system()).add_system(color_squares.system()).add_system(select_square.system());
    }
}


fn create_board(commands: &mut Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane {size: 1.}));

    // spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: if (i+j+1) % 2 == 0 {
                    materials.add(Color::rgb(1., 0.9, 0.9).into())
                } else {
                    materials.add(Color::rgb(0., 0.1, 0.1).into())
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            }).with(PickableMesh::default())
            .with(Square {x: i, y: j});
        }
    }
}

fn color_squares(pick_state: Res<PickState>, selected_square: Res<SelectedSquare>, mut materials: ResMut<Assets<StandardMaterial>>, query: Query<(Entity, &Square, &Handle<StandardMaterial>)>) {
    // get entity under the cursor if there is one
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };

    for (entity, square, material_handle) in query.iter() {
        // get acutal material
        let material = materials.get_mut(material_handle).unwrap();

        // change matirial color
        material.albedo = if Some(entity) == top_entity {
            Color::rgb(0.8, 0.3, 0.3)
        } else if Some(entity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            Color::rgb(1., 0.9, 0.9)
        } else {
            Color::rgb(0., 0.1, 0.1)
        };
    }
}

fn select_square(commands: &mut Commands, pick_state: Res<PickState>, mouse_button_inputs: Res<Input<MouseButton>>, mut selected_square: ResMut<SelectedSquare>, mut selected_piece: ResMut<SelectedPiece>, mut turn: ResMut<PlayerTurn>, mut app_exit_events: ResMut<Events<AppExit>>, squares_query: Query<&Square>, mut pieces_query: Query<(Entity, &mut Piece, &Children)>) {
    // only run if the lef button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // get the square under the cursor and set it as selected
    if let Some((square_entity, _intersection)) = pick_state.top(Group::default()) {
        // get the actual square
        if let Ok(square) = squares_query.get(*square_entity) {
            // mark it as selected
            selected_square.entity = Some(*square_entity);
            if let Some(selected_piece_entity) = selected_piece.entity {
                let pieces_vec = pieces_query.iter_mut().map(|(_, piece, _)| *piece).collect();
                let pieces_entity_vec: Vec<(Entity, Piece, Vec<Entity>)> = pieces_query.iter_mut().map(|(entity, piece, children)| (entity, *piece, children.iter().map(|entity| *entity).collect())).collect();
                // move selected piece to the selected square
                if let Ok((_piece_entity, mut piece, other_children)) = pieces_query.get_mut(selected_piece_entity) {
                    if piece.is_move_valid((square.x, square.y), pieces_vec) {
                        // check if piece of the opposite color exists on selected square and despawn it
                        for (other_entity, other_piece, other_children) in pieces_entity_vec {
                            if other_piece.x == square.x && other_piece.y == square.y && other_piece.color != piece.color {

                                // if the king is taken end the game
                                if other_piece.piece_type == PieceType::King {
                                    app_exit_events.send(AppExit);
                                }

                                // despawn piece
                                commands.despawn(other_entity);
                                // despawn all children of it
                                for child in other_children {
                                    commands.despawn(child);
                                }
                            }
                        }
                        piece.x = square.x;
                        piece.y = square.y;

                        // change turn
                        turn.0 = match turn.0 {
                            PieceColor::White => PieceColor::Black,
                            PieceColor::Black => PieceColor::White
                        }
                    }
                }
                selected_square.entity = None;
                selected_piece.entity = None;
            } else {
                // select the piece in the currently selected square
                for (piece_entity, piece, _) in pieces_query.iter_mut() {
                    // select piece only if its the right turn
                    if piece.x == square.x && piece.y == square.y && piece.color == turn.0 {
                        // piece_entity is now the entity in the same square
                        selected_piece.entity = Some(piece_entity);
                        break;
                    }
                }
            }
        }
    } else {
        // player clicked outside of the board, deselect everything
        selected_square.entity = None;
        selected_piece.entity = None;
    };
}