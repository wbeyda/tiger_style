use bevy::prelude::*;
use crate::player::components::{AnimationConfig, Direction, MovementState, Player, Animator, create_anim_hashmap};

pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        commands.spawn(Camera2dBundle::default());

        let texture = asset_server.load("player_01.png");
        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(48, 96),
            56,
            19,
            None,
            None,
            );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_config = AnimationConfig::new(56, 56 + 5, 10);
        let animation_bank = create_anim_hashmap();
    
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_scale(Vec3::splat(2.0))
                    .with_translation(Vec3::new(-50.0, 0.0, 0.0)),
                texture: texture.clone(),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index,
            },
            Animator {
                animation_bank,
                current_animation: "idle_r".to_string(),
                timer: 0.0,
                last_animation: "idle_r".to_string(),
                cooldown: 0.1,
            },
            Player,
            animation_config,
            MovementState::Idle { last_direction: Some(Direction::Right) },
        ));
    }