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

pub fn idx_to_square(idx: usize) -> String {
    /*let row = match idx {
        x if x < 8 => 1
        x if x > 7 && x < 16 => 2,
        x if x > 15 && x < 24 => 3,
        x if x > 23 && x < 32 => 4,
        x if x > 31 && x < 40 => 5,
        x if x > 39 && x < 48 => 6,
        x if x > 47 && x < 56 => 7,
        x if x > 55 && x < 64 => 8,
        _ => unreachable!(),
    };*/

    let row = idx.y() + 1;

    let file = match idx.x() {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => unreachable!(),
    };

    format!("{}{}", file, row)
}

pub fn square_to_idx(square: &str) -> usize {
    let (file, row) = square.split_at(1);

    let file = match file {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        "g" => 6,
        "h" => 7,
        _ => unreachable!(),
    };

    (row.parse::<usize>().unwrap() - 1) * 8 + file
}
