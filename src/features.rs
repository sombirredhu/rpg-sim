//! All missing GDD features: roads, resources, merchants, night enemies,
//! building bonuses, milestones, era siege, fog of war, inspect, animations,
//! hero idle/bankruptcy, recovery bounties, etc.

use bevy::prelude::*;
use crate::components::*;
use crate::camera::cursor_to_world_2d;
use crate::sprites::{
    SpriteAssets,
    monster_den_scale_for_tier,
    monster_den_texture_for_tier,
    spawn_enemy_with_sprite,
};
use crate::map_layout::TILE_SIZE;
use std::collections::HashSet;
use std::collections::HashMap;
use std::f32::consts::TAU;

// ================================================================
// 1. ROAD NETWORK — R key to place, 30% speed boost
// ================================================================

pub fn road_placement_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &Transform, &OrthographicProjection), With<MainCamera>>,
    mut economy: ResMut<GameEconomy>,
    mut road_network: ResMut<RoadNetwork>,
    game_phase: Res<GamePhase>,
    sprites: Res<SpriteAssets>,
    mut _alerts: ResMut<GameAlerts>,
) {
    // Place roads while holding R key OR road tool is active
    let tool_active = keyboard.pressed(KeyCode::R) || game_phase.road_tool_active;
    if game_phase.build_mode || game_phase.show_build_menu || game_phase.bounty_board_open || !tool_active {
        return;
    }
    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }

    let window = match windows.get_primary() {
        Some(w) => w,
        None => return,
    };
    if let Ok((_cam, cam_t, projection)) = camera.get_single() {
        let world_pos = match cursor_to_world_2d(window, cam_t, projection) {
            Some(pos) => pos,
            None => return,
        };

        // Snap to tile grid for neat road alignment
        let snapped_pos = Vec2::new(
            (world_pos.x / TILE_SIZE).round() * TILE_SIZE,
            (world_pos.y / TILE_SIZE).round() * TILE_SIZE,
        );

        // Don't place if too close to existing road tile
        if road_network.is_on_road(snapped_pos) {
            return;
        }

        let cost = 5.0;
        if economy.gold < cost {
            return;
        }
        economy.gold -= cost;
        economy.total_spent += cost;

        road_network.tiles.push(snapped_pos);

        // Use the grassland stone road texture instead of plain color
        commands.spawn_bundle(SpriteBundle {
            texture: sprites.road_stone_tex.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(12.0, 12.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(snapped_pos.x, snapped_pos.y, 1.0)),
            ..Default::default()
        })
        .insert(Road);
    }
}

// ================================================================
// 1b. ROAD CONNECTION BONUSES — buildings connected by roads get bonuses
// ================================================================

/// Recalculates road connection bonuses every few seconds.
/// Market connected to other buildings via roads → tax bonus (+10% per connection).
/// Blacksmith connected via roads → craft/ATK bonus (+15% per connection).
pub fn road_connection_bonus_system(
    road_network: Res<RoadNetwork>,
    buildings: Query<(&Building, &Transform)>,
    mut bonuses: ResMut<BuildingBonuses>,
    time: Res<Time>,
    mut timer: Local<f32>,
) {
    // Only recalculate every 3 seconds to avoid performance issues with BFS
    *timer -= time.delta_seconds();
    if *timer > 0.0 { return; }
    *timer = 3.0;

    let connection_radius = 50.0; // Building must be within 50px of a road tile

    // Collect building positions and types
    let building_data: Vec<(BuildingType, Vec2)> = buildings.iter()
        .filter(|(b, _)| !b.is_destroyed)
        .map(|(b, t)| (b.building_type, Vec2::new(t.translation.x, t.translation.y)))
        .collect();

    let mut tax_bonus = 0.0_f32;
    let mut craft_bonus = 0.0_f32;
    let mut connected_pairs = 0_u32;

    // Check each pair of buildings for road connectivity
    for i in 0..building_data.len() {
        for j in (i + 1)..building_data.len() {
            let (type_a, pos_a) = building_data[i];
            let (type_b, pos_b) = building_data[j];

            // Only check pairs where at least one is Market or Blacksmith
            let has_market = type_a == BuildingType::Market || type_b == BuildingType::Market;
            let has_blacksmith = type_a == BuildingType::Blacksmith || type_b == BuildingType::Blacksmith;

            if !has_market && !has_blacksmith { continue; }

            if road_network.are_connected(pos_a, pos_b, connection_radius) {
                connected_pairs += 1;

                // Market road connections: +10% tax per connected building
                if has_market {
                    tax_bonus += 10.0;
                }

                // Blacksmith road connections: +15% craft/ATK per connected building
                if has_blacksmith {
                    craft_bonus += 15.0;
                }
            }
        }
    }

    bonuses.road_tax_bonus_pct = tax_bonus;
    bonuses.road_craft_bonus_pct = craft_bonus;
    bonuses.road_connected_pairs = connected_pairs;
}

// ================================================================
// 2. DESTROYABLE MONSTER DENS — heroes attack dens near bounties
// ================================================================

pub fn den_destruction_system(
    mut commands: Commands,
    mut dens: Query<(Entity, &mut MonsterDen, &Transform)>,
    heroes: Query<(&Hero, &HeroStats, &HeroState, &Transform)>,
    game_time: Res<GameTime>,
    time: Res<Time>,
    mut economy: ResMut<GameEconomy>,
    mut alerts: ResMut<GameAlerts>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 { return; }

    for (den_entity, mut den, den_transform) in dens.iter_mut() {
        let den_pos = Vec2::new(den_transform.translation.x, den_transform.translation.y);

        // Heroes near dens damage them
        for (_hero, stats, state, hero_t) in heroes.iter() {
            let hero_pos = Vec2::new(hero_t.translation.x, hero_t.translation.y);
            let dist = (hero_pos - den_pos).length();

            if dist < stats.attack_range + 20.0 {
                // Hero is fighting near den — den takes chip damage
                if matches!(state, HeroState::AttackingEnemy { .. } | HeroState::PursuingBounty { .. }) {
                    den.hp -= stats.attack * 0.3 * dt;
                }
            }
        }

        if den.hp <= 0.0 {
            let reward = 50.0 * den.threat_tier as f32;
            economy.gold += reward;
            economy.total_earned += reward;
            alerts.push(format!("Monster den destroyed! +{:.0} gold", reward));
            commands.entity(den_entity).despawn();
        }
    }
}

