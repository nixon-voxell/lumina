use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;

pub struct KDAPlugin;

impl Plugin for KDAPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (init_kda, player_death))
            .observe(on_ammo_hit);
    }
}

fn init_kda(
    mut commands: Commands,
    q_spaceships: Query<
        Entity,
        (
            With<Spaceship>,
            With<SourceEntity>,
            With<PlayerId>,
            Without<LastDamage>,
        ),
    >,
) {
    for entity in q_spaceships.iter() {
        commands.entity(entity).insert(KDABundle::default());
    }
}

fn on_ammo_hit(trigger: Trigger<AmmoHit>, mut commands: Commands, player_infos: Res<PlayerInfos>) {
    let ammo_hit = trigger.event();

    let Some(hit_id) = ammo_hit.hit_player_id else {
        return;
    };

    let Some(&hit_entity) = player_infos[PlayerInfoType::Spaceship].get(&hit_id) else {
        return;
    };

    if let Some(mut cmd) = commands.get_entity(hit_entity) {
        // Record the last damage player.
        cmd.try_insert(LastDamage(Some(ammo_hit.origin_player_id)));
    }
}

fn player_death(
    mut q_spaceships: Query<
        (&mut DeathCount, &mut StreakCount, &LastDamage, &PlayerId),
        (Added<Dead>, With<Spaceship>, With<SourceEntity>),
    >,
    mut q_kill_counts: Query<&mut KillCount>,
    player_infos: Res<PlayerInfos>,
    mut connection_manager: ResMut<ConnectionManager>,
) {
    for (mut death_count, mut streak_count, last_damage, dead_id) in q_spaceships.iter_mut() {
        death_count.0 += 1;
        // Reset streak on death.
        streak_count.0 = 0;

        let Some(id) = last_damage.0 else {
            continue;
        };

        if let Some(mut kill_count) = player_infos[PlayerInfoType::Spaceship]
            .get(&id)
            .and_then(|e| q_kill_counts.get_mut(*e).ok())
        {
            kill_count.0 += 1;

            let _ = connection_manager
                .send_message::<OrdReliableChannel, _>(id.0, &KilledPlayer(*dead_id));
        }
    }
}

#[derive(Bundle, Default)]
pub struct KDABundle {
    pub last_damage: LastDamage,
    pub kill_count: KillCount,
    pub streak_count: StreakCount,
    pub death_count: DeathCount,
}

/// The most recent damage dealt from another player.
#[derive(Component, Default, Deref, DerefMut)]
pub struct LastDamage(Option<PlayerId>);

/// The number of kills the player currently has.
#[derive(Component, Default, Deref, DerefMut)]
pub struct KillCount(pub u32);

#[derive(Component, Default, Deref, DerefMut)]
pub struct StreakCount(pub u32);

/// The number of deaths the player currently has.
#[derive(Component, Default, Deref, DerefMut)]
pub struct DeathCount(pub u32);
