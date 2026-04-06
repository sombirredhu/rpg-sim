# API Documentation

Generated: 2026-04-06 22:55

## art_catalog.rs

### Structs

- `BuildingSpriteSpec`
- `UnitSpriteSpec`
- `ArtCatalog`

### Functions

- `build`

## audio.rs

### Structs

- `AudioAssets`

### Enums

- `SfxEvent`

### Functions

- `setup_audio`
- `play_sfx_system`

## building.rs

### Functions

- `building_placement_system`
- `building_upgrade_system`
- `building_repair_system`
- `guard_tower_attack_system`
- `spawn_initial_buildings`

## camera.rs

### Functions

- `cursor_to_world_2d`
- `camera_control_system`

## combat.rs

### Functions

- `hero_attack_system`
- `enemy_attack_system`
- `healer_system`
- `warrior_fortify_aura_system`
- `enemy_reward_system`
- `arcane_surge_ai_system`
- `arcane_surge_channel_system`
- `arcane_surge_effect_system`

## components.rs

### Structs

- `GameUiRoot`
- `SpeedButton`
- `PauseButton`
- `BuildButton`
- `BountyButton`
- `ExpandButton`
- `RoadToolButton`
- `BuildingMenuItem`
- `UiFont`
- `BuildingHighlight`
- `SelectedBuilding`
- `SelectedBuildingInfo`
- `BuildingMenuUi`
- `BuildingMenuText`
- `BuildingInfoUi`
- `BuildingInfoText`
- `RepairButton`
- `RepairButtonText`
- `LegacyUpgradeScreen`
- `LegacyPointsText`
- `TaxUpgradeRow`
- `HeroStartUpgradeRow`
- `BuildingHpUpgradeRow`
- `BountyCostUpgradeRow`
- `TaxUpgradeButton`
- `HeroStartUpgradeButton`
- `BuildingHpUpgradeButton`
- `BountyCostUpgradeButton`
- `TaxUpgradeLabel`
- `HeroStartUpgradeLabel`
- `BuildingHpUpgradeLabel`
- `BountyCostUpgradeLabel`
- `LegacyButton`
- `LegacyBackButton`
- `ChallengeIndicator`
- `ChallengeIndicatorText`
- `HeroStats`
- `Hero`
- `HeroDecisionTimer`
- `Building`
- `BuildingVisualTier`
- `AlchemistCraft`
- `EnemyStats`
- `Enemy`
- `EnemyAi`
- `MonsterDen`
- `MonsterDenVisualTier`
- `Bounty`
- `GameEconomy`
- `BountyBoard`
- `GameTime`
- `KingdomState`
- `AttackCooldown`
- `HealthBar`
- `ArcaneSurgeCooldown`
- `ArcaneSurgeEffect`
- `StealthCooldown`
- `Stealthed`
- `SanctuaryReviveEvent`
- `SanctuaryCooldown`
- `GoldText`
- `DayNightText`
- `HeroPanelText`
- `HeroPanelUi`
- `KingdomRankText`
- `AlertText`
- `BountyBoardUi`
- `BountyBoardText`
- `BuildMenuUi`
- `NightOverlay`
- `SpeedText`
- `MainCamera`
- `EconomyButton`
- `EconomyBreakdownPanel`
- `BountyCompletedEvent`
- `HeroDeathEvent`
- `BuildingDestroyedEvent`
- `EnemyDeathEvent`
- `ThreatEscalationEvent`
- `HeroSpawnEvent`
- `GamePhase`
- `GameAlerts`
- `Road`
- `MapDecoration`
- `RoadNetwork`
- `ResourceNode`
- `Merchant`
- `TradeCaravan`
- `ActiveBuffs`
- `FogOfWar`
- `TerrainGrid`
- `FogTile`
- `DayNightArcIndicator`
- `EraScoreScreen`
- `EraScoreLegacyText`
- `EraScoreStatsText`
- `EraContinueButton`
- `SpriteAnimation`
- `AnimationSet`
- `TorchHalo`
- `LegendaryGlow`
- `InspectText`
- `InspectTarget`
- `Milestones`
- `LegacyUpgrades`
- `EraState`
- `EraScoreData`
- `Equipment`
- `HeroEquipment`
- `BuildingBonuses`
- `SfxVolume`
- `MusicVolume`
- `CameraSpeed`
- `LongPressState`

