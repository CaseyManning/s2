// mod canvas;
// mod nlib;
mod player;

// use canvas::Canvas;
// use nlib::GameState;
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

use lazy_static::*;
use std::sync::Mutex;

lazy_static! {
    static ref plyr: Mutex<Player> = Mutex::new(Player::new(50.0, 50.0));
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
        context.set_fill_style(&JsValue::from_str("orange"));
        let mut guard = (plyr).lock().unwrap();
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
        // let player = &player.clone();
        let mut guard = plyr.lock().unwrap();
        // log_1(&JsValue::from_f64(guard.x));
        guard.set_target(e.client_x(), e.client_y());
        drop(guard);
    }) as Box<dyn FnMut(_)>);
    window
        .add_event_listener_with_callback("mousemove", mouse_move_handler.as_ref().unchecked_ref())
        .unwrap();
    mouse_move_handler.forget();
}
