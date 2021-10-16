pub mod chess;
pub mod ui;

#[cfg(test)]
mod tests {
    use crate::chess::{Board, Side};

    #[test]
    fn new_board() {
        let board = Board::empty();

        assert_eq!(board.pieces().len(), 64);
    }

    #[test]
    fn basic_moves() {
        let board = Board::default();
        let piece = board.piece_at(8).as_ref().unwrap();

        let moves = board.generate_moves(8, &piece);

        assert_eq!(vec![16, 24], moves);
    }

    #[test]
    fn r() {
        let board = Board::default();
        let piece = board.piece_at(1).as_ref().unwrap();

        assert_eq!(piece.side(), &Side::White);
    }
}
