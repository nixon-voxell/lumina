use bevy::prelude::*;
use lightyear::prelude::client::*;
use lightyear::prelude::*;

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, handle_connection.after(MainSet::Receive))
            .add_systems(Startup, test_connect);
    }
}

/// Marker component for the debug text displaying the `ClientId`
#[derive(Component)]
struct ClientIdText;

fn handle_connection(
    mut commands: Commands,
    mut connection_event: EventReader<ConnectEvent>,
    debug_text: Query<Entity, With<ClientIdText>>,
) {
    for event in connection_event.read() {
        let client_id = event.client_id();
        if let Ok(entity) = debug_text.get_single() {
            commands.entity(entity).despawn_recursive();
        }
        commands.spawn((
            TextBundle::from_section(
                format!("Client {}", client_id),
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            ClientIdText,
        ));
    }
}

fn test_connect(mut commands: Commands) {
    commands.connect_client();
}
