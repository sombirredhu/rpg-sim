use bevy::prelude::*;
use crate::components::*;

/// System: Heroes attack enemies they're targeting
pub fn hero_attack_system(
    mut heroes: Query<(Entity, &Hero, &HeroStats, &HeroEquipment, &HeroState, &mut AttackCooldown, &Transform)>,
    mut enemies: Query<(Entity, &mut EnemyStats, &Transform), (With<Enemy>, Without<Hero>)>,
    active_buffs: Res<ActiveBuffs>,
    game_time: Res<GameTime>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds() * game_time.speed_multiplier;
    if dt == 0.0 {
        return;
    }

    // Pre-collect enemy positions for Archer Volley AoE lookups
    let enemy_positions: Vec<(Entity, Vec2, f32)> = {
        let mut positions = Vec::new();
        for (e, stats, t) in enemies.iter_mut() {
            positions.push((e, Vec2::new(t.translation.x, t.translation.y), stats.hp));
        }
        positions
    };

    // Collect deferred damage: (entity, damage) for AoE splash
    let mut volley_splashes: Vec<(Entity, f32)> = Vec::new();

    for (_hero_entity, hero, stats, equipment, state, mut cooldown, hero_transform) in heroes.iter_mut() {
        cooldown.timer -= dt;

        if let HeroState::AttackingEnemy { target_entity } = state {
            if cooldown.timer > 0.0 {
                continue;
            }

            if let Ok((_entity, mut enemy_stats, enemy_transform)) = enemies.get_mut(*target_entity) {
                let hero_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.y);
                let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
                let dist = (enemy_pos - hero_pos).length();

                if dist <= stats.attack_range && enemy_stats.hp > 0.0 {
                    // Calculate damage (base + equipment bonuses)
                    let total_attack = stats.attack + equipment.total_atk_bonus() + active_buffs.atk_bonus;
                    let mut damage = total_attack - enemy_stats.defense;

                    // Class-specific bonuses
                    match hero.class {
                        HeroClass::Rogue => {
                            // Backstab: extra crit chance
                            if rand::random::<f32>() < 0.3 {
                                damage *= 2.0;
                            }
                        }
                        HeroClass::Warrior => {
                            // Warriors focus on protecting allies via Fortify aura
                        }
                        HeroClass::Mage => {
                            // Arcane Surge: AoE potential (simplified to bonus damage)
                            damage *= 1.3;
                        }
                        HeroClass::Archer => {
                            // Volley: AoE arrow rain on clustered enemies
                            // Primary target gets full damage; nearby enemies take 50% splash
                            if dist > 80.0 {
                                damage *= 1.2;
                            }
                            let splash_radius = 60.0;
                            let splash_damage = (stats.attack * 0.5).max(1.0);
                            for &(splash_entity, splash_pos, splash_hp) in &enemy_positions {
                                if splash_entity == *target_entity || splash_hp <= 0.0 {
                                    continue;
                                }
                                if (splash_pos - enemy_pos).length() <= splash_radius {
                                    volley_splashes.push((splash_entity, splash_damage));
                                }
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

    // Apply Archer Volley AoE splash damage to clustered enemies
    for (entity, splash_damage) in volley_splashes {
        if let Ok((_e, mut enemy_stats, _t)) = enemies.get_mut(entity) {
            if enemy_stats.hp > 0.0 {
                enemy_stats.hp -= splash_damage;
            }
        }
    }
}

/// System: Enemies attack heroes and buildings
pub fn enemy_attack_system(
    mut enemies: Query<(&Enemy, &EnemyStats, &EnemyAi, &mut AttackCooldown, &Transform)>,
    mut heroes: Query<(Entity, &mut HeroStats, &HeroEquipment, &mut HeroState, &Transform), (With<Hero>, Without<Enemy>)>,
    mut buildings: Query<(&mut Building, &Transform), (Without<Hero>, Without<Enemy>)>,
    active_buffs: Res<ActiveBuffs>,
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
            if let Ok((_hero_entity, mut hero_stats, hero_equipment, mut hero_state, hero_transform)) = heroes.get_mut(target) {
                let hero_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.y);
                let dist = (hero_pos - enemy_pos).length();

                if dist <= stats.attack_range && hero_stats.hp > 0.0 {
                    let total_defense = hero_stats.defense + hero_equipment.total_def_bonus() + active_buffs.def_bonus;
                    let base_damage = (stats.attack - total_defense).max(1.0);
                    let damage = (base_damage * (1.0 - hero_stats.fortify_reduction)).max(1.0);
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

/// Fortify aura radius in world units
const FORTIFY_AURA_RADIUS: f32 = 100.0;
/// Damage reduction percentage per nearby warrior (20%)
const FORTIFY_REDUCTION_PER_WARRIOR: f32 = 0.2;
/// Maximum fortify damage reduction cap (40%)
const FORTIFY_REDUCTION_CAP: f32 = 0.4;

/// System: Warriors emit a Fortify aura that grants nearby allies damage reduction.
/// Recalculated every frame so the buff is always spatially accurate.
/// Uses QuerySet to avoid conflicting access to Hero + HeroStats.
pub fn warrior_fortify_aura_system(
    mut query_set: QuerySet<(
        // q0: read warrior positions
        QueryState<(&Hero, &HeroStats, &Transform)>,
        // q1: write fortify_reduction on all heroes
        QueryState<(&mut HeroStats, &Transform)>,
    )>,
) {
    // Phase 1: Collect warrior positions (only living warriors)
    let mut warrior_positions: Vec<Vec2> = Vec::new();
    for (hero, stats, transform) in query_set.q0().iter() {
        if hero.class == HeroClass::Warrior && stats.hp > 0.0 {
            warrior_positions.push(Vec2::new(transform.translation.x, transform.translation.y));
        }
    }

    // Phase 2: For each hero, count nearby warriors and set fortify_reduction
    for (mut hero_stats, transform) in query_set.q1().iter_mut() {
        let hero_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let nearby_warriors = warrior_positions.iter().filter(|wp| {
            (**wp - hero_pos).length() <= FORTIFY_AURA_RADIUS
        }).count();

        hero_stats.fortify_reduction = (nearby_warriors as f32 * FORTIFY_REDUCTION_PER_WARRIOR)
            .min(FORTIFY_REDUCTION_CAP);
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
