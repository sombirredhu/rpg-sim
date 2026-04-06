use bevy::prelude::*;

/// All preloaded audio handles
pub struct AudioAssets {
    pub music_theme: Handle<AudioSource>,
    pub sfx_coin_reward: Handle<AudioSource>,
    pub sfx_hit_impact: Handle<AudioSource>,
    pub sfx_death_womp: Handle<AudioSource>,
}

/// Events that other systems fire to trigger sound effects
pub enum SfxEvent {
    CoinReward,
    HitImpact,
    DeathWomp,
    SiegeSiren,
}

/// Startup system: load all audio assets and start background music
pub fn setup_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let music_theme: Handle<AudioSource> = asset_server.load("Audio/Music/medieval_theme.ogg");
    let sfx_coin_reward: Handle<AudioSource> = asset_server.load("Audio/SFX/coin_reward.ogg");
    let sfx_hit_impact: Handle<AudioSource> = asset_server.load("Audio/SFX/hit_impact.ogg");
    let sfx_death_womp: Handle<AudioSource> = asset_server.load("Audio/SFX/death_womp.ogg");

    // Play background music
    audio.play(music_theme.clone());

    commands.insert_resource(AudioAssets {
        music_theme,
        sfx_coin_reward,
        sfx_hit_impact,
        sfx_death_womp,
    });
    // Note: SiegeSiren will reuse death_womp as placeholder until a dedicated siren sound is added
}

/// System: reads SfxEvent events and plays the matching sound effect
pub fn play_sfx_system(
    mut events: EventReader<SfxEvent>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for event in events.iter() {
        let handle = match event {
            SfxEvent::CoinReward => audio_assets.sfx_coin_reward.clone(),
            SfxEvent::HitImpact => audio_assets.sfx_hit_impact.clone(),
            SfxEvent::DeathWomp => audio_assets.sfx_death_womp.clone(),
            SfxEvent::SiegeSiren => audio_assets.sfx_death_womp.clone(), // Placeholder: use death_womp until dedicated siren asset added
        };
        audio.play(handle);
    }
}
