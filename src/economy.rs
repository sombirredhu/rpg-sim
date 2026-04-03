use bevy::prelude::*;
use crate::components::*;

/// System: Collect tax income from buildings every game-minute
pub fn tax_collection_system(
    mut economy: ResMut<GameEconomy>,
    game_time: Res<GameTime>,
    buildings: Query<&Building>,
    bonuses: Res<BuildingBonuses>,
    time: Res<Time>,
) {
    // Calculate income per real second based on building taxes
    let mut total_tax_per_minute = 0.0;
    for building in buildings.iter() {
        if !building.is_destroyed {
            total_tax_per_minute += building.building_type.tax_income(building.tier);
        }
    }

    // Apply road connection tax bonus (Market connected to buildings via roads)
    let road_multiplier = 1.0 + bonuses.road_tax_bonus_pct / 100.0;
    total_tax_per_minute *= road_multiplier;

    economy.income_per_minute = total_tax_per_minute;

    // Add income scaled by time and game speed
    let income_per_second = total_tax_per_minute / 60.0;
    let earned = income_per_second * time.delta_seconds() * game_time.speed_multiplier;
    economy.gold += earned;
    economy.total_earned += earned;
}

/// System: Pay out bounties when completed
pub fn bounty_payout_system(
    mut economy: ResMut<GameEconomy>,
    mut bounty_board: ResMut<BountyBoard>,
    mut events: EventReader<BountyCompletedEvent>,
    mut heroes: Query<&mut Hero>,
    mut alerts: ResMut<GameAlerts>,
) {
    for event in events.iter() {
        if let Some(reward) = bounty_board.complete_bounty(event.bounty_id) {
            // Pay the hero
            if let Ok(mut hero) = heroes.get_mut(event.hero_entity) {
                hero.gold_carried += reward * 0.9; // Hero gets 90%
                hero.xp += 25.0; // Bonus XP for bounty completion
            }
            // 10% bounty tax returns to treasury
            let tax = reward * 0.1;
            economy.gold += tax;
            economy.total_earned += tax;

            alerts.push(format!("Bounty completed! +{:.0} gold tax", tax));
        }
    }

    // Clean up completed bounties periodically
    bounty_board.cleanup_completed();
}

/// System: Auto-create bounties for monster dens
pub fn auto_bounty_system(
    mut bounty_board: ResMut<BountyBoard>,
    dens: Query<(Entity, &MonsterDen, &Transform)>,
    game_time: Res<GameTime>,
) {
    for (entity, den, transform) in dens.iter() {
        let pos = Vec2::new(transform.translation.x, transform.translation.y);

        // Check if a bounty already exists for this den
        let has_bounty = bounty_board.bounties.iter().any(|b| {
            b.target_entity == Some(entity) && !b.is_completed
        });

        if !has_bounty {
            // Auto-create bounty based on threat level
            let reward = 20.0 * den.threat_tier as f32 * game_time.threat_multiplier();
            bounty_board.add_bounty(
                BountyType::Monster,
                reward,
                pos,
                Some(entity),
                den.threat_tier,
            );
        }
    }
}

/// System: Update kingdom rank based on buildings and heroes
pub fn kingdom_progression_system(
    mut kingdom: ResMut<KingdomState>,
    buildings: Query<&Building>,
    heroes: Query<&Hero>,
    mut alerts: ResMut<GameAlerts>,
) {
    let building_count = buildings.iter().filter(|b| !b.is_destroyed).count() as u32;
    let hero_count = heroes.iter().count() as u32;
    kingdom.buildings_count = building_count;
    kingdom.hero_count = hero_count;

    let old_rank = kingdom.rank;
    kingdom.rank = if building_count >= 8 && hero_count >= 15 {
        KingdomRank::Kingdom
    } else if building_count >= 6 && hero_count >= 10 {
        KingdomRank::City
    } else if building_count >= 4 && hero_count >= 7 {
        KingdomRank::Town
    } else if building_count >= 2 && hero_count >= 3 {
        KingdomRank::Village
    } else {
        KingdomRank::Hamlet
    };

    if kingdom.rank != old_rank {
        alerts.push(format!("Kingdom upgraded to {}!", kingdom.rank.display_name()));
    }

    // Score calculation
    kingdom.score = (kingdom.hero_count * 100)
        + (kingdom.buildings_count * 200)
        + (kingdom.era_day * 10);
}
