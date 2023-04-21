use bevy::prelude::*;

#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Default, Component)]
pub enum Tile {
    Empty,
    Wall,
    #[default]
    Floor,
    Door,
    Stairs,
}

impl Tile {
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        match self {
            Tile::Empty => " ".to_string(),
            Tile::Wall => "█".to_string().gray(),
            Tile::Floor => "░".to_string().green(),
            Tile::Door => "▒".to_string().white(),
            Tile::Stairs => "▓".to_string().yellow(),
        }
    }
}