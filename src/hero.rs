use bevy::prelude::*;
use crate::components::*;
use crate::sprites::{SpriteAssets, spawn_hero_with_sprite};
use std::f32::consts::TAU;

/// Returns the signing bonus required for a hero class (0 for basic classes)
fn get_signing_bonus(class: HeroClass) -> f32 {
    match class {
        HeroClass::Mage => 100.0,
        HeroClass::Archer => 80.0,
        _ => 0.0,
    }
}

/// Set the LPC direction row based on movement direction.
/// Row 0 = up, 1 = left, 2 = down, 3 = right.
fn apply_hero_facing(anim: &mut SpriteAnimation, move_dir: Vec2) {
    if move_dir.y.abs() > move_dir.x.abs() {
        // Vertical movement dominates
        if move_dir.y > 0.0 {
            anim.row_offset = 0; // up
        } else {
            anim.row_offset = 2; // down
        }
    } else {
        // Horizontal movement dominates
        if move_dir.x < 0.0 {
            anim.row_offset = 1; // left
        } else {
            anim.row_offset = 3; // right
        }
    }
}

/// System: Hero AI decision-making
/// Each hero evaluates bounties, threats, and personal needs to decide their next action
pub fn hero_ai_system(
    mut commands: Commands,
    mut heroes: Query<(Entity, &mut Hero, &mut HeroStats, &mut HeroState, &mut HeroDecisionTimer, &Transform)>,
    mut bounty_board: ResMut<BountyBoard>,
    game_time: Res<GameTime>,
    enemies: Query<(Entity, &Transform, &EnemyStats), With<Enemy>>,
    buildings: Query<(Entity, &Transform, &Building)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (hero_entity, mut hero, stats, mut state, mut decision_timer, transform) in heroes.iter_mut() {
        // Handle dead heroes
        if let HeroState::Dead { respawn_timer } = &mut *state {
            bounty_board.unassign_hero(hero_entity);
            *respawn_timer -= dt;
            if *respawn_timer <= 0.0 {
                // Permanent death — hero leaves the kingdom
                commands.entity(hero_entity).despawn();
            }
            continue;
        }

        // Decision timer - heroes reconsider every 2-3 seconds
        decision_timer.0 -= dt;
        if decision_timer.0 > 0.0 {
            continue;
        }
        decision_timer.0 = 2.0 + rand::random::<f32>() * 1.0;

        // Skip heroes currently channeling spells
        if matches!(&*state, HeroState::Casting { .. }) {
            continue;
        }

        let hero_pos = Vec2::new(transform.translation.x, transform.translation.y);

        // Priority 1: If HP is very low, seek inn/rest
        if stats.hp < stats.max_hp * 0.25 {
            bounty_board.unassign_hero(hero_entity);
            // Find nearest inn
            if let Some((_, inn_transform, _)) = buildings.iter()
                .filter(|(_, _, b)| b.building_type == BuildingType::Inn && !b.is_destroyed)
                .min_by(|(_, ta, _), (_, tb, _)| {
                    let da = (Vec2::new(ta.translation.x, ta.translation.y) - hero_pos).length();
                    let db = (Vec2::new(tb.translation.x, tb.translation.y) - hero_pos).length();
                    da.partial_cmp(&db).unwrap()
                })
            {
                let inn_pos = Vec2::new(inn_transform.translation.x, inn_transform.translation.y);
                if (inn_pos - hero_pos).length() < 30.0 {
                    *state = HeroState::Resting;
                } else {
                    *state = HeroState::MovingTo { target: inn_pos };
                }
                continue;
            }
        }

        // Priority 2: Low morale at night - refuse to leave inn
        if game_time.is_night() && hero.morale < 30.0 {
            bounty_board.unassign_hero(hero_entity);
            *state = HeroState::Resting;
            continue;
        }

        let current_bounty_id = match &*state {
            HeroState::PursuingBounty { bounty_id } => Some(*bounty_id),
            _ => None,
        };

        if let Some(bounty_id) = current_bounty_id {
            let still_valid = bounty_board
                .get_bounty(bounty_id)
                .map(|b| b.assigned_heroes.contains(&hero_entity))
                .unwrap_or(false);

            if still_valid {
                continue;
            }

            bounty_board.unassign_hero(hero_entity);
            *state = HeroState::Idle;
        }

        // Priority 3: Evaluate available bounties
        let available = bounty_board.available_bounties();
        if !available.is_empty() {
            // Score each bounty based on hero AI factors
            let mut best_bounty: Option<(u32, f32)> = None;
            let mut best_score = f32::MIN;

            for bounty in &available {
                let distance = (bounty.location - hero_pos).length();
                let danger = bounty.danger_level as f32;

                // Base score from gold reward, adjusted for squad size (each hero's expected share)
                let mut score = bounty.gold_reward / bounty.required_heroes.max(1) as f32;

                // Distance penalty
                score -= distance * 0.1;

                // Danger evaluation based on personality
                let danger_factor = match hero.personality {
                    HeroPersonality::Brave => danger * 2.0,    // Brave: danger barely matters
                    HeroPersonality::Cautious => -danger * 15.0, // Cautious: heavily penalises danger
                    HeroPersonality::Greedy => 0.0,             // Greedy: only cares about gold
                    HeroPersonality::Loyal => -danger * 5.0,    // Loyal: moderate caution
                };
                score += danger_factor;

                // Risk tolerance check
                if danger as f32 > stats.risk_tolerance * 5.0 + 1.0 {
                    if hero.personality != HeroPersonality::Brave {
                        score *= 0.1; // Very unlikely to take this
                    }
                }

                // Hero class affinity
                match (hero.class, bounty.bounty_type) {
                    (HeroClass::Warrior, BountyType::Monster) => score *= 1.5,
                    (HeroClass::Rogue, BountyType::Exploration) => score *= 1.5,
                    (HeroClass::Archer, BountyType::Monster) => score *= 1.3,
                    (HeroClass::Mage, BountyType::Monster) => score *= 1.4,
                    (HeroClass::Healer, BountyType::Objective) => score *= 1.5,
                    _ => {}
                }

                // Night penalty for non-brave heroes
                if game_time.is_night() && hero.personality != HeroPersonality::Brave {
                    score *= 0.7;
                }

                if score > best_score {
                    best_score = score;
                    best_bounty = Some((bounty.id, score));
                }
            }

            if let Some((bounty_id, _score)) = best_bounty {
                if best_score > 0.0 && bounty_board.assign_bounty(bounty_id, hero_entity) {
                    *state = HeroState::PursuingBounty { bounty_id };
                    continue;
                }
            }
        }

        // Priority 3.5: Check for panic (low HP + low morale + many nearby enemies)
        if stats.hp < stats.max_hp * 0.5 && hero.morale < 30.0 {
            let nearby_enemies: Vec<_> = enemies.iter()
                .filter(|(_, et, es)| {
                    let dist = (Vec2::new(et.translation.x, et.translation.y) - hero_pos).length();
                    dist < 120.0 && es.hp > 0.0
                })
                .collect();
            if nearby_enemies.len() >= 2 {
                let avg_enemy_pos: Vec2 = nearby_enemies.iter()
                    .map(|(_, et, _)| Vec2::new(et.translation.x, et.translation.y))
                    .fold(Vec2::ZERO, |acc, p| acc + p) / nearby_enemies.len() as f32;
                let flee_dir = (hero_pos - avg_enemy_pos).normalize();
                let flee_target = hero_pos + flee_dir * 100.0;
                bounty_board.unassign_hero(hero_entity);
                *state = HeroState::MovingTo { target: flee_target };
                continue;
            }
        }

        // Priority 4: Shopping - idle heroes with surplus gold visit the market
        if matches!(&*state, HeroState::Shopping) {
            // Shopping heroes will leave via hero_rest_system (shared handling)
            continue;
        }

        if matches!(&*state, HeroState::Idle) && hero.gold_carried >= 20.0 {
            if let Some((_, market_transform, _)) = buildings.iter()
                .filter(|(_, _, b)| b.building_type == BuildingType::Market && !b.is_destroyed)
                .min_by(|(_, ta, _), (_, tb, _)| {
                    let da = (Vec2::new(ta.translation.x, ta.translation.y) - hero_pos).length();
                    let db = (Vec2::new(tb.translation.x, tb.translation.y) - hero_pos).length();
                    da.partial_cmp(&db).unwrap()
                })
            {
                let market_pos = Vec2::new(market_transform.translation.x, market_transform.translation.y);
                let dist = (market_pos - hero_pos).length();
                if dist < 30.0 {
                    *state = HeroState::Shopping;
                } else {
                    *state = HeroState::MovingTo { target: market_pos };
                }
                continue;
            }
        }

        // Priority 5: Attack nearby enemies
        if let Some((enemy_entity, _, _)) = enemies.iter()
            .filter(|(_, et, es)| {
                let dist = (Vec2::new(et.translation.x, et.translation.y) - hero_pos).length();
                dist < 200.0 && es.hp > 0.0
            })
            .min_by(|(_, ta, _), (_, tb, _)| {
                let da = (Vec2::new(ta.translation.x, ta.translation.y) - hero_pos).length();
                let db = (Vec2::new(tb.translation.x, tb.translation.y) - hero_pos).length();
                da.partial_cmp(&db).unwrap()
            })
        {
            bounty_board.unassign_hero(hero_entity);
            *state = HeroState::AttackingEnemy { target_entity: enemy_entity };
            continue;
        }

        // Priority 5: Idle - wander randomly
        bounty_board.unassign_hero(hero_entity);
        let angle = rand::random::<f32>() * TAU;
        let wander_target = hero_pos + Vec2::new(angle.cos(), angle.sin()) * 60.0;
        *state = HeroState::MovingTo { target: wander_target };
    }
}