/// New monster dens spawn at map edges periodically
pub fn new_den_spawn_system(
    mut commands: Commands,
    dens: Query<&MonsterDen>,
    game_time: Res<GameTime>,
    kingdom: Res<KingdomState>,
    sprites: Res<SpriteAssets>,
    time: Res<Time>,
    mut timer: Local<f32>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    *timer -= dt;
    if *timer > 0.0 { return; }
    *timer = game_time.day_length * 3.0; // Every 3 game-days

    let den_count = dens.iter().count();
    let max_dens = match kingdom.rank {
        KingdomRank::Hamlet => 4,
        KingdomRank::Village => 5,
        KingdomRank::Town => 6,
        KingdomRank::City => 8,
        KingdomRank::Kingdom => 10,
    };

    if den_count >= max_dens { return; }

    let angle = rand::random::<f32>() * TAU;
    let radius = 350.0 + rand::random::<f32>() * 200.0;
    let pos = Vec2::new(angle.cos() * radius, angle.sin() * radius);

    let enemy_type = if rand::random::<f32>() < 0.5 { EnemyType::Goblin } else { EnemyType::Bandit };

    let den = MonsterDen::new(enemy_type);
    let tier = den.threat_tier;

    commands.spawn_bundle(SpriteBundle {
        texture: monster_den_texture_for_tier(&sprites, tier),
        transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 4.0))
            .with_scale(Vec3::splat(monster_den_scale_for_tier(tier))),
        ..Default::default()
    })
    .insert(den)
    .insert(MonsterDenVisualTier { tier });
}

// ================================================================
// 3. NIGHT-ONLY ENEMIES — werewolves & shadow bandits
// ================================================================

pub fn night_enemy_spawn_system(
    mut commands: Commands,
    game_time: Res<GameTime>,
    sprites: Res<SpriteAssets>,
    time: Res<Time>,
    mut timer: Local<f32>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    *timer -= dt;
    if *timer > 0.0 { return; }
    *timer = 20.0; // Check every 20 seconds

    if !game_time.is_night() { return; }

    // Spawn 1-2 night creatures at random positions around the map
    let count = 1 + (rand::random::<f32>() * 1.5) as u32;
    for _ in 0..count {
        let angle = rand::random::<f32>() * TAU;
        let radius = 300.0 + rand::random::<f32>() * 200.0;
        let pos = Vec3::new(angle.cos() * radius, angle.sin() * radius, 8.0);

        let enemy_type = if rand::random::<bool>() {
            EnemyType::Werewolf
        } else {
            EnemyType::ShadowBandit
        };

        spawn_enemy_with_sprite(&mut commands, &sprites, enemy_type, pos);
    }
}

/// Despawn night-only enemies when day comes
pub fn night_enemy_despawn_system(
    mut commands: Commands,
    enemies: Query<(Entity, &Enemy)>,
    game_time: Res<GameTime>,
) {
    if game_time.is_night() { return; }

    for (entity, enemy) in enemies.iter() {
        if enemy.enemy_type.is_night_only() {
            commands.entity(entity).despawn();
        }
    }
}

// ================================================================
// 4. MERCHANT CARAVANS
// ================================================================

pub fn merchant_spawn_system(
    mut commands: Commands,
    game_time: Res<GameTime>,
    kingdom: Res<KingdomState>,
    time: Res<Time>,
    mut timer: Local<f32>,
    mut alerts: ResMut<GameAlerts>,
) {
    if game_time.is_night() { return; } // Merchants only during day
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    *timer -= dt;
    if *timer > 0.0 { return; }
    *timer = game_time.day_length * 0.5; // Twice per game-day

    let trade_value = 20.0 + (kingdom.rank as u32 as f32) * 15.0;

    let angle = rand::random::<f32>() * TAU;
    let start = Vec2::new(angle.cos() * 500.0, angle.sin() * 500.0);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.9, 0.8, 0.3),
            custom_size: Some(Vec2::new(12.0, 12.0)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(start.x, start.y, 9.0)),
        ..Default::default()
    })
    .insert(Merchant {
        gold_value: trade_value,
        destination: Vec2::ZERO, // Move toward town center
        has_arrived: false,
        leave_timer: 15.0,
    });

    alerts.push(format!("A merchant caravan approaches! ({:.0}g trade)", trade_value));
}

