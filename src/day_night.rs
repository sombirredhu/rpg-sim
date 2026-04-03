use bevy::prelude::*;
use crate::components::*;

/// System: Progress game time and day/night cycle
pub fn day_night_cycle_system(
    mut game_time: ResMut<GameTime>,
    mut kingdom: ResMut<KingdomState>,
    time: Res<Time>,
) {
    if game_time.is_paused {
        return;
    }

    let dt = time.delta_seconds() * game_time.speed_multiplier;
    game_time.time_seconds += dt;

    // Calculate day progress (0.0 to 1.0)
    let day_time = game_time.time_seconds % game_time.day_length;
    game_time.day_progress = day_time / game_time.day_length;

    // Determine time of day
    // 0.0-0.15 = Dawn, 0.15-0.55 = Day, 0.55-0.7 = Dusk, 0.7-1.0 = Night
    game_time.time_of_day = if game_time.day_progress < 0.15 {
        TimeOfDay::Dawn
    } else if game_time.day_progress < 0.55 {
        TimeOfDay::Day
    } else if game_time.day_progress < 0.70 {
        TimeOfDay::Dusk
    } else {
        TimeOfDay::Night
    };

    // Track current day
    let new_day = (game_time.time_seconds / game_time.day_length) as u32 + 1;
    if new_day > game_time.current_day {
        game_time.current_day = new_day;
        kingdom.era_day = new_day;
    }
}

/// System: Apply night overlay visual effect
pub fn night_overlay_system(
    game_time: Res<GameTime>,
    mut overlay: Query<&mut Sprite, With<NightOverlay>>,
) {
    for mut sprite in overlay.iter_mut() {
        sprite.color = game_time.ambient_color();
    }
}

/// System: Speed toggle with keyboard
pub fn speed_control_system(
    keyboard: Res<Input<KeyCode>>,
    game_phase: Res<GamePhase>,
    mut game_time: ResMut<GameTime>,
    mut alerts: ResMut<GameAlerts>,
) {
    // Let 1-9 keys drive build selection while the build menu is open.
    if game_phase.show_build_menu {
        return;
    }

    if keyboard.just_pressed(KeyCode::Key1) {
        game_time.speed_multiplier = 1.0;
        game_time.is_paused = false;
        alerts.push("Speed: 1x".to_string());
    }
    if keyboard.just_pressed(KeyCode::Key2) {
        game_time.speed_multiplier = 2.0;
        game_time.is_paused = false;
        alerts.push("Speed: 2x".to_string());
    }
    if keyboard.just_pressed(KeyCode::Key3) {
        game_time.speed_multiplier = 3.0;
        game_time.is_paused = false;
        alerts.push("Speed: 3x".to_string());
    }
    if keyboard.just_pressed(KeyCode::Space) {
        game_time.is_paused = !game_time.is_paused;
        if game_time.is_paused {
            alerts.push("PAUSED".to_string());
        } else {
            alerts.push("Resumed".to_string());
        }
    }
}

/// Startup: Create night overlay
pub fn spawn_night_overlay(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(2000.0, 2000.0)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 50.0)),
        ..Default::default()
    })
    .insert(NightOverlay);
}
