use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;

use crate::effector::{InteractedEffector, TesseractEffector};

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

    let _ = connection_manager.send_message::<OrdReliableChannel, _>(&DepositLumina);

    // Clear interaction marker.
    commands
        .entity(effector_entity)
        .remove::<InteractedEffector>();
}
