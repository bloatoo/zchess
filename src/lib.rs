pub mod chess;
pub mod ui;

#[cfg(test)]
mod tests {
    use crate::chess::{Board, Square};

    #[test]
    fn new_board() {
        let board = Board::empty();

        assert_eq!(board.pieces().len(), 64);
        assert_eq!(Square::new(2, 2), Board::get_square(18));
        assert_eq!(Square::new(7, 7), Board::get_square(63));
    }
}
