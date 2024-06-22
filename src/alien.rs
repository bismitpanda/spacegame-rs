use raylib::core::drawing::RaylibDraw;
use raylib::prelude::*;

#[derive(Clone, Copy)]
pub enum Kind {
    Skull,
    Bug,
    Octopus,
}

pub struct Alien<'a> {
    pub pos: Vector2,
    pub kind: Kind,
    pub image: &'a Texture2D,
}

impl<'a> Alien<'a> {
    pub fn new(kind: Kind, pos: Vector2, image: &'a Texture2D) -> Self {
        Self { pos, kind, image }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_texture_v(self.image, self.pos, Color::WHITE);
    }

    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: self.image.width as f32,
            height: self.image.height as f32,
        }
    }
}
