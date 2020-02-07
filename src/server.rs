use vessels::{
    channel::IdChannel,
    core::{
        hal::{crypto::Hasher, network::Server},
        register, run,
    },
    format::Cbor,
    kind::{Future, Serde},
    log,
    replicate::{Share, Shared},
};

mod nlib;
pub use self::nlib::GameState;
pub use self::nlib::NetPlayer;

pub struct Game {
    pub players: Vec<NetPlayer>,
}

impl GameState for Game {
    fn get_players(&self) -> Future<Serde<Vec<NetPlayer>>> {
        let mut players = self.players.clone();
        log!("Giving Player Positions");
        Box::pin(async move { Serde(players) })
    }
    fn update_pos(&mut self, player: NetPlayer, id: usize) -> Future<i32> {
        if id >= self.players.len() {
            self.players.push(player)
        } else {
            self.players[id] = player;
        }
        log!("Got a player position");
        Box::pin(async move { 2 })
    }

    fn new_id(&mut self) -> Future<i32> {
        let i: i32 = self.players.len() as i32;
        log!("Giving Out a new Id");
        Box::pin(async move { i })
    }
}

impl Game {
    pub fn new() -> Self {
        Self { players: vec![] }
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
        server
            .listen::<Box<dyn GameState>, IdChannel, Cbor>(
                BIND.parse().unwrap(),
                Box::new(move || {
                    let board = board.share();
                    Box::pin(async move { Box::new(board.share()) as Box<dyn GameState> })
                }),
            )
            .await
            .unwrap();
    });
}
