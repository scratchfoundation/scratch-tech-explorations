use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(add_default_sprites)
        .add_system(hello_world)
        .add_system(print_sprite_names)
        .run();
}

fn hello_world() {
    println!("hello, world!");
}

#[derive(Component)]
struct Sprite;

#[derive(Component)]
struct Name(String);

fn add_default_sprites(mut commands: Commands) {
    commands.spawn().insert(Sprite).insert(Name("Sprite 1".to_string()));
    commands.spawn().insert(Sprite).insert(Name("Sprite 2".to_string()));
}

fn print_sprite_names(query: Query<&Name, With<Sprite>>) {
    for name in query.iter() {
        println!("found a sprite named {}", name.0);
    }
}
