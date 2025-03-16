pub mod grid;
pub mod snake;
pub mod textures;
pub mod traits;

use grid::Grid;
use macroquad::{miniquad::conf::Platform, prelude::*};
use snake::Snake;
use traits::{Draw, Update};

fn window_config() -> Conf {
    Conf {
        window_title: "Snake".into(),
        window_width: 1600,
        window_height: 1000,
        window_resizable: false,
        fullscreen: true,
        icon: None,
        high_dpi: true,
        sample_count: 1,
        platform: Platform::default(),
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut paused = true;
    let grid = Grid::new(screen_width(), screen_height());
    let mut snake = Snake::new(&grid).await;
    loop {
        clear_background(Color::from_rgba(74, 38, 1, 255));
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if !paused {
            snake.update(&grid).await;
        }
        snake.draw().await;
        next_frame().await;
    }
}
