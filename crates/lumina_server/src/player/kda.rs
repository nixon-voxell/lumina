use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;

pub struct KdaPlugin;

impl Plugin for KdaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (init_kda, player_death))
            .observe(on_ammo_hit)
            .observe(on_kill);
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
        commands.entity(entity).insert(KdaBundle::default());
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
    mut commands: Commands,
    mut q_spaceships: Query<
        (&mut DeathCount, &mut StreakCount, &LastDamage, &PlayerId),
        (Added<Dead>, With<Spaceship>, With<SourceEntity>),
    >,
) {
    for (mut death_count, mut streak_count, last_damage, &dead_id) in q_spaceships.iter_mut() {
        death_count.0 += 1;
        // Reset streak on death.
        streak_count.0 = 0;

        let Some(alive_id) = last_damage.0 else {
            continue;
        };

        commands.trigger(Kill { alive_id, dead_id });
    }
}

fn on_kill(
    trigger: Trigger<Kill>,
    mut q_kill_counts: Query<(
        &mut KillCount,
        &mut StreakCount,
        Has<ShadowAbilityConfig>,
        &mut Health,
        &MaxHealth,
    )>,
    player_infos: Res<PlayerInfos>,
    mut connection_manager: ResMut<ConnectionManager>,
) {
    let Kill { alive_id, dead_id } = trigger.event();
    if let Some((mut kill_count, mut streak_count, is_shadow, mut health, max_health)) =
        player_infos[PlayerInfoType::Spaceship]
            .get(alive_id)
            .and_then(|e| q_kill_counts.get_mut(*e).ok())
    {
        kill_count.0 += 1;
        streak_count.0 += 1;

        // Apply life steal for assassin.
        if is_shadow {
            **health = (**health + 10.0).min(**max_health);
        }

        let _ = connection_manager.send_message::<OrdReliableChannel, _>(
            alive_id.0,
            &KilledPlayer {
                killed_id: *dead_id,
                streak_count: streak_count.0 as u8,
            },
        );
    }
}

#[derive(Bundle, Default)]
pub struct KdaBundle {
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

#[derive(Event, Clone, Copy)]
struct Kill {
    /// The id of player that secures the kill.
    pub alive_id: PlayerId,
    /// The id of player that is being killed.
    pub dead_id: PlayerId,
}
