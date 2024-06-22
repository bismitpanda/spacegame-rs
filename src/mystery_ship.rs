use raylib::prelude::*;

pub struct MysteryShip<'a> {
    pub alive: bool,
    pos: Vector2,
    image: &'a Texture2D,
    speed: i32,
}

impl<'a> MysteryShip<'a> {
    pub fn new(image: &'a Texture2D) -> Self {
        Self {
            alive: false,
            pos: Vector2::zero(),
            speed: 0,
            image,
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        if self.alive {
            draw_handle.draw_texture_v(&self.image, self.pos, Color::WHITE);
        }
    }

    pub fn update(&mut self, handle: &RaylibHandle) {
        if self.alive {
            self.pos.x += self.speed as f32;

            if self.pos.x as i32 > handle.get_screen_width() - self.image.width - 25
                || self.pos.x < 25.
            {
                self.alive = false;
            }
        }
    }

    pub fn get_rect(&self) -> Rectangle {
        if self.alive {
            Rectangle {
                x: self.pos.x,
                y: self.pos.y,
                width: self.image.width as f32,
                height: self.image.height as f32,
            }
        } else {
            Rectangle {
                x: self.pos.x,
                y: self.pos.y,
                width: 0.0,
                height: 0.0,
            }
        }
    }

    pub fn spawn(&mut self, handle: &RaylibHandle) {
        self.pos.y = 90.0;
        let side = handle.get_random_value::<i32>(0..1);

        if side == 0 {
            self.pos.x = 25.0;
            self.speed = 3;
        } else {
            self.pos.x = (handle.get_screen_width() - self.image.width - 25) as f32;
            self.speed = -3;
        }

        self.alive = true;
    }
}
