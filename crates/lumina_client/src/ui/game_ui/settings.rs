use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

pub(super) struct SettingsUiPlugin;

use crate::audio::AudioVolumeSettings;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<SettingsUi>()
            .compile_typst_func::<SettingsUi, SettingsFunc>()
            .push_to_main_window::<SettingsUi, SettingsFunc, _>(
                MainWindowSet::Foreground,
                |overlay: Res<SettingsOverlay>| overlay.visible, // Render only if visible
            )
            .recompile_on_interaction::<SettingsFunc>(|func| &mut func.dummy_update)
            .init_resource::<SettingsFunc>()
            .init_resource::<SettingsOverlay>()
            .init_resource::<AudioSettings>()
            .add_systems(Update, (adjust_audio, close_settings));
    }
}

fn close_settings(interactions: InteractionQuery, mut overlay: ResMut<SettingsOverlay>) {
    if interactions.pressed("btn:close") {
        overlay.visible = false;
    }
}

fn adjust_audio(
    mut func: ResMut<SettingsFunc>,
    interactions: InteractionQuery,
    mut audio_settings: ResMut<AudioVolumeSettings>,
) {
    if interactions.pressed("btn:decrease_bgm") {
        audio_settings.bgm_volume = (audio_settings.bgm_volume - 0.1).max(0.0);
    }
    if interactions.pressed("btn:increase_bgm") {
        audio_settings.bgm_volume = (audio_settings.bgm_volume + 0.1).min(1.0);
    }
    if interactions.pressed("btn:decrease_vfx") {
        audio_settings.vfx_volume = (audio_settings.vfx_volume - 0.1).max(0.0);
    }
    if interactions.pressed("btn:increase_vfx") {
        audio_settings.vfx_volume = (audio_settings.vfx_volume + 0.1).min(1.0);
    }

    func.bgm_volume = audio_settings.bgm_volume;
    func.vfx_volume = audio_settings.vfx_volume;
}

#[derive(TypstFunc, Resource)]
#[typst_func(name = "settings", layer = 1)]
pub struct SettingsFunc {
    bgm_volume: f64,
    vfx_volume: f64,
    dummy_update: u8,
}

impl Default for SettingsFunc {
    fn default() -> Self {
        let default_audio = AudioVolumeSettings::default();
        SettingsFunc {
            bgm_volume: default_audio.bgm_volume,
            vfx_volume: default_audio.vfx_volume,
            dummy_update: 0,
        }
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/settings.typ"]
pub struct SettingsUi;

#[derive(Resource, Default)]
pub struct SettingsOverlay {
    pub visible: bool,
}
