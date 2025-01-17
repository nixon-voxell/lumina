use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;

use crate::effector::InteractedEffector;

pub(super) struct TesseractPugin;

impl Plugin for TesseractPugin {
    fn build(&self, app: &mut App) {
        app.observe(deposit_tesseract);
    }
}

/// Action performed after the tesseract effector is being triggered.
fn deposit_tesseract(
    trigger: Trigger<TesseractEffector>,
    mut commands: Commands,
    mut connection_manager: ResMut<ConnectionManager>,
) {
    let effector_entity = trigger.entity();
    info!(
        "Tesseract effector triggered by entity {:?}",
        effector_entity
    );

    let _ = connection_manager.send_message::<ReliableChannel, _>(&DepositLumina);

    // Clear interaction marker.
    commands
        .entity(effector_entity)
        .remove::<InteractedEffector>();
}
