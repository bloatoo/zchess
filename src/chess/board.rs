use super::{Piece, PieceKind, Side};

use crate::chess::moves::bishop::generate_bishop_moves;
use crate::chess::moves::king::generate_king_moves;
use crate::chess::moves::knight::generate_knight_moves;
use crate::chess::moves::pawn::generate_pawn_moves;
use crate::chess::moves::queen::generate_queen_moves;
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

pub struct Move {
    pub x: isize,
    pub y: isize,
    pub constraints: Vec<MoveConstraint>,
}

impl Move {
    pub fn invert_coordinates(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            constraints: self.constraints.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pieces: Vec<Option<Piece>>,
    en_passant: Option<usize>,
    turn: Side,
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
            turn: Side::White,
            en_passant: None,
        }
    }

    pub fn from_str(fen: &str, turn: Side) -> Self {
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
            turn,
            en_passant: None,
        }
    }

    pub fn generate_moves(&self, sq: usize, piece: &Piece) -> Vec<usize> {
        use PieceKind::*;

        match piece.kind() {
            Pawn => generate_pawn_moves(&self, sq, piece),
            Rook => generate_rook_moves(&self, sq, piece),
            Knight => generate_knight_moves(&self, sq, piece),
            Bishop => generate_bishop_moves(&self, sq, piece),
            Queen => generate_queen_moves(&self, sq, piece),
            King => generate_king_moves(&self, sq, piece),
        }
    }

    pub fn make_move(&mut self, source: usize, dest: usize) {
        let piece = self.piece_at(source).clone().unwrap();
        self.set_piece(dest, Some(piece));
        self.set_piece(source, None);

        self.turn = match self.turn {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };
    }

    fn set_piece(&mut self, dest: usize, piece: Option<Piece>) {
        let p = self.pieces.get_mut(dest).unwrap();
        *p = piece;
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

    pub fn pieces_mut(&mut self) -> &mut Vec<Option<Piece>> {
        &mut self.pieces
    }

    pub fn turn(&self) -> &Side {
        &self.turn
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_str(
            "RNBQKBNR/PPPPPPPP/8/8/2BQ1b2/8/pppppppp/rnbqkbnr",
            Side::White,
        )
    }
}
