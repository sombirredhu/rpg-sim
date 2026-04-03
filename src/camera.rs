use bevy::prelude::*;

/// System: Camera pan with WASD/arrow keys and zoom with scroll
pub fn camera_control_system(
    keyboard: Res<Input<KeyCode>>,
    mut scroll_events: EventReader<bevy::input::mouse::MouseWheel>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

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
    }
}
