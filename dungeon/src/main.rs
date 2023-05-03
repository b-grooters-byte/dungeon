mod dungeon;

use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    camera_setup(commands);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut vert = vec![
        [0.0, 0.5, 0.0],
        [1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, -0.0, 0.0],
        [0.0, -0.5, 0.0],
        [1.0, 0.0, 0.0],
    ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vert);


}