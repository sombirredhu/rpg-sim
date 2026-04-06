//! Mouse interaction systems: HUD button clicks, camera drag, entity inspect,
//! building selection/upgrade, road tool toggle, map expand.

use bevy::prelude::*;
use crate::components::*;
use crate::camera::cursor_to_world_2d;
use crate::sprites::SpriteAssets;
use std::f32::consts::TAU;
use rand::random;

// ============================================================
// Camera drag — hold right mouse button and drag to pan
// ============================================================

pub fn camera_drag_system(
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    game_phase: Res<GamePhase>,
) {
    if !game_phase.game_started || game_phase.build_mode || game_phase.show_build_menu || game_phase.bounty_board_open {
        for _ in mouse_motion.iter() {}
        return;
    }

    if mouse_input.pressed(MouseButton::Right) {
        for (mut transform, projection) in camera.iter_mut() {
            for ev in mouse_motion.iter() {
                // Drag right = camera moves left (pan follows cursor)
                transform.translation.x -= ev.delta.x * projection.scale;
                transform.translation.y += ev.delta.y * projection.scale;
            }
        }
    } else {
        for _ in mouse_motion.iter() {}
    }
}

// ============================================================
// HUD button click handlers
// ============================================================

pub fn speed_button_click(
    speed_btn: Query<&Interaction, With<SpeedButton>>,
    mut game_time: ResMut<GameTime>,
    game_phase: Res<GamePhase>,
    mut alerts: ResMut<GameAlerts>,
) {
    if !game_phase.game_started { return; }
    for interaction in speed_btn.iter() {
        if matches!(interaction, Interaction::Clicked) {
            game_time.speed_multiplier = match game_time.speed_multiplier as u32 {
                1 => 2.0,
                2 => 3.0,
                _ => 1.0,
            };
            game_time.is_paused = false;
            alerts.push(format!("Speed: {}x", game_time.speed_multiplier as u32));
        }
    }
}

pub fn pause_button_click(
    pause_btn: Query<&Interaction, With<PauseButton>>,
    mut game_time: ResMut<GameTime>,
    game_phase: Res<GamePhase>,
    mut alerts: ResMut<GameAlerts>,
) {
    if !game_phase.game_started { return; }
    for interaction in pause_btn.iter() {
        if matches!(interaction, Interaction::Clicked) {
            game_time.is_paused = !game_time.is_paused;
            if game_time.is_paused {
                alerts.push("PAUSED".to_string());
            } else {
                alerts.push("Resumed".to_string());
            }
        }
    }
}

pub fn build_button_click(
    build_btn: Query<&Interaction, With<BuildButton>>,
    mut game_phase: ResMut<GamePhase>,
    kingdom: Res<KingdomState>,
    mut alerts: ResMut<GameAlerts>,
) {
    if !game_phase.game_started { return; }
    for interaction in build_btn.iter() {
        if matches!(interaction, Interaction::Clicked) {
            if game_phase.build_mode {
                game_phase.build_mode = false;
                game_phase.selected_building = None;
                alerts.push("Build mode OFF".to_string());
            } else {
                game_phase.bounty_board_open = false;
                game_phase.road_tool_active = false;
                game_phase.show_build_menu = !game_phase.show_build_menu;
                if game_phase.show_build_menu {
                    let available = kingdom.rank.available_buildings();
                    let mut msg = "BUILD MENU:\n".to_string();
                    for (i, bt) in available.iter().enumerate() {
                        msg.push_str(&format!("  {}: {} ({:.0}g)\n", i + 1, bt.display_name(), bt.cost()));
                    }
                    msg.push_str("Press number or click building name to select");
                    alerts.push(msg);
                } else {
                    alerts.push("Build menu closed".to_string());
                }
            }
        }
    }
}

