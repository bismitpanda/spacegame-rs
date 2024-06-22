use raylib::prelude::*;

use crate::{
    alien::{Alien, Kind as AlienKind},
    laser::Laser,
    mystery_ship::MysteryShip,
    obstacle::{Obstacle, GRID},
    space_ship::SpaceShip,
};

pub struct Game<'a> {
    pub run: bool,
    pub lives: i32,
    pub score: i32,
    pub music: Music<'a>,
    space_ship: SpaceShip<'a>,
    obstacles: Vec<Obstacle>,
    aliens: Vec<Alien<'a>>,
    aliens_horiz_directions: f32,
    aliens_vert_directions: f32,
    alien_lasers: Vec<Laser>,
    time_last_alien_fired: f64,
    mystery_ship: MysteryShip<'a>,
    mystery_ship_spawn_interval: f64,
    time_last_spawn: f64,
    explosion_sound: &'a Sound<'a>,
    alien_images: [&'a Texture2D; 3],
}

impl<'a> Game<'a> {
    pub fn new(
        space_ship: SpaceShip<'a>,
        mystery_ship: MysteryShip<'a>,
        alien_images: [&'a Texture2D; 3],
        music: Music<'a>,
        explosion_sound: &'a Sound<'a>,
    ) -> Self {
        Self {
            run: false,
            lives: 0,
            score: 0,
            music,
            space_ship,
            obstacles: Vec::new(),
            aliens: Vec::new(),
            aliens_horiz_directions: 0.0,
            aliens_vert_directions: 0.0,
            alien_lasers: Vec::new(),
            time_last_alien_fired: 0.0,
            mystery_ship,
            mystery_ship_spawn_interval: 0.0,
            time_last_spawn: 0.0,
            explosion_sound,
            alien_images,
        }
    }

    pub const ALIEN_LASER_SHOOT_INTERVAL: f64 = 0.35;

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        self.space_ship.draw(draw_handle);

        for laser in &self.space_ship.lasers {
            laser.draw(draw_handle);
        }

        for obstacle in &self.obstacles {
            obstacle.draw(draw_handle);
        }

        for alien in &self.aliens {
            alien.draw(draw_handle);
        }

        for laser in &self.alien_lasers {
            laser.draw(draw_handle);
        }

        self.mystery_ship.draw(draw_handle);
    }

    pub fn handle_input(&mut self, handle: &RaylibHandle) {
        if self.run {
            if handle.is_key_down(KeyboardKey::KEY_LEFT) {
                self.space_ship.move_left();
            } else if handle.is_key_down(KeyboardKey::KEY_RIGHT) {
                self.space_ship.move_right(handle);
            } else if handle.is_key_down(KeyboardKey::KEY_SPACE) {
                self.space_ship.fire_laser(handle);
            }
        }
    }

    pub fn update(&mut self, handle: &RaylibHandle) {
        if self.run {
            let curr_time = handle.get_time();

            if curr_time - self.time_last_spawn > self.mystery_ship_spawn_interval {
                self.mystery_ship.spawn(handle);
                self.time_last_spawn = handle.get_time();
                self.mystery_ship_spawn_interval = handle.get_random_value(10..20);
            }

            for laser in &mut self.space_ship.lasers {
                laser.update(handle);
            }

            self.move_aliens(handle);
            self.alien_shoot_laser(handle);

            for laser in &mut self.alien_lasers {
                laser.update(handle);
            }

            self.delete_inactive_lasers();
            self.mystery_ship.update(handle);

            self.check_for_collisions();
        } else if handle.is_key_down(KeyboardKey::KEY_ENTER) {
            self.reset(handle);
            self.init_game(handle);
        }
    }

    pub fn delete_inactive_lasers(&mut self) {
        self.space_ship.lasers.retain(|laser| laser.active);
        self.alien_lasers.retain(|laser| laser.active);
    }

    pub fn move_aliens(&mut self, handle: &RaylibHandle) {
        for alien in &mut self.aliens {
            if alien.pos.x as i32 + self.alien_images[alien.kind as usize].width
                > handle.get_screen_width() - 25
            {
                self.aliens_horiz_directions = -1.0;
            }

            if alien.pos.x < 25.0 {
                self.aliens_horiz_directions = 1.0;
            }

            if alien.pos.y as i32 + self.alien_images[alien.kind as usize].height
                > handle.get_screen_height() - 210
            {
                self.aliens_vert_directions = -1.0;
            }

            if alien.pos.y < 110.0 {
                self.aliens_vert_directions = 1.0;
            }

            alien.pos.y += self.aliens_vert_directions;
            alien.pos.x += self.aliens_horiz_directions;
        }
    }

