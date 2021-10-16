use super::{Piece, PieceKind, Side};

use crate::chess::moves::pawn::generate_pawn_moves;
use crate::chess::moves::rook::generate_rook_moves;

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
        (*self as f32 / 8.0).floor() as usize
    }

    fn pos(&self) -> (usize, usize) {
        (self.x(), self.y())
    }
}

#[derive(Debug, Clone)]
pub enum MoveConstraint {
    MaxMoves(usize),
    PieceOnTargetSquare,
}

#[derive(Debug, Clone)]
pub enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}

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
            constraints: self.constraints,
        }
    }
}

fn calculate_squares_to_edge(edge: Edge, sq: usize) -> usize {
    use Edge::*;

    match edge {
        Right => 7 - sq.x(),
        Left => sq.x(),
        Top => 7 - sq.y(),
        Bottom => sq.y(),
    }
}

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

        match piece.kind() {
            Pawn => generate_pawn_moves(&self, sq, piece),
            Rook => generate_rook_moves(&self, sq, piece),
            _ => vec![],
        }
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
        Self::from_str("RNBQKBNR/PPPPPPPP/3p4/5R2/8/4p3/pppppppp/rnbqkbnr")
    }
}
