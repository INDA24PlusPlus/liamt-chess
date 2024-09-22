use chess::*;

fn print_board(board: &Board) {
    println!("  ┌─────────────────┐");
    for i in 0..8 {
        print!("{} │ ", 8 - i);
        for j in 0..8 {
            match &board[(7 - i) * 8 + j] {
                Some(piece) => {
                    let color_offset = if piece.color == Color::White { 0 } else { 32 };
                    match piece.piece_type {
                        PieceType::King => print!("{} ", (b'K' + color_offset) as char),
                        PieceType::Queen => print!("{} ", (b'Q' + color_offset) as char),
                        PieceType::Rook => print!("{} ", (b'R' + color_offset) as char),
                        PieceType::Bishop => print!("{} ", (b'B' + color_offset) as char),
                        PieceType::Knight => print!("{} ", (b'N' + color_offset) as char),
                        PieceType::Pawn => print!("{} ", (b'P' + color_offset) as char),
                    }
                }
                None => print!(". "),
            }
        }
        println!("│");
    }
    println!("  └─────────────────┘");
    println!("    a b c d e f g h");
}

fn main() {
    let mut chess = Chess::new();
    //let mut chess = Chess::from_fen("4k3/8/8/8/8/8/8/R3K3 w").unwrap();

    loop {
        println!();
        print_board(&chess.board);
        println!("\nTurn: {:?}, Status: {:?}", chess.turn, chess.status);
        println!("Enter move (e.g. 'a2 a3'): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        let positions: Vec<&str> = input.split_whitespace().collect();
        if positions.len() != 2 {
            println!("Invalid input");
            continue;
        }
        let from = Position::from_str(positions[0]);
        let to = Position::from_str(positions[1]);

        let validation_res = chess.move_piece(from, to);

        match validation_res {
            ValidationResult::Valid(status) => {
                if let Status::Checkmate(_) = status {
                    println!("Checkmate! {:?} wins", chess.winner.unwrap());
                    break;
                } else if let Status::Draw(d) = status {
                    println!("Draw! {:?}", d);
                    break;
                } else if let Status::Check(c) = status {
                    println!("Check on {:?}!", c);
                } else if status == Status::AwaitingPromotion {
                    let new_status = chess.promote_piece(PieceType::Queen);
                    println!("New status: {:?}", new_status);
                }
            }
            _ => {
                println!("ERROR: {:?}", validation_res);
                continue;
            }
        }
    }
}
