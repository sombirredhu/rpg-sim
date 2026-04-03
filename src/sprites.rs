//! Centralised sprite asset loading and mapping.
//!
//! Every entity that needs a visual uses handles stored in [`SpriteAssets`],
//! which is populated once during startup.

use bevy::prelude::*;
use crate::components::*;

/// Pre-loaded texture handles for every visual in the game.
pub struct SpriteAssets {
    // ── Heroes (Blue team) ──────────────────────────────────────────
    pub warrior_atlas: Handle<TextureAtlas>,  // Daniel_Blue  40×32 → 1 col
    pub archer_atlas: Handle<TextureAtlas>,   // RangeCreep_Blue 128×32 → 4 cols of 32×32
    pub mage_tex: Handle<Image>,              // Raja_Blue 32×32 single
    pub rogue_tex: Handle<Image>,             // Robin_Blue 32×32 single
    pub healer_tex: Handle<Image>,            // Sami_Blue 32×32 single

    // ── Enemies (Red team) ──────────────────────────────────────────
    pub goblin_atlas: Handle<TextureAtlas>,   // MeleeCreep_Red 96×24 → 4 cols of 24×24
    pub bandit_atlas: Handle<TextureAtlas>,   // RangeCreep_Red 128×32 → 4 cols of 32×32
    pub troll_tex: Handle<Image>,             // Maori_Red 40×32 single
    pub goblin_elite_tex: Handle<Image>,      // Daniel_Red 40×32 single
    pub boss_tex: Handle<Image>,              // Rollo_Red 40×32 single

    // ── Buildings ───────────────────────────────────────────────────
    pub building_blue_atlas: Handle<TextureAtlas>, // BlueBuilding 240×128 → 2 cols (big 96×128 + small 96×128)
    pub building_red_atlas: Handle<TextureAtlas>,  // RedBuilding  240×128

    // ── Environment ─────────────────────────────────────────────────
    pub grass_atlas: Handle<TextureAtlas>,    // grass.png 88×40 → tiles of 8×8
    pub trees_atlas: Handle<TextureAtlas>,    // Trees.png 80×48 → 2 cols of 40×48
    pub shadow_tex: Handle<Image>,            // shadow.png 32×12

    // ── UI / Effects ────────────────────────────────────────────────
    pub healthbar_tex: Handle<Image>,         // HealthBar.png 26×13
    pub target_tex: Handle<Image>,            // target.png 32×32
}