pub fn bounty_button_click(
    bounty_btn: Query<&Interaction, With<BountyButton>>,
    mut game_phase: ResMut<GamePhase>,
    mut alerts: ResMut<GameAlerts>,
) {
    if !game_phase.game_started { return; }
    for interaction in bounty_btn.iter() {
        if matches!(interaction, Interaction::Clicked) {
            game_phase.bounty_board_open = !game_phase.bounty_board_open;
            if game_phase.bounty_board_open {
                game_phase.build_mode = false;
                game_phase.show_build_menu = false;
                game_phase.selected_building = None;
                game_phase.road_tool_active = false;
                alerts.push(format!(
                    "Bounty Board open - Up/Down to adjust ({:.0}g) - Click to place",
                    game_phase.manual_bounty_amount
                ));
            } else {
                alerts.push("Bounty Board closed".to_string());
            }
        }
    }
}

pub fn expand_button_click(
    expand_btn: Query<&Interaction, With<ExpandButton>>,
    mut fog: ResMut<FogOfWar>,
    mut economy: ResMut<GameEconomy>,
    kingdom: Res<KingdomState>,
    mut alerts: ResMut<GameAlerts>,
    game_phase: Res<GamePhase>,
    mut commands: Commands,
    _sprites: Res<SpriteAssets>,
    fog_tiles: Query<(Entity, &Transform), With<FogTile>>,
) {
    if !game_phase.game_started { return; }
    for interaction in expand_btn.iter() {
        if matches!(interaction, Interaction::Clicked) {
            if game_phase.build_mode || game_phase.bounty_board_open {
                return;
            }
            let max = kingdom.rank.max_expansions();
            if fog.expansions >= max {
                alerts.push(format!(
                    "Cannot expand further at {} rank!",
                    kingdom.rank.display_name()
                ));
                return;
            }

            let cost = KingdomRank::expansion_cost(fog.expansions);
            if economy.gold < cost {
                alerts.push(format!("Not enough gold to expand! Need {:.0}g", cost));
                return;
            }

            economy.gold -= cost;
            economy.total_spent += cost;

            let old_radius = fog.revealed_radius;
            fog.expansions += 1;
            fog.revealed_radius += 100.0;
            let new_radius = fog.revealed_radius;

            // Remove fog tiles within new radius
            let tiles_to_remove: Vec<Entity> = fog_tiles.iter()
                .filter(|(_, t)| {
                    let pos = Vec2::new(t.translation.x, t.translation.y);
                    pos.length() < new_radius
                })
                .map(|(e, _)| e)
                .collect();
            for e in tiles_to_remove {
                commands.entity(e).despawn();
            }

            // Spawn new monster den in expanded zone
            let den_angle = random::<f32>() * TAU;
            let den_radius = old_radius + 50.0 + random::<f32>() * 40.0;
            let den_pos = Vec2::new(den_angle.cos() * den_radius, den_angle.sin() * den_radius);

            let den_type = match fog.expansions {
                1..=2 => EnemyType::Goblin,
                3 => EnemyType::Bandit,
                _ => if random::<bool>() { EnemyType::Bandit } else { EnemyType::Troll },
            };

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.6, 0.3, 0.6),
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(den_pos.x, den_pos.y, 5.0)),
                ..Default::default()
            })
            .insert(MonsterDen::new(den_type));

            alerts.push(format!(
                "Map expanded! New zone revealed. Spawned {} den for {:.0}g",
                den_type.display_name(), cost
            ));
        }
    }
}

pub fn road_tool_button_click(
    road_btn: Query<&Interaction, With<RoadToolButton>>,
    mut game_phase: ResMut<GamePhase>,
    mut alerts: ResMut<GameAlerts>,
) {
    if !game_phase.game_started { return; }
    for interaction in road_btn.iter() {
        if matches!(interaction, Interaction::Clicked) {
            if game_phase.build_mode || game_phase.show_build_menu || game_phase.bounty_board_open {
                return;
            }
            game_phase.road_tool_active = !game_phase.road_tool_active;
            if game_phase.road_tool_active {
                alerts.push("Road Tool ON — Left-click to paint roads, 5g per tile".to_string());
            } else {
                alerts.push("Road Tool OFF".to_string());
            }
        }
    }
}

// ============================================================
// Map click — inspect entities, select buildings, place bounties/buildings
// ============================================================

