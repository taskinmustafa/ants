use bevy::{prelude::*, window::PrimaryWindow, render::view::window};
use rand::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_ant)
    .add_startup_system(spawn_food)
    .add_system(print_ants)
    .add_system(ant_movement)
    .run();
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

#[derive(Component)]
pub struct Food {}

pub const ANT_SPEED: f32 = 500.0;

pub fn ant_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut ant_query: Query<&mut Transform, With<Ant>>,
    time: Res<Time>
){
    if let Ok(mut transform) = ant_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A){
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W){
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D){
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S){
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * ANT_SPEED * time.delta_seconds();

    }
}

pub fn spawn_food(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
){
    let window = window_query.get_single().unwrap();
    for i in 0..4 {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        
        commands.spawn(
            (
                SpriteBundle{
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/bread.png"),
                    ..default()
                },
                Food {}
            )
        );
    }
}