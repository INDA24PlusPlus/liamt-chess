#[cfg(test)]
mod tests {
    use chess::*;

    #[test]
    fn check_turn_on_default_board() {
        let chess = Chess::new();
        assert_eq!(chess.turn, Color::White);
    }

    #[test]
    fn check_2_kings_on_default_board() {
        let chess = Chess::new();
        let kings = chess
            .board
            .iter()
            .filter(|&p| match p {
                Some(piece) => piece.piece_type == PieceType::King,
                None => false,
            })
            .collect::<Vec<_>>();
        assert_eq!(kings.len(), 2);
    }

    #[test]
    fn check_invalid_fen() {
        assert!(Chess::from_fen("k7/8/8/8/8/8/8/1R6").is_err()); // missing turn
        assert!(Chess::from_fen("k7/8/8/8/8/8/8/1R6 a").is_err()); // invalid turn
        assert!(Chess::from_fen("k7/8/8/8/8/8/8/8/8 w").is_err()); // too many rows
        assert!(Chess::from_fen("k7/8/8/8/8/8/8/9 w").is_err()); // too many columns
    }

    #[test]
    fn check_check() {
        let mut chess = Chess::from_fen("k7/8/8/8/8/8/8/1R6 w").unwrap();
        chess.move_piece(Position::from_str("b1"), Position::from_str("a1"));
        assert_eq!(chess.status, Status::Check(Color::Black));

        let mut chess = Chess::from_fen("7k/8/7N/8/8/8/8/8 w").unwrap();
        chess.move_piece(Position::from_str("h6"), Position::from_str("f7"));
        assert_eq!(chess.status, Status::Check(Color::Black));
    }

    #[test]
    fn check_valid_moves() {
        let mut chess = Chess::from_fen("k7/8/8/8/8/8/8/1Q6 w").unwrap();

        let res = chess.move_piece(Position::from_str("a8"), Position::from_str("a7"));
        assert!(matches!(res, ValidationResult::InvalidTurn));

        let res = chess.move_piece(Position::from_str("b1"), Position::from_str("a1"));
        assert_eq!(res, ValidationResult::Valid(Status::Check(Color::Black)));

        let res = chess.move_piece(Position::from_str("a8"), Position::from_str("a7"));
        assert!(matches!(res, ValidationResult::InvalidMove));
    }

    #[test]
    fn check_stalemate() {
        let mut chess = Chess::from_fen("k7/8/2Q5/8/8/8/8/K7 w").unwrap();
        chess.move_piece(Position::from_str("c6"), Position::from_str("b6"));
        assert_eq!(chess.status, Status::Draw(DrawType::Stalemate));

        let mut chess = Chess::from_fen("1B6/8/8/3k4/8/B7/8/2R1R3 w").unwrap();
        chess.move_piece(Position::from_str("a3"), Position::from_str("b2"));
        assert_eq!(chess.status, Status::Draw(DrawType::Stalemate));

        let mut chess = Chess::from_fen("k7/8/8/8/8/8/8/K1R3Q1 w").unwrap();
        let res = chess.move_piece(Position::from_str("c1"), Position::from_str("b1"));
        assert_eq!(
            res,
            ValidationResult::Valid(Status::Draw(DrawType::Stalemate))
        );
    }

    #[test]
    fn check_checkmate() {
        let mut chess = Chess::from_fen("k7/7R/2Q5/8/8/8/8/K7 w").unwrap();
        chess.move_piece(Position::from_str("c6"), Position::from_str("b7"));
        assert_eq!(chess.status, Status::Checkmate(Color::Black));
        assert_eq!(chess.winner, Some(Color::White));

        let mut chess = Chess::from_fen("k7/2QN3R/1P6/1N6/8/8/8/K7 w").unwrap();
        chess.move_piece(Position::from_str("b6"), Position::from_str("b7"));
        assert_eq!(chess.status, Status::Checkmate(Color::Black));
        assert_eq!(chess.winner, Some(Color::White));

        let chess = Chess::from_fen("k7/8/8/8/8/8/8/QR6 w").unwrap();
        assert_eq!(chess.status, Status::Checkmate(Color::Black));
        assert_eq!(chess.winner, Some(Color::White));

        let mut chess = Chess::from_fen("k7/8/8/8/8/8/1R6/KR4Q1 w").unwrap();
        let res = chess.move_piece(Position::from_str("b2"), Position::from_str("a2"));
        assert_eq!(
            res,
            ValidationResult::Valid(Status::Checkmate(Color::Black))
        );
        assert_eq!(chess.winner, Some(Color::White));
    }

    #[test]
    fn check_promotion() {
        let mut chess = Chess::from_fen("7k/P7/8/8/8/8/8/P6K w").unwrap();
        chess.move_piece(Position::from_str("a7"), Position::from_str("a8"));
        assert_eq!(chess.status, Status::AwaitingPromotion);
        chess.promote_piece(PieceType::Queen);
        assert_eq!(chess.status, Status::Check(Color::Black));
    }

    #[test]
    fn check_en_passant() {
        let mut chess = Chess::from_fen("k7/2p5/8/3P4/8/8/8/K7 b").unwrap();
        chess.move_piece(Position::from_str("c7"), Position::from_str("c5"));

        let res = chess.move_piece(Position::from_str("d5"), Position::from_str("c6"));

        assert!(matches!(res, ValidationResult::Valid(_)));
        assert!(chess.board[4 * 8 + 2].is_none());
    }

