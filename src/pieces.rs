use crate::{Chess, Piece, PieceType, Position, ValidationResult};

pub fn validate_piece_move(chess: &Chess, piece: Piece, to: Position) -> ValidationResult {
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
}
