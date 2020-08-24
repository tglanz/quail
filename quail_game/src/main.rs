extern crate bevy;
use bevy::prelude::*;

struct DirtyTile;
struct DirtyGridPosition;

#[derive(Debug)]
enum Tile {
    Ground,
    Wall,
}

struct GridPosition {
    row: u8,
    col: u8,
}

struct GameConfig {
    rows: u8,
    cols: u8,
    tile_size: (f32, f32),
    tile_offset: (f32, f32),
    tile_gap: (f32, f32),
    tile_ground_color: (f32, f32, f32),
    tile_wall_color: (f32, f32, f32),
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            rows: 40,
            cols: 80,
            tile_size: (20.0, 20.0),
            tile_offset: (0.0, 0.0),
            tile_gap: (1.0, 1.0),
            tile_ground_color: (1.0, 1.0, 1.0),
            tile_wall_color: (0.0, 0.0, 0.0),
        }
    }
}

struct GameTimer(Timer);

struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(GameTimer(Timer::from_seconds(0.5, true)))
            .add_resource(GameConfig::default())
            .add_resource(WindowDescriptor {
                title: "quaile".to_string(),
                width: 800,
                height: 600,
                resizable: false,
                mode: bevy::window::WindowMode::Windowed,
                vsync: true,
                ..Default::default()
            })
            .add_startup_system(startup_tiles.system())
            .add_startup_system(startup_ui.system())
            .add_system(set_tile_translation.system())
            .add_system(set_tile.system())
            ;
    }   
}

fn startup_ui(mut commands: Commands) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());

}

fn set_tile(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &Tile, &DirtyTile, &mut Handle<ColorMaterial>)>
) {
    for (entity, tile, _, mut material) in &mut query.iter() {

        let color = match tile {
            Tile::Wall => config.tile_wall_color,
            Tile::Ground => config.tile_ground_color,
        };

        *material = materials.add(Color::rgb(
            color.0, color.1, color.2).into());
            
        commands.remove_one::<DirtyTile>(entity);
    }
}

fn set_tile_translation(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut query: Query<(Entity, &GridPosition, &DirtyGridPosition, &mut Translation)>,
) {
    let offset = Vec2::from(config.tile_offset);
    let size = Vec2::from(config.tile_size);
    let gap = Vec2::from(config.tile_gap);
    
    for (entity, grid_position, _, mut translation) in &mut query.iter() {
        let row = grid_position.row as f32;
        let col = grid_position.col as f32;
    
        *translation = Translation(Vec3::new(
            offset.x() + (col - config.cols as f32 / 2.0) * (size.x() + gap.x()),
            offset.y() + (row- config.rows as f32 / 2.0) * (size.y() + gap.y()),
            0.0
        ));

        commands.remove_one::<DirtyGridPosition>(entity);
    }

}

fn startup_tiles(
    mut commands: Commands,
    config: Res<GameConfig>,
) {
    for row in 0..config.rows {
        for col in 0..config.cols {
            let tile = if col == 0 || col == config.cols - 1 || row == 0 || row == config.rows - 1 {
                Tile::Wall
            } else {
                Tile::Ground
            };

            let sprite_components = SpriteComponents {
                sprite: Sprite {
                    size: Vec2::from(config.tile_size)
                },
                ..Default::default()
            };

            commands
                .spawn(sprite_components)
                .with(tile)
                .with(GridPosition {col, row})
                .with(DirtyTile)
                .with(DirtyGridPosition);
        }
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(SnakePlugin)
        .run();
}
