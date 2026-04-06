use bevy::prelude::*;
use crate::components::*;
use crate::sprites::{
    SpriteAssets,
    monster_den_scale_for_tier,
    monster_den_texture_for_tier,
    spawn_enemy_with_sprite,
};
use crate::map_layout::MAP_HALF_EXTENT;
use std::f32::consts::TAU;

/// System: Spawn enemies from monster dens using real sprites
pub fn monster_den_spawn_system(
    mut commands: Commands,
    mut dens: Query<(Entity, &mut MonsterDen, &Transform)>,
    sprites: Res<SpriteAssets>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (_entity, mut den, transform) in dens.iter_mut() {
        den.spawn_timer -= dt;

        let interval = den.spawn_interval / game_time.threat_multiplier();

        if den.spawn_timer <= 0.0 && den.current_spawned < den.max_spawned {
            den.spawn_timer = interval;

            let den_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let offset = Vec2::new(
                (rand::random::<f32>() - 0.5) * 40.0,
                (rand::random::<f32>() - 0.5) * 40.0,
            );
            let spawn_pos = den_pos + offset;

            spawn_enemy_with_sprite(
                &mut commands,
                &sprites,
                den.enemy_type,
                Vec3::new(spawn_pos.x, spawn_pos.y, 8.0),
            );

            den.current_spawned += 1;
        }
    }
}

/// System: Enemy AI - wander and attack heroes/buildings
pub fn enemy_ai_system(
    mut enemies: Query<(Entity, &Enemy, &EnemyStats, &mut EnemyAi, &mut Transform, Option<&mut SpriteAnimation>), Without<Hero>>,
    heroes: Query<(Entity, &Transform), (With<Hero>, Without<Enemy>, Without<Stealthed>)>,
    buildings: Query<(Entity, &Transform, &Building), (Without<Enemy>, Without<Hero>)>,
    caravans: Query<(Entity, &TradeCaravan, &Transform)>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (_entity, enemy, stats, mut ai, mut transform, mut anim_opt) in enemies.iter_mut() {
        if stats.hp <= 0.0 {
            continue;
        }

        let pos = Vec2::new(transform.translation.x, transform.translation.y);
        let mut nearest_target: Option<(Entity, Vec2, f32)> = None;

        for (hero_entity, hero_transform) in heroes.iter() {
            let hero_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.y);
            let dist = (hero_pos - pos).length();
            if dist < 200.0 {
                if nearest_target.is_none() || dist < nearest_target.unwrap().2 {
                    nearest_target = Some((hero_entity, hero_pos, dist));
                }
            }
        }

        if nearest_target.is_none() || enemy.enemy_type == EnemyType::Bandit {
            // Consider caravans as targets (range 200)
            for (caravan_entity, caravan, caravan_transform) in caravans.iter() {
                if caravan.has_arrived {
                    continue; // Safe in town
                }
                let cpos = Vec2::new(caravan_transform.translation.x, caravan_transform.translation.y);
                let dist = (cpos - pos).length();
                if dist < 200.0 {
                    if nearest_target.is_none() || dist < nearest_target.unwrap().2 {
                        nearest_target = Some((caravan_entity, cpos, dist));
                    }
                }
            }

            // Consider buildings as targets (range 300, but only at night or bandits)
            for (building_entity, building_transform, building) in buildings.iter() {
                if building.is_destroyed {
                    continue;
                }
                let bpos = Vec2::new(building_transform.translation.x, building_transform.translation.y);
                let dist = (bpos - pos).length();
                if dist < 300.0 && (game_time.is_night() || enemy.enemy_type == EnemyType::Bandit) {
                    if nearest_target.is_none() || dist < nearest_target.unwrap().2 * 0.5 {
                        nearest_target = Some((building_entity, bpos, dist));
                    }
                }
            }
        }

        if let Some((target_entity, target_pos, dist)) = nearest_target {
            ai.target = Some(target_entity);
            if dist > stats.attack_range {
                let dir = (target_pos - pos).normalize();
                transform.translation.x += dir.x * stats.speed * dt;
                transform.translation.y += dir.y * stats.speed * dt;

                // Update direction: use SpriteAnimation row for LPC enemies, flip for others
                if let Some(ref mut anim) = anim_opt {
                    if dir.y.abs() > dir.x.abs() {
                        anim.row_offset = if dir.y > 0.0 { 0 } else { 2 };
                    } else {
                        anim.row_offset = if dir.x < 0.0 { 1 } else { 3 };
                    }
                } else {
                    let base_scale = transform.scale.x.abs();
                    if dir.x < 0.0 {
                        transform.scale.x = -base_scale;
                    } else {
                        transform.scale.x = base_scale;
                    }
                }
            }
        } else {
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

    if *escalation_timer > 0.0 {
        return;
    }
    *escalation_timer = game_time.day_length;

    for mut den in dens.iter_mut() {
        den.weeks_unaddressed += 1;

        if den.weeks_unaddressed >= 7 && den.threat_tier < 3 {
            den.threat_tier += 1;
            den.max_spawned += 2;
            den.spawn_interval *= 0.8;
            den.weeks_unaddressed = 0;

            if den.threat_tier >= 2 && den.enemy_type == EnemyType::Goblin {
                den.enemy_type = EnemyType::GoblinElite;
            }

            // At tier 3, the den spawns a Goblin Warlord ( BossWarlord ) that raids the town directly
            if den.threat_tier >= 3 {
                den.enemy_type = EnemyType::BossWarlord;
            }

            alerts.push(format!(
                "Threat escalated! {} den is now tier {}!",
                den.enemy_type.display_name(),
                den.threat_tier
            ));
        }
    }
}

/// System: Boss raid events - now with real boss sprite
pub fn boss_raid_system(
    mut commands: Commands,
    game_time: Res<GameTime>,
    kingdom: Res<KingdomState>,
    sprites: Res<SpriteAssets>,
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

    *boss_timer = game_time.day_length * 5.0;

    if matches!(kingdom.rank, KingdomRank::City | KingdomRank::Kingdom) {
        let angle = rand::random::<f32>() * TAU;
        let spawn_pos = Vec2::new(angle.cos() * 1200.0, angle.sin() * 1200.0);

        spawn_enemy_with_sprite(
            &mut commands,
            &sprites,
            EnemyType::BossWarlord,
            Vec3::new(spawn_pos.x, spawn_pos.y, 9.0),
        );

        alerts.push("BOSS RAID! A Warlord approaches the kingdom!".to_string());
    }
}

/// Startup: Spawn initial monster dens at structured zone positions.
pub fn spawn_initial_dens(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
) {
    let den_positions = crate::map_layout::CORE_MONSTER_DENS;

    for &(x, y, enemy_type) in den_positions {
        let pos = Vec2::new(x, y);
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

            for mut den in dens.iter_mut() {
                if den.enemy_type == enemy.enemy_type && den.current_spawned > 0 {
                    den.current_spawned -= 1;
                    den.weeks_unaddressed = 0;
                    break;
                }
            }

            commands.entity(entity).despawn();
        }
    }
}