pub fn merchant_movement_system(
    mut commands: Commands,
    mut merchants: Query<(Entity, &mut Merchant, &mut Transform)>,
    mut economy: ResMut<GameEconomy>,
    bonuses: Res<BuildingBonuses>,
    game_time: Res<GameTime>,
    time: Res<Time>,
    mut alerts: ResMut<GameAlerts>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 { return; }

    for (entity, mut merchant, mut transform) in merchants.iter_mut() {
        let pos = Vec2::new(transform.translation.x, transform.translation.y);

        if !merchant.has_arrived {
            let dir = (merchant.destination - pos).normalize_or_zero();
            transform.translation.x += dir.x * 40.0 * dt;
            transform.translation.y += dir.y * 40.0 * dt;

            if (merchant.destination - pos).length() < 30.0 {
                merchant.has_arrived = true;
                let income = merchant.gold_value + bonuses.market_trade_bonus;
                economy.gold += income;
                economy.total_earned += income;
                economy.total_merchant_trade_earned += income;
                alerts.push(format!("Merchant traded! +{:.0} gold", income));
            }
        } else {
            merchant.leave_timer -= dt;
            if merchant.leave_timer <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

// ================================================================
// 4b. TRADE CARAVANS — Market Tier 2+ rare item caravans
// ================================================================

/// Spawns trade caravans with rare items when Market is Tier 2+
pub fn trade_caravan_spawn_system(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    game_time: Res<GameTime>,
    buildings: Query<&Building>,
    economy: Res<GameEconomy>,
    time: Res<Time>,
    mut timer: Local<f32>,
    mut alerts: ResMut<GameAlerts>,
) {
    if game_time.is_night() { return; } // Caravans only during day

    // Check if any Market is Tier 2+
    let has_market_t2 = buildings.iter().any(|b| {
        b.building_type == BuildingType::Market && b.tier >= 2 && !b.is_destroyed
    });
    if !has_market_t2 { return; }

    let dt = time.delta_seconds() * game_time.speed_multiplier;
    *timer -= dt;
    if *timer > 0.0 { return; }
    *timer = game_time.day_length * 2.0; // Once every 2 game-days

    let item = RareItem::random();
    let cost = item.cost();

    // Don't spawn if player can't afford it
    if economy.gold < cost { return; }

    let angle = rand::random::<f32>() * TAU;
    let start = Vec2::new(angle.cos() * 550.0, angle.sin() * 550.0);

    commands.spawn_bundle(SpriteBundle {
        texture: sprites.caravan_sprites.lvl1.clone(),
        transform: Transform::from_translation(Vec3::new(start.x, start.y, 9.0))
            .with_scale(Vec3::splat(0.25)),
        ..Default::default()
    })
    .insert(TradeCaravan {
        item,
        destination: Vec2::ZERO,
        has_arrived: false,
        leave_timer: 20.0,
    });

    alerts.push(format!(
        "Trade caravan with {} approaching! (cost: {:.0}g)",
        item.display_name(),
        cost
    ));
}

/// Moves trade caravans toward town, applies rare item buff on arrival
pub fn trade_caravan_movement_system(
    mut commands: Commands,
    mut caravans: Query<(Entity, &mut TradeCaravan, &mut Transform)>,
    mut economy: ResMut<GameEconomy>,
    mut active_buffs: ResMut<ActiveBuffs>,
    mut heroes: Query<(&mut Hero, &mut HeroStats)>,
    game_time: Res<GameTime>,
    time: Res<Time>,
    mut alerts: ResMut<GameAlerts>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 { return; }

    for (entity, mut caravan, mut transform) in caravans.iter_mut() {
        let pos = Vec2::new(transform.translation.x, transform.translation.y);

        if !caravan.has_arrived {
            let dir = (caravan.destination - pos).normalize_or_zero();
            transform.translation.x += dir.x * 35.0 * dt;
            transform.translation.y += dir.y * 35.0 * dt;

            if (caravan.destination - pos).length() < 30.0 {
                caravan.has_arrived = true;
                let cost = caravan.item.cost();

                // Auto-purchase if player can afford it
                if economy.gold >= cost {
                    economy.gold -= cost;
                    economy.total_spent += cost;

                    // Apply the rare item effect
                    match caravan.item {
                        RareItem::EnchantedWeapons => {
                            active_buffs.atk_bonus = 5.0;
                            active_buffs.atk_timer = caravan.item.buff_duration();
                            alerts.push("Enchanted Weapons! All heroes +5 ATK for 2 days".into());
                        }
                        RareItem::BlessedArmor => {
                            active_buffs.def_bonus = 4.0;
                            active_buffs.def_timer = caravan.item.buff_duration();
                            alerts.push("Blessed Armor! All heroes +4 DEF for 2 days".into());
                        }
                        RareItem::HealingElixirs => {
                            for (_, mut stats) in heroes.iter_mut() {
                                stats.hp = (stats.hp + stats.max_hp * 0.3).min(stats.max_hp);
                            }
                            alerts.push("Healing Elixirs! All heroes restored 30% HP".into());
                        }
                        RareItem::SwiftBoots => {
                            active_buffs.speed_bonus = 0.15;
                            active_buffs.speed_timer = caravan.item.buff_duration();
                            alerts.push("Swift Boots! All heroes +15% speed for 2 days".into());
                        }
                        RareItem::MoraleBanner => {
                            for (mut hero, _) in heroes.iter_mut() {
                                hero.morale = (hero.morale + 20.0).min(100.0);
                            }
                            alerts.push("Morale Banner! All heroes +20 morale".into());
                        }
                    }
                } else {
                    alerts.push(format!(
                        "Can't afford {} ({:.0}g) — caravan leaving",
                        caravan.item.display_name(),
                        cost
                    ));
                }
            }
        } else {
            caravan.leave_timer -= dt;
            if caravan.leave_timer <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Tick down active buff timers and clear expired buffs
pub fn active_buffs_system(
    mut active_buffs: ResMut<ActiveBuffs>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 { return; }

    if active_buffs.atk_timer > 0.0 {
        active_buffs.atk_timer -= dt;
        if active_buffs.atk_timer <= 0.0 {
            active_buffs.atk_bonus = 0.0;
        }
    }
    if active_buffs.def_timer > 0.0 {
        active_buffs.def_timer -= dt;
        if active_buffs.def_timer <= 0.0 {
            active_buffs.def_bonus = 0.0;
        }
    }
    if active_buffs.speed_timer > 0.0 {
        active_buffs.speed_timer -= dt;
        if active_buffs.speed_timer <= 0.0 {
            active_buffs.speed_bonus = 0.0;
        }
    }
}

// ================================================================
// 5. RESOURCE NODES — mining & lumber with resource bounties
// ================================================================

pub fn resource_node_system(
    mut economy: ResMut<GameEconomy>,
    mut nodes: Query<(&mut ResourceNode, &Transform)>,
    heroes: Query<(&Hero, &Transform), Without<ResourceNode>>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 { return; }

    for (mut node, node_t) in nodes.iter_mut() {
        let node_pos = Vec2::new(node_t.translation.x, node_t.translation.y);

        // Check if any hero is nearby to keep it active
        let hero_nearby = heroes.iter().any(|(_, ht)| {
            let hp = Vec2::new(ht.translation.x, ht.translation.y);
            (hp - node_pos).length() < 60.0
        });

        node.is_active = hero_nearby;

        if node.is_active {
            node.gather_timer += dt;
            if node.gather_timer >= 5.0 {
                node.gather_timer = 0.0;
                let income = node.resource_type.income_per_tick();
                economy.gold += income;
                economy.total_earned += income;
            }
        }
    }
}

pub fn resource_bounty_system(
    mut bounty_board: ResMut<BountyBoard>,
    nodes: Query<(Entity, &ResourceNode, &Transform)>,
) {
    for (entity, node, transform) in nodes.iter() {
        if !node.is_active {
            let pos = Vec2::new(transform.translation.x, transform.translation.y);
            let has_bounty = bounty_board.bounties.iter().any(|b| {
                b.bounty_type == BountyType::Resource && b.target_entity == Some(entity) && !b.is_completed
            });
            if !has_bounty {
                bounty_board.add_bounty(BountyType::Resource, 15.0, pos, Some(entity), 1);
            }
        }
    }
}

pub fn spawn_resource_nodes(mut commands: Commands) {
    // Structured placement: mines in gold mine zones, lumber in forest zones.
    let nodes = [
        // Gold mines — NE and SW mine zones
        (Vec2::new(480.0, 280.0), ResourceType::Mine),
        (Vec2::new(-320.0, -280.0), ResourceType::Mine),
        // Lumber mills — NW and SE forest zones
        (Vec2::new(-450.0, 280.0), ResourceType::LumberMill),
        (Vec2::new(500.0, -320.0), ResourceType::LumberMill),
    ];

    for (pos, rtype) in nodes {
        let color = match rtype {
            ResourceType::Mine => Color::rgb(0.5, 0.4, 0.3),
            ResourceType::LumberMill => Color::rgb(0.3, 0.5, 0.2),
        };
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(24.0, 24.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 3.5)),
            ..Default::default()
        })
        .insert(ResourceNode::new(rtype));
    }
}

// ================================================================
// 6. BUILDING TIER ABILITIES — recalculate from active buildings
// ================================================================

pub fn building_bonuses_system(
    buildings: Query<&Building>,
    mut bonuses: ResMut<BuildingBonuses>,
) {
    // Preserve road connection bonuses (calculated separately on a timer)
    let road_tax = bonuses.road_tax_bonus_pct;
    let road_craft = bonuses.road_craft_bonus_pct;
    let road_pairs = bonuses.road_connected_pairs;

    // Reset
    *bonuses = BuildingBonuses::default();

    // Restore road bonuses
    bonuses.road_tax_bonus_pct = road_tax;
    bonuses.road_craft_bonus_pct = road_craft;
    bonuses.road_connected_pairs = road_pairs;

    for building in buildings.iter() {
        if building.is_destroyed { continue; }
        let tier = building.tier;

        match building.building_type {
            BuildingType::Inn => {
                if tier >= 1 { bonuses.inn_heal_speed = 1.5; }
                if tier >= 2 { bonuses.inn_heal_speed = 2.0; }
            }
            BuildingType::Market => {
                if tier >= 1 { bonuses.market_trade_bonus += 10.0; }
                if tier >= 2 { bonuses.market_trade_bonus += 20.0; } // Weekly caravans
            }
            BuildingType::Temple => {
                if tier >= 1 { bonuses.temple_morale_aura = 2.0; }
                if tier >= 2 { bonuses.temple_morale_aura = 5.0; }
                if tier >= 3 { bonuses.temple_pilgrim_income = 50.0; } // Cathedral
            }
            BuildingType::Blacksmith => {
                if tier >= 1 { bonuses.blacksmith_atk_bonus = 3.0; bonuses.blacksmith_def_bonus = 2.0; }
                if tier >= 2 { bonuses.blacksmith_atk_bonus = 6.0; bonuses.blacksmith_def_bonus = 4.0; }
            }
            BuildingType::Alchemist => {
                if tier >= 1 { bonuses.alchemist_recovery_speed = 1.5; }
                if tier >= 2 { bonuses.alchemist_recovery_speed = 2.0; }
            }
            BuildingType::Barracks => {
                if tier >= 1 { bonuses.barracks_hero_cap_bonus = 2; }
                if tier >= 2 { bonuses.barracks_hero_cap_bonus = 5; }
            }
            BuildingType::WizardTower => {
                if tier >= 1 { bonuses.wizard_research_bonus = 1.2; }
                if tier >= 2 { bonuses.wizard_research_bonus = 1.5; }
            }
            _ => {}
        }
    }

    // Apply road craft bonus to blacksmith stats
    if road_craft > 0.0 {
        let craft_mult = 1.0 + road_craft / 100.0;
        bonuses.blacksmith_atk_bonus *= craft_mult;
        bonuses.blacksmith_def_bonus *= craft_mult;
    }
}

// ================================================================
// 7. HEROES LEAVE IF IDLE + BANKRUPTCY
// ================================================================

pub fn hero_idle_leave_system(
    mut commands: Commands,
    heroes: Query<(Entity, &Hero, &HeroState)>,
    economy: Res<GameEconomy>,
    bounty_board: Res<BountyBoard>,
    game_time: Res<GameTime>,
    time: Res<Time>,
    mut idle_tracker: Local<Vec<(Entity, f32)>>,
    mut alerts: ResMut<GameAlerts>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;

    let no_bounties = bounty_board.available_bounties().is_empty();
    let bankrupt = economy.gold <= 0.0;

    for (entity, hero, state) in heroes.iter() {
        if matches!(state, HeroState::Dead { .. }) { continue; }

        let is_idle = matches!(state, HeroState::Idle) && no_bounties;
        let should_track = is_idle || (bankrupt && hero.morale < 20.0);

        if let Some(entry) = idle_tracker.iter_mut().find(|(e, _)| *e == entity) {
            if should_track {
                entry.1 += dt;
                if entry.1 > 120.0 { // 2 minutes idle = leave
                    alerts.push(format!("A {} left the kingdom!", hero.class.display_name()));
                    commands.entity(entity).despawn();
                }
            } else {
                entry.1 = 0.0;
            }
        } else if should_track {
            idle_tracker.push((entity, dt));
        }
    }

    // Clean up entries for despawned entities
    idle_tracker.retain(|(e, _)| heroes.get(*e).is_ok());
}

// ================================================================
// 8. MILESTONE REWARDS
// ================================================================

pub fn milestone_system(
    mut milestones: ResMut<Milestones>,
    mut economy: ResMut<GameEconomy>,
    kingdom: Res<KingdomState>,
    heroes: Query<&Hero>,
    dens: Query<&MonsterDen>,
    _buildings: Query<&Building>,
    mut alerts: ResMut<GameAlerts>,
) {
    // First den cleared
    if !milestones.cleared_first_den && dens.iter().count() < 4 {
        milestones.cleared_first_den = true;
        economy.gold += 100.0;
        economy.total_earned += 100.0;
        alerts.push("MILESTONE: First den cleared! +100 gold".to_string());
    }

    if !milestones.reached_village && kingdom.rank == KingdomRank::Village {
        milestones.reached_village = true;
        economy.gold += 150.0;
        economy.total_earned += 150.0;
        alerts.push("MILESTONE: Reached Village rank! +150 gold".to_string());
    }

    if !milestones.reached_town && kingdom.rank == KingdomRank::Town {
        milestones.reached_town = true;
        economy.gold += 250.0;
        economy.total_earned += 250.0;
        alerts.push("MILESTONE: Reached Town rank! +250 gold".to_string());
    }

    if !milestones.reached_city && kingdom.rank == KingdomRank::City {
        milestones.reached_city = true;
        economy.gold += 400.0;
        economy.total_earned += 400.0;
        alerts.push("MILESTONE: Reached City rank! +400 gold".to_string());
    }

    if !milestones.first_legendary_hero {
        if heroes.iter().any(|h| h.is_legendary) {
            milestones.first_legendary_hero = true;
            economy.gold += 200.0;
            economy.total_earned += 200.0;
            alerts.push("MILESTONE: First Legendary hero! +200 gold".to_string());
        }
    }

    if !milestones.ten_heroes && heroes.iter().count() >= 10 {
        milestones.ten_heroes = true;
        economy.gold += 150.0;
        economy.total_earned += 150.0;
        alerts.push("MILESTONE: 10 heroes! +150 gold".to_string());
    }
}

// ================================================================
// 9. RECOVERY BOUNTY + SIGNING BONUS
// ================================================================

pub fn recovery_bounty_system(
    mut bounty_board: ResMut<BountyBoard>,
    heroes: Query<(Entity, &Hero, &HeroState, &Transform)>,
) {
    for (entity, _hero, state, transform) in heroes.iter() {
        if let HeroState::Dead { .. } = state {
            let pos = Vec2::new(transform.translation.x, transform.translation.y);
            let has_bounty = bounty_board.bounties.iter().any(|b| {
                b.bounty_type == BountyType::Objective && b.target_entity == Some(entity) && !b.is_completed
            });
            if !has_bounty {
                bounty_board.add_bounty(BountyType::Objective, 20.0, pos, Some(entity), 1);
            }
        }
    }
}

// ================================================================
// 10. OBJECTIVE BOUNTIES — defend building missions
// ================================================================

pub fn objective_bounty_system(
    mut bounty_board: ResMut<BountyBoard>,
    buildings: Query<(Entity, &Building, &Transform)>,
    enemies: Query<(&Enemy, &Transform)>,
) {
    for (b_entity, building, b_transform) in buildings.iter() {
        if building.is_destroyed { continue; }
        let bpos = Vec2::new(b_transform.translation.x, b_transform.translation.y);

        // Check if enemies are near this building
        let threat_near = enemies.iter().any(|(_, et)| {
            let epos = Vec2::new(et.translation.x, et.translation.y);
            (epos - bpos).length() < 150.0
        });

        if threat_near {
            let has_bounty = bounty_board.bounties.iter().any(|b| {
                b.bounty_type == BountyType::Objective && b.target_entity == Some(b_entity) && !b.is_completed
            });
            if !has_bounty {
                let reward = 25.0 + building.building_type.cost() * 0.1;
                bounty_board.add_bounty(BountyType::Objective, reward, bpos, Some(b_entity), 2);
            }
        }
    }
}

// ================================================================
// 11. ERA FINAL SIEGE + LEGACY POINTS
// ================================================================

pub fn era_siege_system(
    mut commands: Commands,
    mut era: ResMut<EraState>,
    mut kingdom: ResMut<KingdomState>,
    game_time: Res<GameTime>,
    sprites: Res<SpriteAssets>,
    time: Res<Time>,
    mut alerts: ResMut<GameAlerts>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;

    // Check if era end is reached
    if !era.siege_active && kingdom.era_day >= era.era_length_days {
        era.siege_active = true;
        era.siege_waves_remaining = 5;
        era.siege_spawn_timer = 0.0;
        alerts.push("THE FINAL SIEGE BEGINS! Defend the kingdom!".to_string());
    }

    if !era.siege_active { return; }

    era.siege_spawn_timer -= dt;
    if era.siege_spawn_timer > 0.0 { return; }
    era.siege_spawn_timer = 15.0; // Wave every 15 seconds

    if era.siege_waves_remaining > 0 {
        era.siege_waves_remaining -= 1;

        // Spawn wave of enemies from all directions
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * TAU + rand::random::<f32>() * 0.5;
            let pos = Vec3::new(angle.cos() * 500.0, angle.sin() * 500.0, 8.0);

            let enemy_type = match era.siege_waves_remaining {
                0 => EnemyType::BossWarlord,
                1 => EnemyType::Troll,
                _ => if rand::random::<bool>() { EnemyType::GoblinElite } else { EnemyType::Bandit },
            };

            spawn_enemy_with_sprite(&mut commands, &sprites, enemy_type, pos);
        }

        alerts.push(format!("Siege wave {}! {} remaining",
            5 - era.siege_waves_remaining, era.siege_waves_remaining));
    } else {
        // Siege complete — award legacy points
        era.siege_active = false;
        let points = 10 + kingdom.score / 100;
        kingdom.legacy_points += points;
        kingdom.era += 1;
        alerts.push(format!("ERA COMPLETE! +{} Legacy Points! Starting Era {}", points, kingdom.era));

        // Reset for next era
        era.era_length_days = 45;
        era.siege_waves_remaining = 0;
    }
}

// ================================================================
// 12. TORCH / LIGHT DEFENSE BONUS AT NIGHT
// ================================================================

pub fn torch_defense_system(
    mut commands: Commands,
    buildings: Query<(Entity, &Building, &Transform)>,
    mut heroes: Query<(Entity, &mut HeroStats, &Transform), With<Hero>>,
    mut torch_halos: Query<(Entity, &mut TorchHalo, &mut Sprite)>,
    game_time: Res<GameTime>,
    mut buffed_heroes: Local<HashSet<Entity>>,
    time: Res<Time>,
) {
    const TORCH_DEFENSE_BONUS: f32 = 3.0;
    const TORCH_RADIUS: f32 = 80.0;

    // Target alpha based on time of day for smooth fade transitions
    let target_alpha = match game_time.time_of_day {
        TimeOfDay::Day => 0.0,
        TimeOfDay::Dawn => 0.05,
        TimeOfDay::Dusk => 0.10,
        TimeOfDay::Night => 0.15,
    };

    // Ensure every non-destroyed building has a torch halo child
    let existing_parents: HashSet<Entity> =
        torch_halos.iter().map(|(_, h, _)| h.parent_building).collect();

    // Spawn halos for buildings that don't have one yet
    for (building_entity, building, transform) in buildings.iter() {
        if building.is_destroyed || existing_parents.contains(&building_entity) {
            continue;
        }
        let bpos = Vec2::new(transform.translation.x, transform.translation.y);
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 0.7, 0.3, 0.0),
                custom_size: Some(Vec2::new(TORCH_RADIUS * 2.0, TORCH_RADIUS * 2.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(bpos.x, bpos.y, 2.0)),
            ..Default::default()
        })
        .insert(TorchHalo {
            parent_building: building_entity,
            pulse_timer: rand::random::<f32>() * std::f32::consts::TAU,
            target_alpha,
        });
    }

    // Despawn halos for destroyed buildings
    let dead_parents: Vec<Entity> = torch_halos
        .iter()
        .filter(|(_, halo, _)| {
            buildings
                .get(halo.parent_building)
                .map(|(_, b, _)| b.is_destroyed)
                .unwrap_or(true)
        })
        .map(|(e, _, _)| e)
        .collect();
    for e in dead_parents {
        commands.entity(e).despawn();
    }

    // Update torch halo visuals: smooth fade + pulse
    for (_, mut halo, mut sprite) in torch_halos.iter_mut() {
        halo.pulse_timer += time.delta_seconds();
        halo.target_alpha = target_alpha;

        let pulse = (halo.pulse_timer * 1.5).sin() * 0.015;
        let current = sprite.color.a();
        let new_color = if halo.target_alpha > 0.0 {
            let target_with_pulse = (halo.target_alpha + pulse).max(0.0);
            current + (target_with_pulse - current) * (10.0 * time.delta_seconds()).min(1.0)
        } else {
            current + (0.0 - current) * (10.0 * time.delta_seconds()).min(1.0)
        };

        sprite.color = Color::rgba(1.0, 0.7, 0.3, new_color);
    }

    // Defense bonus for heroes near buildings at night
    if game_time.is_night() {
        for (entity, mut stats, hero_t) in heroes.iter_mut() {
            let hpos = Vec2::new(hero_t.translation.x, hero_t.translation.y);
            let near_building = buildings.iter().any(|(_, building, transform)| {
                !building.is_destroyed
                    && (Vec2::new(transform.translation.x, transform.translation.y) - hpos)
                        .length()
                        < 100.0
            });

            if near_building {
                if buffed_heroes.insert(entity) {
                    stats.defense += TORCH_DEFENSE_BONUS;
                }
            } else if buffed_heroes.remove(&entity) {
                stats.defense -= TORCH_DEFENSE_BONUS;
            }
        }
    } else {
        // Remove defense bonus during day
        for (entity, mut stats, _) in heroes.iter_mut() {
            if buffed_heroes.remove(&entity) {
                stats.defense -= TORCH_DEFENSE_BONUS;
            }
        }
        buffed_heroes.clear();
    }
}

