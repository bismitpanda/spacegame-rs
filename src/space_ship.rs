use raylib::prelude::*;

use crate::laser::Laser;

pub struct SpaceShip<'a> {
    pub lasers: Vec<Laser>,
    image: &'a Texture2D,
    pos: Vector2,
    last_fire_time: f64,
    laser_sound: &'a Sound<'a>,
}

impl<'a> SpaceShip<'a> {
    pub fn new(handle: &RaylibHandle, image: &'a Texture2D, laser_sound: &'a Sound<'a>) -> Self {
        let img_width = image.width;
        let img_height = image.height;

        Self {
            lasers: Vec::new(),
            image,
            pos: Vector2::new(
                ((handle.get_screen_width() - img_width) / 2) as f32,
                (handle.get_screen_height() - img_height - 100) as f32,
            ),
            last_fire_time: 0.0,
            laser_sound,
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_texture_v(self.image, self.pos, Color::WHITE);
    }

    pub fn move_left(&mut self) {
        self.pos.x -= 7.0;
        if self.pos.x < 25.0 {
            self.pos.x = 25.0;
        }
    }

    pub fn move_right(&mut self, handle: &RaylibHandle) {
        self.pos.x += 7.0;
        if self.pos.x > (handle.get_screen_width() - self.image.width - 25) as f32 {
            self.pos.x = (handle.get_screen_width() - self.image.width - 25) as f32;
        }
    }

    pub fn fire_laser(&mut self, handle: &RaylibHandle) {
        if handle.get_time() - self.last_fire_time >= 0.35 {
            let laser = Laser::new(
                Vector2::new(self.pos.x + (self.image.width / 2 - 2) as f32, self.pos.y),
                -6,
            );
            self.lasers.push(laser);

            self.last_fire_time = handle.get_time();
            self.laser_sound.play();
        }
    }

    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: self.image.width as f32,
            height: self.image.height as f32,
        }
    }

    pub fn reset(&mut self, handle: &RaylibHandle) {
        self.pos.x = (handle.get_screen_width() - self.image.width) as f32 / 2.0;
        self.pos.y = (handle.get_screen_height() - self.image.height - 100) as f32;

        self.lasers.clear();
    }
}
