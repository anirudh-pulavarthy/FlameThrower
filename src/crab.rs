use macroquad::prelude::*;

const CRAB_SIZE: f32 = 50.0;
const CRAB_SPEED: f32 = 500.0;

pub struct Crab {
    pub rect: Rect,
    texture: Texture2D,
    pub is_alive: bool,
    pub bounty: bool,
}

impl Crab {
    pub async fn new(pos: Vec2) -> Self {
        Self {
            // We will use a rect to represent the crab bounds
            rect: Rect::new(pos.x, pos.y, CRAB_SIZE, CRAB_SIZE),
            texture: load_texture("res/crab.png").await.unwrap(),
            is_alive: true,
            bounty: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_alive {
            // Update the rect position
            self.rect.x -= dt * CRAB_SPEED;
        }
    }

    pub fn draw(&self) {
        if self.is_alive {
            // Draw the crab
            draw_texture(&self.texture, self.rect.x, self.rect.y, WHITE);
        }
    }
}
