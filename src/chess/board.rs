use super::{Piece, PieceKind, Side};
use std::cmp::Ordering;

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
    x: isize,
    y: isize,
    constraints: &'a [MoveConstraint],
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

const PAWN_MOVES: &[Move] = &[
    Move {
        x: 0,
        y: 2,
        constraints: &[MoveConstraint::MaxMoves(0)],
    },
    Move {
        x: 0,
        y: 1,
        constraints: &[],
    },
    Move {
        x: 1,
        y: 1,
        constraints: &[MoveConstraint::PieceOnTargetSquare],
    },
    Move {
        x: -1,
        y: 1,
        constraints: &[MoveConstraint::PieceOnTargetSquare],
    },
];

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
                    for mv in PAWN_MOVES.iter() {
                        let idx_change = mv.y * 8 + mv.x;

                        let final_sq = (sq as isize + idx_change) as usize;

                        if let None = self.piece_at(final_sq) {
                            match mv.x.cmp(&0) {
                                Ordering::Greater => {
                                    let to_edge = calculate_squares_to_edge(Edge::Right, sq);
                                    if to_edge >= mv.x as usize {
                                        moves.push(final_sq);
                                    }
                                }
                                Ordering::Less => {
                                    //let to_edge = calculate_squares_to_edge(Edge::Left, sq);

                                    /*if to_edge as isize >= mv.x {
                                        moves.push(final_sq);
                                    }*/
                                    moves.push(final_sq);
                                }
                                _ => {
                                    moves.push(final_sq);
                                }
                            }
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        };

        /*match piece.kind() {
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
                        if let None = self.piece_at(sq + 8) {
                            moves.push(sq + 8);
                        }
                    }
                }

                Side::Black => {
                    if sq.y() == 6 {
                        let mut valid = true;

                        for mv in [sq - 8, sq - 16] {
                            if !valid {
                                continue;
                            }
                            if let Some(_) = self.piece_at(mv) {
                                valid = false;
                                continue;
                            } else {
                                moves.push(mv);
                            }
                        }
                    } else {
                        if let None = self.piece_at(sq - 8) {
                            moves.push(sq - 8);
                        }
                    }

                    let upper = sq - 8;
                }
            },
            _ => (),
        }*/

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
