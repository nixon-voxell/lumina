use bevy::prelude::*;
use bevy_typst::{prelude::*, typst_element::prelude::*};
use lightyear::prelude::client::*;

use crate::{
    protocol::{JoinLobby, Lobbies, ReliableChannel},
    ui::{typst_scene, vello_scene, EmptyNodeBundle, UiState, UiTemplate},
};

use super::lobby::LobbyState;

pub(super) struct ClientUiPlugin;

impl Plugin for ClientUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Loaded), create_lobby_ui)
            .add_systems(Update, enter_lobby);
    }
}

fn create_lobby_ui(
    mut commands: Commands,
    ui_template: Res<UiTemplate>,
    compiler: Res<TypstCompiler>,
) {
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
                ..default()
            },
            StateScoped(LobbyState::None),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(300.0),
                            height: Val::Px(100.0),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    EnterLobbyBtn,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Enter Lobby",
                        TextStyle {
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
        });
}

fn enter_lobby(
    q_interactions: Query<&Interaction, With<EnterLobbyBtn>>,
    mut connection_manager: ResMut<ConnectionManager>,
    lobbies: Option<Res<Lobbies>>,
    mut next_lobby_state: ResMut<NextState<LobbyState>>,
) {
    let Some(lobbies) = lobbies.as_ref().map(|l| &l.lobbies) else {
        return;
    };

    for interaction in q_interactions.iter() {
        match interaction {
            Interaction::Pressed => {
                connection_manager
                    .send_message::<ReliableChannel, _>(&JoinLobby { lobby_id: 0 })
                    .unwrap();

                next_lobby_state.set(LobbyState::Joined);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[derive(Component)]
struct EnterLobbyBtn;
