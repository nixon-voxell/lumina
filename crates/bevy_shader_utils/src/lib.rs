use bevy::asset::load_internal_asset;
use bevy::prelude::*;

// Some wgsl from https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39
// Noise Functions
pub(crate) const PERLIN_NOISE_2D: Handle<Shader> =
    Handle::weak_from_u128(19040624558549619711502781022329531134);
pub(crate) const PERLIN_NOISE_3D: Handle<Shader> =
    Handle::weak_from_u128(56177246048735587762956655317336754703);
pub(crate) const SIMPLEX_NOISE_2D: Handle<Shader> =
    Handle::weak_from_u128(202025645217084136442921375689607846692);
pub(crate) const SIMPLEX_NOISE_3D: Handle<Shader> =
    Handle::weak_from_u128(175755189139151107887358476890643268085);
pub(crate) const FBM: Handle<Shader> =
    Handle::weak_from_u128(136934631361267469364261179635614511287);
pub(crate) const VORONOISE: Handle<Shader> =
    Handle::weak_from_u128(318899545933921117948526946754151128551);
// Other utility functions
pub(crate) const MOCK_FRESNEL: Handle<Shader> =
    Handle::weak_from_u128(153900884340255605653929282667932002714);

pub struct ShaderUtilsPlugin;

impl Plugin for ShaderUtilsPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PERLIN_NOISE_2D,
            "shaders/perlin_noise_2d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            PERLIN_NOISE_3D,
            "shaders/perlin_noise_3d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SIMPLEX_NOISE_2D,
            "shaders/simplex_noise_2d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SIMPLEX_NOISE_3D,
            "shaders/simplex_noise_3d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(app, FBM, "shaders/fbm.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, VORONOISE, "shaders/voronoise.wgsl", Shader::from_wgsl);
        load_internal_asset!(
            app,
            MOCK_FRESNEL,
            "shaders/mock_fresnel.wgsl",
            Shader::from_wgsl
        );
    }
}
