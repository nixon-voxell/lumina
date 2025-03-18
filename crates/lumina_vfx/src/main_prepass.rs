use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::window::PrimaryWindow;

pub struct MainPrepassPlugin;

impl Plugin for MainPrepassPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_main_prepass_texture,
                replicate_prepass_camera_components,
            ),
        );
    }
}

pub struct MainPrepassComponentPlugin<T: PrepassComponent>(PhantomData<T>);

impl<T: PrepassComponent> Plugin for MainPrepassComponentPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_prepass_scale::<T>,
                update_prepass_image::<T>.run_if(resource_changed::<MainPrepassTexture>),
            ),
        );
    }
}

impl<T: PrepassComponent> Default for MainPrepassComponentPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

fn update_prepass_scale<T: PrepassComponent>(
    q_prepass_camera: Query<
        &OrthographicProjection,
        (With<MainPrepassCamera>, Changed<OrthographicProjection>),
    >,
    mut q_components: Query<&mut T>,
) {
    let Ok(projection) = q_prepass_camera.get_single() else {
        return;
    };

    for mut comp in q_components.iter_mut() {
        *comp.camera_scale_mut() = projection.scale;
    }
}

fn update_prepass_image<T: PrepassComponent>(
    mut q_components: Query<&mut T>,
    texture: Res<MainPrepassTexture>,
) {
    for mut comp in q_components.iter_mut() {
        *comp.image_mut() = texture.image_handle.clone_weak();
    }
}

/// Replicate [`OrthographicProjection`] & [`Transform`] of [`GameCamera`] to [`MainPrepassCamera`].
fn replicate_prepass_camera_components(
    q_main_camera: Query<
        (&OrthographicProjection, &Transform),
        (
            Or<(Changed<OrthographicProjection>, Changed<Transform>)>,
            Without<MainPrepassCamera>,
        ),
    >,
    mut q_prepass_camera: Query<
        (
            &MainPrepassCamera,
            &mut OrthographicProjection,
            &mut Transform,
        ),
        With<MainPrepassCamera>,
    >,
) {
    if let Ok((&MainPrepassCamera(main_entity), mut prepass_projection, mut prepass_transform)) =
        q_prepass_camera.get_single_mut()
    {
        if let Ok((main_projection, main_transform)) = q_main_camera.get(main_entity) {
            *prepass_projection = main_projection.clone();
            *prepass_transform = *main_transform;
        }
    }
}

fn update_main_prepass_texture(
    q_window: Query<&Window, (Changed<Window>, With<PrimaryWindow>)>,
    mut q_camera: Query<&mut Camera, With<MainPrepassCamera>>,
    mut prepass_texture: ResMut<MainPrepassTexture>,
    mut images: ResMut<Assets<Image>>,
) {
    let (Ok(window), Ok(mut camera)) = (q_window.get_single(), q_camera.get_single_mut()) else {
        return;
    };

    if prepass_texture.size != window.size() {
        prepass_texture.size = window.size();
        // Skip resizing texture if it's too small.
        if window.width() <= f32::EPSILON || window.height() <= f32::EPSILON {
            return;
        }

        if let Some(mut image) = images.remove(prepass_texture.image_handle()) {
            let size = Extent3d {
                width: window.width() as u32,
                height: window.height() as u32,
                ..default()
            };

            image.resize(size);

            let image_handle = images.add(image);
            camera.target = image_handle.clone_weak().into();
            prepass_texture.image_handle = image_handle;
        }
    }
}

/// Stores the main camera's entity for copying it's
/// [`OrthographicProjection`] and [`Transform`] component.
#[derive(Component, Clone, Copy)]
pub struct MainPrepassCamera(pub Entity);

#[derive(Resource)]
pub struct MainPrepassTexture {
    image_handle: Handle<Image>,
    size: Vec2,
}

impl MainPrepassTexture {
    pub fn new(image_handle: Handle<Image>, size: Vec2) -> Self {
        Self { image_handle, size }
    }

    pub fn image_handle(&self) -> &Handle<Image> {
        &self.image_handle
    }
}

pub trait PrepassComponent: Component {
    fn image_mut(&mut self) -> &mut Handle<Image>;

    fn camera_scale_mut(&mut self) -> &mut f32;
}
