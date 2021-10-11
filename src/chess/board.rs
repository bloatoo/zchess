use super::Piece;

#[derive(Debug, Clone, PartialEq)]
pub struct Square {
    x: usize,
    y: usize,
}

impl Square {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
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

    pub fn get_square(square: usize) -> Square {
        let y = Self::get_row(square);
        let x = square - y * 8;

        Square { x, y }
    }

    pub fn get_row(square: usize) -> usize {
        (square as f32 / 8.0).floor() as usize
    }

    pub fn pieces(&self) -> &Vec<Option<Piece>> {
        &self.pieces
    }
}
