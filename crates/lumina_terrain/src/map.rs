use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy_transform_interpolation::NoTranslationInterpolation;
use lumina_common::prelude::*;
use noisy_bevy::*;

use crate::config::TerrainConfigAsset;
use crate::{GenerateTerrain, TerrainType};

#[derive(bevy::ecs::system::SystemParam)]
pub struct Terrain<'w, 's> {
    commands: Commands<'w, 's>,
    pub q_maps: Query<'w, 's, (&'static mut TerrainTiles, &'static mut TerrainStates)>,
    pub pools: ResMut<'w, EntityPools<TerrainType>>,
    pub tile_ref: Res<'w, TileRef>,
}

impl Terrain<'_, '_> {
    /// Returns true if successfully cleared the terrain and false if failed to find the entity.
    pub fn clear_terrain(&mut self, entity: Entity) -> bool {
        if let Ok((mut tiles, mut states)) = self.q_maps.get_mut(entity) {
            for TerrainTile { entity, .. } in tiles.drain(..) {
                self.pools[TerrainType::Tile as usize].set_unused(entity);
                self.commands
                    .entity(entity)
                    .remove::<(RigidBody, Collider)>()
                    .insert(Visibility::Hidden);
            }

            states.clear();
            return true;
        }

        false
    }

    pub fn generate_terrain(
        &mut self,
        map_entity: Entity,
        config: &TerrainConfigAsset,
        gen: &GenerateTerrain,
    ) {
        self.clear_terrain(map_entity);
        let states = TerrainStates::new_map(config, gen);
        let mut tiles = TerrainTiles::default();

        // Spawn the tiles.
        for (x, y, &state) in states.iter() {
            if state == false {
                continue;
            }

            // Use pool if exists.
            let entity = self.pools[TerrainType::Tile as usize]
                .get_unused_or_spawn(|| self.commands.spawn_empty().id());

            self.commands.entity(entity).insert(TileBundle::new(
                &self.tile_ref,
                Vec2::new(config.tile_size * x as f32, config.tile_size * y as f32),
                config.tile_size,
            ));

            let filled_neighbor_count = states
                .get_neighbors(x, y)
                .iter()
                // Is filled or it's nothing beyond (at the border).
                .filter(|&&s| s.is_some_and(|&s| s) || s.is_none())
                .count();

            if filled_neighbor_count < 4 {
                self.commands
                    .entity(entity)
                    .insert(TileColliderBundle::new(&self.tile_ref, gen));
            }

            tiles.push(TerrainTile { entity, x, y });
        }

        // Inserts the terrain map into the holder entity.
        self.commands
            .entity(map_entity)
            .insert(TerrainMapBundle { states, tiles });
    }
}

#[derive(Bundle)]
pub struct TerrainMapBundle {
    pub states: TerrainStates,
    pub tiles: TerrainTiles,
}

#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct TerrainTiles(Vec<TerrainTile>);

