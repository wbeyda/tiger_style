use bevy::prelude::*;
use bevy::DefaultPlugins; // Correct import for DefaultPlugins
mod player;

use player::systems::*;
use player::setup::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, (player_animation, execute_animations))
        .run();
}