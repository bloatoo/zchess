use crate::chess::{board::Edge, Board, Move, MoveConstraint, Piece, PieceKind, Side};

pub const KING_MOVES: &[Move] = &[
    Move {
        x: 1,
        y: 0,
        constraints: &[],
    },
    Move {
        x: 2,
        y: 0,
        constraints: &[MoveConstraint::Castling],
    },
    Move {
        x: -2,
        y: 0,
        constraints: &[MoveConstraint::Castling],
    },
    Move {
        x: 1,
        y: 1,
        constraints: &[],
    },
    Move {
        x: 0,
        y: 1,
        constraints: &[],
    },
    Move {
        x: -1,
        y: 1,
        constraints: &[],
    },
    Move {
        x: -1,
        y: 0,
        constraints: &[],
    },
    Move {
        x: 0,
        y: -1,
        constraints: &[],
    },
    Move {
        x: -1,
        y: -1,
        constraints: &[],
    },
    Move {
        x: -1,
        y: 1,
        constraints: &[],
    },
];

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

        if final_sq > 64 {
            continue;
        }

        for (idx, p) in board.pieces().iter().enumerate() {
            if let Some(p) = p {
                if p.side() == piece.side() {
                    continue;
                }

                if p.kind() == &PieceKind::King && p.side() == board.turn() {
                    continue;
                }

                if board.generate_moves(idx, p).contains(&final_sq) {
                    continue 'moves;
                }
            }
        }

        if let Some(ref p) = board.piece_at(final_sq) {
            if p.side() == piece.side() {
                continue 'moves;
            }
        }

        moves.push(final_sq);
    }
    moves
}
