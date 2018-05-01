extern crate libc;
extern crate blit;
extern crate image;

extern crate minifb;
use minifb::{Window, WindowOptions};

extern crate rlua;
use rlua::{Lua, Table, Function};

mod graphics;
use graphics::Graphics;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::prelude::*;

struct Fisk {
    window: Window,
    graphics: Arc<Mutex<Graphics>>,
    lua: Lua,
}

impl Fisk {
    fn new() -> Self {
        let fisk = Self {
            window: Window::new("fisk", graphics::WIDTH, graphics::HEIGHT, WindowOptions::default()).expect("Unable to open window"),
            graphics: Arc::new(Mutex::new(Graphics::new())),
            lua: Lua::new(),
        };

        //New scope so fisk.lua is not borrowed afterwards
        {
            //The "fisk" table in lua
            let lua_fisk = fisk.lua.create_table().unwrap();

            let cloned_graphics = fisk.graphics.clone();
            lua_fisk.set(
                "new_image",
                fisk.lua.create_function(move |_, path: String| {
                    Ok(cloned_graphics.lock().unwrap().new_image(&path))
                }).unwrap()
            ).unwrap();

            let cloned_graphics = fisk.graphics.clone();
            lua_fisk.set(
                "draw_image",
                fisk.lua.create_function(move |_, (id, x, y): (usize, i32, i32)| {
                    cloned_graphics.lock().unwrap().draw_image(id, x, y);
                    Ok(())
                }).unwrap()
            ).unwrap();

            //Set the fisk array with the fisk functions to fisk in lua
            fisk.lua.globals().set("fisk", lua_fisk).unwrap();
        }

        fisk
    }

    fn load_script(&self, script: &str) {
        self.lua.exec::<()>(script, None).expect("Error in loaded script");
    }

    fn run_forerver(&mut self) {
        //Find functions
        let load: Function;
        let update: Function;
        let draw: Function;
        {
            let lua_fisk = self.lua.globals().get::<&str, Table>("fisk").unwrap();
            load = lua_fisk.get::<&str, Function>("load").unwrap();
            update = lua_fisk.get::<&str, Function>("update").unwrap();
            draw = lua_fisk.get::<&str, Function>("draw").unwrap();
        }
        //Call load
        load.call::<_, ()>(()).unwrap();

        while self.window.is_open() {
            update.call::<_, ()>((/*Delta time here*/)).unwrap();

            //Clear screen
            self.graphics.lock().unwrap().clear();
            draw.call::<_, ()>(()).unwrap();

            //This applies the computed array buffer
            self.window.update_with_buffer(&self.graphics.lock().unwrap().buffer).unwrap();
        }
    }
}

pub fn main() {
    let mut fisk = Fisk::new();

    let mut script = String::new();
    let mut file = File::open("main.lua").expect("File not found");
    file.read_to_string(&mut script).expect("Can't read file");
    fisk.load_script(&script);

    fisk.run_forerver();
}
