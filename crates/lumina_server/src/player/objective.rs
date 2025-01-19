use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;

pub(super) struct ObjectivePlugin;

impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_ores);
    }
}

fn init_ores(
    mut commands: Commands,
    q_ores: Query<(&WorldIdx, Entity), (With<OreType>, Added<WorldIdx>)>,
    q_parents: Query<&Parent>,
    mut q_areas: Query<&mut ObjectiveArea>,
    mut room_manager: ResMut<RoomManager>,
) {
    for (world_id, entity) in q_ores.iter() {
        for parent in q_parents.iter_ancestors(entity) {
            if let Ok(mut area) = q_areas.get_mut(parent) {
                area.ores.insert(entity);
                commands.entity(entity).insert(Replicate {
                    sync: SyncTarget {
                        prediction: NetworkTarget::All,
                        interpolation: NetworkTarget::All,
                    },
                    relevance_mode: NetworkRelevanceMode::InterestManagement,
                    ..default()
                });

                room_manager.add_entity(entity, world_id.room_id());
                break;
            }
        }
    }
}
