// main.rs
mod entities;
mod game;
mod input;
mod rendering;

use game::Game;
use std::time::{Duration, Instant};

fn main() {
    let mut game = Game::new();
    let target_fps = 10;
    let frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);

    loop {
        let frame_start = Instant::now();

        if !game.update() {
            break;
        }
        game.render();

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
}
