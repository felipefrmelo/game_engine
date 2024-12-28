use game_engine::*;

use crate::{game::GameListener, models::GameEvent};

pub struct GameEngineListener;

impl GameEngineListener {
    pub fn new() -> Self {
        Self
    }
}
impl From<GameEvent> for u32 {
    fn from(key: GameEvent) -> Self {
        match key {
            GameEvent::Up => GLFW_KEY_UP,
            GameEvent::Left => GLFW_KEY_LEFT,
            GameEvent::Down => GLFW_KEY_DOWN,
            GameEvent::Right => GLFW_KEY_RIGHT,
            GameEvent::Space => GLFW_KEY_SPACE,
        }
    }
}

impl GameListener for GameEngineListener {
    fn get_events(&self) -> Vec<GameEvent> {
        let mut events = Vec::new();

        GameEvent::iter().for_each(|event| {
            let key: u32 = event.clone().into();
            on_key_press!(key, {
                events.push(event.clone());
            });
        });

        events
    }
}

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub struct TerminalListener;

impl TerminalListener {
    pub fn new() -> Self {
        enable_raw_mode().expect("Failed to enable raw mode");
        Self
    }
}

impl GameListener for TerminalListener {
    fn get_events(&self) -> Vec<GameEvent> {
        let mut events = Vec::new();
        if event::poll(std::time::Duration::from_millis(10)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('w') => events.push(GameEvent::Up),
                    KeyCode::Char('a') => events.push(GameEvent::Left),
                    KeyCode::Char('s') => events.push(GameEvent::Down),
                    KeyCode::Char('d') => events.push(GameEvent::Right),
                    KeyCode::Char(' ') => events.push(GameEvent::Space),
                    KeyCode::Esc => {
                        disable_raw_mode().expect("Failed to disable raw mode");
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        }
        events
    }
}
