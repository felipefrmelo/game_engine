use game_event_macro::IterableEnum;

pub struct Color(pub u8, pub u8, pub u8);

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Sprite {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: &[u8; 3]) -> Self {
        let color = Color(color[0], color[1], color[2]);
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }
}

#[derive(Debug, IterableEnum, Clone)]
pub enum GameEvent {
    Up,
    Left,
    Down,
    Right,
    Space,
}
