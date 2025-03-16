use async_trait::async_trait;

use crate::grid::Grid;

#[async_trait]
pub trait Update {
    async fn update(&mut self, grid: &Grid);
}

#[async_trait]
pub trait Draw {
    async fn draw(&self);
}
