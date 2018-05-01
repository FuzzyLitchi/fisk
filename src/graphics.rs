use std::mem;
use libc;

use blit::{BlitExt, Color};

use image;
use image::DynamicImage;

pub const WIDTH: usize = 512;
pub const HEIGHT: usize = 512;

pub struct Graphics {
    images: Vec<DynamicImage>,
    pub buffer: Vec<u32>,
}

impl Graphics {
    pub fn new() -> Self {
        Self {
            images: Vec::new(),
            buffer: vec![0x00FFFFFF; WIDTH * HEIGHT],
        }
    }

    pub fn new_image(&mut self, path: &str) -> usize {
        self.images.push(image::open(path).expect("Image not found"));
        self.images.len() - 1
    }

    pub fn draw_image(&mut self, id: usize, x: i32, y: i32) {
        let image = &self.images[id];

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

    pub fn clear(&mut self) {
        unsafe {
            libc::memset(
                self.buffer.as_mut_ptr() as _,
                0x00FFFFFF,
                self.buffer.len() * mem::size_of::<u32>(),
            );
        }
    }
}
