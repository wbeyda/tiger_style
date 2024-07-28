use bevy::prelude::*;
use crate::player::components::{AnimationConfig, Direction, MovementState, Player};

const PLAYER_SPEED: f32 = 500.0;

pub fn trigger_animation(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AnimationConfig, &mut Transform, &mut TextureAtlas, &mut MovementState), With<Player>>,
) {
    let (mut animation, mut transform, mut atlas, mut movement_state) = query.single_mut();

    let mut direction = Vec3::ZERO;
    let mut new_animation_indices = None;
    let mut last_direction = None;

    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, 0.0, 0.0);
        new_animation_indices = Some((112 + 0, 112 + 5));
        last_direction = Some(Direction::Right);
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
        new_animation_indices = Some((112 + 12, 112 + 17));
        last_direction = Some(Direction::Left);
    } else if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        direction += Vec3::new(0.0, 1.0, 0.0);
        new_animation_indices = Some((112 + 6, 112 + 11));
        last_direction = Some(Direction::Up);
    } else if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        direction += Vec3::new(0.0, -1.0, 0.0);
        new_animation_indices = Some((112 + 18, 112 + 23));
        last_direction = Some(Direction::Down);
    } else {
        // Set to idle animation if no movement key is pressed
        let idle_indices = match *movement_state {
            MovementState::Idle { last_direction } => match last_direction {
                Direction::Right => (56, 61),
                Direction::Left => (68, 73),
                Direction::Up => (62, 67),
                Direction::Down => (74, 79),
            },
            MovementState::Moving { last_direction } => match last_direction {
                Some(Direction::Right) => (56, 61),
                Some(Direction::Left) => (68, 73),
                Some(Direction::Up) => (62, 67),
                Some(Direction::Down) => (74, 79),
                None => (56, 61),
            },
        };

        if !matches!(*movement_state, MovementState::Idle { .. }) {
            animation.first_sprite_index = idle_indices.0;
            animation.last_sprite_index = idle_indices.1;
            atlas.index = animation.first_sprite_index;
            animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
            if let MovementState::Moving { last_direction } = *movement_state {
                *movement_state = MovementState::Idle {
                    last_direction: last_direction.unwrap_or(Direction::Right),
                };
            }
        }
    }

    if let Some((first, last)) = new_animation_indices {
        if !matches!(*movement_state, MovementState::Moving {..}) || animation.first_sprite_index != first || animation.last_sprite_index != last {
            animation.first_sprite_index = first;
            animation.last_sprite_index = last;
            atlas.index = first;
            animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
            *movement_state = MovementState::Moving { last_direction: last_direction.unwrap_or(Direction::Right).into() };
        }
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }

    // Handle animation
    animation.frame_timer.tick(time.delta());
    if animation.frame_timer.just_finished() {
        if atlas.index == animation.last_sprite_index {
            atlas.index = animation.first_sprite_index;
        } else {
            atlas.index += 1;
        }
        animation.frame_timer.reset();
    }
}

pub fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut config, mut atlas) in &mut query {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            if atlas.index == config.last_sprite_index {
                atlas.index = config.first_sprite_index;
            } else {
                atlas.index += 1;
            }
        }
    }
}