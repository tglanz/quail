extern crate bevy;
extern crate log;
extern crate log4rs;

use bevy::prelude::*;

struct Grid {
    width: f32,  // relative to screen [0, 1]
    height: f32, // relative to screen [0, 1]
    rows: usize,
    cols: usize,
    gap: f32,
}

struct TilePosition {
    row: usize,
    col: usize,
}

impl TilePosition {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

enum TileType {
    Wall,
    Floor
}

impl Default for Grid{
    fn default() -> Self {
        Self {
            width: 0.5,
            height: 0.5,
            rows: 20,
            cols: 20,
            gap: 1.0,
        }
    }
}

struct GameState {
    window_resized_event_reader: EventReader<bevy::window::WindowResized>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            window_resized_event_reader: Default::default(),
        }
    }
}

struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(WindowDescriptor {
                title: "quail".to_string(),
                width: 800,
                height: 600,
                resizable: false,
                mode: bevy::window::WindowMode::Windowed,
                vsync: true,
                ..Default::default()
            })
            .add_resource(GameState::default())
            .add_resource(Grid::default())
            .add_startup_system(setup_ui.system())
            .add_startup_system(setup_tiles.system())
            .add_startup_system(setup_debug_points.system())
            .add_system(handle_window_resize.system())
            .add_system(bevy::input::system::exit_on_esc_system.system());
    }   
}

fn setup_ui(
    mut commands: Commands,
) {
    log::debug!("setup_ui");
    commands.spawn(Camera2dComponents::default());
}

fn handle_window_resize(
    mut game_state: ResMut<GameState>,
    grid: Res<Grid>,
    events: Res<Events<bevy::window::WindowResized>>,
    mut tiles_query: Query<(&TilePosition, &mut Transform, &mut Sprite)>
) {
    let gap = grid.gap;

    for event in game_state.window_resized_event_reader.iter(&events) {
        log::debug!("window resized; width={:?}, height={:?}", event.width, event.height);

        let grid_rows = grid.rows as f32;
        let grid_cols = grid.cols as f32;

        let window_width = event.width as f32;
        let window_height = event.height as f32;

        let grid_width = (grid.width * window_width) - (gap * grid_cols);
        let grid_height = grid.height * window_height - (gap * grid_rows);
 
        let tile_width = grid_width / grid_cols;
        let tile_height = grid_height / grid_rows;

        let offset_x = (-grid_cols * gap / 2.0) + (-grid_width / 2.0) + (tile_width / 2.0);
        let offset_y = (-grid_rows * gap / 2.0) + (-grid_height / 2.0) + (tile_height / 2.0);

        for (tile_position, mut transform, mut sprite) in &mut tiles_query.iter() {
            let tile_x = offset_x +
                (tile_position.col as f32) * (tile_width + gap);

            let tile_y = offset_y +
                (tile_position.row as f32) * (tile_height + gap);

            transform.set_translation(Vec3::new(tile_x, tile_y, 100.0));
            sprite.size = Vec2::new(tile_width, tile_height);
        }
    }
}

fn setup_debug_points(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window_width = 300.0;
    let window_height = 300.0;

    let material = materials.add(Color::rgb(1.0, 0., 0.).into());
    let size = Vec2::new(9.0, 9.0);

    let xs = [-window_width, -window_width / 2.0, 0.0, window_width / 2.0, window_width];
    let ys = [-window_height, -window_height / 2.0, 0.0, window_height / 2.0, window_height];

    for &x in xs.iter() {
        for &y in ys.iter() {
            commands
                .spawn(SpriteComponents {
                    material: material,
                    transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
                    sprite: Sprite::new(size),
                    ..Default::default()
                });
        }
    }
}

fn determine_tile_type(grid: &Grid, tile_position: &TilePosition) -> TileType {
    let row = tile_position.row;
    let col = tile_position.col;

    if row == 0 || col == 0 || row == grid.rows - 1 || col == grid.cols - 1 {
        TileType::Wall
    } else {
        TileType::Floor
    }
}

fn setup_tiles(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid: Res<Grid>,
) {
    let floor_material = materials.add(Color::WHITE.into());
    let wall_material = materials.add(Color::BLACK.into());

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let tile_position = TilePosition::new(row, col);
            let tile_type = determine_tile_type(&grid, &tile_position);
            let material = match tile_type {
                TileType::Floor => floor_material,
                TileType::Wall => wall_material,
            };

            commands
                .spawn(SpriteComponents {
                    material: material,
                    transform: Transform::default(),
                    sprite: Sprite::default(),
                    ..Default::default()
                })
                .with(tile_position)
                .with(tile_type);
        }
    }
}

fn main() {

    if let Err(error) = log4rs::init_file("assets/log4rs.yaml", Default::default()) {
        println!("failed to load log configuration: {}", error);
        return;
    }

    App::build()
        .add_default_plugins()
        .add_plugin(GamePlugin)
        .run();
}
