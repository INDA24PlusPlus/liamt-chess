use crate::{Chess, Piece, PieceType, Position, ValidationResult};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    pub piece: Piece,
    pub from: Position,
    pub to: Position,
    pub take_piece: bool,
}

pub type ValidBoardMoves = [Option<Vec<Move>>; 64];

pub fn generate_moves(chess: &Chess) -> ValidBoardMoves {
    //hacky solution to get around bitchy compiler
    let mut valid_moves = std::array::from_fn(|_| None);
    for i in 0..8 {
        for j in 0..8 {
            let tile = chess.board[(7 - i) * 8 + j];

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
                    valid_moves[i * 8 + j] = Some(res);
                }
                None => valid_moves[i * 8 + j] = None,
            };
        }
    }

    valid_moves
}
fn valid_moves_king(chess: &Chess, piece: Piece) -> Vec<Move> {
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
    Vec::new()
}
fn valid_moves_queen(chess: &Chess, piece: Piece) -> Vec<Move> {
    Vec::new()
}
fn valid_moves_rook(chess: &Chess, piece: Piece) -> Vec<Move> {
    Vec::new()
}
fn valid_moves_bishop(chess: &Chess, piece: Piece) -> Vec<Move> {
    Vec::new()
}
fn valid_moves_knight(chess: &Chess, piece: Piece) -> Vec<Move> {
    let moves = [
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
    ];
    Vec::new()
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
