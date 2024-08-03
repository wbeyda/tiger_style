use crate::player::components::{Direction, MovementState, Player, Animator, animate_sprite};
use bevy::{asset::io::memory::Dir, prelude::*};

const PLAYER_SPEED: f32 = 200.0;

pub fn player_animation(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut Animator,
        &mut Transform,
        &mut TextureAtlas,
        &mut MovementState
    ), With<Player>>,
) {
    for (mut animator, mut transform, mut sprite, mut movement_state) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            // animator.last_animation = animator.current_animation.clone();
            animator.current_animation = "walk_r".to_string();
            // print!("pressed D ");
            *movement_state = MovementState::Moving {
                last_direction: Some(Direction::Right),
            };
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            // print!("pressed A");
            // animator.last_animation = animator.current_animation.clone();
            animator.current_animation = "walk_l".to_string();
            *movement_state = MovementState::Moving {
                last_direction: Some(Direction::Left),
            };
        } else if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            // print!("pressed W");
            // animator.last_animation = animator.current_animation.clone();
            animator.current_animation = "walk_u".to_string();
            *movement_state = MovementState::Moving {
                last_direction: Some(Direction::Up),
            };
        } else if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
            // print!("pressed S");
            // animator.last_animation = animator.current_animation.clone();
            animator.current_animation = "walk_d".to_string();
            *movement_state = MovementState::Moving {
                last_direction: Some(Direction::Down),
            };
        } else if keyboard_input.pressed(KeyCode::Space) {
            print!("pressed Space ");
            // animator.last_animation = animator.current_animation.clone();
            if let Some(last_direction) = match *movement_state {
                MovementState::Idle { last_direction }
                | MovementState::Moving { last_direction }
                | MovementState::Lifting { last_direction } => last_direction,
            } {
                let lift_animation_key = match last_direction {
                    Direction::Right => "lift_r",
                    Direction::Up => "lift_u",
                    Direction::Left => "lift_l",
                    Direction::Down => "lift_d",
                };

                animator.current_animation = lift_animation_key.to_string();
                *movement_state = MovementState::Lifting {
                    last_direction: Some(last_direction), // Unwrapped last_dir is used here
                };
            } else {
                animator.current_animation = "lift_r".to_string();
                *movement_state = MovementState::Lifting { 
                    last_direction: Some(Direction::Right),
                }
            }
        } else {
            // Default to idle if no input
            animator.last_animation = animator.current_animation.clone();
            let idle_animation_key = match *movement_state {
                MovementState::Moving { last_direction } => match last_direction {
                    Some(Direction::Right) => "idle_r".to_string(),
                    Some(Direction::Left) => "idle_l".to_string(),
                    Some(Direction::Up) => "idle_u".to_string(),
                    Some(Direction::Down) => "idle_d".to_string(),
                    _ => "idle_r".to_string(), // Default to "idle_r" if no last direction
                },
                MovementState::Idle { last_direction } => match last_direction {
                    Some(Direction::Right) => "idle_r".to_string(),
                    Some(Direction::Left) => "idle_l".to_string(),
                    Some(Direction::Up) => "idle_u".to_string(),
                    Some(Direction::Down) => "idle_d".to_string(),
                    _ => "idle_r".to_string(), // Default to "idle_r" if no last direction
                },
                MovementState::Lifting { last_direction } => match last_direction {
                    Some(Direction::Right) => "idle_r".to_string(),
                    Some(Direction::Left) => "idle_l".to_string(),
                    Some(Direction::Up) => "idle_u".to_string(),
                    Some(Direction::Down) => "idle_d".to_string(),
                    _ => "idle_r".to_string(), // Default to "idle_r" if no last direction
                },
            };

            animator.current_animation = idle_animation_key;
        }

        // Handle movement
        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        }

        // Handle animation
        animate_sprite(&time, &mut animator, &mut sprite);
    }
}