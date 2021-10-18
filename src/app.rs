use crate::chess::Board;
use crate::config::Config;

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
    config: Config,
}

impl App {
    pub fn new(game: Game) -> Self {
        Self {
            game: Some(game),
            config: Config::new().unwrap(),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: None,
            config: Config::new().unwrap(),
        }
    }
}
