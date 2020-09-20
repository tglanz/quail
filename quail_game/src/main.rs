extern crate bevy;
use bevy::prelude::*;

struct GameConfig {
    temp: bool
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            temp: false
        }
    }
}

struct GameState {
    config: GameConfig,
    camera_entity: Option<Entity>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            config: Default::default(),
            camera_entity: Default::default(),
        }
    }
}

struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(WindowDescriptor {
                title: "quaile".to_string(),
                width: 800,
                height: 600,
                resizable: false,
                mode: bevy::window::WindowMode::Windowed,
                vsync: true,
                ..Default::default()
            })
            .add_startup_system(setup.system())
            .add_system(bevy::input::system::exit_on_esc_system.system())
            ;
    }   
}

fn setup(
    mut commands: Commands,
) {
    let mut game_state = GameState::default();

    if game_state.config.temp {
        println!("setup");
    }

    game_state.camera_entity = commands
        .spawn(Camera2dComponents::default())
        .current_entity();

    commands
        .spawn(UiCameraComponents::default());

    commands.insert_resource(game_state);
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(GamePlugin)
        .run();
}