pub fn map_click_system(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &Transform, &OrthographicProjection), With<MainCamera>>,
    mut game_phase: ResMut<GamePhase>,
    heroes: Query<(Entity, &Hero, &HeroStats, &HeroEquipment, &HeroState, &Transform), Without<Building>>,
    buildings: Query<(Entity, &Building, &Transform), Without<Hero>>,
    enemies: Query<(Entity, &Enemy, &EnemyStats, &Transform), (Without<Hero>, Without<Building>)>,
    mut alerts: ResMut<GameAlerts>,
    mut selected_building: ResMut<SelectedBuilding>,
    mut selected_building_info: ResMut<SelectedBuildingInfo>,
    mut commands: Commands,
    highlights: Query<(Entity, &BuildingHighlight)>,
    mut building_info_ui: Query<&mut Visibility, With<BuildingInfoUi>>,
    _economy: ResMut<GameEconomy>,
    button_interactions: Query<&Interaction, (With<Button>, Changed<Interaction>)>,
) {
    // Handle right-click for building info
    if mouse_input.just_pressed(MouseButton::Right) {
        if !game_phase.game_started {
            return;
        }

        // Don't show info if in build mode or menus open
        if game_phase.build_mode || game_phase.show_build_menu || game_phase.bounty_board_open {
            return;
        }

        let window = match windows.get_primary() {
            Some(w) => w,
            None => return,
        };
        if let Ok((_cam, cam_t, projection)) = camera.get_single() {
            let world = match cursor_to_world_2d(window, cam_t, projection) {
                Some(pos) => pos,
                None => return,
            };

            // Check buildings for info panel
            for (entity, _building, t) in buildings.iter() {
                let pos = Vec2::new(t.translation.x, t.translation.y);
                if (pos - world).length() < 50.0 {
                    // Show building info panel
                    selected_building_info.entity = Some(entity);

                    // Show the building info UI
                    for mut vis in building_info_ui.iter_mut() {
                        vis.is_visible = true;
                    }
                    return;
                }
            }

            // Clicked on empty space - hide info panel
            selected_building_info.entity = None;
            for mut vis in building_info_ui.iter_mut() {
                vis.is_visible = false;
            }
        }
        return;
    }

    // Handle left-click for building placement/menu
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    if !game_phase.game_started {
        return;
    }

    // Prevent map interaction when clicking HUD buttons
    if button_interactions.iter().any(|&i| i == Interaction::Clicked) {
        return;
    }

    // If build menu is open, ignore world clicks (selection done via UI buttons)
    if game_phase.show_build_menu {
        return;
    }

    // Don't inspect if in build mode or bounty board open — those handle their own clicks
    if game_phase.build_mode || game_phase.bounty_board_open {
        return;
    }

    let window = match windows.get_primary() {
        Some(w) => w,
        None => return,
    };
    if let Ok((_cam, cam_t, projection)) = camera.get_single() {
        let world = match cursor_to_world_2d(window, cam_t, projection) {
            Some(pos) => pos,
            None => return,
        };

        // Check heroes first
        for (_e, hero, stats, equipment, state, t) in heroes.iter() {
            let pos = Vec2::new(t.translation.x, t.translation.y);
            if (pos - world).length() < 25.0 {
                let state_str = match state {
                    HeroState::Idle => "Idle",
                    HeroState::MovingTo { .. } => "Moving",
                    HeroState::AttackingEnemy { .. } => "Fighting",
                    HeroState::PursuingBounty { .. } => "On Bounty",
                    HeroState::Resting => "Resting",
                    HeroState::Shopping => "Shopping",
                    HeroState::Dead { .. } => "Dead",
                    HeroState::Casting { .. } => "Casting",
                };
                let leg = if hero.is_legendary { " [LEGENDARY]" } else { "" };
                let equip_atk = equipment.total_atk_bonus();
                let equip_def = equipment.total_def_bonus();
                alerts.push(format!(
                    "{}{} Lv{} | HP:{:.0}/{:.0} ATK:{:.0} DEF:{:.0} SPD:{:.0} | {} | {:?} | Morale:{:.0}",
                    hero.class.display_name(), leg, hero.level,
                    stats.hp, stats.max_hp, stats.attack + equip_atk, stats.defense + equip_def, stats.speed,
                    state_str, hero.personality, hero.morale
                ));
                return;
            }
        }

        // Check enemies
        for (_e, enemy, enemy_stats, t) in enemies.iter() {
            let pos = Vec2::new(t.translation.x, t.translation.y);
            if (pos - world).length() < 25.0 {
                let threat_str = match enemy_stats.threat_level {
                    0..=2 => "Minor",
                    3..=4 => "Moderate",
                    5..=7 => "Severe",
                    _ => "Critical",
                };
                alerts.push(format!(
                    "{} | HP:{:.0}/{:.0} ATK:{:.0} DEF:{:.0} | Threat: {} ({})",
                    enemy.enemy_type.display_name(), enemy_stats.hp, enemy_stats.max_hp,
                    enemy_stats.attack, enemy_stats.defense,
                    enemy_stats.threat_level, threat_str
                ));
                return;
            }
        }

        // Check buildings — select or inspect
        for (entity, building, t) in buildings.iter() {
            let pos = Vec2::new(t.translation.x, t.translation.y);
            if (pos - world).length() < 50.0 {
                // Remove old highlight
                for (h_entity, _) in highlights.iter() {
                    commands.entity(h_entity).despawn();
                }

                // Create highlight ring
                commands.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.2, 0.8, 0.2, 0.4),
                        custom_size: Some(Vec2::new(60.0, 60.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 10.0)),
                    ..Default::default()
                })
                .insert(BuildingHighlight);

                selected_building.entity = Some(entity);

                let status = if building.is_destroyed { "DESTROYED" } else { "Active" };
                let upgrade_cost = if building.tier < 3 && !building.is_destroyed {
                    Some(building.building_type.upgrade_cost(building.tier + 1))
                } else {
                    None
                };

                let mut msg = format!(
                    "{} Tier {} | HP:{:.0}/{:.0} | {}\n Tax:{:.1}/min",
                    building.building_type.display_name(), building.tier,
                    building.hp, building.max_hp, status,
                    building.building_type.tax_income(building.tier)
                );

                if let Some(cost) = upgrade_cost {
                    msg.push_str(&format!("\n Upgrade to T{}: {:.0}g (click again to upgrade)", building.tier + 1, cost));
                }
                alerts.push(msg);
                return;
            }
        }

        // Clicked on empty space — deselect building
        for (h_entity, _) in highlights.iter() {
            commands.entity(h_entity).despawn();
        }
        selected_building.entity = None;
    }
}

