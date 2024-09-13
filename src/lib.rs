pub mod moves;
use moves::{generate_moves, Move, ValidBoardMoves};

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black = -1,
    White = 1,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Active,
    Check,
    Checkmate,
    Stalemate,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValidationResult {
    Valid,
    InvalidPosition,
    InvalidMove,
    InvalidTurn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn to_str(&self) -> String {
        let x = (self.x as u8 + b'a') as char;
        let y = (self.y as u8 + b'1') as char;
        format!("{}{}", x, y)
    }
    pub fn from_str(s: &str) -> Self {
        let s = s.to_lowercase();
        let x = s.chars().nth(0).unwrap() as usize - 'a' as usize;
        let y = s.chars().nth(1).unwrap() as usize - '1' as usize;
        Position { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub position: Position,
}

pub type Board = [Option<Piece>; 64];

pub struct Chess {
    pub board: Board,
    pub turn: Color,
    pub status: Status,
    pub winner: Option<Color>,
    valid_moves: ValidBoardMoves,
}

impl Default for Chess {
    fn default() -> Self {
        Self::new()
    }
}

impl Chess {
    pub fn new() -> Self {
        Chess::from_fen(STARTING_FEN)
    }

    pub fn parse_fen_board(board_str: &str) -> Board {
        let mut board = [None; 64];
        let mut row = 7;
        let mut col = 0;

        for c in board_str.chars() {
            match c {
                '/' => {
                    row -= 1;
                    col = 0;
                }
                '1'..='8' => {
                    let n = c.to_digit(10).unwrap() as usize;
                    for _ in 0..n {
                        board[row * 8 + col] = None;
                    }
                    col += n;
                }
                _ => {
                    let color = if c.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };
                    let piece_type = match c.to_ascii_lowercase() {
                        'k' => PieceType::King,
                        'q' => PieceType::Queen,
                        'r' => PieceType::Rook,
                        'b' => PieceType::Bishop,
                        'n' => PieceType::Knight,
                        'p' => PieceType::Pawn,
                        _ => panic!("Invalid piece type"),
                    };

                    board[row * 8 + col] = Some(Piece {
                        piece_type,
                        color,
                        position: Position { x: col, y: row },
                    });
                    col += 1;
                }
            }
        }

        board
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut parts = fen.split_whitespace();
        let board = Chess::parse_fen_board(parts.next().unwrap());
        let turn = match parts.next().unwrap() {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid turn"),
        };

        let mut chess = Self {
            board,
            turn,
            winner: None,
            status: Status::Active,
            valid_moves: std::array::from_fn(|_| None),
        };
        chess.valid_moves = generate_moves(&chess);

        chess
    }

    pub fn validate_move(&self, from: Position, to: Position) -> ValidationResult {
        println!("{:?} {:?}", from, to);

        let from_index = from.y * 8 + from.x;
        let to_index = to.y * 8 + to.x;

        if from_index >= 64 || to_index >= 64 {
            return ValidationResult::InvalidPosition;
        }

        let piece = self.board[from_index].unwrap();
        if piece.color != self.turn {
            return ValidationResult::InvalidTurn;
        }

        let valid_piece_moves = self.valid_moves[from_index].as_ref().unwrap();

        println!("{:?}", valid_piece_moves);

        if valid_piece_moves.iter().any(|m| m.to == to) {
            ValidationResult::Valid
        } else {
            ValidationResult::InvalidMove
        }
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> ValidationResult {
        let validation_res = self.validate_move(from, to);
        if validation_res != ValidationResult::Valid {
            return validation_res;
        }

        let from_index = from.y * 8 + from.x;
        let to_index = to.y * 8 + to.x;

        let piece = self.board[from_index].unwrap();

        self.board[to_index] = Some(Piece {
            piece_type: piece.piece_type,
            color: piece.color,
            position: to,
        });
        self.board[from_index] = None;

        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        self.valid_moves = generate_moves(self);

        ValidationResult::Valid
    }
}
