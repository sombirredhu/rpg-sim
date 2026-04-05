//! Save/Load system — serializes all game state to JSON files.

use bevy::prelude::*;
use bevy::ecs::query::QueryState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::components::*;
use crate::sprites::{
    SpriteAssets, building_texture_for_tier, building_scale_for_tier,
    monster_den_texture_for_tier, monster_den_scale_for_tier,
};

// ============================================================
// Save paths
// ============================================================

fn save_dir() -> PathBuf {
    let mut path = std::env::current_exe().unwrap_or_default();
    path.pop();
    path.push("saves");
    path
}

fn auto_save_path() -> PathBuf {
    let mut p = save_dir();
    p.push("autosave.json");
    p
}

pub fn has_save() -> bool {
    auto_save_path().exists()
}

// ============================================================
// Resource: tracks auto-save timer and load requests
// ============================================================

#[derive(Default)]
pub struct AutoSaveTimer {
    pub elapsed: f32,
}

#[derive(Default)]
pub struct LoadRequest {
    pub pending: bool,
}

// ============================================================
// Serializable structs
// ============================================================

#[derive(Serialize, Deserialize)]
pub struct SHeroState {
    pub variant: String,
    pub target_x: f32,
    pub target_y: f32,
    pub bounty_id: u32,
    pub respawn_timer: f32,
    pub channel_elapsed: f32,
    pub channel_duration: f32,
}

impl SHeroState {
    pub fn from(state: &HeroState) -> Self {
        match state {
            HeroState::Idle => Self {
                variant: "Idle".into(), target_x: 0.0, target_y: 0.0,
                bounty_id: 0, respawn_timer: 0.0,
                channel_elapsed: 0.0, channel_duration: 0.0,
            },
            HeroState::MovingTo { target } => Self {
                variant: "MovingTo".into(), target_x: target.x, target_y: target.y,
                bounty_id: 0, respawn_timer: 0.0,
                channel_elapsed: 0.0, channel_duration: 0.0,
            },
            HeroState::AttackingEnemy { .. } => Self {
                variant: "Idle".into(), target_x: 0.0, target_y: 0.0,
                bounty_id: 0, respawn_timer: 0.0,
                channel_elapsed: 0.0, channel_duration: 0.0,
            },
            HeroState::PursuingBounty { bounty_id } => Self {
                variant: "PursuingBounty".into(), target_x: 0.0, target_y: 0.0,
                bounty_id: *bounty_id, respawn_timer: 0.0,
                channel_elapsed: 0.0, channel_duration: 0.0,
            },
            HeroState::Resting => Self {
                variant: "Resting".into(), target_x: 0.0, target_y: 0.0,
                bounty_id: 0, respawn_timer: 0.0,
                channel_elapsed: 0.0, channel_duration: 0.0,
            },
            HeroState::Shopping => Self {
                variant: "Shopping".into(), target_x: 0.0, target_y: 0.0,
                bounty_id: 0, respawn_timer: 0.0,
                channel_elapsed: 0.0, channel_duration: 0.0,
            },
            HeroState::Dead { respawn_timer } => Self {
                variant: "Dead".into(), target_x: 0.0, target_y: 0.0,
                bounty_id: 0, respawn_timer: *respawn_timer,
                channel_elapsed: 0.0, channel_duration: 0.0,
            },
            HeroState::Casting { channel_elapsed, channel_duration, .. } => Self {
                variant: "Idle".into(), target_x: 0.0, target_y: 0.0,
                bounty_id: 0, respawn_timer: 0.0,
                channel_elapsed: *channel_elapsed, channel_duration: *channel_duration,
            },
        }
    }

