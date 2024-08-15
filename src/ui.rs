use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::ui::FocusPolicy;
use bevy_typst::{compiler::world::TypstWorldMeta, prelude::*, typst_element::prelude::*};
use bevy_vello_graphics::{
    bevy_vello::{prelude::*, VelloPlugin},
    prelude::*,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VelloPlugin {
                canvas_render_layers: RenderLayers::layer(1),
                ..default()
            },
            VelloGraphicsPlugin,
            TypstPlugin::default(),
        ))
        .init_state::<UiState>()
        .add_systems(Startup, load_ui_asset)
        .add_systems(Update, load_ui_template.run_if(in_state(UiState::Loading)));
    }
}

fn load_ui_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_assets = UiAsset(asset_server.load("main.typ"));

    commands.insert_resource(ui_assets);
}

fn load_ui_template(
    mut commands: Commands,
    ui_asset: Res<UiAsset>,
    typst_mods: Res<Assets<TypstModAsset>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
) {
    let Some(asset) = typst_mods.get(&ui_asset.0) else {
        return;
    };

    let ui_template = UiTemplate::new(asset.module().scope());
    commands.insert_resource(ui_template);
    next_ui_state.set(UiState::Loaded);
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

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UiState {
    #[default]
    Loading,
    Loaded,
}

pub fn typst_scene(writer: impl DocWriter, world: &TypstWorldMeta) -> TypstScene {
    let document = world.compile_content(writer.pack()).unwrap();
    TypstScene::from_document(&document, Abs::zero()).unwrap()
}

pub fn vello_scene(scene: TypstScene) -> VelloSceneBundle {
    VelloSceneBundle {
        scene: VelloScene::from(scene.scene),
        coordinate_space: CoordinateSpace::ScreenSpace,
        ..default()
    }
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct EmptyNodeBundle {
    /// Describes the logical size of the node
    pub node: Node,
    /// Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
    /// Render layer of the ui
    pub render_layer: RenderLayers,
}

impl EmptyNodeBundle {
    pub fn from_typst(scene: &TypstScene) -> Self {
        Self {
            style: Style {
                width: Val::Px(scene.width),
                height: Val::Px(scene.height),
                ..default()
            },
            render_layer: RenderLayers::layer(1),
            ..default()
        }
    }

    pub fn grow(grow: f32) -> Self {
        Self {
            style: Style {
                flex_grow: grow,
                ..default()
            },
            ..default()
        }
    }
}
