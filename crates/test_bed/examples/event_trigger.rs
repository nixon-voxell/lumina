use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<EventFoo>()
        .add_systems(Update, (send_event, receive_event))
        .run();
}

#[derive(Event, Debug)]
struct EventFoo;

fn send_event(mut commands: Commands) {
    commands.trigger(EventFoo);
}

fn receive_event(mut evr_foo: EventReader<EventFoo>) {
    for foo in evr_foo.read() {
        println!("{:?}", foo);
    }
}