// ============================================================
// Selected building action — click on already selected building to upgrade
// ============================================================

pub fn selected_building_action(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &Transform, &OrthographicProjection), With<MainCamera>>,
    selected_building: Res<SelectedBuilding>,
    mut buildings: Query<(&mut Building, &mut Visibility, &Transform)>,
    mut economy: ResMut<GameEconomy>,
    mut alerts: ResMut<GameAlerts>,
    game_phase: Res<GamePhase>,
) {
    if !game_phase.game_started || !mouse_input.just_pressed(MouseButton::Left) || game_phase.build_mode {
        return;
    }

    let entity = match selected_building.entity {
        Some(e) => e,
        None => return,
    };

    let window = match windows.get_primary() {
        Some(w) => w,
        None => return,
    };
    if let Ok((_cam, cam_t, projection)) = camera.get_single() {
        let world = match cursor_to_world_2d(window, cam_t, projection) {
            Some(pos) => pos,
            None => return,
        };

        if let Ok((mut building, mut visibility, transform)) = buildings.get_mut(entity) {
            let pos = Vec2::new(transform.translation.x, transform.translation.y);
            if (pos - world).length() < 50.0 {
                if building.is_destroyed {
                    // Repair destroyed building
                    let repair_cost = building.building_type.cost() * 0.5;
                    if economy.gold >= repair_cost {
                        economy.gold -= repair_cost;
                        economy.total_spent += repair_cost;
                        building.is_destroyed = false;
                        building.hp = building.max_hp * 0.5;
                        visibility.is_visible = true; // Show building immediately when repaired
                        alerts.push(format!(
                            "{} repaired for {:.0} gold!",
                            building.building_type.display_name(),
                            repair_cost
                        ));
                    } else {
                        alerts.push(format!("Need {:.0} gold to repair!", repair_cost));
                    }
                } else if building.tier < 3 && !building.is_destroyed {
                    // Upgrade building
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
                }
            }
        }
    }
}
