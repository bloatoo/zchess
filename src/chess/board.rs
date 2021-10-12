use super::{Piece, PieceKind, Side};

#[derive(Debug, Clone, PartialEq)]
pub struct Square {
    x: usize,
    y: usize,
}

impl Square {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn set_x(&mut self, x: usize) {
        if x < 8 {
            self.x = x;
        }
    }

    pub fn set_xy(&mut self, x: usize, y: usize) {
        if x < 8 {
            self.x = x;
        }

        if y < 8 {
            self.y = y;
        }
    }

    pub fn to_idx(&self) -> usize {
        self.y * 8 + self.x
    }

    pub fn inc_x(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn inc_y(&self) -> Self {
        Self {
            y: self.y + 1,
            x: self.x,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    from: Square,
    to: Square,
}

#[derive(Debug, Clone)]
pub struct Board {
    pieces: Vec<Option<Piece>>,
    en_passant: usize,
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
            en_passant: 69,
        }
    }

    pub fn from_str(fen: &str) -> Self {
        let mut idx = 0;
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

                idx += 1;
            }
        }

        Self {
            pieces,
            en_passant: 69,
        }
    }

    pub fn generate_moves(&self, sq: &Square, piece: &Piece) -> Vec<usize> {
        use PieceKind::*;

        let mut moves: Vec<usize> = Vec::new();

        match piece.kind() {
            Pawn => match piece.side() {
                Side::White => {
                    if sq.y == 1 {
                        let sq = sq.inc_y();
                        if let None = self.piece_at(sq.to_idx()) {
                            moves.push(sq.to_idx());
                            moves.push(sq.inc_y().to_idx());
                        }
                    }
                }
                Side::Black => {}
            },
            _ => (),
        }

        moves
    }

    pub fn get_square(&self, square: usize) -> Square {
        let y = Self::get_row(square);
        let x = square - y * 8;

        Square { x, y }
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
