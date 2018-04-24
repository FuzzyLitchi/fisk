extern crate libc;
extern crate blit;
extern crate image;

extern crate minifb;
use minifb::{Window, WindowOptions, Key};

mod graphics;
use graphics::Graphics;

struct Fisk {
    graphics: Graphics,
    test: usize,
    smiley: usize,
}

impl Fisk {
    fn new() -> Self {
        Self {
            graphics: Graphics::new(),
            test: 0,
            smiley: 0,
        }
    }

    fn load(&mut self) {
        self.test = self.graphics.new_image("test.png");
        self.smiley = self.graphics.new_image("smiley.png");
    }

    fn update(&mut self) {
        //let dt = timer::get_delta(ctx).subsec_nanos() as f64 / 1_000_000_000.0;
    }

    fn draw(&mut self) {
        //Clear the screen
        self.graphics.clear();

        //Tmp test stuff
        let id = self.test;
        self.graphics.draw_image(id, 0, 0);
        let id = self.smiley;
        self.graphics.draw_image(id, 0, 0);
    }
}

pub fn main() {
    let mut fisk = Fisk::new();

    let mut window = Window::new("fisk", graphics::WIDTH, graphics::HEIGHT, WindowOptions::default()).expect("Unable to open window");

    fisk.load();

    while window.is_open() {
        fisk.draw();
        window.update_with_buffer(&fisk.graphics.buffer).unwrap();
    }
}
