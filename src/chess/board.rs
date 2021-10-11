use super::Piece;

/*pub enum Square {
    A1 = 0,
    B1 = 1,
    C1 = 2,
    D1 = 3,
    E1 = 4,
    F1 = 5,
    G1 = 6,
    H1 = 7,

    A2 = 8,
    B2 = 9,
    C2 = 10,
    D2 = 11,
    E2 = 12,
    F2 = 13,
    G2 = 14,
    H2 = 15,

    A3 = 16,
    B3 = 17,
    C3 = 18,
    D3 = 19,
    E3 = 20,
    F3 = 21,
    G3 = 22,
    H3 = 23,
}*/

pub struct Board {
    pieces: Vec<Option<Piece>>,
    en_passant: usize,
}

impl Board {
    // mainly for unit testing
    pub fn empty() -> Self {
        let mut pieces = Vec::with_capacity(64);

        for _ in 0..64 {
            pieces.push(None);
        }

        Self {
            pieces,
            en_passant: 69,
        }
    }

    pub fn pieces(&self) -> &Vec<Option<Piece>> {
        &self.pieces
    }
}
