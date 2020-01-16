#[macro_use]
extern crate stdweb;

mod canvas;
// mod nlib;
mod player;

use canvas::Canvas;
// use nlib::GameState;
use player::Player;

use stdweb::traits::*;
use stdweb::web::{event::KeyDownEvent, event::MouseMoveEvent, IEventTarget};

use std::cell::RefCell;
use std::rc::Rc;
use stdweb::console;
use stdweb::web::event;

// use vessels::{
//     channel::IdChannel,
//     core::{hal::network::Client, run},
//     format::Cbor,
//     log,
// };

pub enum GameEvent {
    KeyDown(event::KeyDownEvent),
    MouseMove(event::MouseMoveEvent),
}

fn main() {
    ::std::panic::set_hook(Box::new(|info| {
        console!(error, format!("!!! RUST PANIC !!! {:?}", info));
    }));

    stdweb::initialize();

    let canvas = Canvas::new("#canvas");

    let player = Rc::new(RefCell::new(Player::new(50.0, 50.0)));

    stdweb::web::document().add_event_listener({
        let player = player.clone();
        move |event: MouseMoveEvent| {
            player
                .borrow_mut()
                .set_target(event.client_x(), event.client_y());
        }
    });

    fn game_loop(player: Rc<RefCell<Player>>, canvas: Rc<Canvas>, time: u32) {
        stdweb::web::set_timeout(
            move || {
                game_loop(player.clone(), canvas.clone(), time);
                player.borrow_mut().update();
                canvas.clear_all();
                player.borrow().draw(&canvas);
            },
            time,
        );
    }

    game_loop(player, Rc::new(canvas), 20);

    stdweb::event_loop();
}
