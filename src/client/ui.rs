use bevy::{color::palettes::css, prelude::*};
use lightyear::prelude::client::*;
// use bevy_typst::{prelude::*, typst_element::prelude::*};

use crate::{
    protocol::{ExitLobby, JoinLobby, Lobbies, ReliableChannel},
    ui::EmptyNodeBundle,
};

use super::{
    lobby::{LobbyState, MyLobbyId},
    Connection, MyClientId,
};

pub(super) struct ClientUiPlugin;

impl Plugin for ClientUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Connection::Disconnected), connect_server_ui)
            .add_systems(OnEnter(Connection::Connect), connecting_server_ui)
            .add_systems(
                Update,
                (
                    join_lobby_ui.run_if(in_state(LobbyState::None)),
                    lobby_ui.run_if(in_state(LobbyState::Joined)),
                ),
            )
            .add_systems(Update, (connect_server_btn, join_lobby_btn, exit_lobby_btn));
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
            parent
                .spawn(EmptyNodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    flex_grow: 0.0,
                                    padding: UiRect::all(Val::Px(10.0)),
                                    margin: UiRect::all(Val::Px(5.0)),
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
                });
        });
}

fn connecting_server_ui(mut commands: Commands) {
    commands
        .spawn((
            EmptyNodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            StateScoped(Connection::Connect),
        ))
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
                    parent.spawn(
                        TextBundle::from("Connecting to server...").with_style(Style {
                            flex_grow: 0.0,
                            justify_self: JustifySelf::Center,
                            ..default()
                        }),
                    );
                });
        });
}

fn join_lobby_ui(
    mut commands: Commands,
    lobbies: Option<Res<Lobbies>>,
    mut ui_entity: Local<Option<Entity>>,
) {
    let Some(lobbies) = lobbies else {
        return;
    };

    if let Some(ui_entity) = *ui_entity {
        // Don't update the ui if the lobby doesn't change.
        // Do this only if there is a ui spawned already.
        if lobbies.is_changed() == false {
            return;
        }

        if let Some(c) = commands.get_entity(ui_entity) {
            c.despawn_recursive()
        }
    }

    let id = commands
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
            parent
                .spawn(EmptyNodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for (i, lobby) in lobbies.lobbies.iter().enumerate() {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        flex_grow: 0.0,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        margin: UiRect::all(Val::Px(5.0)),
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
                });
        })
        .id();

    *ui_entity = Some(id);
}

fn lobby_ui(
    mut commands: Commands,
    lobbies: Option<Res<Lobbies>>,
    my_lobby_id: Res<MyLobbyId>,
    my_client_id: Res<MyClientId>,
    mut ui_entity: Local<Option<Entity>>,
) {
    let Some(lobbies) = lobbies else {
        return;
    };

    if let Some(ui_entity) = *ui_entity {
        // Don't update the ui if the lobby doesn't change.
        // Do this only if there is a ui spawned already.
        if lobbies.is_changed() == false {
            return;
        }

        if let Some(c) = commands.get_entity(ui_entity) {
            c.despawn_recursive()
        }
    }

    let id = commands
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
            StateScoped(LobbyState::Joined),
        ))
        .with_children(|parent| {
            parent
                .spawn(EmptyNodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for (i, player) in lobbies.lobbies[my_lobby_id.0].players.iter().enumerate() {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_grow: 0.0,
                                    padding: UiRect::all(Val::Px(20.0)),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                // Distinguish between my id and other player's ids
                                background_color: match my_client_id.0 == *player {
                                    true => css::LIGHT_BLUE.into(),
                                    false => Color::WHITE.into(),
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(
                                    TextBundle::from_section(
                                        format!("Player #{i}: {player:?}"),
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

                    parent.spawn(EmptyNodeBundle::default().with_height(Val::Px(10.0)));

                    parent
                        .spawn(EmptyNodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            flex_grow: 0.0,
                                            padding: UiRect::all(Val::Px(10.0)),
                                            justify_content: JustifyContent::Center,
                                            justify_self: JustifySelf::Center,
                                            ..default()
                                        },
                                        background_color: css::GREEN.into(),
                                        ..default()
                                    },
                                    StartGameBtn,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "Start Game!",
                                            TextStyle {
                                                color: Color::BLACK,
                                                ..default()
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                justify_self: JustifySelf::Center,
                                                ..default()
                                            },
                                        ),
                                    );
                                });

                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            flex_grow: 0.0,
                                            padding: UiRect::all(Val::Px(10.0)),
                                            justify_content: JustifyContent::Center,
                                            justify_self: JustifySelf::Center,
                                            ..default()
                                        },
                                        background_color: css::RED.into(),
                                        ..default()
                                    },
                                    ExitLobbyBtn(my_lobby_id.0),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "Exit Lobby",
                                            TextStyle {
                                                color: Color::BLACK,
                                                ..default()
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                justify_self: JustifySelf::Center,
                                                ..default()
                                            },
                                        ),
                                    );
                                });
                        });
                });
        })
        .id();

    *ui_entity = Some(id);
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

fn exit_lobby_btn(
    q_interactions: Query<(&Interaction, &ExitLobbyBtn)>,
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
                .send_message::<ReliableChannel, _>(&ExitLobby {
                    lobby_id: lobby_btn.0,
                })
                .unwrap();

            // TODO: Change back to Left and perform the necessary cleanup.
            // next_lobby_state.set(LobbyState::Left);
            next_lobby_state.set(LobbyState::None);
        }
    }
}

#[derive(Component)]
struct ConnectServerBtn;

#[derive(Component)]
struct JoinLobbyBtn(usize);

#[derive(Component)]
struct ExitLobbyBtn(usize);

#[derive(Component)]
struct StartGameBtn;
