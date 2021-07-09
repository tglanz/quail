use bevy::{
    prelude::*,
};

use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<TiledMap> = asset_server.load("maps/demo/map.tmx");

    let map_entity = commands.spawn().id();

    commands.entity(map_entity)
        .insert_bundle(TiledMapBundle {
            tiled_map: handle,
            map: Map::new(0u16, map_entity),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(TilemapPlugin)
            .add_plugin(TiledMapPlugin)
            .add_startup_system(startup.system());
    }
}