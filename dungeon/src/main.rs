mod dungeon;

use bevy::prelude::*;

fn main() {
    App::new()
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
