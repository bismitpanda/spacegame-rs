mod alien;
mod block;
mod game;
mod laser;
mod mystery_ship;
mod obstacle;
mod space_ship;

use game::Game;
use mystery_ship::MysteryShip;
use raylib::prelude::*;
use space_ship::SpaceShip;

fn main() {
    let gray = Color::new(29, 29, 27, 255);
    let yellow = Color::new(243, 216, 63, 255);

    let offset = 50;
    let window_width = 750;
    let window_height = 700;

    let (mut rl, thread) = raylib::init()
        .size(window_width + offset, window_height + 2 * offset)
        .title("Space Invaders")
        .build();

    let ra = RaylibAudio::init_audio_device().unwrap();

    let font = rl
        .load_font_ex(&thread, "assets/font/monogram.ttf", 64, None)
        .unwrap();
    let space_ship_image = rl
        .load_texture(&thread, "assets/images/spaceship.png")
        .unwrap();
    let alien_images = [
        &rl.load_texture(&thread, "assets/images/alien_1.png")
            .unwrap(),
        &rl.load_texture(&thread, "assets/images/alien_2.png")
            .unwrap(),
        &rl.load_texture(&thread, "assets/images/alien_3.png")
            .unwrap(),
    ];
    let mystery = rl
        .load_texture(&thread, "assets/images/mystery.png")
        .unwrap();
    let laser_sound = ra.new_sound("assets/sounds/laser.ogg").unwrap();
    let music = ra.new_music("assets/sounds/music.ogg").unwrap();
    let explosion_sound = ra.new_sound("assets/sounds/explosion.ogg").unwrap();

    rl.set_target_fps(60);

    let space_ship = SpaceShip::new(&rl, &space_ship_image, &laser_sound);
    let mystery_ship = MysteryShip::new(&mystery);

    let mut game = Game::new(
        space_ship,
        mystery_ship,
        alien_images,
        music,
        &explosion_sound,
    );
    game.music.play_stream();

    while !rl.window_should_close() {
        game.music.update_stream();
        game.handle_input(&rl);
        game.update(&rl);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(gray);
        d.draw_rectangle_rounded_lines(
            Rectangle {
                x: 10.0,
                y: 10.0,
                height: 780.0,
                width: 780.0,
            },
            0.18,
            20,
            2.0,
            yellow,
        );
        d.draw_line_ex(
            Vector2::new(25.0, 730.0),
            Vector2::new(775.0, 730.0),
            3.0,
            yellow,
        );

        if game.run {
            d.draw_text_ex(
                &font,
                "LEVEL 01",
                Vector2::new(570.0, 740.0),
                34.0,
                2.0,
                yellow,
            );
        } else {
            d.draw_text_ex(
                &font,
                "GAME OVER",
                Vector2::new(570.0, 740.0),
                34.0,
                2.0,
                yellow,
            );
        }

        for i in 1..=game.lives {
            d.draw_texture_v(
                &space_ship_image,
                Vector2::new(i as f32 * 50.0, 745.0),
                Color::WHITE,
            );
        }

        d.draw_text_ex(&font, "SCORE", Vector2::new(50.0, 15.0), 34.0, 2.0, yellow);
        let score_text = format!("{:<05}", game.score);
        d.draw_text_ex(
            &font,
            &score_text,
            Vector2::new(50.0, 40.0),
            34.0,
            2.0,
            yellow,
        );

        game.draw(&mut d);
    }
}
