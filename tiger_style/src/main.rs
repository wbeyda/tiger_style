// THIS WORKS!
// use std::time::Duration;
// use bevy::prelude::*;

// const PLAYER_SPEED: f32 = 500.0;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
//         .add_systems(Startup, setup)
//         .add_systems(Update, (trigger_animation, execute_animations))
//         .run();
// }

// fn trigger_animation(
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

// #[derive(Component)]
// struct AnimationConfig {
//     first_sprite_index: usize,
//     last_sprite_index: usize,
//     fps: u8,
//     frame_timer: Timer,
// }

// impl AnimationConfig {
//     fn new(first: usize, last: usize, fps: u8) -> Self {
//         Self {
//             first_sprite_index: first,
//             last_sprite_index: last,
//             fps,
//             frame_timer: Self::timer_from_fps(fps),
//         }
//     }

//     fn timer_from_fps(fps: u8) -> Timer {
//         Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
//     }
// }

// fn execute_animations(
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

// #[derive(Component, PartialEq)]
// enum MovementState {
//     Moving { last_direction: Option<Direction> },
//     Idle { last_direction: Direction },
// }

// #[derive(PartialEq, Copy, Clone)]
// enum Direction {
//     Right,
//     Left,
//     Up,
//     Down,
// }

// #[derive(Component)]
// struct Player;

// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     commands.spawn(Camera2dBundle::default());
//     let texture = asset_server.load("player_07.png");
//     let layout = TextureAtlasLayout::from_grid(UVec2::new(48, 96), 56, 19, None, None);
//     let texture_atlas_layout = texture_atlas_layouts.add(layout);
//     let animation_config = AnimationConfig::new(56, 56 + 5, 10);

//     commands.spawn((
//         SpriteBundle {
//             transform: Transform::from_scale(Vec3::splat(6.0))
//                 .with_translation(Vec3::new(-50.0, 0.0, 0.0)),
//             texture: texture.clone(),
//             ..default()
//         },
//         TextureAtlas {
//             layout: texture_atlas_layout.clone(),
//             index: animation_config.first_sprite_index,
//         },
//         Player,
//         animation_config,
//         MovementState::Idle { last_direction: Direction::Right },
//     ));
// }
































use bevy::prelude::*;
use bevy::DefaultPlugins; // Correct import for DefaultPlugins
mod player;

use player::components::*;
use player::systems::*;
use player::setup::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, (trigger_animation, execute_animations))
        .run();
}