pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(attr_id: &str) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let width = canvas.width();
        let height = canvas.height();

        Canvas {
            canvas,
            ctx,
            width,
            height,
        }
    }

    pub fn draw(&self, x: f64, y: f64, color: &str) {
        assert!(x < f64::from(self.width));
        assert!(y < f64::from(self.height));

        self.ctx.set_fill_style_color(color);

        self.ctx.fill_rect(x, y, 20.0, 20.0);
    }

    pub fn clear_all(&self) {
        self.ctx.set_fill_style_color("white");
        self.ctx
            .fill_rect(0.0, 0.0, f64::from(self.width), f64::from(self.height))
    }
}
