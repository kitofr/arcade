use std::ops::Deref;

use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
}

#[macroquad::main("Arcade!")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    const MOVEMENT_SPEED: f32 = 400.0;
    const BALL_SIZE: f32 = 16.0;

    let mut squares : Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
    };

    loop {
        clear_background(DARKGRAY);

        let delta_time = get_frame_time();
        let movement = MOVEMENT_SPEED * delta_time;

         if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
                color: vec![GREEN, RED, BLUE, GRAY].choose().unwrap_or(&GREEN).to_owned(),
            });
        }

        squares.retain(|square| square.y < screen_width() + square.size);

        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            );
        }
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }

        if is_key_down(KeyCode::Right) {
            x += movement;
        }
        if is_key_down(KeyCode::Left) {
            x -= movement;
        }
        if is_key_down(KeyCode::Down) {
            y += movement;
        }
        if is_key_down(KeyCode::Up) {
            y -= movement;
        }

        circle.x = x.min(screen_width()-BALL_SIZE).max(BALL_SIZE);
        circle.y = y.min(screen_height()-BALL_SIZE).max(BALL_SIZE);

        draw_circle(circle.x, circle.y, 16.0, circle.color);

        next_frame().await
    }
}
