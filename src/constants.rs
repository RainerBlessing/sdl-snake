#[derive(Copy, Clone)]
pub enum Type {
    Empty,
    Wall,
    Snake,
    Apple,
}

#[derive(Copy, Clone)]
pub struct PlayField {
    pub  field_type: Type,
    pub x: u32,
    pub y: u32,
}

#[derive(Copy, Clone)]
pub enum GameState {
    Play,
    GameOver,
}
