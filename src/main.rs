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

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

#[macroquad::main("Arcade!")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut gameover = false;
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

        if !gameover {
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            if is_key_down(KeyCode::Right) {
                circle.x += movement;
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= movement;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += movement;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= movement;
            }
        }

        if squares.iter().any(|square| circle.collides_with(square)) {
            gameover = true;
        }

        if gameover {
            let text = "Game Over!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }

        circle.x = circle.x.min(screen_width()-BALL_SIZE).max(BALL_SIZE);
        circle.y = circle.y.min(screen_height()-BALL_SIZE).max(BALL_SIZE);

        draw_circle(circle.x, circle.y, 16.0, circle.color);

        next_frame().await
    }
}
