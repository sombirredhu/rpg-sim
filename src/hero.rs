use bevy::prelude::*;
use crate::components::*;
use std::f32::consts::TAU;

/// System: Hero AI decision-making
/// Each hero evaluates bounties, threats, and personal needs to decide their next action
pub fn hero_ai_system(
    mut heroes: Query<(Entity, &mut Hero, &mut HeroStats, &mut HeroState, &mut HeroDecisionTimer, &Transform)>,
    bounty_board: Res<BountyBoard>,
    game_time: Res<GameTime>,
    enemies: Query<(Entity, &Transform, &EnemyStats), With<Enemy>>,
    buildings: Query<(Entity, &Transform, &Building)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (_hero_entity, mut hero, stats, mut state, mut decision_timer, transform) in heroes.iter_mut() {
        // Handle dead heroes
        if let HeroState::Dead { respawn_timer } = &mut *state {
            *respawn_timer -= dt;
            if *respawn_timer <= 0.0 {
                *state = HeroState::Idle;
                stats.into_inner().hp = stats.max_hp * 0.5;
                hero.morale = 50.0;
            }
            continue;
        }

        // Decision timer - heroes reconsider every 2-3 seconds
        decision_timer.0 -= dt;
        if decision_timer.0 > 0.0 {
            continue;
        }
        decision_timer.0 = 2.0 + rand::random::<f32>() * 1.0;

        let hero_pos = Vec2::new(transform.translation.x, transform.translation.y);

        // Priority 1: If HP is very low, seek inn/rest
        if stats.hp < stats.max_hp * 0.25 {
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
            *state = HeroState::Resting;
            continue;
        }

        // Priority 3: Evaluate available bounties
        let available = bounty_board.available_bounties();
        if !available.is_empty() {
            // Score each bounty based on hero AI factors
            let mut best_bounty: Option<(u32, f32, Vec2)> = None;
            let mut best_score = f32::MIN;

            for bounty in &available {
                let distance = (bounty.location - hero_pos).length();
                let danger = bounty.danger_level as f32;

                // Base score from gold reward
                let mut score = bounty.gold_reward;

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
                    best_bounty = Some((bounty.id, score, bounty.location));
                }
            }

            if let Some((bounty_id, _score, location)) = best_bounty {
                if best_score > 0.0 {
                    *state = HeroState::PursuingBounty { bounty_id };
                    // If not at bounty location, move there
                    if (location - hero_pos).length() > 40.0 {
                        *state = HeroState::MovingTo { target: location };
                    }
                    continue;
                }
            }
        }

        // Priority 4: Attack nearby enemies
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
            *state = HeroState::AttackingEnemy { target_entity: enemy_entity };
            continue;
        }

        // Priority 5: Idle - wander randomly
        let angle = rand::random::<f32>() * TAU;
        let wander_target = hero_pos + Vec2::new(angle.cos(), angle.sin()) * 60.0;
        *state = HeroState::MovingTo { target: wander_target };
    }
}

/// System: Move heroes based on their current state
pub fn hero_movement_system(
    mut heroes: Query<(&Hero, &HeroStats, &mut HeroState, &mut Transform)>,
    enemies: Query<&Transform, (With<Enemy>, Without<Hero>)>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (_hero, stats, mut state, mut transform) in heroes.iter_mut() {
        match &*state {
            HeroState::MovingTo { target } => {
                let pos = Vec2::new(transform.translation.x, transform.translation.y);
                let dir = *target - pos;
                let dist = dir.length();

                if dist < 5.0 {
                    *state = HeroState::Idle;
                } else {
                    let move_dir = dir.normalize();
                    let speed = stats.speed * dt;
                    transform.translation.x += move_dir.x * speed;
                    transform.translation.y += move_dir.y * speed;

                    // Flip sprite based on direction
                    if move_dir.x < 0.0 {
                        transform.scale.x = -transform.scale.x.abs();
                    } else {
                        transform.scale.x = transform.scale.x.abs();
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
                        let speed = stats.speed * dt;
                        transform.translation.x += move_dir.x * speed;
                        transform.translation.y += move_dir.y * speed;
                    }
                } else {
                    // Enemy no longer exists
                    *state = HeroState::Idle;
                }
            }
            HeroState::Resting => {
                // Stay still while resting
            }
            _ => {}
        }
    }
}

/// System: Hero resting at inn restores HP and morale
pub fn hero_rest_system(
    mut heroes: Query<(&mut Hero, &mut HeroStats, &mut HeroState)>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (mut hero, mut stats, mut state) in heroes.iter_mut() {
        if let HeroState::Resting = *state {
            // Heal 10% HP per second while resting
            stats.hp = (stats.hp + stats.max_hp * 0.1 * dt).min(stats.max_hp);
            // Restore morale
            hero.morale = (hero.morale + 5.0 * dt).min(100.0);

            // Stop resting when fully healed
            if stats.hp >= stats.max_hp * 0.9 && hero.morale >= 70.0 {
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

/// System: Spawn heroes when buildings attract them
pub fn hero_attraction_system(
    mut commands: Commands,
    buildings: Query<(&Building, &Transform)>,
    heroes: Query<&Hero>,
    kingdom: Res<KingdomState>,
    game_time: Res<GameTime>,
    time: Res<Time>,
    mut spawn_timer: Local<f32>,
    mut alerts: ResMut<GameAlerts>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    *spawn_timer -= dt;
    if *spawn_timer > 0.0 {
        return;
    }
    *spawn_timer = 15.0 + rand::random::<f32>() * 15.0; // Every 15-30 seconds

    let current_heroes = heroes.iter().count() as u32;
    if current_heroes >= kingdom.rank.max_heroes() {
        return;
    }

    // Gather which classes are attracted by current buildings
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
        // Always have a chance for a basic warrior
        attracted_classes.push((HeroClass::Warrior, Vec2::ZERO));
    }

    // Pick a random class from available attractions
    let idx = (rand::random::<f32>() * attracted_classes.len() as f32) as usize;
    let idx = idx.min(attracted_classes.len() - 1);
    let (class, spawn_near) = attracted_classes[idx];

    // Spawn near the building that attracted them
    let offset = Vec2::new(
        (rand::random::<f32>() - 0.5) * 60.0,
        (rand::random::<f32>() - 0.5) * 60.0,
    );
    let spawn_pos = spawn_near + offset;

    let hero = Hero::new(class);
    let stats = class.base_stats();
    let color = class.color();

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(16.0, 24.0)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(spawn_pos.x, spawn_pos.y, 10.0)),
        ..Default::default()
    })
    .insert(hero)
    .insert(stats)
    .insert(HeroState::Idle)
    .insert(HeroDecisionTimer::default())
    .insert(AttackCooldown::default());

    alerts.push(format!("A new {} has arrived!", class.display_name()));
}

/// System: Morale decay over time, especially at night
pub fn hero_morale_system(
    mut heroes: Query<(&mut Hero, &HeroState)>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (mut hero, state) in heroes.iter_mut() {
        // Morale decays slowly
        let decay = if game_time.is_night() { 1.0 } else { 0.3 };
        hero.morale = (hero.morale - decay * dt).max(0.0);

        // Resting restores morale
        if matches!(state, HeroState::Resting) {
            hero.morale = (hero.morale + 5.0 * dt).min(100.0);
        }
    }
}
