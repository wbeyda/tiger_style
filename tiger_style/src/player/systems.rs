// use bevy::prelude::*;
// use crate::player::components::{AnimationConfig, Direction, MovementState, Player, AnimationInfo, AnimationResource};

// const PLAYER_SPEED: f32 = 500.0;

// pub fn player_animation(
//     time: Res<Time>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<(&mut AnimationConfig, &mut Transform, &mut TextureAtlas, &mut MovementState), With<Player>>,
// ) {
//     let (mut animation, mut transform, mut atlas, mut movement_state) = query.single_mut();

//     let mut direction = Vec3::ZERO;
//     let mut new_animation_indices = None;
//     let mut last_direction = None;

//     if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
//         direction += Vec3::new(1.0, 0.0, 0.0);
//         new_animation_indices = Some((112 + 0, 112 + 5));
//         last_direction = Some(Direction::Right);
//     } else if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
//         direction += Vec3::new(-1.0, 0.0, 0.0);
//         new_animation_indices = Some((112 + 12, 112 + 17));
//         last_direction = Some(Direction::Left);
//     } else if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
//         direction += Vec3::new(0.0, 1.0, 0.0);
//         new_animation_indices = Some((112 + 6, 112 + 11));
//         last_direction = Some(Direction::Up);
//     } else if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
//         direction += Vec3::new(0.0, -1.0, 0.0);
//         new_animation_indices = Some((112 + 18, 112 + 23));
//         last_direction = Some(Direction::Down);
//     } else {
//         // Set to idle animation if no movement key is pressed
//         let idle_indices = match *movement_state {
//             MovementState::Idle { last_direction } => match last_direction {
//                 Direction::Right => (56, 61),
//                 Direction::Left => (68, 73),
//                 Direction::Up => (62, 67),
//                 Direction::Down => (74, 79),
//             },
//             MovementState::Moving { last_direction } => match last_direction {
//                 Some(Direction::Right) => (56, 61),
//                 Some(Direction::Left) => (68, 73),
//                 Some(Direction::Up) => (62, 67),
//                 Some(Direction::Down) => (74, 79),
//                 None => (56, 61),
//             },
//         };

//         if !matches!(*movement_state, MovementState::Idle { .. }) {
//             animation.first_sprite_index = idle_indices.0;
//             animation.last_sprite_index = idle_indices.1;
//             atlas.index = animation.first_sprite_index;
//             animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
//             if let MovementState::Moving { last_direction } = *movement_state {
//                 *movement_state = MovementState::Idle {
//                     last_direction: last_direction.unwrap_or(Direction::Right),
//                 };
//             }
//         }
//     }

//     if let Some((first, last)) = new_animation_indices {
//         if !matches!(*movement_state, MovementState::Moving {..}) || animation.first_sprite_index != first || animation.last_sprite_index != last {
//             animation.first_sprite_index = first;
//             animation.last_sprite_index = last;
//             atlas.index = first;
//             animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
//             *movement_state = MovementState::Moving { last_direction: last_direction.unwrap_or(Direction::Right).into() };
//         }
//     }

//     if direction.length() > 0.0 {
//         direction = direction.normalize();
//         transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
//     }

//     // Handle animation
//     animation.frame_timer.tick(time.delta());
//     if animation.frame_timer.just_finished() {
//         if atlas.index == animation.last_sprite_index {
//             atlas.index = animation.first_sprite_index;
//         } else {
//             atlas.index += 1;
//         }
//         animation.frame_timer.reset();
//     }
// }

// pub fn execute_animations(
//     time: Res<Time>,
//     mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
// ) {
//     for (mut config, mut atlas) in &mut query {
//         config.frame_timer.tick(time.delta());
//         if config.frame_timer.just_finished() {
//             if atlas.index == config.last_sprite_index {
//                 atlas.index = config.first_sprite_index;
//             } else {
//                 atlas.index += 1;
//             }
//         }
//     }
// }

















//this is working
use bevy::prelude::*;
use crate::player::components::{AnimationConfig, Direction, MovementState, Player, AnimationResource, AnimationInfo};

const PLAYER_SPEED: f32 = 500.0;

