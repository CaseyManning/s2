// mod canvas;
mod nlib;
mod player;
// use nlib::GameState;
// use nlib::NetPlayer;
use player::Player;
use std::{cell::RefCell, panic, rc::Rc};
// use stdweb::console;
use futures::lock::Mutex;
use futures::StreamExt;
use lazy_static::*;
// use vessels::{
//     channel::IdChannel,
//     core::{hal::network::Client, run},
//     format::Cbor,
//     kind::Stream,
//     log,
// };
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{console::log_1, window, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

lazy_static! {
    static ref plyr: Mutex<Player> = Mutex::new(Player::new(50.0, 50.0));
}

// lazy_static! {
//     static ref id: Mutex<i32> = Mutex::new(-1);
// }

// lazy_static! {
//     static ref others: Mutex<Stream<Vec<NetPlayer>>> = Mutex::new(Stream::new(vec![1]));
// }

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
    *closure.borrow_mut() = Some(Closure::wrap(Box::new(move |elapsed| {
        // log_1(&JsValue::from_f64(elapsed));
        context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
        context.set_fill_style(&JsValue::from_str("blue"));
        let mut guard = (plyr).try_lock().unwrap();
        context.fill();

        context.fill_rect(guard.x, guard.y, 20.0, 20.0);
        guard.update(elapsed);
        drop(guard);

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
        let mut guard = plyr.try_lock().unwrap();
        guard.set_target(e.client_x() + -18, e.client_y() - 18);
        drop(guard);
    }) as Box<dyn FnMut(_)>);
    window
        .add_event_listener_with_callback("mousemove", mouse_move_handler.as_ref().unchecked_ref())
        .unwrap();
    mouse_move_handler.forget();

    let mouse_click_handler = Closure::wrap(Box::new(|e: MouseEvent| {
        log_1(&JsValue::from_f64(e.button().into()));
    }) as Box<dyn FnMut(_)>);
    window
        .add_event_listener_with_callback("mousedown", mouse_click_handler.as_ref().unchecked_ref())
        .unwrap();
    mouse_click_handler.forget();

    // run(async move {
    //     let mut network = Client::new().unwrap();
    //     let mut server = network
    //         .connect::<Box<dyn GameState>, IdChannel, Cbor>("ws://127.0.0.1:61200".parse().unwrap())
    //         .await
    //         .unwrap();

    //     let mut id_guard = id.lock().await;
    //     *id_guard = server.new_id().await;
    //     drop(id_guard);

    //     let mut stream: Stream<Vec<NetPlayer>> = server.get_players();
    //     // ships.remove(*id_guard as usize);

    //     let mut others_guard = others.lock().await;
    //     std::mem::replace(&mut *others_guard, stream);
    //     drop(others_guard);

    //     // drop(others_guard);

    //     let mut idGuard = id.lock().await;
    //     if *idGuard > -1 {
    //         server.update_pos(NetPlayer::new(0.0, 0.0), *idGuard as usize);
    //         drop(idGuard);
    //     }
    // });
}
