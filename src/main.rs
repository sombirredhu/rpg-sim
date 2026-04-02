//! Realm of Bounties - A 2D Kingdom Simulation
//! Inspired by Majesty: The Fantasy Kingdom Sim
//!
//! The player rules as king through indirect control:
//! - Build buildings to attract heroes
//! - Place bounties to guide hero behavior
//! - Manage economy and defend against threats

mod components;
mod economy;
mod hero;
mod building;
mod enemy;
mod combat;
mod day_night;
mod ui;
mod camera;

use bevy::prelude::*;
use components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Resources
        .insert_resource(GameEconomy::default())
        .insert_resource(BountyBoard::default())
        .insert_resource(GameTime::default())
        .insert_resource(KingdomState::default())
        .insert_resource(GamePhase::default())
        .insert_resource(GameAlerts::default())
        .insert_resource(ClearColor(Color::rgb(0.15, 0.3, 0.15)))
        // Events
        .add_event::<BountyCompletedEvent>()
        .add_event::<HeroDeathEvent>()
        .add_event::<BuildingDestroyedEvent>()
        .add_event::<EnemyDeathEvent>()
        .add_event::<ThreatEscalationEvent>()
        .add_event::<HeroSpawnEvent>()
        // Startup systems
        .add_startup_system(setup_camera)
        .add_startup_system(ui::setup_ui)
        .add_startup_system(building::spawn_initial_buildings)
        .add_startup_system(enemy::spawn_initial_dens)
        .add_startup_system(day_night::spawn_night_overlay)
        .add_startup_system(spawn_initial_heroes)
        // Game logic systems
        .add_system(camera::camera_control_system)
        .add_system(day_night::day_night_cycle_system)
        .add_system(day_night::night_overlay_system)
        .add_system(day_night::speed_control_system)
        .add_system(economy::tax_collection_system)
        .add_system(economy::bounty_payout_system)
        .add_system(economy::auto_bounty_system)
        .add_system(economy::kingdom_progression_system)
        .add_system(hero::hero_ai_system)
        .add_system(hero::hero_movement_system)
        .add_system(hero::hero_rest_system)
        .add_system(hero::hero_progression_system)
        .add_system(hero::hero_attraction_system)
        .add_system(hero::hero_morale_system)
        .add_system(enemy::monster_den_spawn_system)
        .add_system(enemy::enemy_ai_system)
        .add_system(enemy::threat_escalation_system)
        .add_system(enemy::boss_raid_system)
        .add_system(enemy::enemy_death_system)
        .add_system(combat::hero_attack_system)
        .add_system(combat::enemy_attack_system)
        .add_system(combat::healer_system)
        .add_system(combat::enemy_reward_system)
        .add_system(building::building_placement_system)
        .add_system(building::building_upgrade_system)
        .add_system(building::building_repair_system)
        .add_system(building::guard_tower_attack_system)
        // UI systems
        .add_system(ui::update_gold_ui)
        .add_system(ui::update_day_night_ui)
        .add_system(ui::update_hero_panel_ui)
        .add_system(ui::update_kingdom_rank_ui)
        .add_system(ui::update_speed_ui)
        .add_system(ui::update_alerts_ui)
        .add_system(ui::build_menu_system)
        .add_system(ui::manual_bounty_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.8;
    commands.spawn_bundle(camera);
}

/// Spawn a few starting heroes near the Town Hall
fn spawn_initial_heroes(mut commands: Commands) {
    let starting_heroes = [
        (HeroClass::Warrior, Vec2::new(40.0, -20.0)),
        (HeroClass::Warrior, Vec2::new(-30.0, 30.0)),
        (HeroClass::Archer, Vec2::new(50.0, 40.0)),
    ];

    for (class, offset) in starting_heroes {
        let hero = Hero::new(class);
        let stats = class.base_stats();
        let color = class.color();

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(16.0, 24.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(offset.x, offset.y, 10.0)),
            ..Default::default()
        })
        .insert(hero)
        .insert(stats)
        .insert(HeroState::Idle)
        .insert(HeroDecisionTimer::default())
        .insert(AttackCooldown::default());
    }
}
