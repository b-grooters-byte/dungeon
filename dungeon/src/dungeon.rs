use bevy::prelude::*;

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component)]
struct Position(Vec3);

#[derive(Debug, Component)]
struct Name(String);

#[derive(Debug, Resource)]
struct PositionTimer(Timer);

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PositionTimer(Timer::from_seconds(
            3.0,
            TimerMode::Repeating,
        )))
        .add_startup_system(add_players)
        .add_system(show_player_position);
    }
}

fn add_players(mut commands: Commands) {
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

fn show_player_position(
    time: Res<Time>,
    mut timer: ResMut<PositionTimer>,
    query: Query<(&Name, &Position), With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (name, position) in query.iter() {
            println!("{} is at {:?}", name.0, position.0);
        }
    }
}
