extern crate libc;
extern crate blit;
extern crate image;

extern crate minifb;
use minifb::{Window, WindowOptions, Key};

extern crate rlua;
use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

mod graphics;
use graphics::Graphics;
use std::sync::{Arc, Mutex};

struct Fisk {
    window: Window,
    graphics: Graphics,
    test: usize,
    smiley: usize,
}

unsafe impl Send for Fisk{}
unsafe impl Sync for Fisk{}

impl Fisk {
    fn new() -> Self {
        Self {
            window: Window::new("fisk", graphics::WIDTH, graphics::HEIGHT, WindowOptions::default()).expect("Unable to open window"),
            graphics: Graphics::new(),
            test: 0,
            smiley: 0,
        }
    }

    fn run_forerver(&mut self) {
        //Intead of calling self.load, eventually this should be replaced by the scripting engine
        //self.load();

        while self.window.is_open() {
            //Instead of calling self.draw, eventually this should be replaced by the scripting engine
            //self.draw();
            //This applies the computed array buffer
            self.window.update_with_buffer(&self.graphics.buffer).unwrap();
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
    let fisk = Arc::new(Mutex::new(Fisk::new()));

    let lua = Lua::new();
    let globals = lua.globals();

    let cloned_fisk = fisk.clone();
    globals.set(
        "new_image",
        lua.create_function(move |_, path: String| {
            Ok(cloned_fisk.lock().unwrap().graphics.new_image(&path))
        }).unwrap()
    ).unwrap();

    let cloned_fisk = fisk.clone();
    globals.set(
        "draw_image",
        lua.create_function(move |_, (id, x, y): (usize, i32, i32)| {
            Ok(cloned_fisk.lock().unwrap().graphics.draw_image(id, x, y))
        }).unwrap()
    ).unwrap();

    lua.exec::<()>(
        r#"
            smiley = new_image("smiley.png")
            draw_image(smiley, 100, 200)
        "#,
        None
    ).unwrap();

    fisk.lock().unwrap().run_forerver();
}
