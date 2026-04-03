//! Realm of Bounties - A 2D Kingdom Simulation
//! Inspired by Majesty: The Fantasy Kingdom Sim
//!
//! The player rules as king through indirect control:
//! - Build buildings to attract heroes
//! - Place bounties to guide hero behavior
//! - Manage economy and defend against threats

mod components;
mod sprites;
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
use sprites::SpriteAssets;

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
        .insert_resource(ClearColor(Color::rgb(0.18, 0.32, 0.15)))
        // Events
        .add_event::<BountyCompletedEvent>()
        .add_event::<HeroDeathEvent>()
        .add_event::<BuildingDestroyedEvent>()
        .add_event::<EnemyDeathEvent>()
        .add_event::<ThreatEscalationEvent>()
        .add_event::<HeroSpawnEvent>()
        // Startup systems (sprite loading MUST run first)
        .add_startup_system(sprites::load_sprite_assets)
        .add_startup_system(setup_camera)
        .add_startup_system(ui::setup_ui)
        // Deferred startup that needs SpriteAssets
        .add_startup_system_to_stage(StartupStage::PostStartup, sprites::spawn_ground_tiles)
        .add_startup_system_to_stage(StartupStage::PostStartup, sprites::spawn_trees)
        .add_startup_system_to_stage(StartupStage::PostStartup, building::spawn_initial_buildings)
        .add_startup_system_to_stage(StartupStage::PostStartup, enemy::spawn_initial_dens)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_initial_heroes)
        .add_startup_system_to_stage(StartupStage::PostStartup, day_night::spawn_night_overlay)
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

/// Spawn starting heroes using the real sprite assets
fn spawn_initial_heroes(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
) {
    let starting_heroes = [
        (HeroClass::Warrior, Vec2::new(40.0, -20.0)),
        (HeroClass::Warrior, Vec2::new(-30.0, 30.0)),
        (HeroClass::Archer, Vec2::new(50.0, 40.0)),
    ];

    for (class, offset) in starting_heroes {
        sprites::spawn_hero_with_sprite(
            &mut commands,
            &sprites,
            class,
            Vec3::new(offset.x, offset.y, 10.0),
        );
    }
}
