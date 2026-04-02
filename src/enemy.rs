use bevy::prelude::*;
use crate::components::*;
use std::f32::consts::TAU;

/// System: Spawn enemies from monster dens
pub fn monster_den_spawn_system(
    mut commands: Commands,
    mut dens: Query<(Entity, &mut MonsterDen, &Transform)>,
    _enemies: Query<&Enemy>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (_entity, mut den, transform) in dens.iter_mut() {
        den.spawn_timer -= dt;

        // Spawn faster at night
        let interval = den.spawn_interval / game_time.threat_multiplier();

        if den.spawn_timer <= 0.0 && den.current_spawned < den.max_spawned {
            den.spawn_timer = interval;

            let den_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let offset = Vec2::new(
                (rand::random::<f32>() - 0.5) * 40.0,
                (rand::random::<f32>() - 0.5) * 40.0,
            );
            let spawn_pos = den_pos + offset;

            let enemy_type = den.enemy_type;
            let stats = enemy_type.stats();
            let color = enemy_type.color();

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(14.0, 20.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(spawn_pos.x, spawn_pos.y, 8.0)),
                ..Default::default()
            })
            .insert(Enemy { enemy_type })
            .insert(stats)
            .insert(EnemyAi::default())
            .insert(AttackCooldown { timer: 0.0, interval: 1.5 });

            den.current_spawned += 1;
        }
    }
}

/// System: Enemy AI - wander and attack heroes/buildings
pub fn enemy_ai_system(
    mut enemies: Query<(Entity, &Enemy, &EnemyStats, &mut EnemyAi, &mut Transform), Without<Hero>>,
    heroes: Query<(Entity, &Transform), (With<Hero>, Without<Enemy>)>,
    buildings: Query<(Entity, &Transform, &Building), (Without<Enemy>, Without<Hero>)>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (_entity, enemy, stats, mut ai, mut transform) in enemies.iter_mut() {
        if stats.hp <= 0.0 {
            continue;
        }

        let pos = Vec2::new(transform.translation.x, transform.translation.y);

        // Look for nearby heroes or buildings to attack
        let mut nearest_target: Option<(Entity, Vec2, f32)> = None;

        // Check heroes
        for (hero_entity, hero_transform) in heroes.iter() {
            let hero_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.y);
            let dist = (hero_pos - pos).length();
            if dist < 200.0 {
                if nearest_target.is_none() || dist < nearest_target.unwrap().2 {
                    nearest_target = Some((hero_entity, hero_pos, dist));
                }
            }
        }

        // Check buildings (especially at night or if enemy is a bandit)
        if nearest_target.is_none() || enemy.enemy_type == EnemyType::Bandit {
            for (_building_entity, building_transform, building) in buildings.iter() {
                if building.is_destroyed {
                    continue;
                }
                let bpos = Vec2::new(building_transform.translation.x, building_transform.translation.y);
                let dist = (bpos - pos).length();
                if dist < 300.0 && (game_time.is_night() || enemy.enemy_type == EnemyType::Bandit) {
                    if nearest_target.is_none() || dist < nearest_target.unwrap().2 * 0.5 {
                        nearest_target = Some((_building_entity, bpos, dist));
                    }
                }
            }
        }

        if let Some((_target_entity, target_pos, dist)) = nearest_target {
            ai.target = Some(_target_entity);
            // Move toward target
            if dist > stats.attack_range {
                let dir = (target_pos - pos).normalize();
                transform.translation.x += dir.x * stats.speed * dt;
                transform.translation.y += dir.y * stats.speed * dt;

                if dir.x < 0.0 {
                    transform.scale.x = -transform.scale.x.abs();
                } else {
                    transform.scale.x = transform.scale.x.abs();
                }
            }
        } else {
            // Wander
            ai.wander_timer -= dt;
            if ai.wander_timer <= 0.0 {
                ai.wander_angle = rand::random::<f32>() * TAU;
                ai.wander_timer = 2.0 + rand::random::<f32>() * 3.0;
            }

            let dir = Vec2::new(ai.wander_angle.cos(), ai.wander_angle.sin());
            transform.translation.x += dir.x * stats.speed * 0.3 * dt;
            transform.translation.y += dir.y * stats.speed * 0.3 * dt;
        }
    }
}

