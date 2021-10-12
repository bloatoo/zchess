pub mod chess;
pub mod ui;

#[cfg(test)]
mod tests {
    use crate::chess::{Board, Square};

    #[test]
    fn new_board() {
        let board = Board::empty();

        assert_eq!(board.pieces().len(), 64);
        assert_eq!(Square::new(2, 2), board.get_square(18));
        assert_eq!(Square::new(7, 7), board.get_square(63));
    }

    #[test]
    fn basic_moves() {
        let board = Board::default();
        let sq = Square::new(0, 1);
        let piece = board.piece_at(sq.to_idx()).as_ref().unwrap();

        let moves = board.generate_moves(&sq, &piece);

        assert_eq!(vec![16, 24], moves);
    }
}