/// System: Move heroes based on their current state
pub fn hero_movement_system(
    mut heroes: Query<(&Hero, &HeroStats, &mut HeroState, &mut Transform, Option<&mut SpriteAnimation>)>,
    enemies: Query<&Transform, (With<Enemy>, Without<Hero>)>,
    bounty_board: Res<BountyBoard>,
    road_network: Res<RoadNetwork>,
    active_buffs: Res<ActiveBuffs>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (_hero, stats, mut state, mut transform, anim_opt) in heroes.iter_mut() {
        match &*state {
            HeroState::MovingTo { target } => {
                let pos = Vec2::new(transform.translation.x, transform.translation.y);
                let dir = *target - pos;
                let dist = dir.length();

                if dist < 5.0 {
                    *state = HeroState::Idle;
                } else {
                    let move_dir = dir.normalize();
                    let road_mult = road_network.speed_multiplier(pos);
                    let speed = stats.speed * (1.0 + active_buffs.speed_bonus) * road_mult * dt;
                    transform.translation.x += move_dir.x * speed;
                    transform.translation.y += move_dir.y * speed;

                    if let Some(mut anim) = anim_opt {
                        apply_hero_facing(&mut anim, move_dir);
                    }
                }
            }
            HeroState::AttackingEnemy { target_entity } => {
                // Move toward enemy
                if let Ok(enemy_transform) = enemies.get(*target_entity) {
                    let pos = Vec2::new(transform.translation.x, transform.translation.y);
                    let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
                    let dir = enemy_pos - pos;
                    let dist = dir.length();

                    if dist > stats.attack_range {
                        let move_dir = dir.normalize();
                        let road_mult = road_network.speed_multiplier(pos);
                        let speed = stats.speed * (1.0 + active_buffs.speed_bonus) * road_mult * dt;
                        transform.translation.x += move_dir.x * speed;
                        transform.translation.y += move_dir.y * speed;
                        if let Some(mut anim) = anim_opt {
                            apply_hero_facing(&mut anim, move_dir);
                        }
                    }
                } else {
                    // Enemy no longer exists
                    *state = HeroState::Idle;
                }
            }
            HeroState::PursuingBounty { bounty_id } => {
                if let Some(bounty) = bounty_board.get_bounty(*bounty_id) {
                    let pos = Vec2::new(transform.translation.x, transform.translation.y);
                    let dir = bounty.location - pos;
                    let dist = dir.length();

                    if dist > 5.0 {
                        let move_dir = dir.normalize();
                        let road_mult = road_network.speed_multiplier(pos);
                        let speed = stats.speed * road_mult * dt;
                        transform.translation.x += move_dir.x * speed;
                        transform.translation.y += move_dir.y * speed;
                        if let Some(mut anim) = anim_opt {
                            apply_hero_facing(&mut anim, move_dir);
                        }
                    }
                } else {
                    *state = HeroState::Idle;
                }
            }
            HeroState::Resting | HeroState::Shopping => {
                // Stay still while resting or shopping
            }
            HeroState::Casting { .. } => {
                // Stay still while channeling
            }
            _ => {}
        }
    }
}

