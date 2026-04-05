use bevy::prelude::*;
use crate::components::*;
use crate::camera::cursor_to_world_2d;
use crate::sprites::{SpriteAssets, spawn_building_with_sprite};

/// System: Handle building placement from build mode
pub fn building_placement_system(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &Transform, &OrthographicProjection), With<MainCamera>>,
    mut economy: ResMut<GameEconomy>,
    mut game_phase: ResMut<GamePhase>,
    kingdom: Res<KingdomState>,
    buildings: Query<(&Building, &Transform)>,
    sprites: Res<SpriteAssets>,
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
        let window = match windows.get_primary() {
            Some(w) => w,
            None => return,
        };
        if let Ok((_camera, camera_transform, projection)) = camera.get_single() {
            let world_pos = match cursor_to_world_2d(window, camera_transform, projection) {
                Some(pos) => pos,
                None => return,
            };

            let cost = selected.cost();
            if economy.gold < cost {
                alerts.push(format!("Not enough gold! Need {:.0}", cost));
                return;
            }

            if !kingdom.rank.available_buildings().contains(&selected) {
                alerts.push("Building not available at current rank!".to_string());
                return;
            }

            // Overlap check: prevent placing too close to existing buildings
            const BUILDING_MIN_SPACING: f32 = 50.0;
            for (_, transform) in buildings.iter() {
                let existing_pos = Vec2::new(transform.translation.x, transform.translation.y);
                if (existing_pos - world_pos).length() < BUILDING_MIN_SPACING {
                    alerts.push("Building too close to another structure!".to_string());
                    return;
                }
            }

            economy.gold -= cost;
            economy.total_spent += cost;

            spawn_building_with_sprite(
                &mut commands,
                &sprites,
                selected,
                Vec3::new(world_pos.x, world_pos.y, 5.0),
            );

            alerts.push(format!("{} built for {:.0} gold!", selected.display_name(), cost));

            game_phase.build_mode = false;
            game_phase.selected_building = None;
        }
    }

    if mouse_input.just_pressed(MouseButton::Right) {
        game_phase.build_mode = false;
        game_phase.selected_building = None;
    }
}

/// System: Building upgrade on key press (U key) - keyboard fallback
pub fn building_upgrade_system(
    keyboard: Res<Input<KeyCode>>,
    _mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &Transform, &OrthographicProjection), With<MainCamera>>,
    mut buildings: Query<(&mut Building, &Transform), Without<Camera>>,
    mut economy: ResMut<GameEconomy>,
    game_phase: Res<GamePhase>,
    mut alerts: ResMut<GameAlerts>,
) {
    if game_phase.build_mode {
        return;
    }

    if !keyboard.just_pressed(KeyCode::U) {
        return;
    }

    let window = match windows.get_primary() {
        Some(w) => w,
        None => return,
    };
    if let Ok((_camera, camera_transform, projection)) = camera.get_single() {
        let world_pos = match cursor_to_world_2d(window, camera_transform, projection) {
            Some(pos) => pos,
            None => return,
        };

        for (mut building, transform) in buildings.iter_mut() {
            let pos = Vec2::new(transform.translation.x, transform.translation.y);
            let dist = (pos - world_pos).length();

            if dist < 80.0 && building.tier < 3 && !building.is_destroyed {
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
    mut buildings: Query<(&mut Building, &mut Visibility)>,
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

    for (mut building, mut vis) in buildings.iter_mut() {
        if building.is_destroyed {
            vis.is_visible = false; // Hide destroyed buildings
            let repair_cost = building.building_type.cost() * 0.5;
            if economy.gold >= repair_cost {
                economy.gold -= repair_cost;
                economy.total_spent += repair_cost;
                building.is_destroyed = false;
                building.hp = building.max_hp * 0.5;
                vis.is_visible = true;
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
    *attack_timer = 2.0;

    for (building, tower_transform) in towers.iter() {
        if building.building_type != BuildingType::GuardTower || building.is_destroyed {
            continue;
        }

        let tower_pos = Vec2::new(tower_transform.translation.x, tower_transform.translation.y);
        let range = 150.0 + building.tier as f32 * 50.0;
        let damage = 15.0 + building.tier as f32 * 10.0;

        for (mut enemy_stats, enemy_transform) in enemies.iter_mut() {
            let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
            let dist = (enemy_pos - tower_pos).length();

            if dist < range && enemy_stats.hp > 0.0 {
                let actual_damage = (damage - enemy_stats.defense).max(1.0);
                enemy_stats.hp -= actual_damage;
                break;
            }
        }
    }
}

/// Startup system: Place initial buildings using real sprites
pub fn spawn_initial_buildings(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
) {
    // Town Hall at center
    spawn_building_with_sprite(&mut commands, &sprites, BuildingType::TownHall, Vec3::new(0.0, 0.0, 5.0));
    // Inn nearby
    spawn_building_with_sprite(&mut commands, &sprites, BuildingType::Inn, Vec3::new(150.0, 30.0, 5.0));
    // Market
    spawn_building_with_sprite(&mut commands, &sprites, BuildingType::Market, Vec3::new(-130.0, 50.0, 5.0));
}
