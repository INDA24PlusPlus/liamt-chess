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
    AwaitingPromotion,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValidationResult {
    Valid(Status),
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
        chess.valid_moves = generate_moves(&chess.board);

        chess
    }

    pub fn validate_move(&self, from: Position, to: Position) -> ValidationResult {
        println!("{:?} {:?}", from, to);

        let from_index = from.y * 8 + from.x;
        let to_index = to.y * 8 + to.x;

        if from_index >= 64 || to_index >= 64 {
            return ValidationResult::InvalidPosition;
        }

        let piece = self.board[from_index];

        if piece.is_none() {
            return ValidationResult::InvalidPosition;
        }

        let piece = piece.unwrap();

        if piece.color != self.turn {
            return ValidationResult::InvalidTurn;
        }

        let valid_piece_moves = self.valid_moves[from_index].as_ref().unwrap();

        //println!("{:?}", valid_piece_moves);

        if !valid_piece_moves.iter().any(|m| m.to == to) {
            return ValidationResult::InvalidMove;
        }

        let mut validate_board: [Option<Piece>; 64] = self.board;

        validate_board[to_index] = Some(Piece {
            piece_type: piece.piece_type,
            color: piece.color,
            position: to,
        });
        validate_board[from_index] = None;

        let new_valid_moves = generate_moves(&validate_board);

        let is_check = self.check_check(&validate_board, &new_valid_moves);

        if is_check.is_some() {
            let is_check = is_check.unwrap();
            if is_check.contains(&self.turn) {
                return ValidationResult::InvalidMove;
            }
            return ValidationResult::Valid(Status::Check);
        }

        ValidationResult::Valid(Status::Active)
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> ValidationResult {
        let validation_res = self.validate_move(from, to);
        match validation_res {
            ValidationResult::Valid(status) => {
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

                self.status = status;

                self.valid_moves = generate_moves(&self.board);

                ValidationResult::Valid(self.status)
            }
            _ => validation_res,
        }
    }

    pub fn is_check(&self) -> Option<Color> {
        let check_res = self.check_check(&self.board, &self.valid_moves);
        if check_res.is_none() {
            return None;
        }

        //self.board can't be in double check
        Some(check_res.unwrap()[0])
    }

    fn check_check(&self, board: &Board, valid_moves: &ValidBoardMoves) -> Option<Vec<Color>> {
        let colors = [Color::White, Color::Black];
        let mut check_colors: Vec<Color> = Vec::new();
        for color in colors.iter() {
            let king = board.iter().find(|p| {
                if let Some(p) = p {
                    return p.piece_type == PieceType::King && p.color == *color;
                }
                false
            });

            if king.is_none() {
                continue;
            }

            let king = king.unwrap().unwrap();

            let mut is_check = false;

            valid_moves.iter().for_each(|n| {
                if n.is_none() {
                    return;
                }

                if n.as_ref().unwrap().iter().any(|m| {
                    if m.piece.color != king.color && m.to == king.position {
                        return true;
                    }

                    false
                }) {
                    is_check = true;
                }
            });

            if is_check {
                check_colors.push(king.color);
            }
        }

        if check_colors.is_empty() {
            return None;
        }
        Some(check_colors)
    }
}
