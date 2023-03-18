use bevy::{prelude::*, window::PrimaryWindow, render::view::window};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_ant)
    .add_system(print_ants)
    .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Ant{
        alive: true,
        name: "Antoinette".to_string(),
        age: 0,
        health: 100,
        role: Role::Queen,
        hunger: 100,
        strength: 100,
    });
    commands.spawn(Ant{
        alive: true,
        name: "Pippin".to_string(),
        age: 0,
        health: 100,
        role: Role::Male,
        hunger: 100,
        strength: 100,
    });
    commands.spawn(Ant{
        alive: true,
        name: "Ziggy".to_string(),
        age: 0,
        health: 100,
        role: Role::Worker,
        hunger: 100,
        strength: 100,
    });
    commands.spawn(Ant{
        alive: true,
        name: "Biscuit".to_string(),
        age: 0,
        health: 100,
        role: Role::Worker,
        hunger: 100,
        strength: 100,
    });
}

pub fn print_ants(ant_query: Query<&Ant>) {
    for ant in ant_query.iter(){
        println!("Name: {}\tRole: {:?}",ant.name,ant.role);
    }
    
}

pub fn spawn_ant(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        (
            SpriteBundle{
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/ant250x250.png"),
                ..default()
            },
            Ant{
                alive: true,
                name: "Antoinette".to_string(),
                age: 0,
                health: 100,
                role: Role::Queen,
                strength: 100,
                hunger: 100,
            },

        )
    );
    
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );

}

#[derive(Component,Debug)]
pub enum Role {
    Worker,
    Queen,
    Male,
}

#[derive(Component)]
pub struct Ant{
    pub alive: bool,
    pub name: String,
    pub age: u16,
    pub health: i16,
    pub role: Role,
    pub hunger: i16,
    pub strength: i16,
}