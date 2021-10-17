use super::utils::calculate_squares_to_edge;
use crate::chess::{board::Edge, Board, Move, MoveConstraint, Piece, Side};
use std::cmp::Ordering;

pub const PAWN_MOVES: &[Move] = &[
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

pub fn generate_pawn_moves(board: &Board, sq: usize, piece: &Piece) -> Vec<usize> {
    let mut moves = vec![];
    'moves: for mv in PAWN_MOVES.iter() {
        let (x, y): (isize, isize) = match piece.side() {
            Side::White => (mv.x, mv.y),
            Side::Black => {
                let mv = mv.invert_coordinates();
                (mv.x, mv.y)
            }
        };

        let idx_change = y * 8 + x;

        let final_sq = (sq as isize + idx_change) as usize;
        let mut move_constr = false;
        let mut max_constr = false;

        for c in mv.constraints {
            match c {
                MoveConstraint::MaxMoves(a) => {
                    if piece.move_count() > &(*a as u32) {
                        continue 'moves;
                    }

                    max_constr = true;
                }

                &MoveConstraint::PieceOnTargetSquare => {
                    if let None = board.piece_at(final_sq) {
                        continue 'moves;
                    }

                    move_constr = true;
                }
            }
        }

        let is_corner = match x.cmp(&0) {
            Ordering::Greater => calculate_squares_to_edge(Edge::Right, sq) >= x as usize,
            Ordering::Less => calculate_squares_to_edge(Edge::Left, sq) as isize >= -x,
            _ => true,
        };

        if !is_corner {
            continue 'moves;
        }

        if max_constr && (y == 2 || y == -2) {
            if board.piece_at(sq + 8).is_some() || board.piece_at(sq + 16).is_some() {
                continue 'moves;
            }
        }

        if move_constr {
            if let Some(p) = board.piece_at(final_sq) {
                if p.side() != piece.side() {
                    moves.push(final_sq);
                }
            }
        } else {
            if board.piece_at(final_sq).is_none() {
                moves.push(final_sq);
            }
        }
    }

    moves
}