// ================================================================
// 13. SPRITE ANIMATIONS — cycle frames for atlas-based sprites
// ================================================================

pub fn sprite_animation_system(
    mut query: Query<(&mut SpriteAnimation, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (mut anim, mut sprite, atlas_handle) in query.iter_mut() {
        anim.frame_timer += dt;
        if anim.frame_timer >= anim.frame_duration {
            anim.frame_timer = 0.0;
            anim.current_frame = (anim.current_frame + 1) % anim.frame_count;
            let idx = anim.atlas_index();

            // Clamp to atlas bounds to prevent out-of-range panics during
            // mode transitions (walk ↔ attack atlas swaps).
            let max_idx = atlases
                .get(atlas_handle)
                .map(|a| a.textures.len())
                .unwrap_or(1);

            sprite.index = idx.min(max_idx.saturating_sub(1));
        }
    }
}

/// System: Switch between walk / attack / hurt animation atlases based on entity state.
///
/// For heroes: AttackingEnemy → attack atlas, Dead → hurt atlas, else → walk atlas.
/// For enemies with AnimationSet: uses similar logic based on proximity/combat.
pub fn animation_mode_system(
    mut query: Query<(
        &HeroState,
        &mut AnimationSet,
        &mut SpriteAnimation,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (state, mut anim_set, mut anim, mut atlas_handle, mut sprite) in query.iter_mut() {
        let desired = match state {
            HeroState::AttackingEnemy { .. } => AnimMode::Attack,
            HeroState::Dead { .. } => AnimMode::Hurt,
            HeroState::Resting => AnimMode::Rest,
            HeroState::Idle => AnimMode::Idle,
            _ => AnimMode::Walk,
        };

        if desired == anim_set.current_mode {
            continue;
        }

        // Switch animation mode
        anim_set.current_mode = desired;
        anim.current_frame = 0;
        anim.frame_timer = 0.0;

        match desired {
            AnimMode::Walk => {
                *atlas_handle = anim_set.walk_atlas.clone();
                anim.frame_count = anim_set.walk_frames;
                anim.frames_per_row = anim_set.walk_frames;
                anim.frame_duration = 1.0 / 8.0;
            }
            AnimMode::Idle => {
                *atlas_handle = anim_set.idle_atlas.clone();
                anim.frame_count = anim_set.idle_frames;
                anim.frames_per_row = anim_set.idle_frames;
                anim.frame_duration = 1.0 / 4.0;
            }
            AnimMode::Rest => {
                *atlas_handle = anim_set.rest_atlas.clone();
                anim.frame_count = anim_set.rest_frames;
                anim.frames_per_row = anim_set.rest_frames;
                anim.frame_duration = 1.0 / 3.0;
            }
            AnimMode::Attack => {
                *atlas_handle = anim_set.attack_atlas.clone();
                anim.frame_count = anim_set.attack_frames;
                anim.frames_per_row = anim_set.attack_frames;
                anim.frame_duration = 1.0 / 10.0;
            }
            AnimMode::Hurt => {
                *atlas_handle = anim_set.hurt_atlas.clone();
                anim.frame_count = anim_set.hurt_frames;
                anim.frames_per_row = anim_set.hurt_frames;
                if anim_set.hurt_rows == 1 {
                    anim.row_offset = 0;
                }
                anim.frame_duration = 1.0 / 6.0;
            }
        }

        // Immediately sync sprite index to prevent out-of-bounds on the new atlas
        sprite.index = anim.atlas_index();
    }
}

/// System: Switch animation mode for enemies based on their AI state.
pub fn enemy_animation_mode_system(
    mut query: Query<(
        &EnemyAi,
        &EnemyStats,
        &mut AnimationSet,
        &mut SpriteAnimation,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        &Transform,
    ), With<Enemy>>,
    heroes: Query<&Transform, (With<Hero>, Without<Enemy>)>,
) {
    for (ai, stats, mut anim_set, mut anim, mut atlas_handle, mut sprite, transform) in query.iter_mut() {
        // If enemy has a target and is within attack range → attack mode
        let desired = if stats.hp <= 0.0 {
            AnimMode::Hurt
        } else if let Some(target_entity) = ai.target {
            if let Ok(hero_tf) = heroes.get(target_entity) {
                let dist = Vec2::new(
                    transform.translation.x - hero_tf.translation.x,
                    transform.translation.y - hero_tf.translation.y,
                ).length();
                if dist <= stats.attack_range * 1.2 {
                    AnimMode::Attack
                } else {
                    AnimMode::Walk
                }
            } else {
                AnimMode::Walk
            }
        } else {
            AnimMode::Walk
        };

        if desired == anim_set.current_mode {
            continue;
        }

        anim_set.current_mode = desired;
        anim.current_frame = 0;
        anim.frame_timer = 0.0;

        match desired {
            AnimMode::Walk => {
                *atlas_handle = anim_set.walk_atlas.clone();
                anim.frame_count = anim_set.walk_frames;
                anim.frames_per_row = anim_set.walk_frames;
                anim.frame_duration = 1.0 / 6.0;
            }
            AnimMode::Idle => {
                *atlas_handle = anim_set.idle_atlas.clone();
                anim.frame_count = anim_set.idle_frames;
                anim.frames_per_row = anim_set.idle_frames;
                anim.frame_duration = 1.0 / 4.0;
            }
            AnimMode::Rest => {
                *atlas_handle = anim_set.rest_atlas.clone();
                anim.frame_count = anim_set.rest_frames;
                anim.frames_per_row = anim_set.rest_frames;
                anim.frame_duration = 1.0 / 3.0;
            }
            AnimMode::Attack => {
                *atlas_handle = anim_set.attack_atlas.clone();
                anim.frame_count = anim_set.attack_frames;
                anim.frames_per_row = anim_set.attack_frames;
                anim.frame_duration = 1.0 / 8.0;
            }
            AnimMode::Hurt => {
                *atlas_handle = anim_set.hurt_atlas.clone();
                anim.frame_count = anim_set.hurt_frames;
                anim.frames_per_row = anim_set.hurt_frames;
                if anim_set.hurt_rows == 1 {
                    anim.row_offset = 0;
                }
                anim.frame_duration = 1.0 / 4.0;
            }
        }

        // Immediately sync sprite index to prevent out-of-bounds on the new atlas
        sprite.index = anim.atlas_index();
    }
}

// ================================================================
// 14. CLICK-TO-INSPECT — I key + click shows info
// ================================================================

pub fn inspect_system(
    keyboard: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &Transform, &OrthographicProjection), With<MainCamera>>,
    heroes: Query<(Entity, &Hero, &HeroStats, &HeroEquipment, &HeroState, &Transform), Without<Building>>,
    buildings: Query<(Entity, &Building, &Transform), Without<Hero>>,
    enemies: Query<(Entity, &Enemy, &EnemyStats, &Transform), (Without<Hero>, Without<Building>)>,
    mut alerts: ResMut<GameAlerts>,
) {
    if !keyboard.pressed(KeyCode::I) || !mouse_input.just_pressed(MouseButton::Left) {
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

        // Check heroes
        for (_e, hero, stats, equipment, state, t) in heroes.iter() {
            let pos = Vec2::new(t.translation.x, t.translation.y);
            if (pos - world).length() < 20.0 {
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
                let equip_str = if equip_atk > 0.0 || equip_def > 0.0 {
                    let w = equipment.weapon.as_ref().map_or("None".to_string(), |e| e.display_name());
                    let a = equipment.armor.as_ref().map_or("None".to_string(), |e| e.display_name());
                    format!(" | Gear: W:{} A:{} (+{:.0}atk +{:.0}def)", w, a, equip_atk, equip_def)
                } else {
                    String::new()
                };
                alerts.push(format!(
                    "{}{} Lv{} | HP:{:.0}/{:.0} ATK:{:.0} DEF:{:.0} SPD:{:.0} | {} | {:?} | Morale:{:.0}{}",
                    hero.class.display_name(), leg, hero.level,
                    stats.hp, stats.max_hp, stats.attack + equip_atk, stats.defense + equip_def, stats.speed,
                    state_str, hero.personality, hero.morale, equip_str
                ));
                return;
            }
        }

        // Check buildings
        for (_e, building, t) in buildings.iter() {
            let pos = Vec2::new(t.translation.x, t.translation.y);
            if (pos - world).length() < 50.0 {
                let status = if building.is_destroyed { "DESTROYED" } else { "Active" };
                alerts.push(format!(
                    "{} Tier {} | HP:{:.0}/{:.0} | {} | Tax:{:.1}/min",
                    building.building_type.display_name(), building.tier,
                    building.hp, building.max_hp, status,
                    building.building_type.tax_income(building.tier)
                ));
                return;
            }
        }

        // Check enemies
        for (_e, enemy, stats, t) in enemies.iter() {
            let pos = Vec2::new(t.translation.x, t.translation.y);
            if (pos - world).length() < 20.0 {
                alerts.push(format!(
                    "{} | HP:{:.0}/{:.0} ATK:{:.0} DEF:{:.0} | Threat:{}",
                    enemy.enemy_type.display_name(),
                    stats.hp, stats.max_hp, stats.attack, stats.defense, stats.threat_level
                ));
                return;
            }
        }
    }
}

// ================================================================
// 15. FOG OF WAR — dark tiles that clear as heroes explore
// ================================================================

pub fn fog_of_war_system(
    mut fog: ResMut<FogOfWar>,
    heroes: Query<&Transform, With<Hero>>,
    mut fog_tiles: Query<(&mut Visibility, &Transform), (With<FogTile>, Without<Hero>)>,
) {
    // Expand explored areas based on hero positions
    for hero_t in heroes.iter() {
        let hpos = Vec2::new(hero_t.translation.x, hero_t.translation.y);
        let already = fog.explored_areas.iter().any(|a| (*a - hpos).length() < 50.0);
        if !already {
            fog.explored_areas.push(hpos);
        }
    }

    // Update fog tile visibility
    for (mut vis, fog_t) in fog_tiles.iter_mut() {
        let fpos = Vec2::new(fog_t.translation.x, fog_t.translation.y);

        let revealed = fpos.length() < fog.revealed_radius
            || fog.explored_areas.iter().any(|a| (*a - fpos).length() < 80.0);

        vis.is_visible = !revealed;
    }
}

pub fn spawn_fog_of_war(mut commands: Commands) {
    let tile_size = 80.0;
    let half_map = 1500.0;

    let mut x = -half_map;
    while x < half_map {
        let mut y = -half_map;
        while y < half_map {
            let pos = Vec2::new(x, y);
            // Don't fog the starting area
            if pos.length() < 300.0 {
                y += tile_size;
                continue;
            }

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 0.0, 0.0, 0.85),
                    custom_size: Some(Vec2::new(tile_size, tile_size)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 45.0)),
                ..Default::default()
            })
            .insert(FogTile);

            y += tile_size;
        }
        x += tile_size;
    }
}

