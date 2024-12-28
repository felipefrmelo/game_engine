use std::{rc::Rc, sync::Arc, thread, time::Duration};

use api::GameEngineApi;
use game::Game;
use game_engine::*;
use listener::{GameEngineListener, TerminalListener};
use render::{GameEngineRenderer, TerminalRenderer};

mod api;
mod game;
mod listener;
mod models;
mod render;
mod threadpool;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--term" {
        terminal_run();
    } else {
        game_engine_run();
    }
}

fn game_engine_run() {
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

fn terminal_run() {
    let listener = Rc::new(TerminalListener::new());
    let api = Arc::new(GameEngineApi);
    let renderer = Arc::new(TerminalRenderer::new());

    let game = Game::new(listener, api, renderer);

    loop {
        game.tick();
        game.render();
        thread::sleep(Duration::from_millis(100));
    }
}
