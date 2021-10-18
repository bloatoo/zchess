use crate::chess::Board;

pub struct Game {
    board: Board,
    id: String,
}

impl Game {
    pub fn new<T: ToString>(board: Board, id: T) -> Self {
        Self {
            board,
            id: id.to_string(),
        }
    }
}

pub struct App {
    game: Option<Game>,
}

impl App {
    pub fn new(game: Game) -> Self {
        Self { game: Some(game) }
    }
}

impl Default for App {
    fn default() -> Self {
        Self { game: None }
    }
}
