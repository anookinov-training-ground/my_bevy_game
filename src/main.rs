use bevy::prelude::*;

pub struct HelloPlugin;

struct GreetTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // add things to your app here
        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            // .add_system(hello_world.system())
            .add_system(greet_people.system());
    }
}

struct Person;

struct Name(String);

fn main() {
    App::build()
        // .add_plugin(CorePlugin::default())
        // .add_plugin(InputPlugin::default())
        // .add_plugin(WindowPlugin::default())
        // /* more plugins omitted for brevity */
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

// fn hello_world() {
//     println!("hello world!");
// }

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}