/// Startup system – loads every asset once and stores the handles.
pub fn load_sprite_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // ── Hero atlases ────────────────────────────────────────────────
    let warrior_tex = asset_server.load("Character/Leader/Daniel/Daniel_Blue.png");
    let warrior_atlas = texture_atlases.add(TextureAtlas::from_grid(
        warrior_tex, Vec2::new(40.0, 32.0), 1, 1,
    ));

    let archer_tex = asset_server.load("Character/Creep/RangeCreep/RangeCreep_Blue.png");
    let archer_atlas = texture_atlases.add(TextureAtlas::from_grid(
        archer_tex, Vec2::new(32.0, 32.0), 4, 1,
    ));

    let mage_tex: Handle<Image> = asset_server.load("Character/Leader/Raja/Raja_Blue.png");
    let rogue_tex: Handle<Image> = asset_server.load("Character/Leader/Robin/Robin_Blue.png");
    let healer_tex: Handle<Image> = asset_server.load("Character/Leader/Sami/Sami_Blue.png");

    // ── Enemy atlases ───────────────────────────────────────────────
    let goblin_tex = asset_server.load("Character/Creep/MeleeCreep/MeleeCreep_Red.png");
    let goblin_atlas = texture_atlases.add(TextureAtlas::from_grid(
        goblin_tex, Vec2::new(24.0, 24.0), 4, 1,
    ));

    let bandit_tex = asset_server.load("Character/Creep/RangeCreep/RangeCreep_Red.png");
    let bandit_atlas = texture_atlases.add(TextureAtlas::from_grid(
        bandit_tex, Vec2::new(32.0, 32.0), 4, 1,
    ));

    let troll_tex: Handle<Image> = asset_server.load("Character/Leader/Maori/Maori_Red.png");
    let goblin_elite_tex: Handle<Image> = asset_server.load("Character/Leader/Daniel/Daniel_Red.png");
    let boss_tex: Handle<Image> = asset_server.load("Character/Leader/Rollo/Rollo_Red.png");

    // ── Building atlases ────────────────────────────────────────────
    let blue_bld_tex = asset_server.load("Level/Building/BlueBuilding.png");
    let building_blue_atlas = texture_atlases.add(TextureAtlas::from_grid(
        blue_bld_tex, Vec2::new(96.0, 128.0), 2, 1,
    ));

    let red_bld_tex = asset_server.load("Level/Building/RedBuilding.png");
    let building_red_atlas = texture_atlases.add(TextureAtlas::from_grid(
        red_bld_tex, Vec2::new(96.0, 128.0), 2, 1,
    ));

    // ── Environment ─────────────────────────────────────────────────
    let grass_tex = asset_server.load("Level/Ground/grass.png");
    let grass_atlas = texture_atlases.add(TextureAtlas::from_grid(
        grass_tex, Vec2::new(8.0, 8.0), 11, 5,
    ));

    let trees_tex = asset_server.load("Level/Tress/Trees.png");
    let trees_atlas = texture_atlases.add(TextureAtlas::from_grid(
        trees_tex, Vec2::new(40.0, 48.0), 2, 1,
    ));

    let shadow_tex: Handle<Image> = asset_server.load("Character/shadow.png");

    // ── UI / Effects ────────────────────────────────────────────────
    let healthbar_tex: Handle<Image> = asset_server.load("HealthBar/HealthBar.png");
    let target_tex: Handle<Image> = asset_server.load("Effects/target.png");

    commands.insert_resource(SpriteAssets {
        warrior_atlas,
        archer_atlas,
        mage_tex,
        rogue_tex,
        healer_tex,
        goblin_atlas,
        bandit_atlas,
        troll_tex,
        goblin_elite_tex,
        boss_tex,
        building_blue_atlas,
        building_red_atlas,
        grass_atlas,
        trees_atlas,
        shadow_tex,
        healthbar_tex,
        target_tex,
    });
}

// ─── Helper: spawn a hero entity with the correct sprite ────────────────

pub fn spawn_hero_with_sprite(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    class: HeroClass,
    position: Vec3,
) -> Entity {
    let hero = Hero::new(class);
    let stats = class.base_stats();

    match class {
        // Atlas-based heroes (spritesheet)
        HeroClass::Warrior => {
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.warrior_atlas.clone(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::new(-1.5, 1.5, 1.5)),
                sprite: TextureAtlasSprite { index: 0, ..Default::default() },
                ..Default::default()
            })
        }
        HeroClass::Archer => {
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.archer_atlas.clone(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(1.5)),
                sprite: TextureAtlasSprite { index: 0, ..Default::default() },
                ..Default::default()
            })
        }
        HeroClass::Mage => {
            commands.spawn_bundle(SpriteBundle {
                texture: sprites.mage_tex.clone(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(1.5)),
                ..Default::default()
            })
        }
        HeroClass::Rogue => {
            commands.spawn_bundle(SpriteBundle {
                texture: sprites.rogue_tex.clone(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(1.5)),
                ..Default::default()
            })
        }
        HeroClass::Healer => {
            commands.spawn_bundle(SpriteBundle {
                texture: sprites.healer_tex.clone(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(1.5)),
                ..Default::default()
            })
        }
    }
    .insert(hero)
    .insert(stats)
    .insert(HeroState::Idle)
    .insert(HeroDecisionTimer::default())
    .insert(AttackCooldown::default())
    .id()
}

// ─── Helper: spawn an enemy entity with the correct sprite ──────────────

