use crate::chess::{board::Edge, Board, Move, Piece, Side, Square};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref KING_MOVES: Vec<Move> = vec![
        Move {
            x: 1,
            y: 0,
            constraints: Vec::new(),
        },
        Move {
            x: 1,
            y: 1,
            constraints: Vec::new(),
        },
        Move {
            x: 0,
            y: 1,
            constraints: Vec::new(),
        },
        Move {
            x: -1,
            y: 1,
            constraints: Vec::new(),
        },
        Move {
            x: -1,
            y: 0,
            constraints: Vec::new(),
        },
        Move {
            x: 0,
            y: -1,
            constraints: Vec::new(),
        },
        Move {
            x: -1,
            y: -1,
            constraints: Vec::new(),
        },
        Move {
            x: -1,
            y: 1,
            constraints: Vec::new(),
        },
    ];
}

pub fn generate_king_moves(board: &Board, sq: usize, piece: &Piece) -> Vec<usize> {
    let mut moves = vec![];

    'moves: for mv in KING_MOVES.iter() {
        let (x, y) = match piece.side() {
            Side::White => (mv.x, mv.y),
            Side::Black => {
                let mv = mv.invert_coordinates();
                (mv.x, mv.y)
            }
        };

        let final_sq = (sq as isize + (y * 8 + x)) as usize;

        for (idx, psq) in board.pieces().iter().enumerate() {
            if let Some(ref p) = psq {
                if !(p.kind() == piece.kind()) {
                    if board.generate_moves(idx, p).contains(&final_sq) && p.side() != piece.side()
                    {
                        continue 'moves;
                    }
                }
            }
        }

        moves.push(final_sq);
    }

    moves
}
