mod dungeon;

use bevy::prelude::*;

fn main() {
    App::new()
    .add_startup_system(camera_setup)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            
            primary_window: Some(Window{
                title: "Dungeon".to_string(), 
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(dungeon::DungeonPlugin)
        .run();
}


fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}