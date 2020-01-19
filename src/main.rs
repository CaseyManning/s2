// mod canvas;
mod nlib;
mod player;

// use canvas::Canvas;
use nlib::GameState;
use nlib::NetPlayer;
use player::Player;
use std::{cell::RefCell, panic, rc::Rc};
// use stdweb::console;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{console::log_1, window, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use vessels::{
    channel::IdChannel,
    core::{hal::network::Client, run},
    format::Cbor,
    log,
};

use futures::lock::Mutex;
use lazy_static::*;

lazy_static! {
    static ref plyr: Mutex<Player> = Mutex::new(Player::new(50.0, 50.0));
}

lazy_static! {
    static ref id: Mutex<i32> = Mutex::new(-1);
}

lazy_static! {
    static ref others: Mutex<Vec<NetPlayer>> = Mutex::new(vec![]);
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let w = window().expect("Could not get window");
    let document = w.document().expect("Could not get document");

    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .expect("no");

    let context = canvas
        .get_context("2d")
        .expect("uh")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("oh");

    let closure: Rc<RefCell<Option<Closure<dyn Fn(_)>>>> = Rc::new(RefCell::new(None));
    let cb = closure.clone();
    *closure.borrow_mut() = Some(Closure::wrap(Box::new(move |elapsed: f64| {
        // log_1(&JsValue::from_f64(elapsed));
        context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
        context.set_fill_style(&JsValue::from_str("blue"));
        let mut guard = (plyr).try_lock().unwrap();
        context.fill_rect(guard.x, guard.y, 20.0, 20.0);
        guard.update(elapsed);
        drop(guard);

        // let gOthers = (others).try_lock().unwrap();
        // for enemy in gOthers.iter() {
        //     context.set_fill_style(&JsValue::from_str("red"));
        //     context.fill_rect(enemy.x, enemy.y, 20.0, 20.0);
        // }

        // let gid = (id).try_lock().unwrap();
        // log_1(&JsValue::from_f64((*gid as f64)));

        window()
            .unwrap()
            .request_animation_frame(cb.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn Fn(_)>));

    window()
        .unwrap()
        .request_animation_frame(closure.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
    let window = window().unwrap();
    let mouse_move_handler = Closure::wrap(Box::new(|e: MouseEvent| {
        // let player = &player.clone();
        let mut guard = plyr.try_lock().unwrap();
        // log_1(&JsValue::from_f64(guard.x));
        guard.set_target(e.client_x(), e.client_y());
        drop(guard);
    }) as Box<dyn FnMut(_)>);
    window
        .add_event_listener_with_callback("mousemove", mouse_move_handler.as_ref().unchecked_ref())
        .unwrap();
    mouse_move_handler.forget();

    run(async move {
        let mut network = Client::new().unwrap();
        let mut server = network
            .connect::<Box<dyn GameState>, IdChannel, Cbor>("ws://127.0.0.1:61200".parse().unwrap())
            .await
            .unwrap();

        let mut id_guard = id.lock().await;
        *id_guard = server.new_id().await;
        drop(id_guard);

        loop {
            let ships = server.get_players().await.0;
            // ships.remove(*id_guard as usize);

            let mut others_guard = others.lock().await;
            // others_guard = ships;
            std::mem::replace(&mut *others_guard, ships);
            drop(others_guard);
            drop(ships);

            let mut idGuard = id.lock().await;
            if *idGuard > -1 {
                server.update_pos(NetPlayer::new(0.0, 0.0), *idGuard as usize);
                drop(idGuard);
            }
        }
    });
}
