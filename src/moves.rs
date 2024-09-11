use crate::{Chess, Piece, PieceType, Position, ValidationResult};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    pub piece: Piece,
    pub from: Position,
    pub to: Position,
    pub take_piece: bool,
}

pub type ValidBoardMoves = [Option<Vec<Move>>; 64];

//pub type PossibleMove = (i8, i8, Vec<(i8, i8)>);
#[derive(Clone, Debug, PartialEq)]
pub struct PossibleMove {
    pub x: i8,
    pub y: i8,
    pub pre_moves: Option<Vec<(i8, i8)>>,
}

fn valid_position(x: i8, y: i8) -> bool {
    (0..8).contains(&x) && (0..8).contains(&y)
}

pub fn generate_moves(chess: &Chess) -> ValidBoardMoves {
    //hacky solution to get around bitchy compiler
    let mut valid_moves = std::array::from_fn(|_| None);
    for i in 0..64 {
        let tile = chess.board[i];

        match tile {
            Some(piece) => {
                let res = match piece.piece_type {
                    PieceType::King => valid_moves_king(chess, piece),
                    PieceType::Queen => valid_moves_queen(chess, piece),
                    PieceType::Rook => valid_moves_rook(chess, piece),
                    PieceType::Bishop => valid_moves_bishop(chess, piece),
                    PieceType::Knight => valid_moves_knight(chess, piece),
                    PieceType::Pawn => valid_moves_pawn(chess, piece),
                };
                valid_moves[i] = Some(res);
            }
            None => valid_moves[i] = None,
        };
    }

    valid_moves
}

fn validate_possible_moves(
    chess: &Chess,
    piece: Piece,
    possible_moves: Vec<PossibleMove>,
) -> Vec<Move> {
    let mut valid_moves = Vec::new();

    for possible_move in possible_moves.iter() {
        let x = possible_move.x;
        let y = possible_move.y;

        match &possible_move.pre_moves {
            Some(pre_moves) => {
                let mut valid = true;
                for (px, py) in pre_moves.iter() {
                    let tile = chess.board[(py * 8 + px) as usize];
                    if tile.is_some() {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    continue;
                }
            }
            None => {}
        }

        if valid_position(x, y) {
            let tile = chess.board[(y * 8 + x) as usize];
            match tile {
                Some(p) => {
                    if p.color != piece.color {
                        valid_moves.push(Move {
                            piece,
                            from: piece.position,
                            to: Position {
                                x: x as usize,
                                y: y as usize,
                            },
                            take_piece: true,
                        });
                    }
                }
                None => {
                    valid_moves.push(Move {
                        piece,
                        from: piece.position,
                        to: Position {
                            x: x as usize,
                            y: y as usize,
                        },
                        take_piece: false,
                    });
                }
            }
        }
    }

    valid_moves
}

fn generate_directional_possible_moves(
    current_pos: Position,
    directions: Vec<(i8, i8)>,
) -> Vec<PossibleMove> {
    let mut possible_moves = Vec::new();

    for dir in directions.iter() {
        let mut prev_moves: Vec<(i8, i8)> = Vec::new();
        for i in 1..8 {
            let x = current_pos.x as i8 + dir.0 * i;
            let y = current_pos.y as i8 + dir.1 * i;

            if !valid_position(x, y) {
                continue;
            };

            possible_moves.push(PossibleMove {
                x,
                y,
                pre_moves: Some(prev_moves.clone()),
            });

            prev_moves.push((x, y));
        }
    }

    possible_moves
}

fn relative_to_absolute_moves(piece: Piece, relative_moves: Vec<(i8, i8)>) -> Vec<PossibleMove> {
    relative_moves
        .iter()
        .map(|(x, y)| PossibleMove {
            x: piece.position.x as i8 + x,
            y: piece.position.y as i8 + y,
            pre_moves: None,
        })
        .collect()
}

fn valid_moves_king(chess: &Chess, piece: Piece) -> Vec<Move> {
    let moves = vec![
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, 1),
        (-1, 1),
        (-1, -1),
    ];

    let possible_moves = relative_to_absolute_moves(piece, moves);

    validate_possible_moves(chess, piece, possible_moves)
}
fn valid_moves_queen(chess: &Chess, piece: Piece) -> Vec<Move> {
    let directions = vec![
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, 1),
        (-1, 1),
        (-1, -1),
    ];

    let possible_moves = generate_directional_possible_moves(piece.position, directions);

    validate_possible_moves(chess, piece, possible_moves)
}
fn valid_moves_rook(chess: &Chess, piece: Piece) -> Vec<Move> {
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let possible_moves = generate_directional_possible_moves(piece.position, directions);

    validate_possible_moves(chess, piece, possible_moves)
}
fn valid_moves_bishop(chess: &Chess, piece: Piece) -> Vec<Move> {
    let directions = vec![(1, 1), (-1, 1), (-1, 1), (-1, -1)];

    let possible_moves = generate_directional_possible_moves(piece.position, directions);

    validate_possible_moves(chess, piece, possible_moves)
}
fn valid_moves_knight(chess: &Chess, piece: Piece) -> Vec<Move> {
    let moves = vec![
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
    ];

    let possible_moves = relative_to_absolute_moves(piece, moves);

    validate_possible_moves(chess, piece, possible_moves)
}
fn valid_moves_pawn(chess: &Chess, piece: Piece) -> Vec<Move> {
    Vec::new()
}

/* pub fn validate_piece_move(chess: &Chess, piece: Piece, to: Position) -> ValidationResult {
    match piece.piece_type {
        PieceType::King => validate_king(chess, piece, to),
        PieceType::Queen => validate_queen(chess, piece, to),
        PieceType::Rook => validate_rook(chess, piece, to),
        PieceType::Bishop => validate_bishop(chess, piece, to),
        PieceType::Knight => validate_knight(chess, piece, to),
        PieceType::Pawn => validate_pawn(chess, piece, to),
    }
}

fn validate_king(chess: &Chess, piece: Piece, to: Position) -> ValidationResult {
    let moves = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    ValidationResult::Valid
}

fn validate_queen(chess: &Chess, piece: Piece, to: Position) -> ValidationResult {
    ValidationResult::Valid
}

fn validate_rook(chess: &Chess, piece: Piece, to: Position) -> ValidationResult {
    ValidationResult::Valid
}

fn validate_bishop(chess: &Chess, piece: Piece, to: Position) -> ValidationResult {
    ValidationResult::Valid
}

fn validate_knight(chess: &Chess, piece: Piece, to: Position) -> ValidationResult {
    ValidationResult::Valid
}

fn validate_pawn(chess: &Chess, piece: Piece, to: Position) -> ValidationResult {
    ValidationResult::Valid
} */
