use bevy::prelude::*;
use crate::components::*;

/// System: Handle building placement from build mode
pub fn building_placement_system(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &Transform), With<Camera>>,
    mut economy: ResMut<GameEconomy>,
    mut game_phase: ResMut<GamePhase>,
    kingdom: Res<KingdomState>,
    mut alerts: ResMut<GameAlerts>,
) {
    if !game_phase.build_mode {
        return;
    }

    let selected = match game_phase.selected_building {
        Some(b) => b,
        None => return,
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        // Get mouse world position
        let window = match windows.get_primary() {
            Some(w) => w,
            None => return,
        };
        let cursor_pos = match window.cursor_position() {
            Some(p) => p,
            None => return,
        };

        if let Ok((_camera, camera_transform)) = camera.get_single() {
            let window_size = Vec2::new(window.width(), window.height());
            let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
            let world_pos = camera_transform.translation.truncate()
                + ndc * Vec2::new(window_size.x, window_size.y) * 0.3; // 0.6 scale / 2

            // Check if we can afford it
            let cost = selected.cost();
            if economy.gold < cost {
                alerts.push(format!("Not enough gold! Need {:.0}", cost));
                return;
            }

            // Check if building is available at current rank
            if !kingdom.rank.available_buildings().contains(&selected) {
                alerts.push("Building not available at current rank!".to_string());
                return;
            }

            // Spend gold and place building
            economy.gold -= cost;
            economy.total_spent += cost;

            let building = Building::new(selected);
            let size = selected.size();
            let color = selected.color();

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 5.0)),
                ..Default::default()
            })
            .insert(building);

            alerts.push(format!("{} built for {:.0} gold!", selected.display_name(), cost));

            // Exit build mode after placing
            game_phase.build_mode = false;
            game_phase.selected_building = None;
        }
    }

    // Cancel build mode with right click
    if mouse_input.just_pressed(MouseButton::Right) {
        game_phase.build_mode = false;
        game_phase.selected_building = None;
    }
}

/// System: Building upgrade on double-click / key press
pub fn building_upgrade_system(
    keyboard: Res<Input<KeyCode>>,
    _mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &Transform), With<Camera>>,
    mut buildings: Query<(&mut Building, &Transform), Without<Camera>>,
    mut economy: ResMut<GameEconomy>,
    game_phase: Res<GamePhase>,
    mut alerts: ResMut<GameAlerts>,
) {
    if game_phase.build_mode {
        return;
    }

    // Press U to upgrade nearest building to cursor
    if !keyboard.just_pressed(KeyCode::U) {
        return;
    }

    let window = match windows.get_primary() {
        Some(w) => w,
        None => return,
    };
    let cursor_pos = match window.cursor_position() {
        Some(p) => p,
        None => return,
    };

    if let Ok((_camera, camera_transform)) = camera.get_single() {
        let window_size = Vec2::new(window.width(), window.height());
        let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
        let world_pos = camera_transform.translation.truncate()
            + ndc * Vec2::new(window_size.x, window_size.y) * 0.3;

        // Upgrade first building within range
        for (mut building, transform) in buildings.iter_mut() {
            let pos = Vec2::new(transform.translation.x, transform.translation.y);
            let dist = (pos - world_pos).length();

            if dist < 60.0 && building.tier < 3 && !building.is_destroyed {
                let cost = building.building_type.upgrade_cost(building.tier + 1);
                if economy.gold >= cost {
                    economy.gold -= cost;
                    economy.total_spent += cost;
                    building.tier += 1;
                    building.max_hp *= 1.3;
                    building.hp = building.max_hp;
                    alerts.push(format!(
                        "{} upgraded to Tier {}! (-{:.0} gold)",
                        building.building_type.display_name(),
                        building.tier,
                        cost
                    ));
                } else {
                    alerts.push(format!("Need {:.0} gold to upgrade!", cost));
                }
                break;
            }
        }
    }
}

/// System: Repair destroyed buildings
pub fn building_repair_system(
    mut buildings: Query<&mut Building>,
    mut economy: ResMut<GameEconomy>,
    time: Res<Time>,
    game_time: Res<GameTime>,
    mut repair_timer: Local<f32>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    *repair_timer -= dt;
    if *repair_timer > 0.0 {
        return;
    }
    *repair_timer = 5.0;

    for mut building in buildings.iter_mut() {
        if building.is_destroyed {
            let repair_cost = building.building_type.cost() * 0.5;
            if economy.gold >= repair_cost {
                // Auto-repair if treasury allows
                economy.gold -= repair_cost;
                economy.total_spent += repair_cost;
                building.is_destroyed = false;
                building.hp = building.max_hp * 0.5;
            }
        }
    }
}

/// System: Guard towers auto-attack nearby enemies
pub fn guard_tower_attack_system(
    towers: Query<(&Building, &Transform)>,
    mut enemies: Query<(&mut EnemyStats, &Transform), With<Enemy>>,
    time: Res<Time>,
    game_time: Res<GameTime>,
    mut attack_timer: Local<f32>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    *attack_timer -= dt;
    if *attack_timer > 0.0 {
        return;
    }
    *attack_timer = 2.0; // Attack every 2 seconds

    for (building, tower_transform) in towers.iter() {
        if building.building_type != BuildingType::GuardTower || building.is_destroyed {
            continue;
        }

        let tower_pos = Vec2::new(tower_transform.translation.x, tower_transform.translation.y);
        let range = 150.0 + building.tier as f32 * 50.0;
        let damage = 15.0 + building.tier as f32 * 10.0;

        // Find and damage nearest enemy in range
        for (mut enemy_stats, enemy_transform) in enemies.iter_mut() {
            let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
            let dist = (enemy_pos - tower_pos).length();

            if dist < range && enemy_stats.hp > 0.0 {
                let actual_damage = (damage - enemy_stats.defense).max(1.0);
                enemy_stats.hp -= actual_damage;
                break; // One target per tower per attack
            }
        }
    }
}

/// Startup system: Place the initial Town Hall
pub fn spawn_initial_buildings(
    mut commands: Commands,
) {
    // Town Hall - always at center
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: BuildingType::TownHall.color(),
            custom_size: Some(BuildingType::TownHall.size()),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        ..Default::default()
    })
    .insert(Building::new(BuildingType::TownHall));

    // Start with an Inn nearby
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: BuildingType::Inn.color(),
            custom_size: Some(BuildingType::Inn.size()),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(120.0, 30.0, 5.0)),
        ..Default::default()
    })
    .insert(Building::new(BuildingType::Inn));

    // And a Market
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: BuildingType::Market.color(),
            custom_size: Some(BuildingType::Market.size()),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(-100.0, 50.0, 5.0)),
        ..Default::default()
    })
    .insert(Building::new(BuildingType::Market));
}
