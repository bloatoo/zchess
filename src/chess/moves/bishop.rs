use crate::chess::{board::Edge, Board, Move, Piece, Side, Square};
use std::cmp::Ordering;

pub const BISHOP_MOVES: &[Move] = &[
    Move {
        x: 8,
        y: 8,
        constraints: &[],
    },
    Move {
        x: -8,
        y: 8,
        constraints: &[],
    },
    Move {
        x: 8,
        y: -8,
        constraints: &[],
    },
    Move {
        x: -8,
        y: -8,
        constraints: &[],
    },
];

pub fn generate_bishop_moves(board: &Board, sq: usize, piece: &Piece) -> Vec<usize> {
    let mut moves = vec![];

    'moves: for mv in BISHOP_MOVES {
        let (x, y) = match piece.side() {
            Side::White => (mv.x, mv.y),
            Side::Black => {
                let mv = mv.invert_coordinates();
                (mv.x, mv.y)
            }
        };

        match y.cmp(&0) {
            Ordering::Greater => match x.cmp(&0) {
                Ordering::Greater => {
                    for idx in 1..=8 {
                        let square = sq + (idx * 8 + idx) as usize;

                        if let Some(ref p) = board.piece_at(square) {
                            if p.side() != piece.side() {
                                moves.push(square);
                            }

                            continue 'moves;
                        }

                        if square.y() == 7 || square.x() == 7 {
                            moves.push(square);
                            continue 'moves;
                        }

                        if square < 63 {
                            moves.push(square);
                        }
                    }
                }
                Ordering::Less => {
                    for idx in 1..=8 {
                        let square = sq + (idx * 8 - idx) as usize;

                        if let Some(ref p) = board.piece_at(square) {
                            if p.side() != piece.side() {
                                moves.push(square);
                            }

                            continue 'moves;
                        }

                        if square.y() == 0 || square.x() == 0 {
                            moves.push(square);
                            continue 'moves;
                        }
                        if square > 0 {
                            moves.push(square);
                        }
                    }
                }
                _ => (),
            },
            Ordering::Less => match x.cmp(&0) {
                Ordering::Greater => {
                    for idx in 1..=sq.y() {
                        let square = sq - (idx as usize * 8) + idx;

                        if let Some(ref p) = board.piece_at(square) {
                            if p.side() != piece.side() {
                                moves.push(square);
                            }

                            continue 'moves;
                        }

                        if square.y() == 7 || square.x() == 7 {
                            moves.push(square);
                            continue 'moves;
                        }

                        if square > 0 && square < 63 {
                            moves.push(square);
                        }
                    }
                }
                Ordering::Less => {
                    for idx in 1..=sq.y() {
                        let square = sq - (idx as usize * 8) - idx;

                        if let Some(ref p) = board.piece_at(square) {
                            if p.side() != piece.side() {
                                moves.push(square);
                            }

                            continue 'moves;
                        }

                        if square.y() == 0 || square.x() == 0 {
                            moves.push(square);
                            continue 'moves;
                        }

                        if square > 0 && square < 63 {
                            moves.push(square);
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
    moves
}
