use crate::chess::Square;

#[derive(Debug, Clone)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone)]
pub struct Piece {
    kind: PieceKind,
    side: Side,
}

impl Piece {
    pub fn new(kind: PieceKind, side: Side) -> Self {
        Self { kind, side }
    }

    pub fn kind(&self) -> &PieceKind {
        &self.kind
    }

    pub fn side(&self) -> &Side {
        &self.side
    }
}

impl AsRef<str> for Piece {
    fn as_ref(&self) -> &'static str {
        use PieceKind::*;
        match self.kind {
            Pawn => "p",
            Knight => "n",
            Bishop => "b",
            Rook => "r",
            Queen => "q",
            King => "k",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Side {
    White,
    Black,
}