/// System: Dynamically spawn enemies from map edges based on difficulty
/// Spawns enemy waves at intervals, scaling with game day and threat level.
pub fn edge_spawn_system(
    mut commands: Commands,
    game_time: Res<GameTime>,
    kingdom: Res<KingdomState>,
    sprites: Res<SpriteAssets>,
    time: Res<Time>,
    mut spawn_timer: Local<f32>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    // Spawn interval decreases with speed multiplier and scales with game progress
    let base_interval = 30.0; // seconds at 1x speed
    let interval = base_interval / game_time.speed_multiplier.max(1.0);
    *spawn_timer -= dt;
    if *spawn_timer <= 0.0 {
        *spawn_timer = interval;

        // Determine wave size based on current day and kingdom rank (difficulty)
        let day_factor = (game_time.current_day as f32 / 20.0).max(1.0);
        let rank_factor = match kingdom.rank {
            KingdomRank::Hamlet => 1.0,
            KingdomRank::Village => 1.2,
            KingdomRank::Town => 1.5,
            KingdomRank::City => 1.8,
            KingdomRank::Kingdom => 2.2,
        };
        let wave_size = (3.0 * day_factor * rank_factor).ceil() as usize;

        // Spawn enemies at random map edge positions
        for _ in 0..wave_size {
            // Choose a random side (0=top,1=bottom,2=right,3=left)
            let side = rand::random::<u8>() % 4;
            let map_edge = MAP_HALF_EXTENT as f32 - 50.0; // ensure within bounds
            let spawn_x = rand::random::<f32>() * map_edge * 2.0 - map_edge;
            let spawn_y = rand::random::<f32>() * map_edge * 2.0 - map_edge;
            let (x, y) = match side {
                0 => (spawn_x, map_edge),      // top
                1 => (spawn_x, -map_edge),     // bottom
                2 => (map_edge, spawn_y),      // right
                _ => (-map_edge, spawn_y),     // left
            };

            // Choose enemy type based on simple progression
            // Early game: Goblins; later: mix of Bandits, Trolls; end: tougher enemies
            let enemy_type = if game_time.current_day < 10 {
                EnemyType::Goblin
            } else if game_time.current_day < 25 {
                if rand::random::<f32>() < 0.3 {
                    EnemyType::Bandit
                } else {
                    EnemyType::Goblin
                }
            } else if game_time.current_day < 50 {
                // Include trolls occasionally
                let r = rand::random::<f32>();
                if r < 0.4 {
                    EnemyType::Goblin
                } else if r < 0.7 {
                    EnemyType::Bandit
                } else {
                    EnemyType::Troll
                }
            } else {
                // Late game: elites
                let r = rand::random::<f32>();
                if r < 0.3 {
                    EnemyType::GoblinElite
                } else if r < 0.6 {
                    EnemyType::Bandit
                } else {
                    EnemyType::Troll
                }
            };

            spawn_enemy_with_sprite(
                &mut commands,
                &sprites,
                enemy_type,
                Vec3::new(x, y, 10.0),
            );
        }
    }
}
