use bevy::prelude::*;

pub struct EffectorPlugin;

impl Plugin for EffectorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EffectorPopupMsg>()
            .register_type::<InteractableEffector>();
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct EffectorPopupMsg(pub String);

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct InteractableEffector;
