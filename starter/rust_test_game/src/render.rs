use std::io::{stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
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

pub struct TerminalRenderer;

impl TerminalRenderer {
    pub fn new() -> Self {
        execute!(stdout(), Hide).unwrap();
        Self
    }
}

impl GameRenderer for TerminalRenderer {
    fn render(&self, sprite: models::Sprite) {
        let mut stdout = stdout();

        execute!(stdout, Clear(ClearType::All)).unwrap();

        execute!(stdout, MoveTo(sprite.x as u16, sprite.y as u16)).unwrap();

        let sprite_representation = ["  ██  ", "██████", "  ██  ", " ████ "];

        for line in sprite_representation {
            execute!(
                stdout,
                MoveTo((sprite.x as u16) % 150_u16, (sprite.y as u16) % 50_u16)
            )
            .unwrap();
            execute!(stdout, Print(line)).unwrap();
        }

        execute!(
            stdout,
            MoveTo(0, 0),
            Print(format!(
                "Rendering Sprite at ({}, {}), Size: ({}, {})",
                sprite.x, sprite.y, sprite.width, sprite.height
            ))
        )
        .unwrap();

        stdout.flush().unwrap();
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        // Restore the cursor visibility on exit
        execute!(stdout(), Show).unwrap();
    }
}