#[derive(Debug)]
pub struct TerrainTile {
    pub entity: Entity,
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct TerrainStates(Vec2d<bool>);

impl TerrainStates {
    pub fn new_map(config: &TerrainConfigAsset, gen: &GenerateTerrain) -> Self {
        let mut states = Vec2d::new_from_default(config.size.x as usize, config.size.y as usize);
        let mut hash = XorShift32::new(gen.seed);

        // Seeds will go from 0 -> 1000.0
        let left_seed = ((hash.next_u32() % 1000000) as f32) * 0.01;
        let right_seed = ((hash.next_u32() % 1000000) as f32) * 0.01;
        let bottom_seed = ((hash.next_u32() % 1000000) as f32) * 0.01;
        let top_seed = ((hash.next_u32() % 1000000) as f32) * 0.01;

        for y in 0..config.size.y {
            for x in 0..config.size.x {
                fn remaped_noise(v: Vec2) -> f32 {
                    // Remap from -1.0 -> 1.0 to 0.0 -> 1.0
                    // gradient_noise(v) * 0.5 + 0.5
                    simplex_noise_2d(v) * 0.5 + 0.5
                }

                let left_dist = u32::abs_diff(x + 1, 0);
                let right_dist = u32::abs_diff(x, config.size.x);
                let bottom_dist = u32::abs_diff(y + 1, 0);
                let top_dist = u32::abs_diff(y, config.size.y);

                let surr_width = config.noise_surr_width as f32;

                // Convert absolute distance to relative distance (may go out of 1.0)
                let gradient = |abs_dist: u32| -> f32 {
                    f32::powf(abs_dist as f32 / surr_width, config.gradient_pow)
                };

                // Calculate absolute distances to the terrain edges.
                let left_grad = gradient(left_dist);
                let right_grad = gradient(right_dist);
                let bottom_grad = gradient(bottom_dist);
                let top_grad = gradient(top_dist);

                let calc_noise = |scalar: u32, seed: f32, gradient: f32| -> f32 {
                    f32::min(
                        1.0,
                        remaped_noise(Vec2::new(config.noise_scale * scalar as f32, seed))
                            * gradient,
                    )
                };

                let left_noise = calc_noise(y, left_seed, left_grad);
                let right_noise = calc_noise(y, right_seed, right_grad);
                let bottom_noise = calc_noise(x, bottom_seed, bottom_grad);
                let top_noise = calc_noise(x, top_seed, top_grad);

                // let noise = left_noise * right_noise * bottom_noise * top_noise;
                let noise = left_noise.min(right_noise).min(bottom_noise).min(top_noise);
                let edge_dist = left_dist.min(right_dist).min(bottom_dist).min(top_dist);

                if noise > config.noise_threshold || edge_dist > config.noise_surr_width {
                    continue;
                }

                states.set(x as usize, y as usize, true);
            }
        }

        // Filter away tiles with less than 2 connected neighbors.
        // for y in 0..config.size.y as usize {
        //     for x in 0..config.size.x as usize {
        //         let state = *states.get(x, y);
        //         if state == false {
        //             continue;
        //         }

        //         let neighbors = states.get_neighbors(x, y);
        //         let empty_neighbor_count =
        //             neighbors.iter().filter(|&&s| s.is_some_and(|&s| s)).count();

        //         if empty_neighbor_count < 2 {
        //             states.set(x, y, false);
        //         }
        //     }
        // }

        Self(states)
    }

    pub fn get_map_corners_without_noise_surr(config: &TerrainConfigAsset) -> (Vec2, Vec2) {
        // Bottom-left corner, adjusted to avoid noise surrounding width
        let bottom_left = Vec2::new(
            config.noise_surr_width as f32 * config.tile_size,
            config.noise_surr_width as f32 * config.tile_size,
        );

        // Upper-right corner, adjusted to avoid noise surrounding width
        let upper_right = Vec2::new(
            (config.size.x - config.noise_surr_width - 1) as f32 * config.tile_size,
            (config.size.y - config.noise_surr_width - 1) as f32 * config.tile_size,
        );

        (bottom_left, upper_right)
    }
}

#[derive(Resource)]
pub struct TileRef {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
    pub collider: Collider,
}

#[derive(Bundle)]
pub struct TileBundle {
    pub color_mesh: ColorMesh2dBundle,
    pub position: Position,
    pub no_translation_interp: NoTranslationInterpolation,
}

impl TileBundle {
    pub fn new(tile_ref: &TileRef, position: Vec2, size: f32) -> Self {
        Self {
            color_mesh: ColorMesh2dBundle {
                mesh: Mesh2dHandle(tile_ref.mesh.clone()),
                material: tile_ref.material.clone(),
                visibility: Visibility::Inherited,
                transform: Transform::from_xyz(position.x, position.y, 0.0)
                    .with_scale(Vec3::splat(size)),
                ..default()
            },
            position: Position(position),
            no_translation_interp: NoTranslationInterpolation,
        }
    }
}

#[derive(Bundle)]
pub struct TileColliderBundle {
    pub collider: Collider,
    pub rigidbody: RigidBody,
    pub layers: CollisionLayers,
    pub world_id: WorldIdx,
}

impl TileColliderBundle {
    pub fn new(tile_ref: &TileRef, gen: &GenerateTerrain) -> Self {
        Self {
            collider: tile_ref.collider.clone(),
            rigidbody: RigidBody::Static,
            layers: gen.layers,
            world_id: gen.world_id,
        }
    }
}
