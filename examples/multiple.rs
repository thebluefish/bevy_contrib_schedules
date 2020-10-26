use bevy::{prelude::*, app::ScheduleRunnerPlugin, core::CorePlugin, type_registry::TypeRegistryPlugin};
use bevy_contrib_schedules::*;

fn main() {
    if let Err(e) = simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Error).init() {
        println!("Failed to setup logger!\n{}", e);
    }

    App::build()
        .add_resource(Time::default())
        .add_plugin(TypeRegistryPlugin::default())
        .add_plugin(CorePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_startup_system(build.system())
        .add_system(schedule_runner_system.thread_local_system())
        .run();
}

struct Foo;
struct Bar;

fn build(mut commands: Commands) {
    // TODO: Demonstrate how to later remove schedules conditionally
    // Spoiler: Just `.despawn` the node when you're done!
    commands
        // Always ticks
        .spawn((
            Foo, ScheduleRunner::default()
                .add_system(foo_sys.system())
        ))
        // Ticks 10 times per second
        .spawn((
            Bar, ScheduleRunner::from_rate_inv(10.0)
                .add_system(bar_sys.system())
        ))
    ;
}

fn foo_sys() {
    println!("foo");
}

fn bar_sys() {
    println!("bar");
}