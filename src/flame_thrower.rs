use crate::crab::Crab;

use macroquad::prelude::*;

const FT_SIZE: Vec2 = vec2(75.0, 40.0);
const FT_SPEED: f32 = 300.0;

pub struct FlameThrower {
    pub rect: Rect,
    texture: Texture2D,
    pub can_shoot: bool,
}

impl FlameThrower {
    pub async fn new() -> Self {
        Self {
            // We are using a rect to represent to flame_thrower bounds
            rect: Rect::new(
                0.0,
                screen_height() * 0.5 - FT_SIZE[1] * 2.0,
                FT_SIZE[0],
                FT_SIZE[1],
            ),
            texture: load_texture("res/flame_thrower.png").await.unwrap(),
            can_shoot: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let vel = match (
            is_key_down(KeyCode::Up),
            is_key_down(KeyCode::Down) ,
        ) {
            // If the down arrow is pressed set velocity to -1
            (true, false) => -1.0,

            // If the down arrow is pressed set velocity to 1
            (false, true) => 1.0,
            _ => 0.0,
        };

        self.rect.y += vel * dt * FT_SPEED;


        // Check screen bounds
        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
        }

        if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }
    }

    pub fn draw(&self) {
        // Draw the flame_thrower
        draw_texture(&self.texture, self.rect.x, self.rect.y, WHITE);
    }

    pub fn reset(&mut self) {
        self.can_shoot = true;

        // Reset the rect position
        self.rect.y = screen_height() * 0.5 - FT_SIZE[1] * 2.0;
    }

    pub fn is_collision(&self, crab: &mut Crab) -> bool {
        // If flame hits the crab
        if let Some(_intersection) = crab.rect.intersect(self.rect) {
            return true;
        }

        false
    }
}
