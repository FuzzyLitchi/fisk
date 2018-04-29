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
    graphics: Arc<Mutex<Graphics>>,
    lua: Lua,
}

unsafe impl Send for Fisk{}
unsafe impl Sync for Fisk{}

impl Fisk {
    fn new() -> Self {
        let fisk = Self {
            window: Window::new("fisk", graphics::WIDTH, graphics::HEIGHT, WindowOptions::default()).expect("Unable to open window"),
            graphics: Arc::new(Mutex::new(Graphics::new())),
            lua: Lua::new(),
        };

        //New scope so fisk.lua is not borrowed afterwards
        {
            let globals = fisk.lua.globals();

            let cloned_graphics = fisk.graphics.clone();
            globals.set(
                "new_image",
                fisk.lua.create_function(move |_, path: String| {
                    Ok(cloned_graphics.lock().unwrap().new_image(&path))
                }).unwrap()
            ).unwrap();

            let cloned_graphics = fisk.graphics.clone();
            globals.set(
                "draw_image",
                fisk.lua.create_function(move |_, (id, x, y): (usize, i32, i32)| {
                    Ok(cloned_graphics.lock().unwrap().draw_image(id, x, y))
                }).unwrap()
            ).unwrap();

            fisk.lua.exec::<()>(
                r#"
function load()
    smiley = new_image("smiley.png")
    i = 0
end

function draw()
    i = i + 1
    draw_image(smiley, i, 200)
end
                "#,
                None
            ).unwrap();
        }

        fisk
    }

    fn run_forerver(&mut self) {
        self.lua.eval::<()>("load()", None).unwrap();

        while self.window.is_open() {
            //Clear screen
            self.graphics.lock().unwrap().clear();
            self.lua.eval::<()>("draw()", None).unwrap();

            //This applies the computed array buffer
            self.window.update_with_buffer(&self.graphics.lock().unwrap().buffer).unwrap();
        }
    }
}

pub fn main() {
    let mut fisk = Fisk::new();
    fisk.run_forerver();
}
