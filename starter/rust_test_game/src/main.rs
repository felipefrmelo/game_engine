use std::{rc::Rc, sync::Arc};

use api::GameEngineApi;
use game::Game;
use game_engine::*;
use listener::GameEngineListener;
use render::GameEngineRenderer;

mod api;
mod game;
mod listener;
mod models;
mod render;
mod threadpool;

fn main() {
    let listener = Rc::new(GameEngineListener::new());
    let api = Arc::new(GameEngineApi::new());
    let renderer = Arc::new(GameEngineRenderer::new());

    let game = Game::new(listener, api, renderer.clone());

    start_window_and_game_loop!(
        "Rust Test Game",
        800,
        600,
        {
            println!("Starting sprite position update test...");
        },
        {
            game.tick();
            game.render();
        },
        {
            println!("Finished sprite position update test...");
        }
    );
}
