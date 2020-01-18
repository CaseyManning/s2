// use serde::{Deserialize, Serialize};
// use vessels::{
//     kind::{Infallible, Serde},
//     object,
// };

// #[derive(Serialize, Deserialize, Clone)]
// pub struct NetPlayer {
//     pub x: f64,
//     pub y: f64,
//     pub color: f64,
// }

// #[object]
// pub trait GameState {
//     fn get_players(&self) -> Infallible<Serde<Vec<NetPlayer>>>;
// }
