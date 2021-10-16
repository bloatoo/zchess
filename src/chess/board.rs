use super::{Piece, PieceKind, Side};

pub trait Square {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
    fn pos(&self) -> (usize, usize);
}

impl Square for usize {
    fn x(&self) -> usize {
        self - self.y() * 8
    }

    fn y(&self) -> usize {
        (*self as f32 / 8.0).ceil() as usize
    }

    fn pos(&self) -> (usize, usize) {
        (self.x(), self.y())
    }
}

type Move = isize;

#[derive(Debug, Clone)]
pub struct Board {
    pieces: Vec<Option<Piece>>,
    en_passant: Option<usize>,
}

impl Board {
    // mainly for unit testing
    pub fn empty() -> Self {
        let mut pieces = Vec::with_capacity(64);

        for _ in 0..64 {
            pieces.push(None);
        }

        Self {
            pieces,
            en_passant: None,
        }
    }

    pub fn from_str(fen: &str) -> Self {
        let mut pieces: Vec<Option<Piece>> = vec![];

        for row in fen.split("/") {
            for c in row.split("") {
                use PieceKind::*;
                use Side::*;
                match c {
                    "P" => pieces.push(Some(Piece::new(Pawn, White))),
                    "N" => pieces.push(Some(Piece::new(Knight, White))),
                    "B" => pieces.push(Some(Piece::new(Bishop, White))),
                    "R" => pieces.push(Some(Piece::new(Rook, White))),
                    "Q" => pieces.push(Some(Piece::new(Queen, White))),
                    "K" => pieces.push(Some(Piece::new(King, White))),

                    "p" => pieces.push(Some(Piece::new(Pawn, Black))),
                    "n" => pieces.push(Some(Piece::new(Knight, Black))),
                    "b" => pieces.push(Some(Piece::new(Bishop, Black))),
                    "r" => pieces.push(Some(Piece::new(Rook, Black))),
                    "q" => pieces.push(Some(Piece::new(Queen, Black))),
                    "k" => pieces.push(Some(Piece::new(King, Black))),
                    _ => (),
                }

                if let Ok(res) = c.parse::<usize>() {
                    for _ in 0..res {
                        pieces.push(None);
                    }
                }
            }
        }

        Self {
            pieces,
            en_passant: None,
        }
    }

    pub fn generate_moves(&self, sq: usize, piece: &Piece) -> Vec<usize> {
        use PieceKind::*;

        let mut moves: Vec<usize> = Vec::new();

        match piece.kind() {
            Pawn => match piece.side() {
                Side::White => {
                    let upper = sq + 8;

                    let (left, right) = (upper - 1, upper + 1);

                    if let Some(_) = self.piece_at(right) {
                        moves.push(right);
                    }

                    if sq.y() == 1 {
                        let sq = sq + 8;
                        if let None = self.piece_at(sq) {
                            moves.push(sq);

                            let other = sq + 8;
                            if let None = self.piece_at(other) {
                                moves.push(other);
                            }
                        }
                    } else {
                    }
                }
                Side::Black => {}
            },
            _ => (),
        }

        moves
    }

    pub fn piece_at(&self, square: usize) -> &Option<Piece> {
        if square > 63 {
            return &None;
        }

        self.pieces.get(square).unwrap()
    }

    pub fn get_row(square: usize) -> usize {
        (square as f32 / 8.0).floor() as usize
    }

    pub fn pieces(&self) -> &Vec<Option<Piece>> {
        &self.pieces
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_str("RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr")
    }
}
