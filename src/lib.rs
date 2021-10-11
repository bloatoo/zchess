pub mod chess;
pub mod ui;

#[cfg(test)]
mod tests {
    use crate::chess::Board;

    #[test]
    fn new_board() {
        let board = Board::new();

        assert_eq!(board.squares().len(), 64);
    }
}
