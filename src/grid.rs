pub const TILE_SIZE: isize = 32;

/// A point on the grid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub x_actual: f32,
    pub y_actual: f32,
}

impl Point {
    pub fn new(x: isize, y: isize, x_actual: f32, y_actual: f32) -> Self {
        Self {
            x,
            y,
            x_actual,
            y_actual,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Grid {
    rows: isize,
    columns: isize,
}

impl Grid {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        let rows = window_height as isize / TILE_SIZE;
        let columns = window_width as isize / TILE_SIZE;
        Grid { rows, columns }
    }

    pub fn get_point(&self, x_position: f32, y_position: f32) -> Point {
        let y = y_position as isize / TILE_SIZE;
        let x = x_position as isize / TILE_SIZE;

        let x_actual: f32 = (x * TILE_SIZE) as f32;
        let y_actual: f32 = (y * TILE_SIZE) as f32;

        Point::new(x, y, x_actual, y_actual)
    }

    pub fn advance(&self, point: Point, direction: Direction) -> Point {
        match direction {
            Direction::Up => Point {
                x: point.x,
                y: point.y - 1,
                x_actual: point.x_actual,
                y_actual: point.y_actual - TILE_SIZE as f32,
            },
            Direction::Down => Point {
                x: point.x,
                y: point.y + 1,
                x_actual: point.x_actual,
                y_actual: point.y_actual + TILE_SIZE as f32,
            },
            Direction::Left => Point {
                x: point.x - 1,
                y: point.y,
                x_actual: point.x_actual - TILE_SIZE as f32,
                y_actual: point.y_actual,
            },
            Direction::Right => Point {
                x: point.x + 1,
                y: point.y,
                x_actual: point.x_actual + TILE_SIZE as f32,
                y_actual: point.y_actual,
            },
        }
    }

    pub fn get_location(&self, x: isize, y: isize) -> Point {
        self.get_point((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32)
    }

    pub fn rows(&self) -> isize {
        self.rows
    }
    pub fn columns(&self) -> isize {
        self.columns
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
