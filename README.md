# ÖH SUPER NAJS SCHACKBIBLIOTEK
Så vad är då detta för sak??? JO ETT SCHACK BIBBLO.

## Installation
Lägg in följande i din `Cargo.toml` fil:
```toml
chess = { git = "https://github.com/INDA24PlusPlus/liamt-chess.git" }
```

## Användning
Nedan följer ett exempel hur biblioteket kan användas:
```rust
use chess::*;

// Skapa ett nytt schackbräde med standard position
let mut chess: Chess = Chess::new();
// Om du vill importera en position från FEN så kan du göra det såhär:
let mut chess: Chess = Chess::from_fen("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1").unwrap()

// Så du vill nu göra ett drag, låt oss säga att du vill flytta pjäsen på a2 till a3
let res: ValidationResult = chess.move_piece(Position::from_str("a2"), Position::from_str("a3"));
// Eller genom koordinater
let res: ValidationResult = chess.move_piece(Position {x: 0,y: 1}, Position {x: 0,y: 2});

// Du kan sedan hantera resultatet av draget
match res {
    ValidationResult::Valid(status) => {
        if let Status::Checkmate(_) = status {
            println!("Checkmate! {:?} wins", chess.winner.unwrap()); // chess.winner innehåller vinnaren av spelet
            break;
        } else if let Status::Draw(d) = status {
            println!("Draw! {:?}", d);
            break;
        } else if let Status::Check(c) = status {
            println!("Check on {:?}!", c);
        } else if status == Status::AwaitingPromotion {
            let new_status = chess.promote_piece(PieceType::Queen); // Om status är AwaitingPromotion så måste du upgradera en bonde
            println!("New status: {:?}", new_status);
        }
    }
    // Här bör man även hantera resterande resultat
    //...
    _ => {
        println!("ERROR: {:?}", res);
        continue;
    }
}

// Du kanske också vill utföra en rockad, det kan du göra såhär:
let res: ValidationResult = chess.perform_castling(CastlingType::KingSide(Color::White));
let res: ValidationResult = chess.perform_castling(CastlingType::QueenSide(Color::White));
let res: ValidationResult = chess.perform_castling(CastlingType::KingSide(Color::Black));
let res: ValidationResult = chess.perform_castling(CastlingType::QueenSide(Color::Black));

match res {
    ValidationResult::Valid(status) => {
        println("Castling successful!");
    },
    ValidationResult::InvalidMove => {
        println("Castling failed!");
    },
    _ => {
        println!("ERROR: {:?}", res);
        continue;
    }
}

// Du kan även kolla om rokad är möjligt, den returnerar en bool som indikerar om rockad är möjlig
let castling_possible: bool = chess.check_castling_possible(CastlingType::KingSide(Color::White));

// Det finns även lite attributer som ger dig användbar information
let current_turn: Color = chess.turn; // Vems tur det är
let winner: Option<Color> = chess.winner; // Vem som vann spelet, None om spelet pågår
let status: Status = chess.status; // Statusen av spelet, alltså om det är schack, schackmatt, remi etc.

// Du kan även indexera brädet för att få en specifik pjäs
// Detta kan vara användbart när du vill printa ut brädet

for y in 0..8 {
    for x in 0..8 {
        // Vill börja med de svarta rutorna i nedre vänstra hörnet
        let piece: Option<Piece> = board[(7 - i) * 8 + j];

        match piece {
            Some(p) => {
                println!("Färg: {:?}. Pjäs-typ: {:?}. Position: {:?}", p.color, p.piece_type, p.position);
            },
            None => {
                println!("Ingen pjäs på position ({}, {})", x, y);
            }
        }
    }
    println!("Ny rad!");
}

```

