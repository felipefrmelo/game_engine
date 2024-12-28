use game_engine::*;

use crate::{game::GameRenderer, models};

pub struct GameEngineRenderer;

impl GameEngineRenderer {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameRenderer for GameEngineRenderer {
    fn render(&self, sprite: models::Sprite) {
        spawn_sprite!(
            sprite.x,
            sprite.y,
            sprite.width as i32,
            sprite.height as i32,
            sprite.color.0 as i32,
            sprite.color.1 as i32,
            sprite.color.2 as i32
        );
    }
}
