use crate::chess::Board;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct GameState {
    moves: String,
    wtime: u64,
    btime: u64,
    status: String,
}

impl GameState {
    pub fn moves(&self) -> &String {
        &self.moves
    }

    pub fn wtime(&self) -> &u64 {
        &self.wtime
    }

    pub fn btime(&self) -> &u64 {
        &self.btime
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct GameData {
    clock: Clock,
    rated: bool,
    white: Player,
    black: Player,
}

impl GameData {
    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    pub fn rated(&self) -> &bool {
        &self.rated
    }

    pub fn white(&self) -> &Player {
        &self.white
    }

    pub fn black(&self) -> &Player {
        &self.black
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Clock {
    initial: u64,
    increment: u64,
}

impl Clock {
    pub fn initial(&self) -> &u64 {
        &self.initial
    }

    pub fn increment(&self) -> &u64 {
        &self.increment
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Player {
    id: String,
    name: String,
    rating: u32,
}

impl Player {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn rating(&self) -> &u32 {
        &self.rating
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    id: String,
    move_count: u32,
    data: GameData,
    state: GameState,
}

impl Game {
    pub fn new<T: ToString>(id: T, data: GameData, state: GameState) -> Self {
        Self {
            board: Board::default(),
            id: id.to_string(),
            move_count: 0,
            data,
            state,
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

    pub fn move_count(&self) -> &u32 {
        &self.move_count
    }

    pub fn incr_move_count(&mut self) {
        self.move_count += 1;
    }

    pub fn data(&self) -> &GameData {
        &self.data
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }
}
