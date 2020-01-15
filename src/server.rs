use vessels::{
    log,
    kind::{Infallible, Serde},
    replicate::{Share, Shared},
    channel::IdChannel,
    core::{register, run, hal::{network::Server, crypto::Hasher}},
    format::Cbor,
};

mod lib;
pub use self::lib::GameState;
pub use self::lib::Player;

pub struct Game {
    pub players: Vec<Player>,
}

impl GameState for Game {
    fn get_players(&self) -> Infallible<Serde<Vec<Player>>> {
        let players = self.players.clone();
        Box::pin(async move {Ok(Serde(players))})
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: vec![],
        }
    }
}
impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

const BIND: &str = "127.0.0.1:61200";

fn main() {

    let mut server = Server::new().unwrap();
    let board = Shared::new(Box::new(Game::new()) as Box<dyn GameState>);
    register(|| Hasher::new().unwrap());
    run(async move {
            server.listen::<Box<dyn GameState>, IdChannel, Cbor>(
                BIND.parse().unwrap(),
                Box::new(move || {
                    let board = board.share();
                    Box::pin(async move { Box::new(board.share()) as Box<dyn GameState> })
                }),
            ).await.unwrap();
        }
    );
}