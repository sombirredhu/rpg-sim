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
mod audio;
mod combat;
mod day_night;
mod ui;
mod camera;
mod features;
mod mouse;
mod menu;
mod debug;
mod save;
mod map_layout;
mod noise_map;

use bevy::prelude::*;
use components::*;
use sprites::SpriteAssets;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Realm of Bounties".to_string(),
            width: 1440.0,
            height: 900.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb_u8(21, 29, 23)))
        .add_plugins(DefaultPlugins)
        // Resources
        .insert_resource(GameEconomy::default())
        .insert_resource(BountyBoard::default())
        .insert_resource(GameTime::default())
        .insert_resource(KingdomState::default())
        .insert_resource(GamePhase::default())
        .insert_resource(GameAlerts::default())
        .insert_resource(RoadNetwork::default())
        .insert_resource(Milestones::default())
        .insert_resource(LegacyUpgrades::default())
        .insert_resource(EraState::default())
        .insert_resource(EraScoreData::default())
        .insert_resource(BuildingBonuses::default())
        .insert_resource(ActiveBuffs::default())
        .insert_resource(FogOfWar::default())
        .insert_resource(InspectTarget::default())
        .insert_resource(components::SelectedBuilding::default())
        .insert_resource(components::SelectedBuildingInfo::default())
        .insert_resource(components::SfxVolume(1.0))
        .insert_resource(components::MusicVolume(1.0))
        .insert_resource(components::CameraSpeed(1.0))
        .insert_resource(ClearColor(Color::rgb(0.18, 0.32, 0.15)))
        .insert_resource(menu::MenuState::default())
        .insert_resource(debug::DebugConsole::default())
        .insert_resource(debug::DebugCommandHistory::default())
        .insert_resource(save::AutoSaveTimer::default())
        .insert_resource(save::LoadRequest::default())
        
        
        // Events
        .add_event::<BountyCompletedEvent>()
        .add_event::<HeroDeathEvent>()
        .add_event::<BuildingDestroyedEvent>()
        .add_event::<EnemyDeathEvent>()
        .add_event::<ThreatEscalationEvent>()
        .add_event::<HeroSpawnEvent>()
        .add_event::<audio::SfxEvent>()
        .add_event::<SanctuaryReviveEvent>()
        // Startup systems
        // Menu setup (runs first, before game world)
        .add_startup_system(menu::setup_main_menu)
        // Asset loading and world spawn (runs for both menu + game)
        .add_startup_system(sprites::load_sprite_assets)
        .add_startup_system(audio::setup_audio)
        .add_startup_system(setup_camera)
        // Deferred startup that needs SpriteAssets
        .add_startup_system_to_stage(StartupStage::PostStartup, ui::setup_ui)
        .add_startup_system_to_stage(StartupStage::PostStartup, sprites::spawn_ground_tiles)
        .add_startup_system_to_stage(StartupStage::PostStartup, sprites::spawn_terrain_overlays)
        .add_startup_system_to_stage(StartupStage::PostStartup, sprites::spawn_trees)
        .add_startup_system_to_stage(StartupStage::PostStartup, sprites::spawn_map_decorations)
        .add_startup_system_to_stage(StartupStage::PostStartup, building::spawn_initial_buildings)
        .add_startup_system_to_stage(StartupStage::PostStartup, enemy::spawn_initial_dens)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_initial_heroes)
        .add_startup_system_to_stage(StartupStage::PostStartup, day_night::spawn_night_overlay)
        .add_startup_system_to_stage(StartupStage::PostStartup, features::spawn_resource_nodes)
        .add_startup_system_to_stage(StartupStage::PostStartup, features::spawn_fog_of_war)
        .add_startup_system_to_stage(StartupStage::PostStartup, debug::setup_debug_console)
        // Game logic systems
        // Menu systems
        .add_system(menu::menu_pause_system)
        .add_system(menu::menu_button_hover_system)
        .add_system(menu::start_game_button_system)
        .add_system(menu::resume_game_button_system)
        .add_system(menu::settings_button_system)
        .add_system(menu::back_button_system)
        // Settings control systems
        .add_system(menu::sfx_volume_control_system)
        .add_system(menu::music_volume_control_system)
        .add_system(menu::camera_speed_control_system)
        .add_system(menu::quit_button_system)
        // Mouse interaction systems (only after game started)
        .add_system(mouse::camera_drag_system)
        .add_system(mouse::speed_button_click)
        .add_system(mouse::pause_button_click)
        .add_system(mouse::build_button_click)
        .add_system(mouse::bounty_button_click)
        .add_system(mouse::expand_button_click)
        .add_system(mouse::road_tool_button_click)
        .add_system(mouse::map_click_system)
        .add_system(mouse::selected_building_action)
        // Keyboard camera control (WASD — augmented by mouse drag)
        .add_system(day_night::day_night_cycle_system)
        .add_system(day_night::night_overlay_system)
        .add_system(day_night::speed_control_system)
        .add_system(economy::tax_collection_system)
        .add_system(economy::bounty_payout_system)
        .add_system(economy::auto_bounty_system)
        .add_system(economy::treasury_warning_system)
        .add_system(economy::kingdom_progression_system)
        .add_system(economy::update_income_breakdown_system)
        .add_system(hero::hero_ai_system)
        .add_system(hero::hero_movement_system)
        .add_system(hero::bounty_resolution_system)
        .add_system(hero::hero_rest_system)
        .add_system(hero::hero_progression_system)
        .add_system(hero::hero_attraction_system)
        .add_system(hero::hero_morale_system)
        .add_system(hero::legendary_hero_glow_system)
        .add_system(hero::hero_love_system)
        // Healer Sanctuary ability
        .add_system(hero::healer_sanctuary_ai_system)
        .add_system(hero::healer_sanctuary_channel_system)
        .add_system(hero::sanctuary_revive_system)
        // Rogue Stealth ability
        .add_system(hero::rogue_stealth_ai_system)
        .add_system(hero::rogue_stealth_channel_system)
        .add_system(enemy::monster_den_spawn_system)
        .add_system(enemy::enemy_ai_system)
        .add_system(enemy::threat_escalation_system)
        .add_system(enemy::boss_raid_system)
        .add_system(enemy::enemy_death_system)
        .add_system(enemy::edge_spawn_system)
        .add_system(combat::warrior_fortify_aura_system)
        .add_system(combat::hero_attack_system)
        .add_system(combat::enemy_attack_system)
        .add_system(combat::healer_system)
        .add_system(combat::enemy_reward_system)
        .add_system(combat::arcane_surge_ai_system)
        .add_system(combat::arcane_surge_channel_system)
        .add_system(combat::arcane_surge_effect_system)
        .add_system(hero::rogue_stealth_tick_system)
        .add_system(audio::play_sfx_system)
        .add_system(building::building_placement_system)
        .add_system(building::building_upgrade_system)
        .add_system(building::building_repair_system)
        .add_system(building::guard_tower_attack_system)
        .add_system(sprites::sync_building_tier_visuals)
        .add_system(sprites::sync_monster_den_tier_visuals)
        // Features systems
        .add_system(features::road_placement_system)
        .add_system(features::road_connection_bonus_system)
        .add_system(features::den_destruction_system)
        .add_system(features::new_den_spawn_system)
        .add_system(features::night_enemy_spawn_system)
        .add_system(features::night_enemy_despawn_system)
        .add_system(features::merchant_spawn_system)
        .add_system(features::merchant_movement_system)
        .add_system(features::trade_caravan_spawn_system)
        .add_system(features::trade_caravan_movement_system)
        .add_system(features::active_buffs_system)
        .add_system(features::resource_node_system)
        .add_system(features::resource_bounty_system)
        .add_system(features::building_bonuses_system)
        .add_system(features::blacksmith_crafting_system)
        .add_system(features::apply_building_bonuses_system)
        .add_system(features::cathedral_income_system)
        .add_system(features::hero_idle_leave_system)
        .add_system(features::milestone_system)
        .add_system(features::recovery_bounty_system)
        .add_system(hero::recovery_revive_system)
        .add_system(features::objective_bounty_system)
        .add_system(features::era_siege_system)
        .add_system(features::torch_defense_system)
        // Era score screen systems
        .add_system(features::era_score_screen_visibility_system)
        .add_system(features::update_era_score_legacy_system)
        .add_system(features::update_era_score_stats_system)
        .add_system(features::era_continue_button_system)
        // Alchemist potion crafting and consumption
        .add_system(features::alchemist_craft_system)
        .add_system(features::hero_potion_consumption_system)
        .add_system(features::sprite_animation_system)
        .add_system(features::animation_mode_system)
        .add_system(features::enemy_animation_mode_system)
        .add_system(features::inspect_system)
        .add_system(features::fog_of_war_system)
        .add_system(features::map_expansion_system)
        // UI systems
        .add_system(ui::update_gold_ui)
        .add_system(ui::update_day_night_ui)
        .add_system(ui::update_day_night_arc_system)
        .add_system(ui::update_hero_panel_ui)
        .add_system(ui::update_kingdom_rank_ui)
        .add_system(ui::update_speed_ui)
        .add_system(ui::update_alerts_ui)
        .add_system(ui::update_bounty_board_ui)
        .add_system(ui::update_building_menu_ui)
        .add_system(ui::building_menu_button_system)
        .add_system(ui::economy_button_click_system)
        .add_system(ui::update_building_info_ui)
        .add_system(ui::build_menu_system)
        .add_system(ui::manual_bounty_system)
        .add_system(ui::update_repair_button_ui)
        .add_system(ui::repair_button_click_system)
        .add_system(ui::update_economy_breakdown_ui)
        // Legacy Upgrade UI systems
        .add_system(ui::legacy_button_system)
        .add_system(ui::legacy_back_button_system)
        .add_system(ui::update_legacy_upgrades_ui_system)
        // Debug console
        .add_system(debug::debug_console_input)
        .add_system(debug::debug_command_executor)
        .add_system(debug::debug_console_ui_update)
        // Save/Load systems (exclusive — take &mut World)
        .add_system(save::auto_save_system.exclusive_system())
        .add_system(save::quick_save_system.exclusive_system())
        .add_system(save::load_game_system.exclusive_system())
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.8;
    commands.spawn_bundle(camera).insert(MainCamera);
}

/// Spawn starting heroes using the real sprite assets
fn spawn_initial_heroes(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    legacy: Res<LegacyUpgrades>,
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
            legacy.hero_start_level,
        );
    }
}
