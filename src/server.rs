use vessels::{
    channel::IdChannel,
    core::{
        hal::{crypto::Hasher, network::Server},
        register, run,
    },
    format::Cbor,
    kind::{Infallible, Serde},
    log,
    replicate::{Share, Shared},
};

mod lib;
pub use self::lib::State;

pub struct BoardState {
    pub state: Vec<Vec<i32>>,
    pub current_players: i32,
}

impl State for BoardState {
    fn get_players(&self) -> Infallible<Serde<Vec<Vec<i32>>>> {
        log!("Getting the Board");
        let state = self.state.clone();
        Box::pin(async move { Ok(Serde(state)) })
    }

    fn next_id(&mut self) -> Infallible<i32> {
        log!("New player, id: {}", self.current_players);
        let current_players = self.current_players;
        self.current_players += 1;
        self.state.push(vec![0, 0]);
        Box::pin(async move { Ok(current_players) })
    }

    fn update_position(&mut self, id: i32, x: i32, y: i32) -> Infallible<i32> {
        self.state[id as usize] = vec![x, y];
        Box::pin(async move { Ok(0) })
    }
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            state: vec![],
            current_players: 0,
        }
    }
}
impl Default for BoardState {
    fn default() -> Self {
        BoardState::new()
    }
}

const BIND: &str = "127.0.0.1:61200";

fn main() {
    let mut server = Server::new().unwrap();
    let board = Shared::new(Box::new(BoardState::new()) as Box<dyn State>);
    register(|| Hasher::new().unwrap());
    run(async move {
        server
            .listen::<Box<dyn State>, IdChannel, Cbor>(
                BIND.parse().unwrap(),
                Box::new(move || {
                    let board = board.share();
                    Box::pin(async move { Box::new(board.share()) as Box<dyn State> })
                }),
            )
            .await
            .unwrap();
    });
}