/// System: Resolve bounty completion conditions and emit payout events.
pub fn bounty_resolution_system(
    mut heroes: Query<(Entity, &mut HeroState, &Transform), With<Hero>>,
    mut bounty_board: ResMut<BountyBoard>,
    monster_dens: Query<Entity, With<MonsterDen>>,
    resource_nodes: Query<&ResourceNode>,
    buildings: Query<(&Building, &Transform)>,
    enemies: Query<&Transform, With<Enemy>>,
    mut events: EventWriter<BountyCompletedEvent>,
) {
    for (hero_entity, mut state, transform) in heroes.iter_mut() {
        if matches!(&*state, HeroState::Dead { .. }) {
            bounty_board.unassign_hero(hero_entity);
            continue;
        }

        let bounty_id = match &*state {
            HeroState::PursuingBounty { bounty_id } => *bounty_id,
            _ => {
                bounty_board.unassign_hero(hero_entity);
                continue;
            }
        };

        let bounty = match bounty_board.get_bounty(bounty_id).cloned() {
            Some(bounty) => bounty,
            None => {
                *state = HeroState::Idle;
                continue;
            }
        };

        if !bounty.assigned_heroes.contains(&hero_entity) {
            *state = HeroState::Idle;
            continue;
        }

        let hero_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let near_bounty = (hero_pos - bounty.location).length() <= 40.0;

        let completed = match bounty.bounty_type {
            BountyType::Exploration => near_bounty,
            BountyType::Monster => {
                if let Some(target) = bounty.target_entity {
                    monster_dens.get(target).is_err()
                } else {
                    near_bounty
                }
            }
            BountyType::Resource => {
                if let Some(target) = bounty.target_entity {
                    resource_nodes
                        .get(target)
                        .map(|node| node.is_active && near_bounty)
                        .unwrap_or(true)
                } else {
                    near_bounty
                }
            }
            BountyType::Objective => {
                if let Some(target) = bounty.target_entity {
                    if let Ok((building, building_transform)) = buildings.get(target) {
                        let bpos = Vec2::new(building_transform.translation.x, building_transform.translation.y);
                        let enemies_nearby = enemies.iter().any(|enemy_transform| {
                            let epos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
                            (epos - bpos).length() < 150.0
                        });
                        near_bounty && !building.is_destroyed && !enemies_nearby
                    } else {
                        near_bounty
                    }
                } else {
                    near_bounty
                }
            }
        };

        if completed {
            if let Some((reward, assigned_heroes)) = bounty_board.complete_bounty(bounty_id) {
                events.send(BountyCompletedEvent {
                    bounty_id,
                    assigned_heroes,
                    gold_reward: reward,
                    target_entity: bounty.target_entity,
                });
            }
            *state = HeroState::Idle;
        }
    }
}

