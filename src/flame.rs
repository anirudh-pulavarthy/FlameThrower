
use crate::constants::{FLAME_SIZE, FLAME_SPEED};
use crate::crab::Crab;
use macroquad::prelude::*;


pub struct Flame {
    pub rect: Rect,
    texture: Texture2D,
}

impl Flame {
    pub async fn new(pos: Vec2) -> Self {
        Self {
            // Rect to represent the flame bounds
            rect: Rect::new(pos.x, pos.y, FLAME_SIZE, FLAME_SIZE),
            texture: load_texture("res/flame.png").await.unwrap(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update the rect position
        self.rect.x += dt * FLAME_SPEED
    }

    pub fn draw(&self) {
        // Draw the flame
        draw_texture(&self.texture, self.rect.x, self.rect.y, WHITE);
    }

    pub fn on_fire(&self, crab: &mut Crab) -> bool {
        // If crab catches fire
        if let Some(_intersection) = crab.rect.intersect(self.rect) {
            return true;
        }

        false
    }
}
