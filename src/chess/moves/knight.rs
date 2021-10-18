use crate::chess::utils::calculate_squares_to_edge;
use crate::chess::{board::Edge, Board, Move, Piece, Side};
use lazy_static::lazy_static;
use std::cmp::Ordering;

lazy_static! {
    pub static ref KNIGHT_MOVES: Vec<Move> = vec![
        Move {
            x: 1,
            y: 2,
            constraints: Vec::new(),
        },
        Move {
            x: -1,
            y: 2,
            constraints: Vec::new(),
        },
        Move {
            x: 2,
            y: 1,
            constraints: Vec::new(),
        },
        Move {
            x: -2,
            y: 1,
            constraints: Vec::new(),
        },
        Move {
            x: 2,
            y: -1,
            constraints: Vec::new(),
        },
        Move {
            x: -2,
            y: -1,
            constraints: Vec::new(),
        },
        Move {
            x: 1,
            y: -2,
            constraints: Vec::new(),
        },
        Move {
            x: -1,
            y: -2,
            constraints: Vec::new(),
        },
    ];
}

pub fn generate_knight_moves(board: &Board, sq: usize, piece: &Piece) -> Vec<usize> {
    let mut moves = vec![];

    for mv in KNIGHT_MOVES.iter() {
        let (x, y): (isize, isize) = match piece.side() {
            Side::White => (mv.x, mv.y),
            Side::Black => {
                let mv = mv.invert_coordinates();
                (mv.x, mv.y)
            }
        };

        let final_sq = (sq as isize + (y * 8 + x)) as usize;

        let is_horizontal_ok = match x.cmp(&0) {
            Ordering::Greater => {
                let to_right_edge = calculate_squares_to_edge(Edge::Right, sq);

                to_right_edge >= x as usize
            }

            Ordering::Less => {
                let to_edge = calculate_squares_to_edge(Edge::Left, sq);

                to_edge as isize >= -x
            }

            _ => true,
        };

        if !is_horizontal_ok {
            continue;
        }

        let is_vertical_ok = match y.cmp(&0) {
            Ordering::Greater => {
                let to_right_edge = calculate_squares_to_edge(Edge::Top, sq);

                to_right_edge >= y as usize
            }

            Ordering::Less => {
                let to_edge = calculate_squares_to_edge(Edge::Bottom, sq);

                to_edge as isize >= -y
            }

            _ => false,
        };

        if !is_vertical_ok {
            continue;
        }

        let is_piece_ok = match board.piece_at(final_sq) {
            Some(ref p) => piece.side() != p.side(),
            None => true,
        };

        if !is_piece_ok {
            continue;
        }

        moves.push(final_sq);
    }

    moves
}
