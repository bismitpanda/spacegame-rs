use raylib::prelude::*;

pub struct Laser {
    pub active: bool,
    pos: Vector2,
    speed: i32,
}

impl Laser {
    pub fn new(pos: Vector2, speed: i32) -> Self {
        Self {
            active: true,
            pos,
            speed,
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        if self.active {
            draw_handle.draw_rectangle(
                self.pos.x as i32,
                self.pos.y as i32,
                4,
                15,
                Color::new(243, 216, 63, 255),
            );
        }
    }

    pub fn update(&mut self, handle: &RaylibHandle) {
        self.pos.y += self.speed as f32;

        if self.active && self.pos.y as i32 > handle.get_screen_height() - 100 || self.pos.y < 25. {
            self.active = false;
        }
    }

    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: 4.0,
            height: 15.0,
        }
    }
}
