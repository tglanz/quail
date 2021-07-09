use bevy::{
    prelude::*,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(WindowDescriptor {
                title: "quail".into(),
                width: 800.0,
                height: 600.0,
                resizable: false,
                cursor_locked: false,
                cursor_visible: true,
                decorations: false,
                vsync: true,
                ..WindowDescriptor::default()
            });
    }
}