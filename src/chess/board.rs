use super::Square;

pub struct Board {
    squares: Vec<Square>,
}

impl Board {
    pub fn new() -> Self {
        Self { squares: vec![] }
    }
}
