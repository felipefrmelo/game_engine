use std::sync::mpsc;

use reqwest::blocking::Client;
use serde::Deserialize;

use crate::{game::GameApi, models};

pub struct GameEngineApi;

impl GameEngineApi {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
struct SpriteData {
    b: u8,
    g: u8,
    r: u8,
    height: f32,
    width: f32,
    x: f32,
    y: f32,
}

impl From<SpriteData> for models::Sprite {
    fn from(sprite_data: SpriteData) -> Self {
        Self::new(
            sprite_data.x,
            sprite_data.y,
            sprite_data.width,
            sprite_data.height,
            &[sprite_data.r, sprite_data.g, sprite_data.b],
        )
    }
}

impl GameApi for GameEngineApi {
    fn get_sprite(&self, sender: mpsc::Sender<models::Sprite>) {
        let response = Client::new()
            .get("https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler")
            .send()
            .unwrap();
        let sprite_data = response.json::<SpriteData>().unwrap();

        sender.send(sprite_data.into()).unwrap();
    }
}
