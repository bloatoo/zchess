use crate::chess::Board;
use crate::config::Config;
use crate::message::Message;
use crate::ui::UIState;
use futures::stream::StreamExt;
use std::io::Write;
use std::sync::mpsc::Sender;

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

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }
}

pub struct App {
    game: Option<Game>,
    config: Config,
    main_tx: Sender<Message>,
    ui_state: UIState,
}

impl App {
    pub fn new(main_tx: Sender<Message>) -> Self {
        Self {
            game: None,
            main_tx,
            config: Config::new().unwrap(),
            ui_state: UIState::Menu,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn game(&self) -> &Option<Game> {
        &self.game
    }

    pub fn game_mut(&mut self) -> &mut Option<Game> {
        &mut self.game
    }

    pub async fn seek_for_game(&self) {
        let token = format!("Bearer {}", self.config.token());

        tokio::spawn(async move {
            let client = reqwest::Client::new();

            let mut stream = client
                .post("https://lichess.org/api/board/seek?time=10&increment=0")
                .header("Authorization", token)
                .header("Content-Type", "text/plain")
                .send()
                .await
                .unwrap()
                .bytes_stream();

            while let Some(ev) = stream.next().await {
                let e = ev.unwrap();
                if e.len() > 1 {
                    panic!("{:#?}", e);
                }
            }
        });
    }

    pub fn start_new_game<T: ToString>(&mut self, id: T) {
        let id = id.to_string();
        self.game = Some(Game::new(Board::default(), id.clone()));
        self.ui_state = UIState::Game;

        let tx = self.main_tx.clone();

        let token = format!("Bearer {}", self.config.token());

        tokio::spawn(async move {
            let path = format!(
                "https://lichess.org/api/board/game/stream/{}",
                id.to_string()
            );

            let client = reqwest::Client::new();

            let mut stream = client
                .get(path)
                .header("Authorization", token)
                .send()
                .await
                .unwrap()
                .bytes_stream();

            while let Some(_) = stream.next().await {}

            // debugging
            /*let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("/home/quasar/.game_data")
            .unwrap();

            while let Some(ev) = stream.next().await {
                let ev = ev.unwrap().to_vec();
                file.write_all(&ev).unwrap();
            }*/
        });
    }

    pub fn ui_state(&self) -> &UIState {
        &self.ui_state
    }

    pub fn set_ui_state(&mut self, state: UIState) {
        self.ui_state = state;
    }
}

/*impl Default for App {
    fn default() -> Self {
        Self {
            game: None,
            config: Config::new().unwrap(),
            ui_state: UIState::Menu,
        }
    }
}*/
