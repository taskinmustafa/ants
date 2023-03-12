use bevy::{prelude::*};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
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