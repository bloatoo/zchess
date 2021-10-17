use super::utils::calculate_squares_to_edge;
use crate::chess::{board::Edge, Board, Move, Piece};

const ROOK_MOVES: &[Move] = &[
    Move {
        x: 8,
        y: 0,
        constraints: &[],
    },
    Move {
        x: 0,
        y: 8,
        constraints: &[],
    },
];
pub fn generate_rook_moves(board: &Board, sq: usize, piece: &Piece) -> Vec<usize> {
    let mut moves = vec![];

    for mv in ROOK_MOVES {
        if mv.x == 0 {
            let top_edge = calculate_squares_to_edge(Edge::Top, sq);
            let mut valid = true;

            for i in 1..=top_edge {
                if !valid {
                    continue;
                }
                let final_sq = sq + i as usize * 8;

                match board.piece_at(final_sq) {
                    Some(p) => {
                        if p.side() != piece.side() {
                            moves.push(final_sq);
                        }
                        valid = false;
                    }
                    None => moves.push(final_sq),
                };
            }

            let bottom_edge = calculate_squares_to_edge(Edge::Bottom, sq);
            let mut valid = true;

            for i in 1..=bottom_edge {
                if !valid {
                    continue;
                }

                let final_sq = sq - i * 8;
                match board.piece_at(final_sq) {
                    Some(p) => {
                        if p.side() != piece.side() {
                            moves.push(final_sq);
                        }
                        valid = false;
                    }
                    None => moves.push(final_sq),
                }
            }
        } else {
            let right_edge = calculate_squares_to_edge(Edge::Right, sq);
            let mut valid = true;
            for i in 1..=right_edge {
                if !valid {
                    continue;
                }

                let final_sq = sq + i;

                match board.piece_at(final_sq) {
                    Some(p) => {
                        if p.side() != piece.side() {
                            moves.push(final_sq);
                        }
                        valid = false;
                    }
                    None => moves.push(final_sq),
                }
            }

            let left_edge = calculate_squares_to_edge(Edge::Left, sq);
            let mut valid = true;

            for i in 1..=left_edge {
                if !valid {
                    continue;
                }

                let final_sq = sq - i;

                match board.piece_at(final_sq) {
                    Some(p) => {
                        if p.side() != piece.side() {
                            moves.push(final_sq);
                        }
                        valid = false;
                    }
                    None => moves.push(final_sq),
                }
            }
        }
    }

    moves
}
