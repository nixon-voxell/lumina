// GAVE UP!

use bevy::app::AppLabel;
use bevy::prelude::*;

fn main() {
    #[derive(Resource, Default)]
    struct Val(pub i32);

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
    struct ExampleApp;

    // Create an app with a certain resource.
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(Val(10));

    // Create a sub-app with the same resource and a single schedule.
    let sub_app = SubApp::new();
    // Add the sub-app to the main app.
    app.insert_sub_app(ExampleApp, sub_app);
    app.add_systems(Update, || info!("hellooo from app"));

    let sub_app = app.get_sub_app_mut(ExampleApp).unwrap();

    sub_app.insert_resource(Val(100));

    // Setup an extract function to copy the resource's value in the main world.
    sub_app.set_extract(|main_world, sub_world| {
        info!("extract");
        sub_world.resource_mut::<Val>().0 = main_world.resource::<Val>().0;
        sub_world.run_schedule(Update);
    });

    // RenderApp

    // Schedule a system that will verify extraction is working.
    sub_app.add_systems(Update, |counter: Res<Val>| {
        info!("in sub app");
        // The value will be copied during extraction, so we should see 10 instead of 100.
        assert_eq!(counter.0, 10);
    });

    sub_app.add_systems(Update, || info!("hellooo from subappp"));

    // Update the application once (using the default runner).
    app.run();
}

#[derive(AppLabel, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SubSimulation(pub Entity);
