use bevy::prelude::*;

use crate::game_plugin::GamePlugin;
use crate::camera_plugin::CameraPlugin;

pub fn run() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(GamePlugin)
        .run();    
}