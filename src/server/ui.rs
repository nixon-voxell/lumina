use bevy::{prelude::*, render::view::RenderLayers};
// use bevy_typst::{prelude::*, typst_element::prelude::*};

use crate::{
    protocol::Lobbies,
    ui::{EmptyNodeBundle, UiState},
};

pub(super) struct ServerUiPlugin;

impl Plugin for ServerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            lobby_ui
                .run_if(in_state(UiState::Loaded))
                .run_if(resource_changed::<Lobbies>),
        );
    }
}

fn lobby_ui(mut commands: Commands, lobbies: Res<Lobbies>, mut ui_entity: Local<Option<Entity>>) {
    if let Some(ui_entity) = *ui_entity {
        if let Some(c) = commands.get_entity(ui_entity) {
            c.despawn_recursive()
        }
    }

    let id = commands
        .spawn(EmptyNodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(60.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..default()
            },
            render_layer: RenderLayers::layer(1),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(EmptyNodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
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
        })
        .id();

    *ui_entity = Some(id);
}