    pub fn to_state(&self) -> HeroState {
        match self.variant.as_str() {
            "MovingTo" => HeroState::MovingTo {
                target: Vec2::new(self.target_x, self.target_y),
            },
            "PursuingBounty" => HeroState::PursuingBounty {
                bounty_id: self.bounty_id,
            },
            "Resting" => HeroState::Resting,
            "Shopping" => HeroState::Shopping,
            "Dead" => HeroState::Dead {
                respawn_timer: self.respawn_timer,
            },
            _ => HeroState::Idle,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SHero {
    pub class: HeroClass,
    pub level: u32,
    pub xp: f32,
    pub xp_to_next: f32,
    pub morale: f32,
    pub gold_carried: f32,
    pub personality: HeroPersonality,
    pub is_legendary: bool,
    pub stats: HeroStats,
    pub equipment: HeroEquipment,
    pub state: SHeroState,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
struct SBuilding {
    pub building_type: BuildingType,
    pub tier: u32,
    pub hp: f32,
    pub max_hp: f32,
    pub is_destroyed: bool,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
struct SDen {
    pub enemy_type: EnemyType,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub max_spawned: u32,
    pub current_spawned: u32,
    pub threat_tier: u32,
    pub weeks_unaddressed: u32,
    pub hp: f32,
    pub max_hp: f32,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
struct SResourceNode {
    pub resource_type: ResourceType,
    pub is_active: bool,
    pub gather_timer: f32,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
struct SEnemy {
    pub enemy_type: EnemyType,
    pub stats: EnemyStats,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
struct SMerchant {
    pub gold_value: f32,
    pub dest_x: f32,
    pub dest_y: f32,
    pub has_arrived: bool,
    pub leave_timer: f32,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
struct SCaravan {
    pub item: RareItem,
    pub dest_x: f32,
    pub dest_y: f32,
    pub has_arrived: bool,
    pub leave_timer: f32,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
struct SCamera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

#[derive(Serialize, Deserialize)]
struct SaveFile {
    pub economy: GameEconomy,
    pub bounty_board: BountyBoard,
    pub game_time: GameTime,
    pub kingdom_state: KingdomState,
    pub game_phase: GamePhase,
    pub road_network: RoadNetwork,
    pub fog_of_war: FogOfWar,
    pub milestones: Milestones,
    pub legacy_upgrades: LegacyUpgrades,
    pub era_state: EraState,
    pub building_bonuses: BuildingBonuses,
    pub active_buffs: ActiveBuffs,
    pub heroes: Vec<SHero>,
    pub buildings: Vec<SBuilding>,
    pub dens: Vec<SDen>,
    pub resource_nodes: Vec<SResourceNode>,
    pub enemies: Vec<SEnemy>,
    pub merchants: Vec<SMerchant>,
    pub caravans: Vec<SCaravan>,
    pub camera: SCamera,
}

// ============================================================
// Read current world state into SaveFile
// ============================================================

fn read_save_data(world: &mut World) -> Option<SaveFile> {
    let economy = world.get_resource::<GameEconomy>()?.clone();
    let bounty_board = world.get_resource::<BountyBoard>()?.clone();
    let game_time = world.get_resource::<GameTime>()?.clone();
    let kingdom_state = world.get_resource::<KingdomState>()?.clone();
    let game_phase = world.get_resource::<GamePhase>()?.clone();
    let road_network = world.get_resource::<RoadNetwork>()?.clone();
    let fog_of_war = world.get_resource::<FogOfWar>()?.clone();
    let milestones = world.get_resource::<Milestones>()?.clone();
    let legacy_upgrades = world.get_resource::<LegacyUpgrades>()?.clone();
    let era_state = world.get_resource::<EraState>()?.clone();
    let building_bonuses = world.get_resource::<BuildingBonuses>()?.clone();
    let active_buffs = world.get_resource::<ActiveBuffs>()?.clone();

    // Camera
    let mut camera = SCamera { x: 0.0, y: 0.0, zoom: 0.8 };
    {
        let mut q = QueryState::<(&Transform, &OrthographicProjection), With<MainCamera>>::new(world);
        for (t, p) in q.iter(world) {
            camera.x = t.translation.x;
            camera.y = t.translation.y;
            camera.zoom = p.scale;
        }
    }

    // Heroes
    let mut heroes = Vec::new();
    {
        let mut q = QueryState::<(&Hero, &HeroStats, &HeroEquipment, &HeroState, &Transform)>::new(world);
        for (hero, stats, equip, state, t) in q.iter(world) {
            heroes.push(SHero {
                class: hero.class,
                level: hero.level,
                xp: hero.xp,
                xp_to_next: hero.xp_to_next,
                morale: hero.morale,
                gold_carried: hero.gold_carried,
                personality: hero.personality,
                is_legendary: hero.is_legendary,
                stats: stats.clone(),
                equipment: equip.clone(),
                state: SHeroState::from(state),
                x: t.translation.x,
                y: t.translation.y,
            });
        }
    }

    // Buildings
    let mut buildings = Vec::new();
    {
        let mut q = QueryState::<(&Building, &Transform)>::new(world);
        for (b, t) in q.iter(world) {
            buildings.push(SBuilding {
                building_type: b.building_type,
                tier: b.tier,
                hp: b.hp,
                max_hp: b.max_hp,
                is_destroyed: b.is_destroyed,
                x: t.translation.x,
                y: t.translation.y,
            });
        }
    }

    // Dens
    let mut dens = Vec::new();
    {
        let mut q = QueryState::<(&MonsterDen, &Transform)>::new(world);
        for (d, t) in q.iter(world) {
            dens.push(SDen {
                enemy_type: d.enemy_type,
                spawn_timer: d.spawn_timer,
                spawn_interval: d.spawn_interval,
                max_spawned: d.max_spawned,
                current_spawned: d.current_spawned,
                threat_tier: d.threat_tier,
                weeks_unaddressed: d.weeks_unaddressed,
                hp: d.hp,
                max_hp: d.max_hp,
                x: t.translation.x,
                y: t.translation.y,
            });
        }
    }

    // Resource nodes
    let mut resource_nodes = Vec::new();
    {
        let mut q = QueryState::<(&ResourceNode, &Transform)>::new(world);
        for (r, t) in q.iter(world) {
            resource_nodes.push(SResourceNode {
                resource_type: r.resource_type,
                is_active: r.is_active,
                gather_timer: r.gather_timer,
                x: t.translation.x,
                y: t.translation.y,
            });
        }
    }

    // Enemies
    let mut enemies = Vec::new();
    {
        let mut q = QueryState::<(&Enemy, &EnemyStats, &Transform)>::new(world);
        for (e, stats, t) in q.iter(world) {
            enemies.push(SEnemy {
                enemy_type: e.enemy_type,
                stats: stats.clone(),
                x: t.translation.x,
                y: t.translation.y,
            });
        }
    }

    // Merchants
    let mut merchants = Vec::new();
    {
        let mut q = QueryState::<(&Merchant, &Transform)>::new(world);
        for (m, t) in q.iter(world) {
            merchants.push(SMerchant {
                gold_value: m.gold_value,
                dest_x: m.destination.x,
                dest_y: m.destination.y,
                has_arrived: m.has_arrived,
                leave_timer: m.leave_timer,
                x: t.translation.x,
                y: t.translation.y,
            });
        }
    }

    // Trade caravans
    let mut caravans = Vec::new();
    {
        let mut q = QueryState::<(&TradeCaravan, &Transform)>::new(world);
        for (c, t) in q.iter(world) {
            caravans.push(SCaravan {
                item: c.item,
                dest_x: c.destination.x,
                dest_y: c.destination.y,
                has_arrived: c.has_arrived,
                leave_timer: c.leave_timer,
                x: t.translation.x,
                y: t.translation.y,
            });
        }
    }

    Some(SaveFile {
        economy,
        bounty_board,
        game_time,
        kingdom_state,
        game_phase,
        road_network,
        fog_of_war,
        milestones,
        legacy_upgrades,
        era_state,
        building_bonuses,
        active_buffs,
        heroes,
        buildings,
        dens,
        resource_nodes,
        enemies,
        merchants,
        caravans,
        camera,
    })
}

fn ensure_save_dir() {
    let _ = fs::create_dir_all(save_dir());
}

// ============================================================
// Auto-save exclusive system (every 30s)
// ============================================================

pub fn auto_save_system(world: &mut World) {
    let game_started = world
        .get_resource::<GamePhase>()
        .map_or(false, |p| p.game_started);
    if !game_started {
        return;
    }

    let dt = world
        .get_resource::<Time>()
        .map_or(0.0, |t| t.delta_seconds());

    let should_save = {
        let mut timer = world.get_resource_mut::<AutoSaveTimer>().unwrap();
        timer.elapsed += dt;
        if timer.elapsed >= 30.0 {
            timer.elapsed = 0.0;
            true
        } else {
            false
        }
    };

    if !should_save {
        return;
    }

    if let Some(save) = read_save_data(world) {
        ensure_save_dir();
        if let Ok(json) = serde_json::to_string_pretty(&save) {
            let _ = fs::write(auto_save_path(), json);
        }
    }
}

// ============================================================
// Quick save system (K key) — exclusive system
// ============================================================

pub fn quick_save_system(world: &mut World) {
    let pressed = world
        .get_resource::<Input<KeyCode>>()
        .map_or(false, |kb| kb.just_pressed(KeyCode::K));
    if !pressed {
        return;
    }

    let game_started = world
        .get_resource::<GamePhase>()
        .map_or(false, |p| p.game_started);
    if !game_started {
        return;
    }

    if let Some(save) = read_save_data(world) {
        ensure_save_dir();
        if let Ok(json) = serde_json::to_string_pretty(&save) {
            let msg = if fs::write(auto_save_path(), json).is_ok() {
                "Game saved.".to_string()
            } else {
                "Failed to save game.".to_string()
            };
            if let Some(mut alerts) = world.get_resource_mut::<GameAlerts>() {
                alerts.push(msg);
            }
        }
    }
}

// ============================================================
// Load game — exclusive system triggered by LoadRequest
// ============================================================

pub fn load_game_system(world: &mut World) {
    let should_load = world
        .get_resource::<LoadRequest>()
        .map_or(false, |r| r.pending);
    if !should_load {
        return;
    }

    // Clear the request
    if let Some(mut req) = world.get_resource_mut::<LoadRequest>() {
        req.pending = false;
    }

    let path = auto_save_path();
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read save file: {}", e);
            if let Some(mut alerts) = world.get_resource_mut::<GameAlerts>() {
                alerts.push("No save file found.".to_string());
            }
            return;
        }
    };

    let save: SaveFile = match serde_json::from_str(&content) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Save parse error: {}", e);
            if let Some(mut alerts) = world.get_resource_mut::<GameAlerts>() {
                alerts.push("Failed to load save (corrupted).".to_string());
            }
            return;
        }
    };

    // Restore resources
    world.insert_resource(save.economy);
    world.insert_resource(save.bounty_board);
    world.insert_resource(save.game_time);
    world.insert_resource(save.kingdom_state);
    world.insert_resource(save.game_phase);
    world.insert_resource(save.road_network);
    world.insert_resource(save.fog_of_war);
    world.insert_resource(save.milestones);
    world.insert_resource(save.legacy_upgrades);
    world.insert_resource(save.era_state);
    world.insert_resource(save.building_bonuses);
    world.insert_resource(save.active_buffs);

    // Clear alerts
    if let Some(mut alerts) = world.get_resource_mut::<GameAlerts>() {
        alerts.messages.clear();
    }

    // Despawn all game entities
    despawn_all::<Hero>(world);
    despawn_all::<Building>(world);
    despawn_all::<Enemy>(world);
    despawn_all::<MonsterDen>(world);
    despawn_all::<ResourceNode>(world);
    despawn_all::<Merchant>(world);
    despawn_all::<TradeCaravan>(world);

    // Get sprite handles for spawning
    let sprites = world.get_resource::<SpriteAssets>().unwrap().clone();

    // Spawn buildings
    for b in &save.buildings {
        let building = Building {
            building_type: b.building_type,
            tier: b.tier,
            hp: b.hp,
            max_hp: b.max_hp,
            is_destroyed: b.is_destroyed,
        };
        let pos = Vec3::new(b.x, b.y, 5.0);
        world.spawn()
            .insert_bundle(SpriteBundle {
                texture: building_texture_for_tier(&sprites, b.building_type, b.tier),
                transform: Transform::from_translation(pos)
                    .with_scale(Vec3::splat(building_scale_for_tier(b.building_type, b.tier))),
                ..Default::default()
            })
            .insert(building)
            .insert(BuildingVisualTier { tier: b.tier });
    }

    // Spawn heroes
    for h in &save.heroes {
        let hero = Hero {
            class: h.class,
            level: h.level,
            xp: h.xp,
            xp_to_next: h.xp_to_next,
            morale: h.morale,
            gold_carried: h.gold_carried,
            personality: h.personality,
            is_legendary: h.is_legendary,
        };
        let pos = Vec3::new(h.x, h.y, 10.0);
        let class = h.class;
        let (walk_atlas, attack_atlas, hurt_atlas, attack_frames) = match class {
            HeroClass::Warrior => (
                sprites.warrior_atlas.clone(),
                sprites.warrior_attack_atlas.clone(),
                sprites.warrior_hurt_atlas.clone(),
                6,
            ),
            HeroClass::Archer => (
                sprites.archer_atlas.clone(),
                sprites.archer_attack_atlas.clone(),
                sprites.archer_hurt_atlas.clone(),
                13,
            ),
            HeroClass::Mage => (
                sprites.mage_atlas.clone(),
                sprites.mage_attack_atlas.clone(),
                sprites.mage_hurt_atlas.clone(),
                7,
            ),
            HeroClass::Rogue => (
                sprites.rogue_atlas.clone(),
                sprites.rogue_attack_atlas.clone(),
                sprites.rogue_hurt_atlas.clone(),
                6,
            ),
            HeroClass::Healer => (
                sprites.healer_atlas.clone(),
                sprites.healer_attack_atlas.clone(),
                sprites.healer_hurt_atlas.clone(),
                7,
            ),
        };

        let rest_atlas = hurt_atlas.clone();
        let anim_set = AnimationSet {
            walk_atlas: walk_atlas.clone(),
            walk_frames: 9,
            idle_atlas: walk_atlas.clone(),
            idle_frames: 9,
            rest_atlas,
            rest_frames: 6,
            attack_atlas,
            attack_frames,
            hurt_atlas,
            hurt_frames: 6,
            hurt_rows: 1,
            current_mode: AnimMode::Walk,
        };
        let anim = SpriteAnimation::new_directional(9, 8.0);
        let start_index = anim.atlas_index();

        world.spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: walk_atlas,
                transform: Transform::from_translation(pos).with_scale(Vec3::splat(0.7)),
                sprite: TextureAtlasSprite {
                    index: start_index,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(hero)
            .insert(h.stats.clone())
            .insert(h.state.to_state())
            .insert(HeroDecisionTimer::default())
            .insert(AttackCooldown::default())
            .insert(h.equipment.clone())
            .insert(anim)
            .insert(anim_set)
            .insert(ArcaneSurgeCooldown::default());
    }

    // Spawn dens
    for d in &save.dens {
        let den = MonsterDen {
            enemy_type: d.enemy_type,
            spawn_timer: d.spawn_timer,
            spawn_interval: d.spawn_interval,
            max_spawned: d.max_spawned,
            current_spawned: d.current_spawned,
            threat_tier: d.threat_tier,
            weeks_unaddressed: d.weeks_unaddressed,
            hp: d.hp,
            max_hp: d.max_hp,
        };
        let pos = Vec3::new(d.x, d.y, 4.0);
        world.spawn()
            .insert_bundle(SpriteBundle {
                texture: monster_den_texture_for_tier(&sprites, d.threat_tier),
                transform: Transform::from_translation(pos)
                    .with_scale(Vec3::splat(monster_den_scale_for_tier(d.threat_tier))),
                ..Default::default()
            })
            .insert(den)
            .insert(MonsterDenVisualTier { tier: d.threat_tier });
    }

    // Spawn resource nodes
    for r in &save.resource_nodes {
        let color = match r.resource_type {
            ResourceType::Mine => Color::rgb(0.5, 0.4, 0.3),
            ResourceType::LumberMill => Color::rgb(0.3, 0.5, 0.2),
        };
        let mut node = ResourceNode::new(r.resource_type);
        node.is_active = r.is_active;
        node.gather_timer = r.gather_timer;
        let pos = Vec3::new(r.x, r.y, 3.5);
        world.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos),
                ..Default::default()
            })
            .insert(node);
    }

    // Spawn enemies
    for e in &save.enemies {
        let pos = Vec3::new(e.x, e.y, 10.0);
        crate::sprites::spawn_enemy_with_sprite_world(world, &sprites, e.enemy_type, e.stats.clone(), pos);
    }

    // Spawn merchants
    for m in &save.merchants {
        let pos = Vec3::new(m.x, m.y, 9.0);
        world.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.9, 0.8, 0.3),
                    custom_size: Some(Vec2::new(12.0, 12.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos),
                ..Default::default()
            })
            .insert(Merchant {
                gold_value: m.gold_value,
                destination: Vec2::new(m.dest_x, m.dest_y),
                has_arrived: m.has_arrived,
                leave_timer: m.leave_timer,
            });
    }

    // Spawn caravans
    for c in &save.caravans {
        let pos = Vec3::new(c.x, c.y, 9.0);
        world.spawn()
            .insert_bundle(SpriteBundle {
                texture: sprites.caravan_sprites.lvl1.clone(),
                transform: Transform::from_translation(pos)
                    .with_scale(Vec3::splat(0.25)),
                ..Default::default()
            })
            .insert(TradeCaravan {
                item: c.item,
                destination: Vec2::new(c.dest_x, c.dest_y),
                has_arrived: c.has_arrived,
                leave_timer: c.leave_timer,
            });
    }

    // Restore camera
    {
        let mut q = QueryState::<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>::new(world);
        for (mut t, mut proj) in q.iter_mut(world) {
            t.translation.x = save.camera.x;
            t.translation.y = save.camera.y;
            proj.scale = save.camera.zoom;
        }
    }

    if let Some(mut alerts) = world.get_resource_mut::<GameAlerts>() {
        alerts.push("Game loaded!".to_string());
    }
}

/// Despawn all entities with a given component marker
fn despawn_all<T: Component>(world: &mut World) {
    let entities: Vec<Entity> = {
        let mut q = QueryState::<Entity, With<T>>::new(world);
        q.iter(world).collect()
    };
    for e in entities {
        if let Some(entity) = world.get_entity_mut(e) {
            entity.despawn();
        }
    }
}