/// System: Hero resting at inn restores HP and morale; heroes shopping at market spend gold for morale
pub fn hero_rest_system(
    mut heroes: Query<(&mut Hero, &mut HeroStats, &mut HeroState)>,
    bonuses: Res<BuildingBonuses>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (mut hero, mut stats, mut state) in heroes.iter_mut() {
        if let HeroState::Resting = *state {
            // Heal 10% HP per second while resting (boosted by inn tier)
            let heal_rate = stats.max_hp * 0.1 * bonuses.inn_heal_speed;
            stats.hp = (stats.hp + heal_rate * dt).min(stats.max_hp);
            // Restore morale (boosted by temple aura)
            let morale_rate = 5.0 + bonuses.temple_morale_aura;
            hero.morale = (hero.morale + morale_rate * dt).min(100.0);

            // Stop resting when fully healed
            if stats.hp >= stats.max_hp * 0.9 && hero.morale >= 70.0 {
                *state = HeroState::Idle;
            }
        } else if let HeroState::Shopping = *state {
            // Heroes spend ~10 gold at the market in exchange for a morale boost
            let spend = 10.0 * dt * bonuses.market_trade_bonus;
            let spent = spend.min(hero.gold_carried);
            hero.gold_carried -= spent;

            // Shopping boosts morale quickly
            hero.morale = (hero.morale + 6.0 * dt).min(100.0);

            // Leave the market after a short visit (3 seconds) or when out of gold
            if hero.gold_carried < 5.0 {
                *state = HeroState::Idle;
            }
        }
    }
}

