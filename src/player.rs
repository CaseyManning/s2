use crate::canvas::Canvas;

use stdweb::console;

pub struct Player {
    pub x: f64,
    pub y: f64,
    direction: String,
    pub tx: f64,
    pub ty: f64,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Player {
        Player {
            x,
            y,
            direction: String::from("None"),
            tx: x,
            ty: y,
        }
    }
    pub fn update(&mut self) {
        // if self.direction == "Up" {
        //     self.y -= 1.0;
        // }
        // if self.direction == "Down" {
        //     self.y += 1.0;
        // }
        // if self.direction == "Left" {
        //     self.x -= 1.0;
        // }
        // if self.direction == "Right" {
        //     self.x += 1.0;
        // }
        let mag = ((self.tx - self.x) * (self.tx - self.x)
            + (self.ty - self.y) * (self.ty - self.y))
            .sqrt();
        let v = [(self.tx - self.x) / mag, (self.ty - self.y) / mag];

        self.x += v[0];
        self.y += v[1];

        // if self.tx > self.x {
        //     self.x += 1.0;
        // }
        // if self.tx < self.x {
        //     self.x -= 1.0;
        // }
        // if self.ty > self.y {
        //     self.y += 1.0;
        // }
        // if self.ty < self.y {
        //     self.y -= 1.0;
        // }
    }

    pub fn set_target(&mut self, tx: i32, ty: i32) {
        self.tx = tx.into();
        self.ty = ty.into();
    }

    pub fn change_dir(&mut self, direction: String) {
        self.direction = direction;
    }

    pub fn draw(&self, canvas: &Canvas) {
        canvas.draw(self.x, self.y, "orange");
    }
}
