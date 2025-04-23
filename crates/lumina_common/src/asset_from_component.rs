use std::marker::PhantomData;

use bevy::prelude::*;

/// Replicate and update [`Asset`] from it's [`Component`] counterpart.
///
/// Note: Both [`Asset`] and [`Component`] type must be the same type `T`.
pub struct AssetFromComponentPlugin<T: AssetFromComponent>(PhantomData<T>);

impl<T: AssetFromComponent> Plugin for AssetFromComponentPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                replicate_asset_from_component::<T>,
                update_asset_from_component::<T>,
            ),
        );
    }
}

impl<T: AssetFromComponent> Default for AssetFromComponentPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

fn replicate_asset_from_component<T: AssetFromComponent>(
    mut commands: Commands,
    q_components: Query<(&T, Entity), Added<T>>,
    mut assets: ResMut<Assets<T>>,
) {
    for (comp, entity) in q_components.iter() {
        commands.entity(entity).insert(assets.add(comp.clone()));
    }
}

fn update_asset_from_component<T: AssetFromComponent>(
    q_components: Query<(&T, &Handle<T>), Changed<T>>,
    mut assets: ResMut<Assets<T>>,
) {
    for (comp, handle) in q_components.iter() {
        if let Some(asset) = assets.get_mut(handle) {
            *asset = comp.clone();
        }
    }
}

pub trait AssetFromComponent: Component + Asset + Clone {}

impl<T: Component + Asset + Clone> AssetFromComponent for T {}
