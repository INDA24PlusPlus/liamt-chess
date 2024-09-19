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
    fn check_stalemate() {
        let mut chess = Chess::from_fen("k7/8/2Q5/8/8/8/8/K7 w").unwrap();
        chess.move_piece(Position::from_str("c6"), Position::from_str("b6"));
        assert_eq!(chess.status, Status::Stalemate);

        let mut chess = Chess::from_fen("1B6/8/8/3k4/8/B7/8/2R1R3 w").unwrap();
        chess.move_piece(Position::from_str("a3"), Position::from_str("b2"));
        assert_eq!(chess.status, Status::Stalemate);
    }

    #[test]
    fn check_checkmate() {
        let mut chess = Chess::from_fen("k7/7R/2Q5/8/8/8/8/K7 w").unwrap();
        chess.move_piece(Position::from_str("c6"), Position::from_str("b7"));
        assert_eq!(chess.status, Status::Checkmate(Color::Black));

        let mut chess = Chess::from_fen("k7/2QN3R/1P6/1N6/8/8/8/K7 w").unwrap();
        chess.move_piece(Position::from_str("b6"), Position::from_str("b7"));
        assert_eq!(chess.status, Status::Checkmate(Color::Black));

        let chess = Chess::from_fen("k7/8/8/8/8/8/8/QR6 w").unwrap();
        assert_eq!(chess.status, Status::Checkmate(Color::Black));
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
        println!("NOW");
        let res = chess.move_piece(Position::from_str("d5"), Position::from_str("c6"));
        println!("{:?}", res);
        assert!(matches!(res, ValidationResult::Valid(_)));
    }
}
