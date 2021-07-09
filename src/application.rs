use bevy::prelude::*;

use crate::game_plugin::GamePlugin;
use crate::camera_plugin::CameraPlugin;
use crate::map_plugin::MapPlugin;

pub fn run() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MapPlugin)
        .run();    
}