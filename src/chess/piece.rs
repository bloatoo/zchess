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

#[derive(Debug, Clone)]
pub enum Side {
    White,
    Black,
}