// ================================================================
// 16. BLACKSMITH EQUIPMENT CRAFTING
// ================================================================

/// Heroes periodically visit the Blacksmith to get equipment crafted.
/// The treasury pays for crafting. Blacksmith tier determines equipment quality.
/// Heroes prioritise getting a weapon first, then armor.
pub fn blacksmith_crafting_system(
    mut heroes: Query<(Entity, &Hero, &HeroStats, &mut HeroEquipment, &mut HeroState, &Transform)>,
    buildings: Query<(&Building, &Transform), Without<Hero>>,
    mut economy: ResMut<GameEconomy>,
    game_time: Res<GameTime>,
    time: Res<Time>,
    mut alerts: ResMut<GameAlerts>,
    mut craft_cooldown: Local<f32>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 { return; }

    // Only check for crafting every 5 seconds to avoid spamming
    *craft_cooldown -= dt;
    if *craft_cooldown > 0.0 { return; }
    *craft_cooldown = 5.0;

    // Find the best blacksmith (highest tier, not destroyed)
    let best_blacksmith = buildings.iter()
        .filter(|(b, _)| b.building_type == BuildingType::Blacksmith && !b.is_destroyed)
        .max_by_key(|(b, _)| b.tier);

    let (blacksmith, blacksmith_transform) = match best_blacksmith {
        Some(b) => b,
        None => return, // No blacksmith built
    };

    let bs_pos = Vec2::new(blacksmith_transform.translation.x, blacksmith_transform.translation.y);
    let available_tier = EquipmentTier::from_blacksmith_tier(blacksmith.tier);
    let craft_cost = available_tier.craft_cost();

    for (_entity, hero, _stats, mut equipment, mut state, transform) in heroes.iter_mut() {
        // Only craft for idle or resting heroes (not dead, not fighting)
        let can_craft = matches!(*state, HeroState::Idle | HeroState::Resting);
        if !can_craft { continue; }

        // Determine which slot needs an upgrade
        let slot = if equipment.needs_upgrade(EquipmentSlot::Weapon, available_tier) {
            Some(EquipmentSlot::Weapon)
        } else if equipment.needs_upgrade(EquipmentSlot::Armor, available_tier) {
            Some(EquipmentSlot::Armor)
        } else {
            None
        };

        let slot = match slot {
            Some(s) => s,
            None => continue, // Hero is fully equipped at current tier
        };

        let hero_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let dist_to_blacksmith = (hero_pos - bs_pos).length();

        // If hero is near the blacksmith, craft the equipment
        if dist_to_blacksmith < 50.0 {
            if economy.gold < craft_cost {
                continue; // Can't afford it
            }

            economy.gold -= craft_cost;
            economy.total_spent += craft_cost;

            let new_equipment = match slot {
                EquipmentSlot::Weapon => Equipment::weapon(available_tier),
                EquipmentSlot::Armor => Equipment::armor(available_tier),
            };

            let equip_name = new_equipment.display_name();
            match slot {
                EquipmentSlot::Weapon => equipment.weapon = Some(new_equipment),
                EquipmentSlot::Armor => equipment.armor = Some(new_equipment),
            }

            alerts.push(format!(
                "{} crafted {} at the Blacksmith! (-{:.0}g)",
                hero.class.display_name(), equip_name, craft_cost
            ));

            // After crafting, go back to idle
            *state = HeroState::Idle;
        } else {
            // Move hero toward the blacksmith
            *state = HeroState::MovingTo { target: bs_pos };
        }
    }
}

