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
    Idle { last_direction: Direction },
    Lifting { last_direction: Direction },
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

    // pub fn calculate_frame_range(&self, direction: Direction) -> (usize, usize) {
    pub fn calculate_frame_range(&self) -> (usize, usize) {
    //    let direction_index = &self.order;

        // let direction_index = match direction {
        //     Direction::Right => &self.order,
        //     Direction::Up => 1,
        //     Direction::Left => 2,
        //     Direction::Down => 3,
        // };

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