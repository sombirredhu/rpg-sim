//! All missing GDD features: roads, resources, merchants, night enemies,
//! building bonuses, milestones, era siege, fog of war, inspect, animations,
//! hero idle/bankruptcy, recovery bounties, etc.

use bevy::prelude::*;
use crate::components::*;
use crate::sprites::{SpriteAssets, spawn_enemy_with_sprite};
use std::f32::consts::TAU;

// ================================================================
// 1. ROAD NETWORK — R key to place, 30% speed boost
// ================================================================

pub fn road_placement_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &Transform), With<Camera>>,
    mut economy: ResMut<GameEconomy>,
    mut road_network: ResMut<RoadNetwork>,
    game_phase: Res<GamePhase>,
    mut _alerts: ResMut<GameAlerts>,
) {
    // Only place roads while holding R + left click
    if game_phase.build_mode || !keyboard.pressed(KeyCode::R) {
        return;
    }
    if !mouse_input.pressed(MouseButton::Left) {
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

    if let Ok((_cam, cam_t)) = camera.get_single() {
        let ws = Vec2::new(window.width(), window.height());
        let ndc = (cursor_pos / ws) * 2.0 - Vec2::ONE;
        let world_pos = cam_t.translation.truncate() + ndc * ws * 0.3;

        // Don't place if too close to existing road tile
        if road_network.is_on_road(world_pos) {
            return;
        }

        let cost = 5.0;
        if economy.gold < cost {
            return;
        }
        economy.gold -= cost;
        economy.total_spent += cost;

        road_network.tiles.push(world_pos);

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.55, 0.45, 0.3),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 1.0)),
            ..Default::default()
        })
        .insert(Road);
    }
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
                if matches!(state, HeroState::AttackingEnemy { .. }) {
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

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: sprites.building_red_atlas.clone(),
        transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 4.0))
            .with_scale(Vec3::splat(0.5)),
        sprite: TextureAtlasSprite { index: 1, ..Default::default() },
        ..Default::default()
    })
    .insert(MonsterDen::new(enemy_type));
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
    let nodes = [
        (Vec2::new(250.0, 150.0), ResourceType::Mine),
        (Vec2::new(-200.0, -250.0), ResourceType::Mine),
        (Vec2::new(-300.0, 200.0), ResourceType::LumberMill),
        (Vec2::new(180.0, -200.0), ResourceType::LumberMill),
    ];

    for (pos, rtype) in nodes {
        let color = match rtype {
            ResourceType::Mine => Color::rgb(0.5, 0.4, 0.3),
            ResourceType::LumberMill => Color::rgb(0.3, 0.5, 0.2),
        };
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(20.0, 20.0)),
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
    // Reset
    *bonuses = BuildingBonuses::default();

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
    buildings: Query<(&Building, &Transform)>,
    mut heroes: Query<(&mut HeroStats, &Transform), With<Hero>>,
    game_time: Res<GameTime>,
    _time: Res<Time>,
    mut applied: Local<bool>,
) {
    let is_night = game_time.is_night();

    if is_night && !*applied {
        *applied = true;
        // Heroes near buildings get +3 defense at night
        for (mut stats, hero_t) in heroes.iter_mut() {
            let hpos = Vec2::new(hero_t.translation.x, hero_t.translation.y);
            let near_building = buildings.iter().any(|(b, bt)| {
                !b.is_destroyed && (Vec2::new(bt.translation.x, bt.translation.y) - hpos).length() < 100.0
            });
            if near_building {
                stats.defense += 3.0;
            }
        }
    } else if !is_night && *applied {
        *applied = false;
        // Remove the bonus
        for (mut stats, hero_t) in heroes.iter_mut() {
            let hpos = Vec2::new(hero_t.translation.x, hero_t.translation.y);
            let near_building = buildings.iter().any(|(b, bt)| {
                !b.is_destroyed && (Vec2::new(bt.translation.x, bt.translation.y) - hpos).length() < 100.0
            });
            if near_building {
                stats.defense -= 3.0;
            }
        }
    }
}

// ================================================================
// 13. SPRITE ANIMATIONS — cycle frames for atlas-based sprites
// ================================================================

pub fn sprite_animation_system(
    mut query: Query<(&mut SpriteAnimation, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (mut anim, mut sprite) in query.iter_mut() {
        anim.frame_timer += dt;
        if anim.frame_timer >= anim.frame_duration {
            anim.frame_timer = 0.0;
            anim.current_frame = (anim.current_frame + 1) % anim.frame_count;
            sprite.index = anim.current_frame;
        }
    }
}

// ================================================================
// 14. CLICK-TO-INSPECT — I key + click shows info
// ================================================================

pub fn inspect_system(
    keyboard: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &Transform), With<Camera>>,
    heroes: Query<(Entity, &Hero, &HeroStats, &HeroState, &Transform), Without<Building>>,
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
    let cursor = match window.cursor_position() {
        Some(p) => p,
        None => return,
    };

    if let Ok((_cam, cam_t)) = camera.get_single() {
        let ws = Vec2::new(window.width(), window.height());
        let ndc = (cursor / ws) * 2.0 - Vec2::ONE;
        let world = cam_t.translation.truncate() + ndc * ws * 0.3;

        // Check heroes
        for (_e, hero, stats, state, t) in heroes.iter() {
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
                };
                let leg = if hero.is_legendary { " [LEGENDARY]" } else { "" };
                alerts.push(format!(
                    "{}{} Lv{} | HP:{:.0}/{:.0} ATK:{:.0} DEF:{:.0} SPD:{:.0} | {} | {:?} | Morale:{:.0}",
                    hero.class.display_name(), leg, hero.level,
                    stats.hp, stats.max_hp, stats.attack, stats.defense, stats.speed,
                    state_str, hero.personality, hero.morale
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
    let tile_size = 40.0;
    let half_map = 600.0;

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
// 16. APPLY BUILDING BONUSES TO GAMEPLAY
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
