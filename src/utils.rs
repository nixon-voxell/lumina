use bevy::prelude::*;
use lightyear::prelude::*;

// pub fn run_in_state<S: States, M>(
//     system: impl IntoSystemConfigs<M>,
//     state: S,
// ) -> bevy::ecs::schedule::SystemConfigs {
//     system.run_if(in_state(state)).into_configs()
// }

pub trait EntityRoomId {
    fn room_id(self) -> server::RoomId;
}

impl EntityRoomId for Entity {
    fn room_id(self) -> server::RoomId {
        server::RoomId(self.to_bits())
    }
}
