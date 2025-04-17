use std::marker::PhantomData;
use std::time::Duration;

use bevy::ecs::component::StorageType;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use lightyear::prelude::*;

use crate::utils::ThreadSafe;

/// Tracks a [`AutoTimer`] component that automtically updates every frame.
///
/// Once the timer finishes, a [`AutoTimerFinished`] event will be triggered.
pub struct AutoTimerPlugin<M, Schedule = Update, Set = DefaultTimerSet>
where
    M: ThreadSafe,
    Schedule: ScheduleLabel,
    Set: SystemSet,
{
    /// The schedule the timer tick will run in.
    pub schedule: Schedule,
    /// The system set that the timer tick will run in, default to None.
    pub set: Option<Set>,
    /// Whether to remove the [`AutoTimer`] component on finish, default to true.
    ///
    /// If this is not removed on finish, it would be idea to create a system to
    /// do so using triggers or event from [`AutoTimerFinished`].
    ///
    /// Removal happens by default in the [`Last`] schedule,
    /// which allows timer value to be read in between.
    pub remove_on_finish: bool,
    _marker: PhantomData<M>,
}

impl<M, Schedule, Set> Plugin for AutoTimerPlugin<M, Schedule, Set>
where
    M: ThreadSafe,
    Schedule: ScheduleLabel + Clone,
    Set: SystemSet + Clone,
{
    fn build(&self, app: &mut App) {
        let timer_update_system = match self.set.clone() {
            Some(set) => timer_update::<M>.in_set(set).into_configs(),
            None => timer_update::<M>.into_configs(),
        };

        app.add_event::<AutoTimerFinished<M>>()
            .add_systems(self.schedule.clone(), timer_update_system);

        if self.remove_on_finish {
            // Allow other systems to read the value of the timer in between
            // before removing it all together.
            app.add_systems(Last, remove_on_finish_trigger::<M>);
        }
    }
}

impl<M, Schedule, Set> AutoTimerPlugin<M, Schedule, Set>
where
    M: ThreadSafe,
    Schedule: ScheduleLabel,
    Set: SystemSet,
{
    pub fn new(schedule: Schedule) -> Self {
        Self {
            schedule,
            set: None,
            remove_on_finish: true,
            _marker: PhantomData,
        }
    }

    /// Run the timer update in a [`SystemSet`].
    pub fn with_system_set(mut self, set: Set) -> Self {
        self.set = Some(set);
        self
    }

    /// Whether to remove the [`AutoTimer`] component on finish, default to true.
    ///
    /// If this is not removed on finish, it would be idea to create a system to
    /// do so using triggers or event from [`AutoTimerFinished`].
    ///
    /// Removal happens by default in the [`Last`] schedule,
    /// which allows timer value to be read in between.
    pub fn remove_on_finish(mut self, remove_on_finish: bool) -> Self {
        self.remove_on_finish = remove_on_finish;
        self
    }
}

impl<M, Set> Default for AutoTimerPlugin<M, Update, Set>
where
    M: ThreadSafe,
    Set: SystemSet,
{
    fn default() -> Self {
        Self {
            schedule: Update,
            set: None,
            remove_on_finish: true,
            _marker: PhantomData,
        }
    }
}

/// Ticks [`AutoTimer`].
pub fn timer_update<M: ThreadSafe>(
    mut commands: Commands,
    mut q_timers: Query<(&mut AutoTimer<M>, Entity), Without<AutoTimerFinishMarker>>,
    mut evr_timer_finished: EventWriter<AutoTimerFinished<M>>,
    time: Res<Time>,
) {
    for (mut timer, entity) in q_timers.iter_mut() {
        if timer.tick(time.delta()).finished() {
            commands.trigger_targets(AutoTimerFinished::<M>::new(entity), entity);
            evr_timer_finished.send(AutoTimerFinished::<M>::new(entity));

            // Prevent events from sending again!
            commands.entity(entity).insert(AutoTimerFinishMarker);
        }
    }
}

fn remove_on_finish_trigger<M: ThreadSafe>(
    mut commands: Commands,
    mut evr_timer_finished: EventReader<AutoTimerFinished<M>>,
) {
    for AutoTimerFinished(entity, _) in evr_timer_finished.read() {
        // Remove timer on finish.
        if let Some(mut cmd) = commands.get_entity(*entity) {
            cmd.remove::<AutoTimer<M>>();
        }
    }
}

/// A generic timer component that automatically ticks if the
/// [`AutoTimerPlugin`] with the same generic is in use.
#[derive(Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq)]
pub struct AutoTimer<M: ThreadSafe>(#[deref] pub Timer, PhantomData<M>);

impl<M: ThreadSafe> Component for AutoTimer<M> {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_remove(|mut world, entity, _id| {
            // Cleanup marker.
            world
                .commands()
                .entity(entity)
                .remove::<AutoTimerFinishMarker>();
        });
    }
}

impl<M: ThreadSafe> AutoTimer<M> {
    pub fn new(timer: Timer) -> Self {
        Self(timer, PhantomData)
    }
}

#[derive(Event, Deref)]
pub struct AutoTimerFinished<M: ThreadSafe>(#[deref] pub Entity, PhantomData<M>);

/// Marker component inserted when [`AutoTimer`] finishes.
#[derive(Component)]
pub struct AutoTimerFinishMarker;

impl<M: ThreadSafe> AutoTimerFinished<M> {
    pub fn new(entity: Entity) -> Self {
        Self(entity, PhantomData)
    }
}

pub trait StartAutoTimerCommandExt {
    fn start_auto_timer<M: ThreadSafe>(&mut self, duration: Duration) -> &mut Self;
}

impl StartAutoTimerCommandExt for EntityCommands<'_> {
    fn start_auto_timer<M: ThreadSafe>(&mut self, duration: Duration) -> &mut Self {
        self.try_insert(AutoTimer::<M>::new(Timer::new(duration, TimerMode::Once)))
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct DefaultTimerSet;