// ================================================================
// 17. APPLY BUILDING BONUSES TO GAMEPLAY
// ================================================================

/// Apply blacksmith ATK/DEF bonuses and alchemist recovery speed
pub fn apply_building_bonuses_system(
    bonuses: Res<BuildingBonuses>,
    mut heroes: Query<(&mut HeroStats, &mut HeroState, &Hero)>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 { return; }

    for (_stats, mut state, _hero) in heroes.iter_mut() {
        // Alchemist speeds up death recovery
        if let HeroState::Dead { respawn_timer } = &mut *state {
            *respawn_timer -= dt * (bonuses.alchemist_recovery_speed - 1.0);
        }

        // Temple morale aura
        if bonuses.temple_morale_aura > 0.0 {
            // Handled via hero morale system indirectly
        }
    }
}

/// Temple tier 3 (Cathedral) generates pilgrim income
pub fn cathedral_income_system(
    bonuses: Res<BuildingBonuses>,
    mut economy: ResMut<GameEconomy>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    if bonuses.temple_pilgrim_income <= 0.0 { return; }
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    let income = (bonuses.temple_pilgrim_income / 60.0) * dt; // Per minute → per second
    economy.gold += income;
    economy.total_earned += income;
}

// ================================================================
// 17. MAP EXPANSION — E key to expand revealed territory
// ================================================================

