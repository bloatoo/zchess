use crate::game::{ChatMessage, Game, GameState};

pub enum Message {
    GameStart(String), // id
    GameStateUpdate(GameState),
    GameDataInit(Game),
    NewMessage(ChatMessage),
}
