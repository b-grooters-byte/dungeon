use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(add_player)
        .add_system(show_player_position)
        .run();
}

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component)]
struct Position(Vec3);

#[derive(Debug, Component)]
struct Name(String);

fn add_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Name("Bob".to_string()),
        Position(Vec3::new(0.0, 0.0, 0.0)),
    ));
    commands.spawn((
        Player,
        Name("Alice".to_string()),
        Position(Vec3::new(1.0, 0.0, -1.0)),
    ));
    commands.spawn((
        Player,
        Name("Mallory".to_string()),
        Position(Vec3::new(-1.0, 0.0, -1.0)),
    ));
}

fn show_player_position(query: Query<(&Name, &Position), With<Player>>) {
    for (name, position) in query.iter() {
        println!("{} is at {:?}", name.0, position.0);
    }
}