pub fn player_animation(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AnimationConfig, &mut Transform, &mut TextureAtlas, &mut MovementState), With<Player>>,
) {
    let (mut animation, mut transform, mut atlas, mut movement_state) = query.single_mut();

    // let ar: AnimationResource = AnimationResource::new();
    let mut direction = Vec3::ZERO;
    let mut new_animation_indices = None;
    // let mut last_direction = Some(Direction::Down);
    let mut last_direction = None;

    let ar: AnimationResource = AnimationResource::new();
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, 0.0, 0.0);
        if let Some(w) = ar.animations.get("walk_r") {
            // new_animation_indices = Some((112 + 0, 112 + 5));
            new_animation_indices = Some(w.calculate_frame_range());
        } else {
            new_animation_indices = None;
        }
        *movement_state = MovementState::Moving { last_direction: Some(Direction::Right) };
        // last_direction = Some(Direction::Right);
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
        if let Some(w) = ar.animations.get("walk_l"){
            // new_animation_indices = Some((112 + 12, 112 + 17));
            new_animation_indices = Some(w.calculate_frame_range());
        } else { 
            new_animation_indices = None;
        }
        *movement_state = MovementState::Moving { last_direction: Some(Direction::Left) };
        // last_direction = Some(Direction::Left);
    } else if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        direction += Vec3::new(0.0, 1.0, 0.0);
        if let Some(w) = ar.animations.get("walk_u") {
            // new_animation_indices = Some((112 + 6, 112 + 11));
            new_animation_indices = Some(w.calculate_frame_range());
        } else {
            new_animation_indices = None;
        }
        *movement_state = MovementState::Moving { last_direction: Some(Direction::Up) };
        // last_direction = Some(Direction::Up);
    } else if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        direction += Vec3::new(0.0, -1.0, 0.0);
        if let Some(w) = ar.animations.get("walk_d"){
            // new_animation_indices = Some((112 + 18, 112 + 23));
            new_animation_indices = Some(w.calculate_frame_range());
        } else {
            new_animation_indices = None; 
        }
        *movement_state = MovementState::Moving { last_direction: Some(Direction::Down) };
        // last_direction = Some(Direction::Down);
    // Trigger lift animation
    } else if keyboard_input.just_pressed(KeyCode::Space) {

        if let Some(last_dir) = match *movement_state {
            MovementState::Idle { last_direction } => Some(last_direction),
            MovementState::Moving { last_direction } => last_direction,
            MovementState::Lifting { last_direction } => Some(last_direction),
        } {
        // direction += Vec3::new(1.0, 0.0, 0.0);
        // print!("last_direction {:#?}", last_direction);
        // if let Some(last_dir) = last_direction {
            let lift_animation_key = match last_dir {
                Direction::Right => "lift_r",
                Direction::Up => "lift_u",
                Direction::Left => "lift_l",
                Direction::Down => "lift_d",
            };
            
            
            println!("Lift animation key: {}", lift_animation_key);
            if let Some(animation_info) = ar.animations.get(lift_animation_key) {
                println!("Found animation_info for key {}: {:?}", lift_animation_key, animation_info);
                // new_animation_indices = Some((168, 180));
                new_animation_indices = Some(animation_info.calculate_frame_range());
                *movement_state = MovementState::Lifting {
                    last_direction: last_dir, // Unwrapped last_dir is used here
                };
            } else {
                println!("No animation info found for key {}", lift_animation_key);
                new_animation_indices = None;
            }
        }
        // return; // Exit early to avoid overriding the lift animation
    } else {
        if matches!(*movement_state, MovementState::Lifting { .. }) {
            // Skip idle logic if lifting
            return;
        }
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
            MovementState::Lifting { last_direction } => match last_direction {
                Direction::Right => (56, 61),
                Direction::Left => (68, 73),
                Direction::Up => (62, 67),
                Direction::Down => (74, 79),
            },
        };

        //The entire block ensures that when no movement input is detected, the player's state transitions to an idle state. 
        // It sets the appropriate animation frames for the idle state and updates the movement_state to Idle, 
        // preserving the last known direction. This makes sure the player character remains facing the correct direction while idle.
        if !matches!(*movement_state, MovementState::Idle { .. }) {
            animation.first_sprite_index = idle_indices.0;
            animation.last_sprite_index = idle_indices.1;
            atlas.index = animation.first_sprite_index;
            animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
            print!("Setting animation indices: ({}, {})", animation.first_sprite_index, animation.last_sprite_index);
            if let MovementState::Moving { last_direction } = *movement_state {
                *movement_state = MovementState::Idle { last_direction: last_direction.unwrap_or(Direction::Right), };
            }
        }
    }
    // This block ensures that when new animation indices are provided (indicating a change in movement direction or type), 
    // the player's animation  is updated accordingly. It checks if the player's movement state and animation frames need 
    // updating and sets the movement_state to Moving with the appropriate direction. This allows the player's character
    //  to start displaying the correct walking animation for the direction they are moving in.
    if let Some((first, last)) = new_animation_indices {
        if !matches!(*movement_state, MovementState::Moving {..}) &&
            !matches!(*movement_state, MovementState::Lifting{..})
            || animation.first_sprite_index != first || animation.last_sprite_index != last {
            print!("first{:#?}", first);
            print!("last{:#?}", last);
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