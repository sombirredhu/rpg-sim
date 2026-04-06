use bevy::prelude::*;
use crate::components::*;

/// System: Collect tax income from buildings every game-minute
pub fn tax_collection_system(
    mut economy: ResMut<GameEconomy>,
    game_time: Res<GameTime>,
    buildings: Query<&Building>,
    bonuses: Res<BuildingBonuses>,
    legacy: Res<LegacyUpgrades>,
    time: Res<Time>,
) {
    // Calculate income per real second based on building taxes
    let mut property_tax_per_minute = 0.0;
    for building in buildings.iter() {
        if !building.is_destroyed {
            property_tax_per_minute += building.building_type.tax_income(building.tier);
        }
    }

    // Apply road connection tax bonus (Market connected to buildings via roads)
    let road_multiplier = 1.0 + bonuses.road_tax_bonus_pct / 100.0;
    property_tax_per_minute *= road_multiplier;

    // Apply Legacy Upgrades tax bonus
    let legacy_tax_multiplier = 1.0 + legacy.tax_bonus_pct / 100.0;
    property_tax_per_minute *= legacy_tax_multiplier;

    // Store the current per-minute rate (will be averaged with time in breakdown system)
    economy.property_tax_income_per_minute = property_tax_per_minute;

    // Add income scaled by time and game speed
    let income_per_second = property_tax_per_minute / 60.0;
    let earned = income_per_second * time.delta_seconds() * game_time.speed_multiplier;
    economy.gold += earned;
    economy.total_earned += earned;
    economy.total_property_tax_earned += earned;
}

/// System: Warn when treasury reserves drop below safe threshold
pub fn treasury_warning_system(
    economy: Res<GameEconomy>,
    mut alerts: ResMut<GameAlerts>,
    mut warned: Local<bool>,
) {
    const TREASURY_THRESHOLD: f32 = 200.0;

    if economy.gold < TREASURY_THRESHOLD && !*warned {
        alerts.push(format!("Warning: Treasury reserves below {}g! Current: {:.0}g", TREASURY_THRESHOLD, economy.gold));
        *warned = true;
    }

    if economy.gold >= TREASURY_THRESHOLD + 50.0 {
        *warned = false; // Reset warning when treasury recovers above threshold + buffer
    }
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
        let reward = event.gold_reward;
        let hero_count = event.assigned_heroes.len() as f32;
        if hero_count == 0.0 {
            continue;
        }

        // Bounties are paid on completion from the treasury.
        economy.gold -= reward;
        economy.total_spent += reward;

        // Split 90% of reward equally among all assigned heroes
        let hero_share = reward * 0.9 / hero_count;
        for hero_entity in &event.assigned_heroes {
            if let Ok(mut hero) = heroes.get_mut(*hero_entity) {
                hero.gold_carried += hero_share;
                hero.xp += 25.0; // Bonus XP for bounty completion (full XP for each)
            }
        }

        // 10% bounty tax returns to treasury
        let tax = reward * 0.1;
        economy.gold += tax;
        economy.total_earned += tax;
        economy.total_bounty_tax_earned += tax;

        // Track ROI stats on the bounty board
        bounty_board.total_bounties_completed += 1;
        bounty_board.total_bounty_gold_paid += reward;
        bounty_board.total_bounty_tax_returned += tax;

        alerts.push(format!(
            "Bounty completed! {} heroes paid {:.0}g each, treasury tax return +{:.0}g",
            hero_count as u32,
            hero_share,
            tax
        ));
    }

    // Clean up completed bounties periodically
    bounty_board.cleanup_completed();
}

/// System: Auto-create bounties for monster dens
pub fn auto_bounty_system(
    mut bounty_board: ResMut<BountyBoard>,
    dens: Query<(Entity, &MonsterDen, &Transform)>,
    game_time: Res<GameTime>,
    game_phase: Res<GamePhase>,
    legacy: Res<LegacyUpgrades>,
    building_bonuses: Res<BuildingBonuses>,
) {
    if !game_phase.game_started { return; }
    for (entity, den, transform) in dens.iter() {
        let pos = Vec2::new(transform.translation.x, transform.translation.y);

        // Check if a bounty already exists for this den
        let has_bounty = bounty_board.bounties.iter().any(|b| {
            b.target_entity == Some(entity) && !b.is_completed
        });

        if !has_bounty {
            // Auto-create bounty based on threat level
            let mut reward = 20.0 * den.threat_tier as f32 * game_time.threat_multiplier();
            // Apply Legacy Upgrades bounty cost reduction (reward = cost to player)
            let reduction = 1.0 - legacy.bounty_cost_reduction / 100.0;
            reward *= reduction;

            // Determine required heroes based on threat tier and Barracks squad size
            let base_required = ((den.threat_tier + 1) / 2).max(1);
            let required_heroes = base_required.min(building_bonuses.max_squad_size);

            bounty_board.add_bounty(
                BountyType::Monster,
                reward,
                pos,
                Some(entity),
                den.threat_tier,
                required_heroes,
            );
        }
    }
}

/// System: Compute income breakdown rates from cumulative earnings and game time
pub fn update_income_breakdown_system(
    mut economy: ResMut<GameEconomy>,
    game_time: Res<GameTime>,
) {
    // Compute average per-minute rates for merchant trade and bounty tax
    // (property tax rate is computed directly from current buildings each frame)
    let minutes = game_time.time_seconds / 60.0;
    if minutes > 0.0 {
        economy.merchant_trade_income_per_minute = economy.total_merchant_trade_earned / minutes;
        economy.bounty_tax_income_per_minute = economy.total_bounty_tax_earned / minutes;
    } else {
        economy.merchant_trade_income_per_minute = 0.0;
        economy.bounty_tax_income_per_minute = 0.0;
    }

    // Total income per minute is sum of property tax (current) + averaged merchant + bounty
    economy.income_per_minute = economy.property_tax_income_per_minute
        + economy.merchant_trade_income_per_minute
        + economy.bounty_tax_income_per_minute;
}

/// System: Update kingdom rank based on buildings and heroes
pub fn kingdom_progression_system(
    mut kingdom: ResMut<KingdomState>,
    buildings: Query<&Building>,
    heroes: Query<&Hero>,
    game_phase: Res<GamePhase>,
    mut alerts: ResMut<GameAlerts>,
) {
    if !game_phase.game_started { return; }
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
