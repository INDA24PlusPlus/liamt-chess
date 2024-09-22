pub mod moves;
use moves::{generate_moves, ValidBoardMoves};

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black = -1,
    White = 1,
}

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawType {
    Stalemate,
    FiftyMoveRule,
    ThreefoldRepetition,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Chilling,
    Check(Color),
    Checkmate(Color),
    Draw(DrawType),
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
pub enum CastlingType {
    KingSide(Color),
    QueenSide(Color),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveType {
    Normal,
    Castling(CastlingType),
    Promotion,
    EnPassant,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub piece: Piece,
    pub from: Position,
    pub to: Position,
    pub take_piece: bool,
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
    pub fn from_index(index: usize) -> Self {
        Position {
            x: index % 8,
            y: index / 8,
        }
    }
    pub fn to_index(&self) -> usize {
        self.y * 8 + self.x
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub position: Position,
    prev_positions: Vec<Position>,
}

pub type Board = [Option<Piece>; 64];

fn hash_board(board: &Board) -> String {
    let mut hash = String::new();
    for i in 0..64 {
        if let Some(piece) = &board[i] {
            let piece_hash = match piece.piece_type {
                PieceType::King => match piece.color {
                    Color::White => "1 ",
                    Color::Black => "2 ",
                },
                PieceType::Queen => match piece.color {
                    Color::White => "3 ",
                    Color::Black => "4 ",
                },
                PieceType::Rook => match piece.color {
                    Color::White => "5 ",
                    Color::Black => "6 ",
                },
                PieceType::Bishop => match piece.color {
                    Color::White => "7 ",
                    Color::Black => "8 ",
                },
                PieceType::Knight => match piece.color {
                    Color::White => "9 ",
                    Color::Black => "10 ",
                },
                PieceType::Pawn => match piece.color {
                    Color::White => "11 ",
                    Color::Black => "12 ",
                },
            };
            hash.push_str(piece_hash);

            hash.push_str(&format!("{} ", piece.position.to_str()));
        }
    }
    hash
}

pub struct Chess {
    pub board: Board,
    pub turn: Color,
    pub status: Status,
    pub winner: Option<Color>,
    pub awaiting_promotion_piece: Option<Piece>,
    valid_moves: ValidBoardMoves,
    counter_50_move_rule: u8,
    prev_boards: std::collections::HashMap<String, u8>,
}

impl Default for Chess {
    fn default() -> Self {
        Self::new()
    }
}

impl Chess {
    pub fn new() -> Self {
        Chess::from_fen(STARTING_FEN).unwrap()
    }

    pub fn parse_fen_board(board_str: &str) -> Option<Board> {
        const ARRAY_REPEAT_VALUE: Option<Piece> = None;
        let mut board = [ARRAY_REPEAT_VALUE; 64];
        let mut row = 7;
        let mut col = 0;

        for c in board_str.chars() {
            match c {
                '/' => {
                    if row == 0 {
                        return None;
                    }

                    row -= 1;
                    col = 0;
                }
                '1'..='8' => {
                    let n = c.to_digit(10).unwrap() as usize;
                    for _ in 0..n {
                        let index = row * 8 + col;
                        if index >= 64 {
                            return None;
                        }
                        board[index] = None;
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
                        _ => return None,
                    };

                    let index = row * 8 + col;
                    if index >= 64 {
                        return None;
                    }

                    board[index] = Some(Piece {
                        piece_type,
                        color,
                        position: Position { x: col, y: row },
                        prev_positions: Vec::new(),
                    });
                    col += 1;
                }
            }
        }

        Some(board)
    }

    pub fn from_fen(fen: &str) -> Result<Self, &str> {
        let mut parts = fen.split_whitespace();

        let board_str = parts.next();
        if board_str.is_none() {
            return Err("No board");
        }
        let board = Chess::parse_fen_board(board_str.unwrap());

        if board.is_none() {
            return Err("Invalid board");
        }

        let turn_str = parts.next();
        if turn_str.is_none() {
            return Err("No turn");
        }
        let turn = match turn_str.unwrap() {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid turn"),
        };

        let mut chess = Self {
            board: board.unwrap(),
            turn,
            winner: None,
            status: Status::Chilling,
            valid_moves: std::array::from_fn(|_| Vec::new()),
            awaiting_promotion_piece: None,
            counter_50_move_rule: 0,
            prev_boards: std::collections::HashMap::new(),
        };
        chess.update(false);

        Ok(chess)
    }

    fn get_move_type(&self, from: Position, to: Position) -> MoveType {
        let from_index = from.y * 8 + from.x;

        let piece = self.board[from_index].as_ref();

        if piece.is_none() {
            return MoveType::Normal;
        }

        if piece.is_none() {
            return MoveType::Normal;
        }

        let piece = piece.unwrap();

        let valid_piece_moves = self.valid_moves[from_index].clone();

        let mov = valid_piece_moves.iter().find(|m| m.to == to);

        if mov.is_none() {
            return MoveType::Normal;
        }

        let mov = mov.unwrap();

        if mov.piece.piece_type == PieceType::Pawn
            && ((to.y == 0 && piece.color == Color::White)
                || (to.y == 7 && piece.color == Color::Black))
        {
            return MoveType::Promotion;
        }

        let delta = (from.x as i8 - to.x as i8).abs();

        if piece.piece_type == PieceType::King && (delta == 3 || delta == 4) {
            if to.x == 0 {
                return MoveType::Castling(CastlingType::QueenSide(piece.color));
            } else if to.x == 7 {
                return MoveType::Castling(CastlingType::KingSide(piece.color));
            }
        }

        if mov.piece.piece_type == PieceType::Pawn
            && (from.x as i8 - to.x as i8).abs() == 1
            && self.board[to.y * 8 + to.x].is_none()
        {
            return MoveType::EnPassant;
        }

        MoveType::Normal
    }

    fn move_piece_on_board(&self, board: &Board, from: Position, to: Position) -> (Board, bool) {
        let from_index = from.y * 8 + from.x;
        let to_index = to.y * 8 + to.x;

        let piece = board[from_index].as_ref().unwrap().clone();
        let mut board = board.clone();

        let mut prev_positions = piece.prev_positions.clone();
        prev_positions.push(piece.position);

        let mut capture = board[to_index].is_some();

        let move_type = self.get_move_type(from, to);

        match move_type {
            MoveType::Castling(t) => {
                board = self.perform_castling(board, t);
            }
            MoveType::EnPassant => {
                board[to_index] = Some(Piece {
                    piece_type: piece.piece_type,
                    color: piece.color,
                    position: to,
                    prev_positions,
                });
                board[from_index] = None;
                board[to_index - 8 * piece.color as usize] = None;
                capture = true;
            }
            _ => {
                board[to_index] = Some(Piece {
                    piece_type: piece.piece_type,
                    color: piece.color,
                    position: to,
                    prev_positions,
                });
                board[from_index] = None;
            }
        }

        (board, capture)
    }

    pub fn generate_valid_moves(&self) -> [Vec<Move>; 64] {
        const ARRAY_REPEAT_VALUE: Vec<Move> = Vec::new();
        let mut moves = [ARRAY_REPEAT_VALUE; 64];

        let pseudo_possible_moves = generate_moves(&self.board);

        for (i, tile) in pseudo_possible_moves.iter().enumerate() {
            for mov in tile {
                let validate_board = self
                    .move_piece_on_board(&self.board, mov.piece.position, mov.to)
                    .0;

                let new_valid_moves = generate_moves(&validate_board);

                if mov.piece.color != self.turn {
                    continue;
                }

                let status = self.get_board_status(&validate_board, &new_valid_moves, self.turn);

                let mov_type = self.get_move_type(mov.piece.position, mov.to);

                if let MoveType::Castling(t) = mov_type {
                    if !self.check_castling_possible(t) {
                        continue;
                    }
                }

                if status != Status::Check(self.turn) && status != Status::Checkmate(self.turn) {
                    moves[i].push(mov.clone());
                }
            }
        }

        moves
    }

    pub fn validate_move(&self, from: Position, to: Position) -> ValidationResult {
        if self.status != Status::Chilling && !matches!(self.status, Status::Check(_)) {
            return ValidationResult::InvalidTurn;
        }

        let from_index = from.y * 8 + from.x;
        let to_index = to.y * 8 + to.x;

        if from_index >= 64 || to_index >= 64 {
            return ValidationResult::InvalidPosition;
        }

        let piece = &self.board[from_index];

        if piece.is_none() {
            return ValidationResult::InvalidPosition;
        }

        let piece = piece.as_ref().unwrap();

        if piece.color != self.turn {
            return ValidationResult::InvalidTurn;
        }

        let move_type = self.get_move_type(from, to);

        if let MoveType::Castling(t) = move_type {
            if !self.check_castling_possible(t) {
                return ValidationResult::InvalidMove;
            }
        }

        let valid_piece_moves = self.valid_moves[from_index].clone();

        if !valid_piece_moves.iter().any(|m| m.to == to) {
            return ValidationResult::InvalidMove;
        }

        let validate_board = self.move_piece_on_board(&self.board, from, to).0;

        let new_valid_moves = generate_moves(&validate_board);

        ValidationResult::Valid(self.get_board_status(
            &validate_board,
            &new_valid_moves,
            !self.turn,
        ))
    }

    fn cant_move(&self, board: &Board, valid_moves: &ValidBoardMoves, color: Color) -> bool {
        for mov in valid_moves.iter() {
            for m in mov.iter() {
                if m.piece.color != color {
                    continue;
                }

                let validate_board = self.move_piece_on_board(board, m.piece.position, m.to).0;

                let new_valid_moves = generate_moves(&validate_board);

                let is_check = self.check_check(&validate_board, &new_valid_moves);

                if !is_check.is_some_and(|c| c.contains(&color)) {
                    return false;
                }
            }
        }

        true
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> ValidationResult {
        let validation_res = self.validate_move(from, to);
        match validation_res {
            ValidationResult::Valid(_) => {
                if validation_res == ValidationResult::Valid(Status::Check(self.turn)) {
                    return ValidationResult::InvalidMove;
                }

                let from_index = from.y * 8 + from.x;
                let piece = self.board[from_index].as_ref().unwrap().clone();

                let board_res = self.move_piece_on_board(&self.board, from, to);
                self.board = board_res.0;
                let capture = board_res.1;

                if piece.piece_type == PieceType::Pawn || capture {
                    self.counter_50_move_rule = 0;
                } else {
                    self.counter_50_move_rule += 1;
                }

                self.update(true);

                ValidationResult::Valid(self.status)
            }
            _ => validation_res,
        }
    }

    fn update(&mut self, switch_turn: bool) {
        self.awaiting_promotion_piece = self.check_for_promotion();
        if switch_turn {
            self.turn = !self.turn;
        }

        self.valid_moves = generate_moves(&self.board);

        let board_status = self.get_board_status(&self.board, &self.valid_moves, self.turn);

        self.status = board_status;

        if let Status::Checkmate(color) = self.status {
            self.winner = Some(!color);
        } else if self.status == Status::Draw(DrawType::Stalemate) {
            self.winner = None;
        }

        let three_fold_status = self.threefold_rule(true);
        if self.counter_50_move_rule >= 100 {
            self.status = Status::Draw(DrawType::FiftyMoveRule);
        } else if three_fold_status == Status::Draw(DrawType::ThreefoldRepetition) {
            self.status = Status::Draw(DrawType::ThreefoldRepetition);
        } else if self.awaiting_promotion_piece.is_some() {
            self.status = Status::AwaitingPromotion;
        }
    }

    fn get_board_status(
        &self,
        board: &Board,
        valid_moves: &ValidBoardMoves,
        turn: Color,
    ) -> Status {
        let is_check = self.check_check(board, valid_moves);

        let im_stuck = self.cant_move(board, valid_moves, turn);
        let opponent_stuck = self.cant_move(board, valid_moves, !turn);

        if is_check.is_some() {
            let is_check = is_check.unwrap();

            // If the player is in check and can't move
            if im_stuck && is_check.contains(&turn) {
                return Status::Checkmate(turn);
            }

            // If the opponent is in check and its player's turn
            if opponent_stuck && is_check.contains(&!turn) {
                return Status::Checkmate(!turn);
            }

            if is_check.contains(&turn) {
                return Status::Check(turn);
            }

            return Status::Check(!turn);
        } else if im_stuck {
            return Status::Draw(DrawType::Stalemate);
        }

        Status::Chilling
    }

    pub fn is_check(&self) -> Option<Color> {
        let check_res = self.check_check(&self.board, &self.valid_moves);
        match check_res {
            Some(check_colors) => Some(check_colors[0]),
            None => None,
        }
    }

    pub fn promote_piece(&mut self, piece_type: PieceType) -> Option<Status> {
        if self.status != Status::AwaitingPromotion || self.awaiting_promotion_piece.is_none() {
            return None;
        }

        match piece_type {
            PieceType::King | PieceType::Pawn => return None,
            _ => {}
        }

        let piece = self.awaiting_promotion_piece.clone().unwrap();

        let index = piece.position.y * 8 + piece.position.x;

        self.board[index] = Some(Piece {
            piece_type,
            color: piece.color,
            position: piece.position,
            prev_positions: piece.prev_positions.clone(),
        });

        self.awaiting_promotion_piece = None;

        self.update(true);

        Some(self.status)
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

            let king = king.unwrap().as_ref().unwrap();

            let mut is_check = false;

            valid_moves.iter().for_each(|n| {
                if n.iter().any(|m| {
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

    fn check_for_promotion(&self) -> Option<Piece> {
        if self.awaiting_promotion_piece.is_some() {
            return self.awaiting_promotion_piece.clone();
        }

        for piece in self.board.iter().flatten() {
            if piece.piece_type == PieceType::Pawn
                && ((piece.color == Color::White && piece.position.y == 7)
                    || (piece.color == Color::Black && piece.position.y == 0))
            {
                return Some(piece.clone());
            }
        }

        None
    }

    fn check_castling_possible(&self, castling_type: CastlingType) -> bool {
        if self.status != Status::Chilling {
            return false;
        }

        let (side, row) = match castling_type {
            CastlingType::QueenSide(c) => (0, if c == Color::White { 0 } else { 7 }),
            CastlingType::KingSide(c) => (1, if c == Color::White { 0 } else { 7 }),
        };

        if (row == 0 && self.turn == Color::Black) || (row == 7 && self.turn == Color::White) {
            return false;
        }

        let king_index = row * 8 + 4;
        let rook_index = row * 8 + (7 * side);

        let king = &self.board[king_index];
        let rook = &self.board[rook_index];

        if king.is_none() || rook.is_none() {
            return false;
        }

        let king = king.as_ref().unwrap();
        let rook = rook.as_ref().unwrap();

        if !rook.prev_positions.is_empty() || !rook.prev_positions.is_empty() {
            return false;
        }

        let mut x = king.position.x as i8;
        let y = king.position.y as i8;

        let tiles_to_check = if side == 0 { 3 } else { 2 };

        for _ in 0..tiles_to_check {
            x += if side == 0 { -1 } else { 1 };
            let tile = &self.board[(y * 8 + x) as usize];
            if tile.is_some() {
                return false;
            }
        }

        for i in 1..3 {
            let mut validate_board: [Option<Piece>; 64] = self.board.clone();

            let king_to = Position {
                x: ((king.position.x as i8) + (if side == 0 { -i } else { i })) as usize,
                y: king.position.y,
            };
            let rook_to = Position {
                x: ((rook.position.x as i8) + (if side == 0 { 3 } else { -2 })) as usize,
                y: rook.position.y,
            };
            let king_to_index = king_to.y * 8 + king_to.x;
            let rook_to_index = rook_to.y * 8 + rook_to.x;
            let king_from_index = king.position.y * 8 + king.position.x;
            let rook_from_index = rook.position.y * 8 + rook.position.x;

            let mut king_prev_positions = king.prev_positions.clone();
            king_prev_positions.push(king.position);

            let mut rook_prev_positions = rook.prev_positions.clone();
            rook_prev_positions.push(rook.position);

            validate_board[rook_to_index] = Some(Piece {
                piece_type: rook.piece_type,
                color: rook.color,
                position: rook_to,
                prev_positions: rook_prev_positions,
            });

            validate_board[rook_from_index] = None;

            validate_board[king_to_index] = Some(Piece {
                piece_type: king.piece_type,
                color: king.color,
                position: king_to,
                prev_positions: king_prev_positions,
            });

            validate_board[king_from_index] = None;

            let new_valid_moves = generate_moves(&validate_board);

            let new_status = self.get_board_status(&validate_board, &new_valid_moves, self.turn);

            let valid = match new_status {
                Status::Check(c) => c != self.turn,
                Status::Checkmate(c) => c != self.turn,
                Status::Draw(_) => false,
                _ => true,
            };
            if !valid {
                return false;
            }
        }

        true
    }

    fn perform_castling(&self, board: Board, castling_type: CastlingType) -> Board {
        let mut board = board.clone();

        let (side, row) = match castling_type {
            CastlingType::QueenSide(c) => (0, if c == Color::White { 0 } else { 7 }),
            CastlingType::KingSide(c) => (1, if c == Color::White { 0 } else { 7 }),
        };

        let king_index = row * 8 + 4;
        let rook_index = row * 8 + (7 * side);

        let king = board[king_index].as_ref().unwrap().clone();
        let rook = board[rook_index].as_ref().unwrap().clone();

        let king_to = Position {
            x: ((king.position.x as i8) + (if side == 0 { -2 } else { 2 })) as usize,
            y: king.position.y,
        };
        let rook_to = Position {
            x: ((rook.position.x as i8) + (if side == 0 { 3 } else { -2 })) as usize,
            y: rook.position.y,
        };
        let king_to_index = king_to.y * 8 + king_to.x;
        let rook_to_index = rook_to.y * 8 + rook_to.x;
        let king_from_index = king.position.y * 8 + king.position.x;
        let rook_from_index = rook.position.y * 8 + rook.position.x;

        let mut king_prev_positions = king.prev_positions.clone();
        king_prev_positions.push(king.position);

        let mut rook_prev_positions = rook.prev_positions.clone();
        rook_prev_positions.push(rook.position);

        board[king_to_index] = Some(Piece {
            piece_type: king.piece_type,
            color: king.color,
            position: king_to,
            prev_positions: king_prev_positions,
        });

        board[king_from_index] = None;

        board[rook_to_index] = Some(Piece {
            piece_type: rook.piece_type,
            color: rook.color,
            position: rook_to,
            prev_positions: rook_prev_positions,
        });

        board[rook_from_index] = None;

        board
    }

    pub fn threefold_rule(&mut self, modify: bool) -> Status {
        let hash = hash_board(&self.board);

        if modify {
            let entry = self.prev_boards.entry(hash.clone()).or_insert(0);
            *entry += 1;
        }

        let count = self.prev_boards.get(&hash);

        if count.is_some_and(|c| *c >= 3) {
            return Status::Draw(DrawType::ThreefoldRepetition);
        }
        self.status
    }
}