/// System: Hero leveling and progression
pub fn hero_progression_system(
    mut heroes: Query<(&mut Hero, &mut HeroStats)>,
    mut alerts: ResMut<GameAlerts>,
) {
    for (mut hero, mut stats) in heroes.iter_mut() {
        if hero.xp >= hero.xp_to_next {
            hero.xp -= hero.xp_to_next;
            hero.level += 1;
            hero.xp_to_next = 100.0 * (1.0 + hero.level as f32 * 0.3);

            // Stat boosts per level
            stats.max_hp += 10.0;
            stats.hp = stats.max_hp;
            stats.attack += 2.0;
            stats.defense += 1.0;
            stats.speed += 1.0;

            // Legendary at level 10+
            if hero.level >= 10 && !hero.is_legendary {
                hero.is_legendary = true;
                stats.max_hp *= 1.3;
                stats.hp = stats.max_hp;
                stats.attack *= 1.25;
                stats.defense *= 1.2;
                alerts.push(format!("A {} became LEGENDARY!", hero.class.display_name()));
            }

            // Perk points every 5 levels
            if hero.level % 5 == 0 {
                // Apply class-specific perk
                match hero.class {
                    HeroClass::Warrior => {
                        stats.max_hp += 20.0;
                        stats.defense += 3.0;
                    }
                    HeroClass::Archer => {
                        stats.attack += 5.0;
                        stats.attack_range += 20.0;
                    }
                    HeroClass::Mage => {
                        stats.attack += 8.0;
                    }
                    HeroClass::Rogue => {
                        stats.speed += 5.0;
                        stats.attack += 4.0;
                    }
                    HeroClass::Healer => {
                        stats.max_hp += 15.0;
                    }
                }
            }

            alerts.push(format!("{} reached level {}!", hero.class.display_name(), hero.level));
        }
    }
}

/// System: Spawn heroes when buildings attract them (uses real sprites)
pub fn hero_attraction_system(
    mut commands: Commands,
    buildings: Query<(&Building, &Transform)>,
    heroes: Query<&Hero>,
    kingdom: Res<KingdomState>,
    bonuses: Res<BuildingBonuses>,
    sprites: Res<SpriteAssets>,
    game_time: Res<GameTime>,
    game_phase: Res<GamePhase>,
    time: Res<Time>,
    mut economy: ResMut<GameEconomy>,
    mut spawn_timer: Local<f32>,
    mut alerts: ResMut<GameAlerts>,
) {
    if !game_phase.game_started { return; }
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    *spawn_timer -= dt;
    if *spawn_timer > 0.0 {
        return;
    }
    *spawn_timer = 15.0 + rand::random::<f32>() * 15.0;

    let current_heroes = heroes.iter().count() as u32;
    let max = kingdom.rank.max_heroes() + bonuses.barracks_hero_cap_bonus;
    if current_heroes >= max {
        return;
    }

    let mut attracted_classes: Vec<(HeroClass, Vec2)> = Vec::new();
    for (building, transform) in buildings.iter() {
        if building.is_destroyed {
            continue;
        }
        let pos = Vec2::new(transform.translation.x, transform.translation.y);
        for class in building.building_type.attracts_heroes() {
            attracted_classes.push((class, pos));
        }
    }

    if attracted_classes.is_empty() {
        attracted_classes.push((HeroClass::Warrior, Vec2::ZERO));
    }

    let idx = (rand::random::<f32>() * attracted_classes.len() as f32) as usize;
    let idx = idx.min(attracted_classes.len() - 1);
    let (class, spawn_near) = attracted_classes[idx];

    // Check signing bonus requirement for high-tier heroes
    let bonus = get_signing_bonus(class);
    if bonus > 0.0 {
        if economy.gold < bonus {
            alerts.push(format!("Recruitment failed: {} requires {}g signing bonus (have {}g)", class.display_name(), bonus, economy.gold));
            return;
        }
        economy.gold -= bonus;
        economy.total_spent += bonus;
        alerts.push(format!("Paid {}g signing bonus for new {}!", bonus, class.display_name()));
    }

    let offset = Vec2::new(
        (rand::random::<f32>() - 0.5) * 60.0,
        (rand::random::<f32>() - 0.5) * 60.0,
    );
    let spawn_pos = spawn_near + offset;

    spawn_hero_with_sprite(
        &mut commands,
        &sprites,
        class,
        Vec3::new(spawn_pos.x, spawn_pos.y, 10.0),
    );

    alerts.push(format!("A new {} has arrived!", class.display_name()));
}

