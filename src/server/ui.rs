use bevy::{prelude::*, render::view::RenderLayers};
use bevy_typst::{prelude::*, typst_element::prelude::*};

use crate::{
    protocol::Lobbies,
    ui::{typst_scene, vello_scene, EmptyNodeBundle, UiState, UiTemplate},
};

pub(super) struct ServerUiPlugin;

impl Plugin for ServerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            create_lobby_ui
                .run_if(in_state(UiState::Loaded))
                .run_if(resource_changed::<Lobbies>),
        );
    }
}

#[derive(Component)]
struct LobbyUiMarker;

fn create_lobby_ui(
    mut commands: Commands,
    q_lobby_uis: Query<Entity, With<LobbyUiMarker>>,
    ui_template: Res<UiTemplate>,
    compiler: Res<TypstCompiler>,
    lobbies: Res<Lobbies>,
) {
    println!("create server lobby ui");
    for entity in q_lobby_uis.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let world = compiler.world_meta();
    commands
        .spawn((
            EmptyNodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(60.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                render_layer: RenderLayers::layer(1),
                ..default()
            },
            LobbyUiMarker,
        ))
        .with_children(|parent| {
            for (i, lobby) in lobbies.lobbies.iter().enumerate() {
                let player_count = lobby.players.len();
                parent.spawn((
                    TextBundle::from_section(
                        format!("Lobby #{i}, Player Count: {player_count}"),
                        TextStyle {
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    RenderLayers::layer(1),
                ));
            }
        });
}
