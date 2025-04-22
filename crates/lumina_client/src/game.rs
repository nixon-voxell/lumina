use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::game::prelude::*;
use lumina_shared::prelude::*;

use crate::effector::*;

mod ore_vfx;

pub(super) struct GamePugin;

impl Plugin for GamePugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ore_vfx::OreVfxPlugin)
            .observe(teleport_player)
            .observe(disable_teleporter)
            .observe(enable_teleporter)
            .observe(deposit_tesseract);
    }
}

/// Sends a [`Teleport`] message to the server when
/// the [`TeleporterEffector`] is being triggered.
fn teleport_player(
    trigger: Trigger<TeleporterEffector>,
    mut commands: Commands,
    q_teleporters: Query<&Teleporter>,
    mut connection_manager: ResMut<ConnectionManager>,
) {
    let effector_entity = trigger.entity();
    let Ok(&teleporter) = q_teleporters.get(effector_entity) else {
        return;
    };

    let _ = connection_manager.send_message::<OrdReliableChannel, _>(&Teleport { teleporter });

    // Clear interaction marker.
    commands
        .entity(effector_entity)
        .remove::<InteractedEffector>();
}

/// Disable teleporter during cooldown.
fn disable_teleporter(trigger: Trigger<OnAdd, TeleporterCooldown>, mut commands: Commands) {
    let effector_entity = trigger.entity();

    commands.entity(effector_entity).insert(DisabledEffector);
}

/// Enable teleporter when cooldown is complete.
fn enable_teleporter(trigger: Trigger<OnRemove, TeleporterCooldown>, mut commands: Commands) {
    let effector_entity = trigger.entity();

    commands
        .entity(effector_entity)
        .remove::<DisabledEffector>();
}

/// Sends a [`DepositLumina`] message to the server when
/// the [`TesseractEffector`] is being triggered.
fn deposit_tesseract(
    trigger: Trigger<TesseractEffector>,
    mut commands: Commands,
    mut connection_manager: ResMut<ConnectionManager>,
) {
    let effector_entity = trigger.entity();

    let _ = connection_manager.send_message::<OrdReliableChannel, _>(&DepositLumina);

    // Clear interaction marker.
    commands
        .entity(effector_entity)
        .remove::<InteractedEffector>();
}
