use crate::chess::{Board, Piece};

use super::bishop::generate_bishop_moves;
use super::rook::generate_rook_moves;
pub fn generate_queen_moves(board: &Board, sq: usize, piece: &Piece) -> Vec<usize> {
    let mut moves = generate_bishop_moves(board, sq, piece);
    moves.extend(generate_rook_moves(board, sq, piece));

    moves
}
