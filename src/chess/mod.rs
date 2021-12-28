pub mod board;
pub use board::{Board, Square};

pub mod piece;
pub use piece::{Piece, PieceKind, Side};

pub mod moves;
pub mod utils;

pub mod played_move;
pub use played_move::{PlayedMove, PlayedMoveKind};

pub struct Move<'a> {
    pub x: isize,
    pub y: isize,
    pub constraints: &'a [MoveConstraint],
}

impl<'a> Move<'a> {
    pub fn invert_coordinates(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            constraints: self.constraints.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MoveConstraint {
    MaxMoves(usize),
    Castling,
    PieceOnTargetSquare,
}

#[derive(Debug, Clone)]
pub enum CastleKind {
    WhiteLong,
    WhiteShort,
    BlackLong,
    BlackShort,
}
