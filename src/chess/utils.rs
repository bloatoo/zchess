use crate::chess::board::{Edge, Square, SquareColor};

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
        _ => {
            panic!("invalid file, square: {}, {} {}", square, file, row);
        }
    };

    (row.parse::<usize>().unwrap() - 1) * 8 + file
}

pub fn uci_to_idx(uci: &str) -> (usize, usize) {
    let (src, mut dest) = uci.split_at(2);

    if dest.len() > 2 {
        dest = &dest[..dest.len() - 1];
    }

    (square_to_idx(&src), square_to_idx(&dest))
}

pub fn move_to_uci(src: usize, dest: usize) -> String {
    format!("{}{}", idx_to_square(src), idx_to_square(dest))
}

pub fn get_square_color(sq: usize) -> SquareColor {
    match sq.y() % 2 {
        0 => match sq % 2 {
            0 => SquareColor::Light,
            _ => SquareColor::Dark,
        },

        _ => match sq % 2 {
            0 => SquareColor::Dark,
            _ => SquareColor::Light,
        },
    }
}