/// System: Morale decay over time, especially at night (reduced by Temple aura)
pub fn hero_morale_system(
    mut heroes: Query<(&mut Hero, &HeroState)>,
    building_bonuses: Res<BuildingBonuses>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (mut hero, state) in heroes.iter_mut() {
        // Morale decays slowly
        let mut decay = if game_time.is_night() { 1.0 } else { 0.3 };

        // Temple aura reduces decay rate (multiplier effect)
        if building_bonuses.temple_morale_aura > 0.0 {
            decay /= building_bonuses.temple_morale_aura;
        }

        hero.morale = (hero.morale - decay * dt).max(0.0);

        // Resting restores morale
        if matches!(state, HeroState::Resting) {
            hero.morale = (hero.morale + 5.0 * dt).min(100.0);
        }
    }
}

/// Sanctuary radius in world units
const SANCTUARY_RADIUS: f32 = 150.0;
/// Channel duration for Sanctuary (seconds)
const SANCTUARY_CHANNEL: f32 = 2.0;
/// Cooldown duration for Sanctuary (seconds)
const SANCTUARY_COOLDOWN: f32 = 20.0;

/// System: Healer AI decides to cast Sanctuary when dead allies are nearby
pub fn healer_sanctuary_ai_system(
    mut query_set: QuerySet<(
        // q0: mutable access to healers (state, cooldown)
        QueryState<(Entity, &Hero, &mut HeroState, &Transform, &mut SanctuaryCooldown)>,
        // q1: immutable snapshot of all heroes' states and positions
        QueryState<(&HeroState, &Transform)>,
    )>,
    time: Res<Time>,
    game_time: Res<GameTime>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;

    // First, collect positions of all dead heroes (immutable snapshot)
    let dead_positions: Vec<Vec2> = query_set
        .q1()
        .iter()
        .filter(|(state, _)| matches!(state, HeroState::Dead { .. }))
        .map(|(_, transform)| Vec2::new(transform.translation.x, transform.translation.y))
        .collect();

    // Then process each healer with mutable access
    for (_healer_entity, hero, mut state, transform, mut cooldown) in query_set.q0().iter_mut() {
        if hero.class != HeroClass::Healer {
            continue;
        }

        // Update cooldown timer
        cooldown.timer -= dt;
        if cooldown.timer > 0.0 {
            continue;
        }

        // Skip if already casting
        if matches!(&*state, HeroState::Casting { .. }) {
            continue;
        }

        // Check if any dead hero is within sanctuary radius
        let healer_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let dead_nearby = dead_positions.iter().any(|dead_pos| {
            (*dead_pos - healer_pos).length() <= SANCTUARY_RADIUS
        });

        if dead_nearby {
            // Begin casting Sanctuary
            *state = HeroState::Casting {
                channel_elapsed: 0.0,
                channel_duration: SANCTUARY_CHANNEL,
                focus_entity: Entity::from_raw(0), // no specific target, area effect
            };
        }
    }
}

/// System: Healer progresses through Sanctuary channel, then emits revive event
pub fn healer_sanctuary_channel_system(
    mut healers: Query<(Entity, &Hero, &Transform, &mut SanctuaryCooldown)>,
    mut hero_states: Query<&mut HeroState, With<Hero>>,
    time: Res<Time>,
    game_time: Res<GameTime>,
    mut revive_events: EventWriter<SanctuaryReviveEvent>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (healer_entity, hero, healer_transform, mut cooldown) in healers.iter_mut() {
        if hero.class != HeroClass::Healer {
            continue;
        }

        if let Ok(mut state) = hero_states.get_mut(healer_entity) {
            if let HeroState::Casting {
                ref mut channel_elapsed,
                channel_duration,
                ..
            } = *state
            {
                let new_elapsed = *channel_elapsed + dt;

                // Continue channeling
                if new_elapsed < channel_duration {
                    *channel_elapsed = new_elapsed;
                } else {
                    // Channel complete — emit revive event
                    let healer_pos = Vec2::new(healer_transform.translation.x, healer_transform.translation.y);
                    revive_events.send(SanctuaryReviveEvent { position: healer_pos, healer: healer_entity });

                    // Reset cooldown and state
                    cooldown.timer = SANCTUARY_COOLDOWN;
                    *state = HeroState::Idle;
                }
            }
        }
    }
}

