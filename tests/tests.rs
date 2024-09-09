#[cfg(test)]
mod tests {
    use chess::*;

    #[test]
    fn check_turn() {
        let chess = Chess::new();
        assert_eq!(chess.turn, Color::White);
    }

    #[test]
    fn check_kings() {
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
}