### Enums

- `HeroClass`
- `HeroPersonality`
- `HeroState`
- `BuildingType`
- `ZoneType`
- `ChallengeModifier`
- `EnemyType`
- `BountyType`
- `TimeOfDay`
- `KingdomRank`
- `EconIncomeLine`
- `ResourceType`
- `RareItem`
- `AnimMode`
- `EquipmentSlot`
- `EquipmentTier`

### Functions

- `base_stats`
- `display_name`
- `color`
- `new`
- `random`
- `display_name`
- `enemy_types`
- `resource_types`
- `cost`
- `upgrade_cost`
- `tax_income`
- `display_name`
- `color`
- `size`
- `attracts_heroes`
- `new`
- `stats`
- `display_name`
- `is_night_only`
- `color`
- `new`
- `add_bounty`
- `get_bounty`
- `assign_bounty`
- `unassign_hero`
- `complete_bounty`
- `available_bounties`
- `cleanup_completed`
- `is_night`
- `threat_multiplier`
- `ambient_color`
- `display_name`
- `max_heroes`
- `max_expansions`
- `unlocked_zone_types`
- `expansion_cost`
- `available_buildings`
- `push`
- `is_on_road`
- `speed_multiplier`
- `are_connected`
- `display_name`
- `income_per_tick`
- `new`
- `random`
- `display_name`
- `cost`
- `buff_duration`
- `new`
- `new_directional`
- `atlas_index`
- `display_name`
- `craft_cost`
- `from_blacksmith_tier`
- `weapon`
- `armor`
- `display_name`
- `total_atk_bonus`
- `total_def_bonus`
- `best_tier`
- `needs_upgrade`

## day_night.rs

### Functions

- `day_night_cycle_system`
- `night_overlay_system`
- `speed_control_system`
- `spawn_night_overlay`

## debug.rs

### Structs

- `DebugConsole`
- `DebugCommandHistory`
- `DebugConsoleRoot`
- `DebugConsoleInput`
- `DebugConsoleOutput`

### Functions

- `setup_debug_console`
- `debug_console_input`
- `debug_console_ui_update`
- `debug_command_executor`

## economy.rs

### Functions

- `tax_collection_system`
- `treasury_warning_system`
- `bounty_payout_system`
- `auto_bounty_system`
- `update_income_breakdown_system`
- `kingdom_progression_system`

## enemy.rs

### Functions

- `monster_den_spawn_system`
- `enemy_ai_system`
- `threat_escalation_system`
- `boss_raid_system`
- `spawn_initial_dens`
- `enemy_death_system`
- `edge_spawn_system`

## features.rs

*All missing GDD features: roads, resources, merchants, night enemies,*

### Functions

- `road_placement_system`
- `road_connection_bonus_system`
- `den_destruction_system`
- `new_den_spawn_system`
- `night_enemy_spawn_system`
- `night_enemy_despawn_system`
- `merchant_spawn_system`
- `merchant_movement_system`
- `trade_caravan_spawn_system`
- `trade_caravan_movement_system`
- `caravan_death_system`
- `active_buffs_system`
- `resource_node_system`
- `resource_bounty_system`
- `spawn_resource_nodes`
- `building_bonuses_system`
- `hero_idle_leave_system`
- `milestone_system`
- `recovery_bounty_system`
- `objective_bounty_system`
- `era_siege_system`
- `era_score_screen_visibility_system`
- `update_era_score_legacy_system`
- `update_era_score_stats_system`
- `era_continue_button_system`
- `torch_defense_system`
- `sprite_animation_system`
- `animation_mode_system`
- `enemy_animation_mode_system`
- `inspect_system`
- `fog_of_war_system`
- `spawn_fog_of_war`
- `blacksmith_crafting_system`
- `apply_building_bonuses_system`
- `cathedral_income_system`
- `map_expansion_system`
- `alchemist_craft_system`
- `hero_potion_consumption_system`

