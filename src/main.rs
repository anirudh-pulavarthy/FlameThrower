mod exitbutton;
mod flame;
mod crab;
mod score;
mod flame_thrower;
mod constants;

use flame::Flame;
use crab::Crab;
use score::Score;
use flame_thrower::FlameThrower;
use exitbutton::ExitButton;
use constants::{DEFAULT_FLAME_TIME, CRAB_SPAWN_TIME};
use macroquad::prelude::*;

enum UserMode {
    Start,
    GameInProgress,
    GameOver,
}

#[macroquad::main("Mini Flame Thrower")]

async fn main() {

    let mut current_mode = UserMode::Start;
    let global_font = load_ttf_font("res/Amatic-Bold.ttf").await.unwrap();

    let mut flame_thrower = FlameThrower::new().await;
    let mut score = Score::new().await;

    // There will be a lot of flames and crabbies, so they go into a vector
    let mut flames: Vec<Flame> = Vec::new();
    let mut crabbies: Vec<Crab> = Vec::new();

    let mut crab_spawn_timer = DEFAULT_FLAME_TIME;
    let mut timer_flame_cooldown = DEFAULT_FLAME_TIME;

    let mut exit_button = ExitButton::new("Press Esc to Quit".to_string()).await;

    loop {
        clear_background(WHITE);

        if exit_button.is_pressed || is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }

        match current_mode {
            UserMode::Start => {
                if is_key_down(KeyCode::Space) {
                    // Starts game
                    current_mode = UserMode::GameInProgress;
                }

                // Draw start instructions
                draw_text_ex (
                    "Press Space to begin",
                    screen_width() * 0.5 - 140.0,
                    screen_height() * 0.5,
                    TextParams {
                        font: Some(&global_font),
                        font_size: 30,
                        color: BLACK,
                        font_scale: 1.0,
                        font_scale_aspect: 1.0,
                        rotation: 0.0,
                    },
                );
            }

            UserMode::GameInProgress => {
                if timer_flame_cooldown > 0.0 {
                    timer_flame_cooldown -= 1.0;
                }

                if is_key_down(KeyCode::Space) {
                    // Check if the cooldown timer is up and space has been released
                    if timer_flame_cooldown == 0.0 && flame_thrower.can_shoot {

                        flames.push(
                            Flame::new(flame_thrower.rect.point() + flame_thrower.rect.size() * 0.5).await,
                        );

                        // Reset cooldown timer
                        timer_flame_cooldown = DEFAULT_FLAME_TIME;

                        // Stop flame thrower from shooting until space has been released
                        flame_thrower.can_shoot = false;
                    }
                }

                if is_mouse_button_released(MouseButton::Left) || is_key_released(KeyCode::Space) {
                    // FT can now shoot again
                    flame_thrower.can_shoot = true;
                }

                if crab_spawn_timer > 0.0 {
                    crab_spawn_timer -= 0.5;
                } else {
                    // Spawn crab
                    crabbies.push(
                        Crab::new(vec2(
                            screen_width(),
                            rand::gen_range(100.0, screen_height() - 100.0),
                        ))
                        .await,
                    );

                    // Reset crab spawn timer
                    crab_spawn_timer = CRAB_SPAWN_TIME;
                }

                // Draw and update flames
                for flame in &mut flames {
                    flame.draw();
                    flame.update(get_frame_time());
                }

                for crab in &mut crabbies {

                    // Flames that hit crab should increment score
                    for flame in &mut flames {
                        if flame.on_fire(crab) {
                            crab.is_alive = false;

                            if !crab.bounty {
                                score.increment();
                                crab.bounty = true;
                            }
                        }
                    }

                    // Draw and update crabbies
                    crab.draw();
                    crab.update(get_frame_time());

                    // Check if crab is reached the end of the screen or has touched flame thrower
                    if crab.rect.x < 0.0 - flame_thrower.rect.w || flame_thrower.is_collision(crab) {
                        current_mode = UserMode::GameOver;
                    }
                }

                flame_thrower.update(get_frame_time());
            }
            _ => {
                if is_key_down(KeyCode::Space) {
                    current_mode = UserMode::GameInProgress;
                    score.reset();
                }

                // Draw high score
                draw_text_ex(
                    format!("Best Score: {} ", score.high_score).as_str(),
                    screen_width() * 0.5 - 175.0,
                    screen_height() * 0.5 - 180.0,
                    TextParams {
                        font: Some(&global_font),
                        font_size: 30,
                        color: MAROON,
                        font_scale: 1.0,
                        font_scale_aspect: 1.0,
                        rotation: 0.0
                    },
                );

                // Draw score
                draw_text_ex(
                    format!("Current Score: {} ", score.score).as_str(),
                    screen_width() * 0.5 - 175.0,
                    screen_height() * 0.5 - 130.0,
                    TextParams {
                        font: Some(&global_font),
                        font_size: 30,
                        color: BLUE,
                        font_scale: 1.0,
                        font_scale_aspect: 1.0,
                        rotation: 0.0,
                    },
                );

                // Display retry instructions
                draw_text_ex(
                    "Press space to Retry",
                    screen_width() * 0.5 - 175.0,
                    screen_height() * 0.5 + 20.0,
                    TextParams {
                        font: Some(&global_font),
                        font_size: 30,
                        color: BLACK,
                        font_scale: 1.0,
                        font_scale_aspect: 1.0,
                        rotation: 0.0,
                    },
                );

                // Reset game
                flame_thrower.reset();
                flames.clear();
                crabbies.clear();
 
                // Reset timers
                crab_spawn_timer = CRAB_SPAWN_TIME;
                timer_flame_cooldown = DEFAULT_FLAME_TIME;
            }
        }

        flame_thrower.draw();
        score.draw();

        exit_button.draw();
        exit_button.button_clicked();

        next_frame().await
    }
}