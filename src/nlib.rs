use serde::{Deserialize, Serialize};
use vessels::{
    kind::{Future, Serde, Stream},
    object, Kind,
};

#[derive(Serialize, Deserialize, Clone, Kind, Copy)]
pub struct NetPlayer {
    pub x: f64,
    pub y: f64,
    pub color: f64,
}

impl NetPlayer {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, color: 1.0 }
    }
}

#[object]
pub trait GameState {
    fn get_players(&self) -> Stream<Vec<NetPlayer>>;

    fn new_id(&mut self) -> Future<i32>;

    fn update_pos(&mut self, player: NetPlayer, id: usize) -> Future<i32>;
}
