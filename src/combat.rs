use bevy::prelude::*;
use crate::components::*;

/// System: Heroes attack enemies they're targeting
pub fn hero_attack_system(
    mut heroes: Query<(Entity, &Hero, &HeroStats, &HeroState, &mut AttackCooldown, &Transform)>,
    mut enemies: Query<(&mut EnemyStats, &Transform), (With<Enemy>, Without<Hero>)>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (_hero_entity, hero, stats, state, mut cooldown, hero_transform) in heroes.iter_mut() {
        cooldown.timer -= dt;

        if let HeroState::AttackingEnemy { target_entity } = state {
            if cooldown.timer > 0.0 {
                continue;
            }

            if let Ok((mut enemy_stats, enemy_transform)) = enemies.get_mut(*target_entity) {
                let hero_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.y);
                let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
                let dist = (enemy_pos - hero_pos).length();

                if dist <= stats.attack_range && enemy_stats.hp > 0.0 {
                    // Calculate damage
                    let mut damage = stats.attack - enemy_stats.defense;

                    // Class-specific bonuses
                    match hero.class {
                        HeroClass::Rogue => {
                            // Backstab: extra crit chance
                            if rand::random::<f32>() < 0.3 {
                                damage *= 2.0;
                            }
                        }
                        HeroClass::Warrior => {
                            // Fortify: slight damage reduction on self (handled elsewhere)
                            damage *= 1.1; // Consistent damage
                        }
                        HeroClass::Mage => {
                            // Arcane Surge: AoE potential (simplified to bonus damage)
                            damage *= 1.3;
                        }
                        HeroClass::Archer => {
                            // Volley: bonus at range
                            if dist > 80.0 {
                                damage *= 1.2;
                            }
                        }
                        _ => {}
                    }

                    damage = damage.max(1.0);
                    enemy_stats.hp -= damage;

                    cooldown.timer = cooldown.interval;
                }
            }
        }
    }
}

/// System: Enemies attack heroes and buildings
pub fn enemy_attack_system(
    mut enemies: Query<(&Enemy, &EnemyStats, &EnemyAi, &mut AttackCooldown, &Transform)>,
    mut heroes: Query<(Entity, &mut HeroStats, &mut HeroState, &Transform), (With<Hero>, Without<Enemy>)>,
    mut buildings: Query<(&mut Building, &Transform), (Without<Hero>, Without<Enemy>)>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    for (_enemy, stats, ai, mut cooldown, enemy_transform) in enemies.iter_mut() {
        if stats.hp <= 0.0 {
            continue;
        }

        cooldown.timer -= dt;
        if cooldown.timer > 0.0 {
            continue;
        }

        let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);

        if let Some(target) = ai.target {
            // Try to attack hero
            if let Ok((_hero_entity, mut hero_stats, mut hero_state, hero_transform)) = heroes.get_mut(target) {
                let hero_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.y);
                let dist = (hero_pos - enemy_pos).length();

                if dist <= stats.attack_range && hero_stats.hp > 0.0 {
                    let damage = (stats.attack - hero_stats.defense).max(1.0);
                    hero_stats.hp -= damage;

                    if hero_stats.hp <= 0.0 {
                        *hero_state = HeroState::Dead { respawn_timer: 30.0 };
                    }

                    cooldown.timer = cooldown.interval;
                    continue;
                }
            }

            // Try to attack building
            if let Ok((mut building, building_transform)) = buildings.get_mut(target) {
                let bpos = Vec2::new(building_transform.translation.x, building_transform.translation.y);
                let dist = (bpos - enemy_pos).length();

                if dist <= stats.attack_range + 30.0 && !building.is_destroyed {
                    let damage = stats.attack;
                    building.hp -= damage;

                    if building.hp <= 0.0 {
                        building.is_destroyed = true;
                        building.hp = 0.0;
                    }

                    cooldown.timer = cooldown.interval;
                }
            }
        }
    }
}

/// System: Healer heroes heal nearby allies
/// Uses QuerySet to avoid conflicting access to HeroStats
pub fn healer_system(
    mut query_set: QuerySet<(
        // q0: read healer info
        QueryState<(&Hero, &HeroStats, &Transform, &mut AttackCooldown)>,
        // q1: write ally HP
        QueryState<(&mut HeroStats, &Transform)>,
    )>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    // Phase 1: Collect healer info from q0
    let mut heal_targets: Vec<(Vec2, f32, f32)> = Vec::new();
    for (hero, healer_stats, healer_transform, mut cooldown) in query_set.q0().iter_mut() {
        if hero.class != HeroClass::Healer {
            continue;
        }

        cooldown.timer -= dt;
        if cooldown.timer > 0.0 {
            continue;
        }
        cooldown.timer = cooldown.interval;

        let healer_pos = Vec2::new(healer_transform.translation.x, healer_transform.translation.y);
        let heal_amount = healer_stats.attack * 2.0;
        heal_targets.push((healer_pos, healer_stats.attack_range, heal_amount));
    }

    // Phase 2: Apply heals via q1
    for (healer_pos, range, heal_amount) in heal_targets {
        for (mut ally_stats, ally_transform) in query_set.q1().iter_mut() {
            let ally_pos = Vec2::new(ally_transform.translation.x, ally_transform.translation.y);
            let dist = (ally_pos - healer_pos).length();

            if dist < range && ally_stats.hp < ally_stats.max_hp * 0.8 {
                ally_stats.hp = (ally_stats.hp + heal_amount).min(ally_stats.max_hp);
                break;
            }
        }
    }
}

/// System: Award XP and gold when enemies die
pub fn enemy_reward_system(
    mut events: EventReader<EnemyDeathEvent>,
    mut heroes: Query<(&mut Hero, &HeroStats, &Transform)>,
    mut economy: ResMut<GameEconomy>,
    mut alerts: ResMut<GameAlerts>,
) {
    for event in events.iter() {
        // Give XP to nearby heroes
        for (mut hero, _stats, _transform) in heroes.iter_mut() {
            // Simplified: all heroes get some XP from kills (shared XP)
            hero.xp += event.xp_reward * 0.5;
        }

        // Gold goes to treasury
        economy.gold += event.gold_reward;
        economy.total_earned += event.gold_reward;

        if event.gold_reward >= 50.0 {
            alerts.push(format!("Enemy slain! +{:.0} gold", event.gold_reward));
        }
    }
}
