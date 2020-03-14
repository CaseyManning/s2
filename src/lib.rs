use vessels::{
    kind::{Infallible, Serde},
    object,
};

#[object]
pub trait State {
    fn get_players(&self) -> Infallible<Serde<Vec<Vec<i32>>>>;

    fn next_id(&mut self) -> Infallible<i32>;

    fn update_position(&mut self, id: i32, x: i32, y: i32) -> Infallible<i32>;
}
