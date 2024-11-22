use bevy::prelude::*;
use bevy_transform_interpolation::*;
use lightyear::prelude::*;

pub(super) struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                TransformEasingSet,
                TransformSyncSet,
                TransformSystem::TransformPropagate,
            )
                .chain(),
        );

        app.init_resource::<ColorPalette>();
    }
}

/// Propagate component to the children hierarchy.
pub fn propagate_component<C: Component + Clone>(
    mut commands: Commands,
    q_children: Query<
        (&C, &Children),
        // Just added or the children changes.
        Or<(Added<C>, Changed<Children>)>,
    >,
) {
    for (component, children) in q_children.iter() {
        for entity in children.iter() {
            if let Some(mut cmd) = commands.get_entity(*entity) {
                cmd.try_insert(component.clone());
            }
        }
    }
}

/// Runs in [`PostUpdate`] after [`TransformEasingSet`] and before [`TransformSystem::TransformPropagate`].
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TransformSyncSet;

pub trait EntityRoomId {
    fn room_id(self) -> server::RoomId;
}

impl EntityRoomId for Entity {
    fn room_id(self) -> server::RoomId {
        server::RoomId(self.to_bits())
    }
}

#[derive(Default, Debug, Clone)]
pub struct Vec2d<T> {
    values: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Vec2d<T> {
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.values[x + y * self.width]
    }

    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> &T {
        &mut self.values[x + y * self.width]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.values[x + y * self.width] = value;
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.values.iter().enumerate().map(|(i, v)| {
            let x = i % self.width;
            let y = i / self.width;

            (x, y, v)
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.values.iter_mut().enumerate().map(|(i, v)| {
            let x = i % self.width;
            let y = i / self.width;

            (x, y, v)
        })
    }

    /// Get neighbors in the order of \[left, right, bottom, top\].
    pub fn get_neighbors(&self, x: usize, y: usize) -> [Option<&T>; 4] {
        let left = match x > 0 {
            true => Some(self.get(x - 1, y)),
            false => None,
        };
        let right = match x < self.width - 1 {
            true => Some(self.get(x + 1, y)),
            false => None,
        };
        let bottom = match y > 0 {
            true => Some(self.get(x, y - 1)),
            false => None,
        };
        let top = match y < self.height - 1 {
            true => Some(self.get(x, y + 1)),
            false => None,
        };

        [left, right, bottom, top]
    }

    /// Empty all values and reset width and height to zero.
    pub fn clear(&mut self) {
        self.values.clear();
        self.width = 0;
        self.height = 0;
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T: Clone> Vec2d<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> Self {
        Self {
            values: vec![default_value; width * height],
            width,
            height,
        }
    }
}

impl<T: Default + Clone> Vec2d<T> {
    pub fn new_from_default(width: usize, height: usize) -> Self {
        Self {
            values: vec![T::default(); width * height],
            width,
            height,
        }
    }
}

#[derive(Resource)]
pub struct ColorPalette {
    pub red: Color,
    pub orange: Color,
    pub yellow: Color,
    pub green: Color,
    pub blue: Color,
    pub purple: Color,
    pub base0: Color,
    pub base1: Color,
    pub base2: Color,
    pub base3: Color,
    pub base4: Color,
    pub base5: Color,
    pub base6: Color,
    pub base7: Color,
    pub base8: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            red: Color::Srgba(Srgba::hex("#FF6188").unwrap()),
            orange: Color::Srgba(Srgba::hex("#FC9867").unwrap()),
            yellow: Color::Srgba(Srgba::hex("#FFD866").unwrap()),
            green: Color::Srgba(Srgba::hex("#A9DC76").unwrap()),
            blue: Color::Srgba(Srgba::hex("#78DCE8").unwrap()),
            purple: Color::Srgba(Srgba::hex("#AB9DF2").unwrap()),
            base0: Color::Srgba(Srgba::hex("#19181A").unwrap()),
            base1: Color::Srgba(Srgba::hex("#221F22").unwrap()),
            base2: Color::Srgba(Srgba::hex("#2D2A2E").unwrap()),
            base3: Color::Srgba(Srgba::hex("#403E41").unwrap()),
            base4: Color::Srgba(Srgba::hex("#5B595C").unwrap()),
            base5: Color::Srgba(Srgba::hex("#727072").unwrap()),
            base6: Color::Srgba(Srgba::hex("#939293").unwrap()),
            base7: Color::Srgba(Srgba::hex("#C1C0C0").unwrap()),
            base8: Color::Srgba(Srgba::hex("#FCFCFA").unwrap()),
        }
    }
}
/// Pseudorandom number generator using XOR and shift operations.
pub struct XorShift32(u32);

impl XorShift32 {
    /// Create a new hasher from a seed using Wang hash.
    pub fn new(mut seed: u32) -> Self {
        // https://gist.github.com/badboy/6267743#hash-function-construction-principles
        // Wang hash: this has the property that none of the outputs will
        // collide with each other, which is important for the purposes of
        // seeding a random number generator.  This was verified empirically
        // by checking all 2^32 uints.
        seed = (seed ^ 61) ^ (seed >> 16);
        seed = seed.wrapping_mul(9);
        seed = seed ^ (seed >> 4);
        seed = seed.wrapping_mul(0x27d4eb2d);
        seed = seed ^ (seed >> 15);

        Self(seed)
    }

    pub fn next_u32(&mut self) -> u32 {
        let state = self.0;
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;

        state
    }
}
