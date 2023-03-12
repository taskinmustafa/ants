use bevy::{prelude::*};

fn main() {
    App::new()
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
}

pub fn print_ants(ant_query: Query<&Ant>) {
    for ant in ant_query.iter(){
        println!("Name: {},",ant.name);
    }
    
}

#[derive(Component)]
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