    pub fn alien_shoot_laser(&mut self, handle: &RaylibHandle) {
        let current_time = handle.get_time();

        if current_time - self.time_last_alien_fired >= Self::ALIEN_LASER_SHOOT_INTERVAL
            && !self.aliens.is_empty()
        {
            let random_index =
                handle.get_random_value::<i32>(0..self.aliens.len() as i32 - 1) as usize;
            let alien = &self.aliens[random_index];

            self.alien_lasers.push(Laser::new(
                Vector2::new(
                    alien.pos.x + (self.alien_images[alien.kind as usize].width / 2) as f32,
                    alien.pos.y + (self.alien_images[alien.kind as usize].height) as f32,
                ),
                6,
            ));

            self.time_last_alien_fired = handle.get_time();
        }
    }

    pub fn check_for_collisions(&mut self) {
        for laser in &mut self.space_ship.lasers {
            self.aliens.retain(|alien| {
                if alien.get_rect().check_collision_recs(&laser.get_rect()) {
                    self.explosion_sound.play();

                    match alien.kind {
                        AlienKind::Skull => self.score += 100,
                        AlienKind::Bug => self.score += 200,
                        AlienKind::Octopus => self.score += 300,
                    }

                    laser.active = false;
                    false
                } else {
                    true
                }
            });

            for obstacle in &mut self.obstacles {
                obstacle.blocks.retain(|block| {
                    if block.get_rect().check_collision_recs(&laser.get_rect()) {
                        laser.active = false;
                        false
                    } else {
                        true
                    }
                });
            }

            if self
                .mystery_ship
                .get_rect()
                .check_collision_recs(&laser.get_rect())
            {
                self.mystery_ship.alive = false;
                laser.active = false;
                self.score += 500;

                self.explosion_sound.play();
            }
        }

        for laser in &mut self.alien_lasers {
            if laser
                .get_rect()
                .check_collision_recs(&self.space_ship.get_rect())
            {
                laser.active = false;
                self.lives -= 1;
            }

            for obstacle in &mut self.obstacles {
                obstacle.blocks.retain(|block| {
                    if block.get_rect().check_collision_recs(&laser.get_rect()) {
                        laser.active = false;
                        false
                    } else {
                        true
                    }
                });
            }
        }

        for alien in &self.aliens {
            for obstacle in &mut self.obstacles {
                obstacle
                    .blocks
                    .retain(|block| !block.get_rect().check_collision_recs(&alien.get_rect()));
            }

            if alien
                .get_rect()
                .check_collision_recs(&self.space_ship.get_rect())
            {
                self.lives -= 1;
            }
        }

        if self.lives <= 0 {
            self.game_over();
        }
    }

    pub fn game_over(&mut self) {
        self.run = false;
    }

    pub fn create_obstacles(handle: &RaylibHandle) -> Vec<Obstacle> {
        let obstacle_width = (GRID[0].len() * 3) as i32;
        let gap = 4.0f32.mul_add(-(obstacle_width as f32), handle.get_screen_width() as f32) / 5.0;

        (0..4)
            .map(|i| {
                let off_x = (i as f32 + 1.0).mul_add(gap, i as f32 * obstacle_width as f32);

                Obstacle::new(Vector2::new(
                    off_x,
                    (handle.get_screen_height() - 200) as f32,
                ))
            })
            .collect()
    }

    pub fn create_aliens(alien_images: [&'a Texture2D; 3]) -> Vec<Alien> {
        (0..5)
            .map(|row| {
                (0..11).map(move |col| {
                    let alien_kind = if row == 0 {
                        AlienKind::Octopus
                    } else if row == 1 || row == 2 {
                        AlienKind::Bug
                    } else {
                        AlienKind::Skull
                    };

                    Alien::new(
                        alien_kind,
                        Vector2::new((75 + col * 55) as f32, (110 + row * 55) as f32),
                        &alien_images[alien_kind as usize],
                    )
                })
            })
            .flatten()
            .collect()
    }

    pub fn init_game(&mut self, handle: &RaylibHandle) {
        self.obstacles = Self::create_obstacles(handle);
        self.aliens = Self::create_aliens(self.alien_images);
        self.aliens_horiz_directions = 1.0;
        self.aliens_vert_directions = 1.0;
        self.time_last_alien_fired = 0.0;
        self.time_last_spawn = 0.0;
        self.lives = 3;
        self.score = 0;
        self.run = true;
        self.mystery_ship_spawn_interval = handle.get_random_value(10..20);
    }

    pub fn reset(&mut self, handle: &RaylibHandle) {
        self.space_ship.reset(handle);
        self.aliens.clear();
        self.alien_lasers.clear();
        self.obstacles.clear();
    }
}
