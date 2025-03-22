use std::f32::consts::PI;

use async_trait::async_trait;
use macroquad::prelude::*;

use crate::{
    grid::{self, Direction, Grid, Point},
    traits::{Draw, Update},
};

#[derive(Debug, Clone)]
struct SnakeHeadTexture(Texture2D);

#[derive(Debug, Clone)]
struct SnakeNeckTexture(Texture2D);

#[derive(Debug, Clone)]
struct SnakeBodyTexture(Texture2D);

#[derive(Debug, Clone)]
struct SnakeTailTexture(Texture2D);

async fn load_snake_textures() -> (
    SnakeHeadTexture,
    SnakeNeckTexture,
    SnakeBodyTexture,
    SnakeTailTexture,
) {
    let snake_head = load_texture("assets/snake_head.png")
        .await
        .expect("Could not load snake head texture");

    let snake_neck = load_texture("assets/snake_neck.png")
        .await
        .expect("Could not load snake neck texture");

    let snake_body = load_texture("assets/snake_body.png")
        .await
        .expect("Could not load snake body texture");

    let snake_tail = load_texture("assets/snake_tail.png")
        .await
        .expect("Could not load snake tail texture");

    (
        SnakeHeadTexture(snake_head),
        SnakeNeckTexture(snake_neck),
        SnakeBodyTexture(snake_body),
        SnakeTailTexture(snake_tail),
    )
}

#[derive(Debug, Clone, Copy)]
struct SnakeSegment {
    point: Point,
    direction: Direction,
}

impl SnakeSegment {
    pub fn new(point: Point, direction: Direction) -> Self {
        Self { point, direction }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FoodSource {
    None,
    Grow,
    Shrink,
}

#[derive(Debug, Clone)]
pub struct Snake {
    head_texture: SnakeHeadTexture,
    neck_texture: SnakeNeckTexture,
    body_texture: SnakeBodyTexture,
    tail_texture: SnakeTailTexture,
    food_source: FoodSource,
    distance_traveled: f32,
    direction: Direction,
    segments: Vec<SnakeSegment>,
    velocity: u32,
}

impl Snake {
    pub async fn new(grid: &Grid) -> Self {
        let (head, neck, body, tail) = load_snake_textures().await;

        let starting_point = grid.get_point(screen_width() / 2., screen_height() / 2.);

        Snake {
            head_texture: head,
            neck_texture: neck,
            body_texture: body,
            tail_texture: tail,
            direction: Direction::Right,
            distance_traveled: 0.,
            food_source: FoodSource::None,
            segments: vec![
                SnakeSegment {
                    point: starting_point,
                    direction: Direction::Right,
                },
                SnakeSegment {
                    point: grid.get_location(starting_point.x - 1, starting_point.y),
                    direction: Direction::Right,
                },
                SnakeSegment {
                    point: grid.get_location(starting_point.x - 2, starting_point.y),
                    direction: Direction::Right,
                },
                SnakeSegment {
                    point: grid.get_location(starting_point.x - 3, starting_point.y),
                    direction: Direction::Right,
                },
            ],
            velocity: 32,
        }
    }

    fn get_head(&self) -> SnakeSegment {
        self.segments[0]
    }

    fn get_neck(&self) -> SnakeSegment {
        self.segments[1]
    }

    fn get_tail(&self) -> SnakeSegment {
        *self.segments.last().unwrap()
    }

    fn handle_eating(&mut self) {
        match self.food_source {
            FoodSource::None => {
                // Pop once to stay the same size.
                self.segments.pop();
            }
            FoodSource::Grow => {
                self.food_source = FoodSource::None;
            }
            FoodSource::Shrink => {
                // Remove one body segment for motion.
                self.segments.pop();
                if self.segments.len() >= 3 {
                    self.segments.pop();
                }
                self.food_source = FoodSource::None;
            }
        }
    }
}

#[async_trait]
impl Update for Snake {
    async fn update(&mut self, grid: &Grid) {
        let head = self.get_head();
        if is_key_pressed(KeyCode::Up)
            && self.direction != Direction::Down
            && head.direction != Direction::Down
        {
            self.direction = Direction::Up;
        }

        if is_key_pressed(KeyCode::Down)
            && self.direction != Direction::Up
            && head.direction != Direction::Up
        {
            self.direction = Direction::Down;
        }

        if is_key_pressed(KeyCode::Left)
            && self.direction != Direction::Right
            && head.direction != Direction::Right
        {
            self.direction = Direction::Left;
        }

        if is_key_pressed(KeyCode::Right)
            && self.direction != Direction::Left
            && head.direction != Direction::Left
        {
            self.direction = Direction::Right;
        }

        if is_key_pressed(KeyCode::G) && self.food_source == FoodSource::None {
            self.food_source = FoodSource::Grow;
        }

        if is_key_pressed(KeyCode::S) && self.food_source == FoodSource::None {
            self.food_source = FoodSource::Shrink;
        }

        let frame_time = get_frame_time();
        let distance_traveled = frame_time * self.velocity as f32;

        self.distance_traveled += distance_traveled;

        if self.distance_traveled < grid::TILE_SIZE as f32 {
            return;
        }

        self.distance_traveled %= 32.;

        let new_point = grid.advance(self.get_head().point, self.direction);

        let new_head = SnakeSegment::new(new_point, self.direction);

        if self.get_head().direction != new_head.direction {
            self.segments[0].direction = new_head.direction;
        }

        self.handle_eating();
        self.segments.insert(0, new_head);
    }
}

fn texture_rotation(direction: Direction) -> f32 {
    match direction {
        Direction::Up => (3.0 * PI) / 2.0,
        Direction::Down => PI / 2.0,
        Direction::Right => 0.,
        Direction::Left => PI,
    }
}

#[async_trait]
impl Draw for Snake {
    async fn draw(&self) {
        let color = Color::from_rgba(255, 255, 255, 255);

        // Draw the head.
        let head = self.get_head();
        draw_texture_ex(
            &self.head_texture.0,
            head.point.x_actual,
            head.point.y_actual,
            color,
            DrawTextureParams {
                dest_size: Some(Vec2::new(33., 33.)),
                source: None,
                rotation: texture_rotation(head.direction),
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );

        // Draw the neck.
        let neck = self.get_neck();
        draw_texture_ex(
            &self.neck_texture.0,
            neck.point.x_actual,
            neck.point.y_actual,
            color,
            DrawTextureParams {
                dest_size: None,
                source: None,
                rotation: texture_rotation(neck.direction),
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );

        // Draw the body panels, skipping 2 for the head and neck.
        self.segments
            .iter()
            .enumerate()
            .skip(2) // Skip the head and neck.
            .take_while(|(idx, _)| *idx < self.segments.len() - 1) // Don't grab the tail.
            .for_each(|(_, segment)| {
                draw_texture_ex(
                    &self.body_texture.0,
                    segment.point.x_actual,
                    segment.point.y_actual,
                    color,
                    DrawTextureParams {
                        dest_size: None,
                        source: None,
                        rotation: texture_rotation(segment.direction),
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                );
            });

        // Draw the tail
        let tail = self.get_tail();
        draw_texture_ex(
            &self.tail_texture.0,
            tail.point.x_actual,
            tail.point.y_actual,
            color,
            DrawTextureParams {
                dest_size: None,
                source: None,
                rotation: texture_rotation(tail.direction),
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }
}
