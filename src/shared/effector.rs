use avian2d::prelude::*;
use bevy::prelude::*;

pub struct EffectorPlugin;

impl Plugin for EffectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, convert_effector);

        app.register_type::<EffectorPopupMsg>()
            .register_type::<InteractableEffector>()
            .register_type::<Effector>();
    }
}

/// Convert all effectors to colliders and sensors.
fn convert_effector(mut commands: Commands, q_effectors: Query<(&Effector, Entity)>) {
    for (effector, entity) in q_effectors.iter() {
        commands.entity(entity).insert((
            Collider::try_from_constructor(effector.collider.clone()).unwrap(),
            Sensor,
            effector.rigidbody,
        ));

        commands.entity(entity).remove::<Effector>();
    }
}

/// Popup message when player enters the effector collision range.
#[derive(Component, Reflect, Default, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct EffectorPopupMsg(pub String);

/// Popup the interactable button when player enters the effector collision range.
///
/// This also acts as a marker that a particular [`Sensor`] is interactable.
/// The value in this struct determines the long press duration for the interaction to be valid.
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct InteractableEffector {
    pub required_accumulation: f32,
}

/// A constructor for effector which will be converted into avian sensor related components:
///
/// - [`RigidBody`]
/// - [`Collider`]
/// - [`Sensor`]
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Effector {
    pub rigidbody: RigidBody,
    pub collider: ColliderConstructor,
}
