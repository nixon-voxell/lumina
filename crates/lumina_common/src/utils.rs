use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_transform_interpolation::*;
use lightyear::prelude::*;
use strum::EnumCount;

pub(super) struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                TransformEasingSet,
                TransformSyncSet,
                TransformSystem::TransformPropagate,
            )
                .chain(),
        );

        app.init_resource::<ColorPalette>();
    }
}

/// Propagate component to the children hierarchy.
pub fn propagate_component<C: Component + Clone>(
    mut commands: Commands,
    q_children: Query<
        (&C, &Children),
        // Just added or the children changes.
        Or<(Added<C>, Changed<Children>)>,
    >,
) {
    for (component, children) in q_children.iter() {
        for entity in children.iter() {
            if let Some(mut cmd) = commands.get_entity(*entity) {
                cmd.try_insert(component.clone());
            }
        }
    }
}

/// Runs in [`PostUpdate`] after [`TransformEasingSet`] and before [`TransformSystem::TransformPropagate`].
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TransformSyncSet;

pub trait EntityRoomId {
    fn room_id(self) -> server::RoomId;
}

impl EntityRoomId for Entity {
    fn room_id(self) -> server::RoomId {
        server::RoomId(self.to_bits())
    }
}

#[derive(Resource)]
pub struct ColorPalette {
    pub red: Color,
    pub orange: Color,
    pub yellow: Color,
    pub green: Color,
    pub blue: Color,
    pub purple: Color,
    pub base0: Color,
    pub base1: Color,
    pub base2: Color,
    pub base3: Color,
    pub base4: Color,
    pub base5: Color,
    pub base6: Color,
    pub base7: Color,
    pub base8: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            red: Color::Srgba(Srgba::hex("#FF6188").unwrap()),
            orange: Color::Srgba(Srgba::hex("#FC9867").unwrap()),
            yellow: Color::Srgba(Srgba::hex("#FFD866").unwrap()),
            green: Color::Srgba(Srgba::hex("#A9DC76").unwrap()),
            blue: Color::Srgba(Srgba::hex("#78DCE8").unwrap()),
            purple: Color::Srgba(Srgba::hex("#AB9DF2").unwrap()),
            base0: Color::Srgba(Srgba::hex("#19181A").unwrap()),
            base1: Color::Srgba(Srgba::hex("#221F22").unwrap()),
            base2: Color::Srgba(Srgba::hex("#2D2A2E").unwrap()),
            base3: Color::Srgba(Srgba::hex("#403E41").unwrap()),
            base4: Color::Srgba(Srgba::hex("#5B595C").unwrap()),
            base5: Color::Srgba(Srgba::hex("#727072").unwrap()),
            base6: Color::Srgba(Srgba::hex("#939293").unwrap()),
            base7: Color::Srgba(Srgba::hex("#C1C0C0").unwrap()),
            base8: Color::Srgba(Srgba::hex("#FCFCFA").unwrap()),
        }
    }
}

/// Enum variant data stored in the form of a [`Resource`].
#[derive(Resource, Deref, DerefMut, Debug)]
pub struct EnumVariantRes<T: EnumCount, U>(#[deref] Vec<U>, PhantomData<T>);

impl<T: EnumCount, U: Default + Clone> Default for EnumVariantRes<T, U> {
    fn default() -> Self {
        Self(vec![U::default(); T::COUNT], PhantomData)
    }
}
