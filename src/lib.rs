pub mod chess;
pub mod message;
pub mod ui;

#[cfg(test)]
mod tests {
    use crate::chess::Board;

    #[test]
    fn new_board() {
        let board = Board::empty();

        assert_eq!(board.pieces().len(), 64);
    }
}
