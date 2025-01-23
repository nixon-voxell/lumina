use bevy::ecs::entity::EntityHashSet;
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::utils::EnumVariantRes;

/// Stores an array of [`EntityPool`].
///
/// Note: Number of pools needs to match the number of variants in `T`.
pub type EntityPools<T> = EnumVariantRes<T, EntityPool>;

/// A double [`EntityHashSet`] for storing both used and unused entities.
#[derive(Default, Debug, Clone)]
pub struct EntityPool {
    used: EntityHashSet,
    unused: EntityHashSet,
}

impl EntityPool {
    pub fn used(&self) -> &EntityHashSet {
        &self.used
    }

    pub fn unused(&self) -> &EntityHashSet {
        &self.unused
    }
}

impl EntityPool {
    pub fn insert_new_used(&mut self, entity: Entity) -> bool {
        self.used.insert(entity)
    }

    pub fn insert_new_unused(&mut self, entity: Entity) -> bool {
        self.unused.insert(entity)
    }

    /// Get an unused entity from the entity pool and move it from unused to used.
    pub fn get_unused(&mut self) -> Option<Entity> {
        let entity = self.unused.iter().next().copied()?;
        self.unused.remove(&entity);
        self.used.insert(entity);

        Some(entity)
    }

    /// Get an unused entity from the entity pool or spawn a new one
    /// and move it from unused to used.
    pub fn get_unused_or_spawn(&mut self, mut spawn: impl FnMut() -> Entity) -> Entity {
        match self.get_unused() {
            Some(entity) => entity,
            None => {
                let entity = spawn();
                self.insert_new_used(entity);
                entity
            }
        }
    }

    /// Set entity as unused.
    ///
    /// # Returns
    ///
    /// True if successful, false if unsuccessful.
    pub fn set_unused(&mut self, entity: Entity) {
        if self.used.remove(&entity) == false {
            error!("{entity} was not from the pool!");
        }

        self.unused.insert(entity);
    }

    /// Set entity as used.
    ///
    /// # Returns
    ///
    /// True if successful, false if unsuccessful.
    pub fn set_used(&mut self, entity: Entity) {
        if self.unused.remove(&entity) == false {
            error!("{entity} was not from the pool!");
        }

        self.used.insert(entity);
    }

    /// Drain all used entities and insert them into the unused set.
    pub fn set_all_unused(&mut self) {
        for entity in self.used.drain() {
            self.unused.insert(entity);
        }
    }

    /// Drain all unused entities and insert them into the used set.
    pub fn set_all_used(&mut self) {
        for entity in self.unused.drain() {
            self.used.insert(entity);
        }
    }
}

/// Reference entity map.
#[derive(Resource, Deref, DerefMut)]
pub struct RefEntityMap<T>(HashMap<T, Entity>);

impl<T> Default for RefEntityMap<T> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}