/// System: Process Sanctuary revive events to bring dead heroes back to life
pub fn sanctuary_revive_system(
    mut events: EventReader<SanctuaryReviveEvent>,
    mut heroes: Query<(&mut Hero, &mut HeroStats, &mut HeroState, &Transform)>,
    mut alerts: ResMut<GameAlerts>,
) {
    for event in events.iter() {
        let mut revived_count = 0;
        for (mut hero, mut stats, mut state, transform) in heroes.iter_mut() {
            // Only revive if currently dead
            if let HeroState::Dead { .. } = *state {
                let pos = Vec2::new(transform.translation.x, transform.translation.y);
                if (pos - event.position).length() <= SANCTUARY_RADIUS {
                    *state = HeroState::Idle;
                    stats.hp = stats.max_hp * 0.5;
                    hero.morale = 50.0;
                    revived_count += 1;
                }
            }
        }
        if revived_count > 0 {
            alerts.push(format!("Sanctuary revived {} hero{}!", revived_count, if revived_count > 1 { "s" } else { "" }));
        }
    }
}

// ============================================================
// ROGUE STEALTH SYSTEMS
// ============================================================

/// Stealth duration in seconds
const STEALTH_DURATION: f32 = 10.0;
/// Channel duration to enter stealth (seconds)
const STEALTH_CHANNEL: f32 = 1.0;
/// Cooldown after stealth ends (seconds)
const STEALTH_COOLDOWN: f32 = 15.0;

/// System: Rogue AI decides to enter stealth when out of combat and cooldown ready
pub fn rogue_stealth_ai_system(
    mut rogues: Query<(Entity, &Hero, &mut HeroState, &mut StealthCooldown)>,
    time: Res<Time>,
    game_time: Res<GameTime>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    for (_entity, hero, mut state, mut cooldown) in rogues.iter_mut() {
        if hero.class != HeroClass::Rogue {
            continue;
        }

        // Update cooldown timer
        cooldown.timer -= dt;
        if cooldown.timer > 0.0 {
            continue;
        }

        // Skip if already casting or in combat
        if matches!(&*state, HeroState::Casting { .. } | HeroState::AttackingEnemy { .. } | HeroState::PursuingBounty { .. }) {
            continue;
        }

        // Only stealth when idle or moving without urgent task
        if matches!(&*state, HeroState::Idle | HeroState::MovingTo { .. }) {
            // Begin channel to enter stealth
            *state = HeroState::Casting {
                channel_elapsed: 0.0,
                channel_duration: STEALTH_CHANNEL,
                focus_entity: Entity::from_raw(0),
            };
        }
    }
}

/// System: Rogue progresses through stealth channel, then becomes stealthed
pub fn rogue_stealth_channel_system(
    mut commands: Commands,
    mut rogues: Query<(Entity, &Hero, &Transform, &mut StealthCooldown)>,
    mut hero_states: Query<&mut HeroState, With<Hero>>,
    time: Res<Time>,
    game_time: Res<GameTime>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 { return; }

    for (rogue_entity, hero, _transform, mut cooldown) in rogues.iter_mut() {
        if hero.class != HeroClass::Rogue { continue; }

        if let Ok(mut state) = hero_states.get_mut(rogue_entity) {
            if let HeroState::Casting { ref mut channel_elapsed, channel_duration, .. } = *state {
                let new_elapsed = *channel_elapsed + dt;

                if new_elapsed < channel_duration {
                    *channel_elapsed = new_elapsed;
                } else {
                    // Channel complete — add Stealthed component
                    commands.entity(rogue_entity).insert(Stealthed { timer: STEALTH_DURATION });
                    // Start cooldown (prevents re-stealth too soon after this activation)
                    cooldown.timer = STEALTH_COOLDOWN;
                    *state = HeroState::Idle;
                }
            }
        }
    }
}

/// System: Tick down Stealthed timer and remove when expired or when rogue engages in combat
pub fn rogue_stealth_tick_system(
    mut commands: Commands,
    mut rogues: Query<(Entity, &Hero, &mut Stealthed)>,
    time: Res<Time>,
    game_time: Res<GameTime>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 { return; }

    for (entity, hero, mut stealthed) in rogues.iter_mut() {
        if hero.class != HeroClass::Rogue { continue; }

        stealthed.timer -= dt;
        if stealthed.timer <= 0.0 {
            commands.entity(entity).remove::<Stealthed>();
        }
    }
}

/// System: When a recovery bounty (Objective type) is completed, revive the target hero if dead.
pub fn recovery_revive_system(
    mut events: EventReader<BountyCompletedEvent>,
    mut heroes: Query<(&mut Hero, &mut HeroStats, &mut HeroState)>,
    mut alerts: ResMut<GameAlerts>,
) {
    for event in events.iter() {
        // Only process Objective bounties that target a specific hero
        if let Some(target_entity) = event.target_entity {
            if let Ok((mut hero, mut stats, mut state)) = heroes.get_mut(target_entity) {
                if let HeroState::Dead { .. } = *state {
                    *state = HeroState::Idle;
                    stats.hp = stats.max_hp * 0.5;
                    hero.morale = 70.0;
                    alerts.push(format!("Hero {} has been revived by a recovery bounty!", hero.class.display_name()));
                }
            }
        }
    }
}

