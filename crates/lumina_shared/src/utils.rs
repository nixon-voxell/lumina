use bevy::prelude::*;
use lightyear::prelude::*;

pub trait EntityRoomId {
    fn room_id(self) -> server::RoomId;
}

impl EntityRoomId for Entity {
    fn room_id(self) -> server::RoomId {
        server::RoomId(self.to_bits())
    }
}
