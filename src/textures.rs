use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct WallTexture(pub Texture2D);

pub async fn load_wall_texture() -> WallTexture {
    let wall_texture = load_texture("assets/bricks.png")
        .await
        .expect("Could not load wall bricks texture");

    WallTexture(wall_texture)
}

#[derive(Debug, Clone)]
pub struct GrowthFoodTexture(pub Texture2D);

#[derive(Debug, Clone)]
pub struct ShrinkFoodTexture(pub Texture2D);

pub async fn load_food_textures() -> (GrowthFoodTexture, ShrinkFoodTexture) {
    let growth_food = load_texture("assets/growth_food.png")
        .await
        .expect("Could not load growth food texture");

    let shrink_food = load_texture("assets/shrink_food.png")
        .await
        .expect("Could not load shrink food texture");

    (
        GrowthFoodTexture(growth_food),
        ShrinkFoodTexture(shrink_food),
    )
}