/// Player presses E to expand the map perimeter, revealing new zones.
/// Each expansion increases revealed_radius and spawns new content.
pub fn map_expansion_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut fog: ResMut<FogOfWar>,
    mut economy: ResMut<GameEconomy>,
    kingdom: Res<KingdomState>,
    sprites: Res<SpriteAssets>,
    game_phase: Res<GamePhase>,
    mut alerts: ResMut<GameAlerts>,
    mut fog_tiles: Query<(Entity, &Transform), With<FogTile>>,
) {
    if !keyboard.just_pressed(KeyCode::E) { return; }
    if game_phase.build_mode || game_phase.bounty_board_open { return; }

    let max = kingdom.rank.max_expansions();
    if fog.expansions >= max {
        alerts.push(format!(
            "Cannot expand further at {} rank! Grow your kingdom first.",
            kingdom.rank.display_name()
        ));
        return;
    }

    let cost = KingdomRank::expansion_cost(fog.expansions);
    if economy.gold < cost {
        alerts.push(format!("Not enough gold to expand! Need {:.0}g", cost));
        return;
    }

    // Pay the cost
    economy.gold -= cost;
    economy.total_spent += cost;

    // Increase revealed radius by 100 units per expansion
    let old_radius = fog.revealed_radius;
    fog.expansions += 1;
    fog.revealed_radius += 100.0;
    let new_radius = fog.revealed_radius;

    // Remove fog tiles that are now within the expanded radius
    for (entity, fog_t) in fog_tiles.iter_mut() {
        let fpos = Vec2::new(fog_t.translation.x, fog_t.translation.y);
        if fpos.length() < new_radius {
            commands.entity(entity).despawn();
        }
    }

    // Spawn new content in the newly revealed ring
    // New monster den in the expanded zone
    let den_angle = rand::random::<f32>() * TAU;
    let den_radius = old_radius + 50.0 + rand::random::<f32>() * 40.0;
    let den_pos = Vec2::new(den_angle.cos() * den_radius, den_angle.sin() * den_radius);

    let den_type = match fog.expansions {
        1..=2 => EnemyType::Goblin,
        3 => EnemyType::Bandit,
        _ => if rand::random::<bool>() { EnemyType::Bandit } else { EnemyType::Troll },
    };

    let den = MonsterDen::new(den_type);
    let tier = den.threat_tier;

    commands.spawn_bundle(SpriteBundle {
        texture: monster_den_texture_for_tier(&sprites, tier),
        transform: Transform::from_translation(Vec3::new(den_pos.x, den_pos.y, 4.0))
            .with_scale(Vec3::splat(monster_den_scale_for_tier(tier))),
        ..Default::default()
    })
    .insert(den)
    .insert(MonsterDenVisualTier { tier });

    // Spawn a new resource node in the expanded zone
    let node_angle = den_angle + std::f32::consts::PI; // Opposite side from den
    let node_radius = old_radius + 30.0 + rand::random::<f32>() * 50.0;
    let node_pos = Vec2::new(node_angle.cos() * node_radius, node_angle.sin() * node_radius);

    let (rtype, color) = if fog.expansions % 2 == 1 {
        (ResourceType::Mine, Color::rgb(0.5, 0.4, 0.3))
    } else {
        (ResourceType::LumberMill, Color::rgb(0.3, 0.5, 0.2))
    };

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(node_pos.x, node_pos.y, 3.5)),
        ..Default::default()
    })
    .insert(ResourceNode::new(rtype));

    let zone_name = match fog.expansions {
        1 => "Outer Fields",
        2 => "Dark Forest",
        3 => "Mountain Pass",
        4 => "Ancient Ruins",
        5 => "Dragon's Reach",
        _ => "Unknown Territory",
    };

    alerts.push(format!(
        "TERRITORY EXPANDED: {} revealed! (Radius: {:.0}) [-{:.0}g]",
        zone_name, new_radius, cost
    ));

    // Spawn new decorations in the expanded ring so it doesn't look blank
    let num_trees = 8 + (fog.expansions * 2);
    let tree_tex = [
        sprites.deco_pine1.clone(), sprites.deco_pine3.clone(),
        sprites.deco_pine4.clone(), sprites.deco_tree_oak1.clone(),
        sprites.deco_tree_dead1.clone(), sprites.deco_tree_big1.clone(),
    ];
    for _ in 0..num_trees {
        let a = rand::random::<f32>() * TAU;
        let r = old_radius + rand::random::<f32>() * 100.0;
        let pos = Vec2::new(a.cos() * r, a.sin() * r);
        let idx = rand::random::<usize>() % tree_tex.len();
        let scale = 0.8 + rand::random::<f32>() * 0.5;
        commands.spawn_bundle(SpriteBundle {
            texture: tree_tex[idx].clone(),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 4.0))
                .with_scale(Vec3::splat(scale)),
            ..Default::default()
        })
        .insert(MapDecoration);
    }

    // Spawn small rocks and bushes in expanded area
    let small_tex = [
        sprites.deco_rock_small1.clone(),
        sprites.deco_rock_small2.clone(),
        sprites.deco_bush1.clone(),
    ];
    for _ in 0..8 {
        let a = rand::random::<f32>() * TAU;
        let r = old_radius + rand::random::<f32>() * 80.0;
        let pos = Vec2::new(a.cos() * r, a.sin() * r);
        let idx = rand::random::<usize>() % small_tex.len();
        let z = if rand::random::<bool>() { 1.0 } else { 2.0 };
        commands.spawn_bundle(SpriteBundle {
            texture: small_tex[idx].clone(),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, z))
                .with_scale(Vec3::splat(0.7 + rand::random::<f32>() * 0.3)),
            ..Default::default()
        })
        .insert(MapDecoration);
    }
}