pub fn spawn_enemy_with_sprite(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    enemy_type: EnemyType,
    position: Vec3,
) -> Entity {
    let stats = enemy_type.stats();

    match enemy_type {
        EnemyType::Goblin => {
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.goblin_atlas.clone(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(1.5)),
                sprite: TextureAtlasSprite { index: 0, ..Default::default() },
                ..Default::default()
            })
        }
        EnemyType::Bandit => {
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.bandit_atlas.clone(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(1.5)),
                sprite: TextureAtlasSprite { index: 0, ..Default::default() },
                ..Default::default()
            })
        }
        EnemyType::Troll => {
            commands.spawn_bundle(SpriteBundle {
                texture: sprites.troll_tex.clone(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(2.0)),
                ..Default::default()
            })
        }
        EnemyType::GoblinElite => {
            commands.spawn_bundle(SpriteBundle {
                texture: sprites.goblin_elite_tex.clone(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(1.8)),
                ..Default::default()
            })
        }
        EnemyType::BossWarlord => {
            commands.spawn_bundle(SpriteBundle {
                texture: sprites.boss_tex.clone(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(3.0)),
                ..Default::default()
            })
        }
        EnemyType::Werewolf => {
            // Reuse troll sprite with purple tint for werewolf
            commands.spawn_bundle(SpriteBundle {
                texture: sprites.troll_tex.clone(),
                sprite: Sprite {
                    color: Color::rgb(0.6, 0.4, 0.8),
                    ..Default::default()
                },
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(1.8)),
                ..Default::default()
            })
        }
        EnemyType::ShadowBandit => {
            // Reuse bandit atlas with dark tint for shadow bandit
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.bandit_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    color: Color::rgb(0.3, 0.2, 0.4),
                    ..Default::default()
                },
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(1.5)),
                ..Default::default()
            })
        }
    }
    .insert(Enemy { enemy_type })
    .insert(stats)
    .insert(EnemyAi::default())
    .insert(AttackCooldown { timer: 0.0, interval: 1.5 })
    .id()
}

// ─── Helper: spawn a building with the correct sprite ───────────────────

pub fn spawn_building_with_sprite(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    building_type: BuildingType,
    position: Vec3,
) -> Entity {
    let building = Building::new(building_type);

    // Big castle sprite (index 0) for TownHall, small castle (index 1) for others
    let sprite_index = match building_type {
        BuildingType::TownHall => 0,
        _ => 1,
    };

    let scale = match building_type {
        BuildingType::TownHall => 1.2,
        BuildingType::GuardTower => 0.7,
        _ => 0.8,
    };

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: sprites.building_blue_atlas.clone(),
        transform: Transform::from_translation(position)
            .with_scale(Vec3::splat(scale)),
        sprite: TextureAtlasSprite { index: sprite_index, ..Default::default() },
        ..Default::default()
    })
    .insert(building)
    .id()
}

// ─── Startup: spawn grass ground tiles ──────────────────────────────────

pub fn spawn_ground_tiles(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
) {
    let tile_size = 8.0 * 3.0; // 8px tiles scaled 3×
    let half_map = 600.0;

    // Fill the ground with grass tiles
    let mut x = -half_map;
    while x < half_map {
        let mut y = -half_map;
        while y < half_map {
            // Pick a random grass tile variant (first row of the grass atlas has nice tiles)
            let index = ((rand::random::<f32>() * 4.0) as usize).min(3);

            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.grass_atlas.clone(),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0))
                    .with_scale(Vec3::splat(3.0)),
                sprite: TextureAtlasSprite {
                    index,
                    ..Default::default()
                },
                ..Default::default()
            });

            y += tile_size;
        }
        x += tile_size;
    }
}

// ─── Startup: scatter trees around map edges ────────────────────────────

pub fn spawn_trees(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
) {
    // Scatter decorative trees around the outskirts
    let tree_positions: Vec<Vec2> = (0..40)
        .map(|_| {
            let angle = rand::random::<f32>() * std::f32::consts::TAU;
            let radius = 250.0 + rand::random::<f32>() * 300.0;
            Vec2::new(angle.cos() * radius, angle.sin() * radius)
        })
        .collect();

    for pos in tree_positions {
        let tree_variant = if rand::random::<bool>() { 0 } else { 1 };
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: sprites.trees_atlas.clone(),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 3.0))
                .with_scale(Vec3::splat(1.5)),
            sprite: TextureAtlasSprite {
                index: tree_variant,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
