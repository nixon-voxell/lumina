use bevy::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_shared::{
    health::{Health, MaxHealth},
    player::PlayerId,
};

use velyst::prelude::*;

use crate::player::LocalPlayerId;
use crate::ui::game_ui::GameUi;

pub(super) struct HealthUiPlugin;

impl Plugin for HealthUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<GameUi>()
            .compile_typst_func::<GameUi, HealthFunc>()
            .init_resource::<HealthFunc>()
            .add_systems(Update, update_health_ui);
    }
}

fn update_health_ui(
    q_spaceships: Query<
        (&Health, &MaxHealth, &PlayerId),
        (Changed<Health>, With<Spaceship>, With<SourceEntity>),
    >,
    local_player_id: Res<LocalPlayerId>,
    mut health_func: ResMut<HealthFunc>,
) {
    for (health, max_health, _player_id) in q_spaceships
        .iter()
        .filter(|(_, _, id)| **id == local_player_id.0)
    {
        health_func.current_hp = **health as f64;
        health_func.max_hp = **max_health as f64;
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "playerhealth")]
pub struct HealthFunc {
    current_hp: f64,
    max_hp: f64,
}
