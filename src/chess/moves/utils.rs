use crate::chess::board::{Edge, Square};

pub fn calculate_squares_to_edge(edge: Edge, sq: usize) -> usize {
    use Edge::*;

    match edge {
        Right => 7 - sq.x(),
        Left => sq.x(),
        Top => 7 - sq.y(),
        Bottom => sq.y(),
    }
}
