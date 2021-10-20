pub mod app;
pub mod chess;
pub mod config;
pub mod game;
pub mod message;
pub mod ui;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::chess::utils::idx_to_square;
    use crate::chess::utils::square_to_idx;

    #[test]
    fn idx_to_sq() {
        assert_eq!(idx_to_square(63), "h8");
        assert_eq!(square_to_idx("h8"), 63);
    }
}