/// System: Spawn and sync golden glow sprites for legendary heroes
pub fn legendary_hero_glow_system(
    mut commands: Commands,
    heroes: Query<(Entity, &Hero, &Transform), Without<LegendaryGlow>>,
    mut glows: Query<(Entity, &LegendaryGlow, &mut Sprite, &mut Transform)>,
    time: Res<Time>,
) {
    let pulse = ((time.seconds_since_startup() as f32) * 2.0).sin() * 0.05 + 0.35;

    // Spawn glow for newly legendary heroes
    for (hero_entity, hero, hero_t) in heroes.iter() {
        if !hero.is_legendary {
            continue;
        }
        let already_has = glows
            .iter()
            .any(|(_, glow, _, _)| glow.parent_hero == hero_entity);
        if already_has {
            continue;
        }
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 0.84, 0.0, pulse),
                custom_size: Some(Vec2::new(70.0, 70.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                hero_t.translation.x,
                hero_t.translation.y,
                8.5,
            )),
            ..Default::default()
        })
        .insert(LegendaryGlow {
            parent_hero: hero_entity,
        });
    }

    // Update glow position + pulse; despawn if hero no longer legendary
    for (glow_entity, glow, mut sprite, mut glow_t) in glows.iter_mut() {
        if let Ok((_, hero, hero_t)) = heroes.get(glow.parent_hero) {
            if hero.is_legendary {
                glow_t.translation.x = hero_t.translation.x;
                glow_t.translation.y = hero_t.translation.y;
                sprite.color = Color::rgba(1.0, 0.84, 0.0, pulse);
            } else {
                commands.entity(glow_entity).despawn();
            }
        } else {
            commands.entity(glow_entity).despawn();
        }
    }
}


use std::collections::HashSet;

/// System: Emergent gameplay - heroes can fall in love at the inn
/// When two or more heroes are resting at the same inn (near each other),
/// they gain a small morale boost over time.
pub fn hero_love_system(
    resting_query: Query<(Entity, &HeroState, &Transform)>,
    mut heroes: Query<&mut Hero>,
    mut alerts: ResMut<GameAlerts>,
    time: Res<Time>,
    game_time: Res<GameTime>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    // Collect all resting heroes with their positions
    let mut resting: Vec<(Entity, Vec2)> = Vec::new();
    for (entity, state, transform) in resting_query.iter() {
        if let HeroState::Resting = state {
            let pos = Vec2::new(transform.translation.x, transform.translation.y);
            resting.push((entity, pos));
        }
    }

    let love_radius = 50.0; // within this distance, heroes are "together"
    let morale_boost = 3.0 * dt; // boost per tick per pair (but applied once per hero per detection)

    // Find all pairs of heroes within love radius
    let mut paired_entities = HashSet::new();
    for i in 0..resting.len() {
        let (entity_i, pos_i) = resting[i];
        for j in (i + 1)..resting.len() {
            let (entity_j, pos_j) = resting[j];
            if pos_i.distance(pos_j) < love_radius {
                paired_entities.insert(entity_i);
                paired_entities.insert(entity_j);
            }
        }
    }

    // Apply morale boost to all heroes that have at least one partner
    for &entity in paired_entities.iter() {
        if let Ok(mut hero) = heroes.get_mut(entity) {
            hero.morale = (hero.morale + morale_boost).min(100.0);
        }
    }

    // Occasionally send an alert (small chance per tick)
    if !paired_entities.is_empty() && rand::random::<f32>() < 0.002 {
        // Find first pair and alert
        'outer: for i in 0..resting.len() {
            let (entity_i, pos_i) = resting[i];
            for j in (i + 1)..resting.len() {
                let (entity_j, pos_j) = resting[j];
                if pos_i.distance(pos_j) < love_radius {
                    if let (Ok(hero_i), Ok(hero_j)) = (heroes.get(entity_i), heroes.get(entity_j)) {
                        let name_i = hero_i.class.display_name();
                        let name_j = hero_j.class.display_name();
                        alerts.push(format!("{} and {} are in love! (+morale)", name_i, name_j));
                        break 'outer;
                    }
                }
            }
        }
    }
}
