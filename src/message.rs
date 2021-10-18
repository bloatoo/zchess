use crate::game::{Game, GameState};

pub enum Message {
    GameStart(String), // id
    GameStateUpdate(GameState),
    GameDataInit(Game),
}
