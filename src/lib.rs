const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn to_str(&self) -> String {
        let x = (self.x as u8 + b'a') as char;
        let y = (self.y as u8 + b'1') as char;
        format!("{}{}", x, y)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub position: Position,
}

pub type Board = [Option<Piece>; 64];

pub struct Fen {}
impl Fen {
    pub fn parse_fen(fen_str: &str) -> Board {
        let mut board = [None; 64];
        let mut row = 7;
        let mut col = 0;

        for c in fen_str.chars() {
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

        return board;
    }
}

pub struct Chess {
    board: Board,
    turn: Color,
}

impl Chess {
    pub fn new() -> Self {
        Self {
            board: [None; 64],
            turn: Color::White,
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
