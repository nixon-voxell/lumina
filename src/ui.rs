use bevy::prelude::*;
use bevy_typst::{prelude::*, typst_element::prelude::*};
use bevy_vello_graphics::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TypstPlugin::default(), VelloGraphicsPlugin))
            .add_systems(Startup, load_ui_asset)
            .add_systems(
                Update,
                load_ui_template.run_if(not(resource_exists::<UiTemplate>)),
            );
    }
}

fn load_ui_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_assets = UiAsset(asset_server.load("main.typ"));

    commands.insert_resource(ui_assets);
}

fn load_ui_template(
    mut commands: Commands,
    ui_asset: Res<UiAsset>,
    typst_docs: Res<Assets<TypstModAsset>>,
) {
    let Some(server) = typst_docs.get(&ui_asset.0) else {
        return;
    };

    let server_template = UiTemplate::new(server.module().scope());
    commands.insert_resource(server_template);
}

#[derive(Resource)]
pub struct UiAsset(Handle<TypstModAsset>);

typst_template! {
    #[derive(Resource)]
    pub struct UiTemplate {
        foundations::Func => (
            parent,
            frame,
            important_frame,
            danger_frame,
        ),
    }
}
