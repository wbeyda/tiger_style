use bevy::prelude::*;
use bevy::DefaultPlugins; // Correct import for DefaultPlugins
mod player;

use player::systems::*;
use player::setup::*;



fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        // .add_systems(Update, (player_animation, execute_animations))
        .add_systems(Update, (player_animation ))
        .run();
}