    #[test]
    fn check_castling() {
        let mut chess = Chess::from_fen("4k3/8/8/8/8/8/8/R3K3 w").unwrap();
        let res = chess.move_piece(Position::from_str("e1"), Position::from_str("a1"));
        assert_eq!(res, ValidationResult::Valid(Status::Chilling));
        assert_eq!(chess.board[2].as_ref().unwrap().piece_type, PieceType::King);
        assert_eq!(chess.board[3].as_ref().unwrap().piece_type, PieceType::Rook);

        let mut chess = Chess::from_fen("2q1k3/8/8/8/8/8/8/R3K3 w").unwrap();
        let res = chess.move_piece(Position::from_str("e1"), Position::from_str("a1"));
        assert_eq!(res, ValidationResult::InvalidMove);

        let mut chess = Chess::from_fen("3qk3/8/8/8/8/8/8/R3K3 w").unwrap();
        let res = chess.move_piece(Position::from_str("e1"), Position::from_str("a1"));
        assert_eq!(res, ValidationResult::InvalidMove);

        let mut chess = Chess::from_fen("r3kq2/8/8/8/8/8/8/RQ2K3 b").unwrap();
        let res = chess.move_piece(Position::from_str("e8"), Position::from_str("a8"));
        assert_eq!(res, ValidationResult::Valid(Status::Chilling));

        let res = chess.move_piece(Position::from_str("e1"), Position::from_str("a1"));
        assert_eq!(res, ValidationResult::InvalidMove);

        let mut chess = Chess::from_fen("q3k2r/8/8/8/8/8/5Q2/4K2R b").unwrap();
        let res = chess.move_piece(Position::from_str("e8"), Position::from_str("h8"));
        assert_eq!(res, ValidationResult::InvalidMove);
        let res = chess.move_piece(Position::from_str("e8"), Position::from_str("f8"));
        assert_eq!(res, ValidationResult::InvalidMove);
        let res = chess.move_piece(Position::from_str("e8"), Position::from_str("d8"));
        assert_eq!(res, ValidationResult::Valid(Status::Chilling));

        let res = chess.move_piece(Position::from_str("e1"), Position::from_str("h1"));
        assert_eq!(res, ValidationResult::Valid(Status::Chilling));

        let mut chess = Chess::from_fen("q3k2r/8/8/8/8/8/5Q2/4K1R1 w").unwrap();
        let res = chess.move_piece(Position::from_str("g1"), Position::from_str("h1"));
        assert_eq!(res, ValidationResult::Valid(Status::Chilling));
        let res = chess.move_piece(Position::from_str("e8"), Position::from_str("d8"));
        assert_eq!(res, ValidationResult::Valid(Status::Chilling));
        let res = chess.move_piece(Position::from_str("e1"), Position::from_str("h1"));
        assert_eq!(res, ValidationResult::InvalidMove);
    }

    #[test]
    fn check_three_fold_repetition() {
        let mut chess = Chess::from_fen("k7/8/8/8/8/8/8/K7 w").unwrap();
        chess.move_piece(Position::from_str("a1"), Position::from_str("a2"));
        chess.move_piece(Position::from_str("a8"), Position::from_str("a7"));
        chess.move_piece(Position::from_str("a2"), Position::from_str("a1"));
        chess.move_piece(Position::from_str("a7"), Position::from_str("a8"));
        chess.move_piece(Position::from_str("a1"), Position::from_str("a2"));
        chess.move_piece(Position::from_str("a8"), Position::from_str("a7"));
        chess.move_piece(Position::from_str("a2"), Position::from_str("a1"));

        assert_eq!(chess.status, Status::Chilling);

        chess.move_piece(Position::from_str("a7"), Position::from_str("a8"));

        assert_eq!(chess.status, Status::Draw(DrawType::ThreefoldRepetition));
    }

    #[test]
    fn check_50_move_rule() {
        let mut chess = Chess::from_fen("rrrrrrrr/8/8/8/8/8/8/RRRRRRRR w").unwrap();
        for i in 0..3 {
            for j in 0..8 {
                chess.move_piece(Position { x: j, y: i }, Position { x: j, y: i + 1 });
                chess.move_piece(
                    Position { x: j, y: 7 - i },
                    Position {
                        x: j,
                        y: 7 - (i + 1),
                    },
                );
            }
        }

        for i in (1..4).rev() {
            for j in 0..8 {
                chess.move_piece(Position { x: j, y: i }, Position { x: j, y: i - 1 });
                chess.move_piece(
                    Position { x: j, y: 7 - i },
                    Position {
                        x: j,
                        y: 7 - (i - 1),
                    },
                );
            }
        }

        for i in 0..2 {
            chess.move_piece(Position { x: i, y: 0 }, Position { x: i, y: 1 });
            chess.move_piece(Position { x: i, y: 7 }, Position { x: i, y: 6 });
        }

        assert_eq!(chess.status, Status::Draw(DrawType::FiftyMoveRule));
    }

    #[test]
    fn test_possible_moves() {
        let chess = Chess::from_fen("k7/8/8/8/r7/8/7r/K7 w").unwrap();

        let moves = chess.generate_valid_moves();

        assert_eq!(moves[0].len(), 1);
        assert_eq!(moves[0][0].to, Position::from_str("b1"));

        let chess = Chess::from_fen("k2r1r2/8/8/8/8/8/8/R3K3 w").unwrap();

        let moves = chess.generate_valid_moves();

        println!("{:?}", moves[4]);

        assert_eq!(moves[4].len(), 1);
        assert_eq!(moves[4][0].to, Position::from_str("e2"));
    }
}
