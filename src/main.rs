use bevy::prelude::*;
use bevy::app::App;

mod sprite;
mod scene;


pub const HEIGHT: f32 = 600.0;
pub const WIDTH: f32 = 800.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(sprite::SpritePlugin)
        .add_plugin(scene::ScenePlugin)
        .add_plugin(HelloPlugin)
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Technobauble".to_string())));
    commands.spawn((Person, Name("Jim".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_system(greet_people);
    }
}