/// System: Threat escalation - dens get stronger over time
pub fn threat_escalation_system(
    mut dens: Query<&mut MonsterDen>,
    game_time: Res<GameTime>,
    time: Res<Time>,
    mut escalation_timer: Local<f32>,
    mut alerts: ResMut<GameAlerts>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    *escalation_timer -= dt;

    // Check every "game week" (about 56 seconds at 1x speed = 7 game-days * 8 seconds per minute)
    if *escalation_timer > 0.0 {
        return;
    }
    *escalation_timer = game_time.day_length; // One game-day between escalation checks

    for mut den in dens.iter_mut() {
        den.weeks_unaddressed += 1;

        // Escalate every 7 game-days
        if den.weeks_unaddressed >= 7 && den.threat_tier < 3 {
            den.threat_tier += 1;
            den.max_spawned += 2;
            den.spawn_interval *= 0.8; // Spawn faster
            den.weeks_unaddressed = 0;

            // Upgrade enemy type at tier 2
            if den.threat_tier >= 2 && den.enemy_type == EnemyType::Goblin {
                den.enemy_type = EnemyType::GoblinElite;
            }

            alerts.push(format!(
                "Threat escalated! {} den is now tier {}!",
                den.enemy_type.display_name(),
                den.threat_tier
            ));
        }
    }
}

/// System: Boss raid events
pub fn boss_raid_system(
    mut commands: Commands,
    game_time: Res<GameTime>,
    kingdom: Res<KingdomState>,
    time: Res<Time>,
    mut boss_timer: Local<f32>,
    _boss_spawned: Local<bool>,
    mut alerts: ResMut<GameAlerts>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    *boss_timer -= dt;

    if *boss_timer > 0.0 {
        return;
    }

    // Boss raid every ~5 game-days (only at City rank+)
    *boss_timer = game_time.day_length * 5.0;

    if matches!(kingdom.rank, KingdomRank::City | KingdomRank::Kingdom) {
        // Spawn boss at map edge
        let angle = rand::random::<f32>() * TAU;
        let spawn_pos = Vec2::new(angle.cos() * 500.0, angle.sin() * 500.0);

        let stats = EnemyType::BossWarlord.stats();
        let color = EnemyType::BossWarlord.color();

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(32.0, 40.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(spawn_pos.x, spawn_pos.y, 9.0)),
            ..Default::default()
        })
        .insert(Enemy { enemy_type: EnemyType::BossWarlord })
        .insert(stats)
        .insert(EnemyAi::default())
        .insert(AttackCooldown { timer: 0.0, interval: 2.0 });

        alerts.push("⚠ BOSS RAID! A Warlord approaches the kingdom!".to_string());
    }
}

/// Startup: Spawn initial monster dens around the map
pub fn spawn_initial_dens(
    mut commands: Commands,
) {
    let den_positions = [
        (Vec2::new(300.0, 200.0), EnemyType::Goblin),
        (Vec2::new(-350.0, -200.0), EnemyType::Goblin),
        (Vec2::new(200.0, -300.0), EnemyType::Bandit),
        (Vec2::new(-250.0, 350.0), EnemyType::Bandit),
    ];

    for (pos, enemy_type) in den_positions {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.1, 0.1),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 4.0)),
            ..Default::default()
        })
        .insert(MonsterDen::new(enemy_type));
    }
}

/// System: Clean up dead enemies
pub fn enemy_death_system(
    mut commands: Commands,
    enemies: Query<(Entity, &EnemyStats, &Enemy)>,
    mut dens: Query<&mut MonsterDen>,
    mut events: EventWriter<EnemyDeathEvent>,
) {
    for (entity, stats, enemy) in enemies.iter() {
        if stats.hp <= 0.0 {
            events.send(EnemyDeathEvent {
                enemy_entity: entity,
                xp_reward: enemy.enemy_type.stats().xp_reward,
                gold_reward: enemy.enemy_type.stats().gold_reward,
                killer: None,
            });

            // Decrement den spawn count
            for mut den in dens.iter_mut() {
                if den.enemy_type == enemy.enemy_type && den.current_spawned > 0 {
                    den.current_spawned -= 1;
                    den.weeks_unaddressed = 0; // Reset escalation when enemies are killed
                    break;
                }
            }

            commands.entity(entity).despawn();
        }
    }
}
