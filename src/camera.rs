use bevy::prelude::*;
use crate::components::MainCamera;

/// Convert primary-window cursor coordinates into world-space coordinates for a 2D orthographic camera.
pub fn cursor_to_world_2d(
    window: &Window,
    camera_transform: &Transform,
    projection: &OrthographicProjection,
) -> Option<Vec2> {
    let cursor = window.cursor_position()?;
    let window_size = Vec2::new(window.width(), window.height());
    let centered = cursor - (window_size * 0.5);
    Some(camera_transform.translation.truncate() + centered * projection.scale)
}

/// Half-extent of the playable map in world units.
pub const MAP_HALF_EXTENT: f32 = 1500.0;

/// System: Camera pan with WASD/arrow keys and zoom with scroll
pub fn camera_control_system(
    keyboard: Res<Input<KeyCode>>,
    mut scroll_events: EventReader<bevy::input::mouse::MouseWheel>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    windows: Res<Windows>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let window = windows.get_primary().unwrap();
    let win_w = window.width();
    let win_h = window.height();

    for (mut transform, mut projection) in camera.iter_mut() {
        let speed = 200.0 * projection.scale * dt;

        // WASD / Arrow key movement
        if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
            transform.translation.y += speed;
        }
        if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
            transform.translation.y -= speed;
        }
        if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
            transform.translation.x -= speed;
        }
        if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
            transform.translation.x += speed;
        }

        // Zoom with mouse wheel
        for event in scroll_events.iter() {
            let zoom_delta = -event.y * 0.1;
            projection.scale = (projection.scale + zoom_delta).clamp(0.2, 3.0);
        }

        // Zoom with +/- keys
        if keyboard.pressed(KeyCode::Equals) || keyboard.pressed(KeyCode::NumpadAdd) {
            projection.scale = (projection.scale - 0.5 * dt).max(0.2);
        }
        if keyboard.pressed(KeyCode::Minus) || keyboard.pressed(KeyCode::NumpadSubtract) {
            projection.scale = (projection.scale + 0.5 * dt).min(3.0);
        }

        // Clamp camera so the viewport never leaves the map
        let half_view_w = win_w * 0.5 * projection.scale;
        let half_view_h = win_h * 0.5 * projection.scale;
        let max_x = (MAP_HALF_EXTENT - half_view_w).max(0.0);
        let max_y = (MAP_HALF_EXTENT - half_view_h).max(0.0);
        transform.translation.x = transform.translation.x.clamp(-max_x, max_x);
        transform.translation.y = transform.translation.y.clamp(-max_y, max_y);
    }
}