## hero.rs

### Functions

- `hero_ai_system`
- `hero_movement_system`
- `bounty_resolution_system`
- `hero_rest_system`
- `hero_progression_system`
- `hero_attraction_system`
- `hero_morale_system`
- `healer_sanctuary_ai_system`
- `healer_sanctuary_channel_system`
- `sanctuary_revive_system`
- `rogue_stealth_ai_system`
- `rogue_stealth_channel_system`
- `rogue_stealth_tick_system`
- `recovery_revive_system`
- `legendary_hero_glow_system`
- `hero_love_system`

## main.rs

*Realm of Bounties - A 2D Kingdom Simulation*

## map_layout.rs

*Structured map layout with intentional zone placement.*

### Structs

- `ZoneConfig`

### Enums

- `ZoneTerrain`
- `RuinType`

### Functions

- `center`

## menu.rs

### Structs

- `MenuState`
- `MainMenuRoot`
- `SettingsMenuRoot`
- `StartGameButton`
- `ResumeGameButton`
- `SettingsButton`
- `QuitButton`
- `BackButton`
- `SettingsVolumeText`
- `SfxVolumeControl`
- `MusicVolumeControl`
- `CameraSpeedControl`
- `SettingToggleVisual`

### Enums

- `GameMenuState`

### Functions

- `setup_main_menu`
- `start_game_button_system`
- `resume_game_button_system`
- `settings_button_system`
- `back_button_system`
- `quit_button_system`
- `menu_pause_system`
- `menu_button_hover_system`
- `sfx_volume_control_system`
- `music_volume_control_system`
- `camera_speed_control_system`

## mouse.rs

*Mouse interaction systems: HUD button clicks, camera drag, entity inspect,*

### Functions

- `camera_drag_system`
- `speed_button_click`
- `pause_button_click`
- `build_button_click`
- `bounty_button_click`
- `expand_button_click`
- `road_tool_button_click`
- `map_click_system`
- `selected_building_action`

## noise_map.rs

### Enums

- `NoiseTerrain`

### Functions

- `generate_terrain_noise`
- `apply_core_zones`
- `tile_to_world`
- `world_to_tile`

## save.rs

*Save/Load system — serializes all game state to JSON files.*

### Structs

- `AutoSaveTimer`
- `LoadRequest`
- `SHeroState`

### Functions

- `has_save`
- `from`
- `to_state`
- `auto_save_system`
- `quick_save_system`
- `load_game_system`

## sprites.rs

*Centralized sprite asset loading and mapping.*

### Structs

- `BuildingSpriteSet`
- `SpriteAssets`

### Functions

- `for_tier`
- `building_texture_for_tier`
- `building_scale_for_tier`
- `monster_den_texture_for_tier`
- `monster_den_scale_for_tier`
- `load_sprite_assets`
- `spawn_hero_with_sprite`
- `spawn_enemy_with_sprite`
- `spawn_enemy_with_sprite_world`
- `spawn_building_with_sprite`
- `sync_building_tier_visuals`
- `sync_monster_den_tier_visuals`
- `spawn_ground_tiles`
- `spawn_terrain_overlays`
- `spawn_trees`
- `spawn_map_decorations`

## ui.rs

### Functions

- `setup_ui`
- `update_gold_ui`
- `update_day_night_ui`
- `update_day_night_arc_system`
- `update_hero_panel_ui`
- `update_kingdom_rank_ui`
- `update_speed_ui`
- `update_alerts_ui`
- `update_bounty_board_ui`
- `update_building_menu_ui`
- `building_menu_button_system`
- `update_building_info_ui`
- `update_repair_button_ui`
- `build_menu_system`
- `manual_bounty_system`
- `economy_button_click_system`
- `update_economy_breakdown_ui`
- `repair_button_click_system`
- `legacy_button_system`
- `legacy_back_button_system`
- `update_legacy_upgrades_ui_system`

