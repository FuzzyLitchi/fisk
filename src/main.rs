use std::collections::HashMap;
use std::mem;

extern crate libc;

extern crate blit;
use blit::{BlitExt, Color};

extern crate minifb;
use minifb::{Window, WindowOptions, Key};

extern crate image;
use image::DynamicImage;

extern crate rand;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

struct Fisk {
    images: HashMap<u64, DynamicImage>,
    buffer: Vec<u32>,
    test: u64,
    smiley: u64,
}

impl Fisk {
    fn new() -> Self {
        Fisk {
            images: HashMap::new(),
            buffer: vec![0x00FFFFFF; WIDTH * HEIGHT],
            test: 0,
            smiley: 0,
        }
    }

    fn new_image(&mut self, path: &str) -> u64 {
        let id = rand::random::<u64>();
        self.images.insert(id, image::open(path).unwrap());
        id
    }

    fn draw_image(&mut self, id: u64, x: i32, y: i32) {
        let image = self.images.get(&id).unwrap();

        match image.as_rgba8() {
            Some(rgba) => rgba.blit(
                    &mut self.buffer,
                    WIDTH,
                    (x, y),
                    Color::from_u32(0xFF00FF)
                ),
            None => image.as_rgb8().unwrap().blit(
                    &mut self.buffer,
                    WIDTH,
                    (x, y),
                    Color::from_u32(0xFF00FF)
                )
        };
    }

    fn load(&mut self) {
        self.test = self.new_image("test.png");
        self.smiley = self.new_image("smiley.png");
    }

    fn update(&mut self) {
        //let dt = timer::get_delta(ctx).subsec_nanos() as f64 / 1_000_000_000.0;
    }

    fn draw(&mut self) {
        unsafe {
            libc::memset(
                self.buffer.as_mut_ptr() as _,
                0x00FFFFFF,
                self.buffer.len() * mem::size_of::<u32>(),
            );
        }
        let id = self.test;
        self.draw_image(id, 0, 0);
        let id = self.smiley;
        self.draw_image(id, 0, 0);
    }
}

pub fn main() {
    let mut fisk = Fisk::new();

    let mut window = Window::new("fisk", WIDTH, HEIGHT, WindowOptions::default()).expect("Unable to open window");

    fisk.load();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        fisk.draw();
        window.update_with_buffer(&fisk.buffer).unwrap();
    }
}
