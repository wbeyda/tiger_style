use std::{collections::HashMap, time::Duration};
use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
    }
}

#[derive(Component, PartialEq)]
pub enum MovementState {
    Moving { last_direction: Option<Direction> },
    Idle { last_direction: Option<Direction>},
    Lifting { last_direction: Option<Direction>},
}

#[derive(PartialEq, Copy, Clone, Component, Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug)]
pub struct AnimationInfo {
    pub start: usize,
    pub end: usize,
    pub frame_count: usize,
    pub order: usize,
}

impl AnimationInfo {
    pub fn new(start: usize, end: usize, frame_count: usize, order: usize) -> Self {
        Self {
            start,
            end,
            frame_count,
            order,
        }
    }

    pub fn calculate_frame_range(&self) -> (usize, usize) {
        let frames_per_direction = (self.end - self.start + 1) / 4;
        let start_frame = self.start + frames_per_direction * self.order;
        let end_frame = start_frame + frames_per_direction - 1;
        (start_frame, end_frame)
    }
}

#[derive(Resource)]
pub struct AnimationResource {
    pub animations: HashMap<String, AnimationInfo>,
}

impl AnimationResource {
    pub fn new() -> Self {
        let animations = HashMap::from([
            ("idle".to_string(), AnimationInfo::new(56, 79, 4, 0)),
            ("walk_r".to_string(), AnimationInfo::new(112, 135, 6, 0)),
            ("walk_u".to_string(), AnimationInfo::new(112, 135, 6, 1)),
            ("walk_l".to_string(), AnimationInfo::new(112, 135, 6, 2)),
            ("walk_d".to_string(), AnimationInfo::new(112, 135, 6, 3)),
            // ("lift_r".to_string(), AnimationInfo::new(616, 663, 12, 0)),
            ("lift_r".to_string(), AnimationInfo::new(168, 204, 12, 0)),
        ]);
        AnimationResource { animations }
    }
}


pub fn animate_sprite(
    time: &Res<Time>,
    animator: &mut Animator,
    sprite: &mut TextureAtlas,
) {
    // print!("current frame: {} \r", sprite.index);
    let anim = animator.animation_bank.get(animator.current_animation.as_str())
        .expect("Animation not found in the bank");

    print!("cur anim: {} \n", animator.current_animation.as_str());
    if animator.last_animation != animator.current_animation {
        sprite.index = anim.start; // Start at the first frame of the animation
        animator.timer = anim.cooldown; // Reset the timer
    }
    animator.timer -= time.delta().as_secs_f32();
    if animator.timer <= 0. {
        // Timer has elapsed, update the frame index
        animator.timer = anim.cooldown; // Reset timer for the next frame
        if anim.looping {
            // Looping animation
            sprite.index += 1;
            if sprite.index > anim.end {
                sprite.index = anim.start; // Loop back to the start
            }
        } else {
            // Non-looping animation
            sprite.index += 1;
            if sprite.index > anim.end {
                sprite.index = anim.end; // Clamp to the last frame
            }
        }
    }
    animator.last_animation = animator.current_animation.clone();
}




#[derive(Clone, Copy)] pub struct Animation {
    pub cooldown: f32,
    pub start: usize,
    pub end: usize,
    pub looping: bool,
}

#[derive(Clone, Component)]
pub struct Animator {
    pub animation_bank: HashMap<String, Animation>,
    pub current_animation: String,
    pub last_animation: String,
    pub timer: f32,
    pub cooldown: f32,
}

impl Default for Animator {
    fn default() -> Self {
        Animator {
            animation_bank: create_anim_hashmap(),
            timer: 0.,
            cooldown: 0.1,
            last_animation: " ".to_string(),
            current_animation: "Idle".to_string(),
        }
    }
}

pub fn create_anim_hashmap() -> HashMap<String, Animation> {
    let mut hash_map = HashMap::new();
    hash_map.insert( "idle_r".to_string(), Animation { start: 56, end: 61, looping: true, cooldown: 0.1, },);
    hash_map.insert( "idle_l".to_string(), Animation { start: 68, end: 73, looping: true, cooldown: 0.1, },);
    hash_map.insert( "idle_u".to_string(), Animation { start: 62, end: 67, looping: true, cooldown: 0.1, },);
    hash_map.insert( "idle_d".to_string(), Animation { start: 74, end: 79, looping: true, cooldown: 0.1, },);
    hash_map.insert( "walk_r".to_string(), Animation { start: 112, end: 117, looping: true, cooldown: 0.1, },);
    hash_map.insert( "walk_l".to_string(), Animation { start: 124, end: 129, looping: true, cooldown: 0.1, },);
    hash_map.insert( "walk_u".to_string(), Animation { start: 118, end: 123, looping: true, cooldown: 0.1, },);
    hash_map.insert( "walk_d".to_string(), Animation { start: 130, end: 135, looping: true, cooldown: 0.1, },);
    hash_map.insert( "lift_r".to_string(), Animation { start: 616, end: 629, looping: false, cooldown: 0.1, },);
    hash_map.insert( "lift_u".to_string(), Animation { start: 630, end: 643, looping: false, cooldown: 0.1, },);
    hash_map.insert( "lift_l".to_string(), Animation { start: 644, end: 657, looping: false, cooldown: 0.1, },);
    hash_map.insert( "lift_d".to_string(), Animation { start: 658, end: 671, looping: false, cooldown: 0.1, },);
    // Add other animations as needed
    hash_map
}