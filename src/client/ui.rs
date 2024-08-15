use bevy::prelude::*;
use lightyear::prelude::client::*;
// use bevy_typst::{prelude::*, typst_element::prelude::*};

use crate::{
    protocol::{JoinLobby, Lobbies, ReliableChannel},
    ui::{EmptyNodeBundle, UiState},
};

use super::{lobby::LobbyState, Connection};

pub(super) struct ClientUiPlugin;

impl Plugin for ClientUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Loaded), connect_server_ui)
            .add_systems(OnEnter(Connection::Connect), connecting_server_ui)
            .add_systems(
                Update,
                join_lobby_ui
                    .run_if(in_state(LobbyState::None))
                    .run_if(resource_exists::<Lobbies>.and_then(resource_changed::<Lobbies>)),
            )
            .add_systems(Update, connect_server_btn)
            .add_systems(Update, join_lobby_btn);
    }
}

fn connect_server_ui(mut commands: Commands) {
    commands
        .spawn((
            EmptyNodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(60.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            StateScoped(Connection::Disconnected),
        ))
        .with_children(|parent| {
            parent.spawn(EmptyNodeBundle::grow(1.0));

            parent
                .spawn(EmptyNodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(EmptyNodeBundle::grow(1.0));

                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    flex_grow: 0.0,
                                    padding: UiRect::all(Val::Px(40.0)),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            ConnectServerBtn,
                        ))
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Connect to server.",
                                    TextStyle {
                                        color: Color::BLACK,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    justify_self: JustifySelf::Center,
                                    ..default()
                                }),
                            );
                        });

                    parent.spawn(EmptyNodeBundle::grow(1.0));
                });

            parent.spawn(EmptyNodeBundle::grow(1.0));
        });
}

fn connecting_server_ui(mut commands: Commands) {
    commands
        .spawn((
            EmptyNodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    justify_items: JustifyItems::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                ..default()
            },
            StateScoped(Connection::Connect),
        ))
        .with_children(|parent| {
            parent.spawn(EmptyNodeBundle::grow(1.0));

            parent.spawn(
                TextBundle::from("Connecting to server...").with_style(Style {
                    flex_grow: 0.0,
                    justify_self: JustifySelf::Center,
                    ..default()
                }),
            );

            parent.spawn(EmptyNodeBundle::grow(1.0));
        });
}

fn join_lobby_ui(mut commands: Commands, lobbies: Res<Lobbies>) {
    commands
        .spawn((
            EmptyNodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(60.0)),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            StateScoped(LobbyState::None),
        ))
        .with_children(|parent| {
            parent.spawn(EmptyNodeBundle::grow(1.0));

            parent
                .spawn(EmptyNodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(EmptyNodeBundle::grow(1.0));

                    for (i, lobby) in lobbies.lobbies.iter().enumerate() {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        flex_grow: 0.0,
                                        padding: UiRect::all(Val::Px(40.0)),
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    background_color: Color::WHITE.into(),
                                    ..default()
                                },
                                JoinLobbyBtn(i),
                            ))
                            .with_children(|parent| {
                                parent.spawn(
                                    TextBundle::from_section(
                                        format!("Join lobby #{i} ({})", lobby.players.len()),
                                        TextStyle {
                                            color: Color::BLACK,
                                            ..default()
                                        },
                                    )
                                    .with_style(Style {
                                        justify_self: JustifySelf::Center,
                                        ..default()
                                    }),
                                );
                            });
                    }

                    parent.spawn(EmptyNodeBundle::grow(1.0));
                });

            parent.spawn(EmptyNodeBundle::grow(1.0));
        });
}

fn connect_server_btn(
    q_interactions: Query<&Interaction, With<ConnectServerBtn>>,
    mut connection: ResMut<NextState<Connection>>,
) {
    for interaction in q_interactions.iter() {
        if interaction == &Interaction::Pressed {
            connection.set(Connection::Connect)
        }
    }
}

fn join_lobby_btn(
    q_interactions: Query<(&Interaction, &JoinLobbyBtn)>,
    mut connection_manager: ResMut<ConnectionManager>,
    lobbies: Option<Res<Lobbies>>,
    mut next_lobby_state: ResMut<NextState<LobbyState>>,
) {
    if lobbies.is_none() {
        return;
    };

    for (interaction, lobby_btn) in q_interactions.iter() {
        if interaction == &Interaction::Pressed {
            connection_manager
                .send_message::<ReliableChannel, _>(&JoinLobby {
                    lobby_id: lobby_btn.0,
                })
                .unwrap();

            next_lobby_state.set(LobbyState::Joined);
        }
    }
}

#[derive(Component)]
struct ConnectServerBtn;

#[derive(Component)]
struct JoinLobbyBtn(usize);
