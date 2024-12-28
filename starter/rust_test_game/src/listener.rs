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
