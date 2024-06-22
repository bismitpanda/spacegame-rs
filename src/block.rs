use raylib::prelude::*;

pub struct Block {
    pos: Vector2,
}

impl Block {
    pub fn new(pos: Vector2) -> Self {
        Self { pos }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_rectangle(
            self.pos.x as i32,
            self.pos.y as i32,
            3,
            3,
            Color::new(243, 216, 63, 255),
        );
    }

    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: 3.0,
            height: 3.0,
        }
    }
}
