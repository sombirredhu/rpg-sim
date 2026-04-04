//! Centralized sprite asset loading and mapping.
//!
//! Every entity that needs a visual uses handles stored in [`SpriteAssets`],
//! which is populated once during startup.

use bevy::prelude::*;

use crate::components::*;

#[derive(Clone)]
pub struct BuildingSpriteSet {
    pub lvl1: Handle<Image>,
    pub lvl2: Handle<Image>,
    pub lvl3: Handle<Image>,
}

impl BuildingSpriteSet {
    pub fn for_tier(&self, tier: u32) -> Handle<Image> {
        match tier {
            0 | 1 => self.lvl1.clone(),
            2 => self.lvl2.clone(),
            _ => self.lvl3.clone(),
        }
    }
}

/// Pre-loaded texture handles for every visual in the game.
pub struct SpriteAssets {
    // Heroes
    pub warrior_atlas: Handle<TextureAtlas>,
    pub archer_atlas: Handle<TextureAtlas>,
    pub mage_tex: Handle<Image>,
    pub rogue_tex: Handle<Image>,
    pub healer_tex: Handle<Image>,

    // Enemies
    pub goblin_atlas: Handle<TextureAtlas>,
    pub bandit_atlas: Handle<TextureAtlas>,
    pub troll_tex: Handle<Image>,
    pub goblin_elite_tex: Handle<Image>,
    pub boss_tex: Handle<Image>,

    // Buildings from assets/GameplayAssetsV2/buildings
    pub townhall_sprites: BuildingSpriteSet,
    pub inn_sprites: BuildingSpriteSet,
    pub market_sprites: BuildingSpriteSet,
    pub temple_sprites: BuildingSpriteSet,
    pub guard_tower_sprites: BuildingSpriteSet,
    pub wizard_tower_sprites: BuildingSpriteSet,
    pub blacksmith_sprites: BuildingSpriteSet,
    pub alchemist_sprites: BuildingSpriteSet,
    pub barracks_sprites: BuildingSpriteSet,
    pub monster_den_sprites: BuildingSpriteSet,

    // Environment
    pub grass_atlas: Handle<TextureAtlas>,
    pub trees_atlas: Handle<TextureAtlas>,
    pub shadow_tex: Handle<Image>,

    // UI / effects
    pub healthbar_tex: Handle<Image>,
    pub target_tex: Handle<Image>,
}

fn load_building_set(asset_server: &Res<AssetServer>, folder: &str) -> BuildingSpriteSet {
    BuildingSpriteSet {
        lvl1: asset_server.load(&format!("GameplayAssetsV2/buildings/{}/lvl1.png", folder)),
        lvl2: asset_server.load(&format!("GameplayAssetsV2/buildings/{}/lvl2.png", folder)),
        lvl3: asset_server.load(&format!("GameplayAssetsV2/buildings/{}/lvl3.png", folder)),
    }
}

fn building_texture_for_tier(
    sprites: &SpriteAssets,
    building_type: BuildingType,
    tier: u32,
) -> Handle<Image> {
    match building_type {
        BuildingType::TownHall => sprites.townhall_sprites.for_tier(tier),
        BuildingType::Inn => sprites.inn_sprites.for_tier(tier),
        BuildingType::Market => sprites.market_sprites.for_tier(tier),
        BuildingType::Temple => sprites.temple_sprites.for_tier(tier),
        BuildingType::GuardTower => sprites.guard_tower_sprites.for_tier(tier),
        BuildingType::WizardTower => sprites.wizard_tower_sprites.for_tier(tier),
        BuildingType::Blacksmith => sprites.blacksmith_sprites.for_tier(tier),
        BuildingType::Alchemist => sprites.alchemist_sprites.for_tier(tier),
        BuildingType::Barracks => sprites.barracks_sprites.for_tier(tier),
    }
}

fn building_scale_for_tier(building_type: BuildingType, tier: u32) -> f32 {
    let base = match building_type {
        BuildingType::TownHall => 0.34,
        BuildingType::GuardTower => 0.30,
        BuildingType::Temple => 0.31,
        BuildingType::WizardTower => 0.31,
        BuildingType::Barracks => 0.30,
        _ => 0.29,
    };
    let tier_bonus = match tier {
        0 | 1 => 0.00,
        2 => 0.02,
        _ => 0.04,
    };
    base + tier_bonus
}

pub fn monster_den_texture_for_tier(sprites: &SpriteAssets, tier: u32) -> Handle<Image> {
    sprites.monster_den_sprites.for_tier(tier)
}

pub fn monster_den_scale_for_tier(tier: u32) -> f32 {
    match tier {
        0 | 1 => 0.27,
        2 => 0.30,
        _ => 0.33,
    }
}

/// Startup system - loads every asset once and stores the handles.
pub fn load_sprite_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Hero atlases
    let warrior_tex = asset_server.load("Character/Leader/Daniel/Daniel_Blue.png");
    let warrior_atlas = texture_atlases.add(TextureAtlas::from_grid(
        warrior_tex,
        Vec2::new(40.0, 32.0),
        1,
        1,
    ));

    let archer_tex = asset_server.load("Character/Creep/RangeCreep/RangeCreep_Blue.png");
    let archer_atlas = texture_atlases.add(TextureAtlas::from_grid(
        archer_tex,
        Vec2::new(32.0, 32.0),
        4,
        1,
    ));

    let mage_tex: Handle<Image> = asset_server.load("Character/Leader/Raja/Raja_Blue.png");
    let rogue_tex: Handle<Image> = asset_server.load("Character/Leader/Robin/Robin_Blue.png");
    let healer_tex: Handle<Image> = asset_server.load("Character/Leader/Sami/Sami_Blue.png");

    // Enemy atlases
    let goblin_tex = asset_server.load("Character/Creep/MeleeCreep/MeleeCreep_Red.png");
    let goblin_atlas = texture_atlases.add(TextureAtlas::from_grid(
        goblin_tex,
        Vec2::new(24.0, 24.0),
        4,
        1,
    ));

    let bandit_tex = asset_server.load("Character/Creep/RangeCreep/RangeCreep_Red.png");
    let bandit_atlas = texture_atlases.add(TextureAtlas::from_grid(
        bandit_tex,
        Vec2::new(32.0, 32.0),
        4,
        1,
    ));

    let troll_tex: Handle<Image> = asset_server.load("Character/Leader/Maori/Maori_Red.png");
    let goblin_elite_tex: Handle<Image> = asset_server.load("Character/Leader/Daniel/Daniel_Red.png");
    let boss_tex: Handle<Image> = asset_server.load("Character/Leader/Rollo/Rollo_Red.png");

    // Building sets from processed NewAssets
    let townhall_sprites = load_building_set(&asset_server, "townhall");
    let inn_sprites = load_building_set(&asset_server, "inn");
    let market_sprites = load_building_set(&asset_server, "market");
    let temple_sprites = load_building_set(&asset_server, "temple");
    let guard_tower_sprites = load_building_set(&asset_server, "guard_tower");
    let wizard_tower_sprites = load_building_set(&asset_server, "wizard_tower");
    let blacksmith_sprites = load_building_set(&asset_server, "blacksmith");
    let alchemist_sprites = load_building_set(&asset_server, "alchemist");
    let barracks_sprites = load_building_set(&asset_server, "barracks");
    let monster_den_sprites = load_building_set(&asset_server, "monster_den");

    // Environment
    let grass_tex = asset_server.load("Level/Ground/grass.png");
    let grass_atlas = texture_atlases.add(TextureAtlas::from_grid(
        grass_tex,
        Vec2::new(8.0, 8.0),
        11,
        5,
    ));

    let trees_tex = asset_server.load("Level/Tress/Trees.png");
    let trees_atlas = texture_atlases.add(TextureAtlas::from_grid(
        trees_tex,
        Vec2::new(40.0, 48.0),
        2,
        1,
    ));

    let shadow_tex: Handle<Image> = asset_server.load("Character/shadow.png");

    // UI / effects
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
        townhall_sprites,
        inn_sprites,
        market_sprites,
        temple_sprites,
        guard_tower_sprites,
        wizard_tower_sprites,
        blacksmith_sprites,
        alchemist_sprites,
        barracks_sprites,
        monster_den_sprites,
        grass_atlas,
        trees_atlas,
        shadow_tex,
        healthbar_tex,
        target_tex,
    });
}

pub fn spawn_hero_with_sprite(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    class: HeroClass,
    position: Vec3,
) -> Entity {
    let hero = Hero::new(class);
    let stats = class.base_stats();

    match class {
        HeroClass::Warrior => commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: sprites.warrior_atlas.clone(),
            transform: Transform::from_translation(position).with_scale(Vec3::new(-1.5, 1.5, 1.5)),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        }),
        HeroClass::Archer => commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: sprites.archer_atlas.clone(),
            transform: Transform::from_translation(position).with_scale(Vec3::splat(1.5)),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        }),
        HeroClass::Mage => commands.spawn_bundle(SpriteBundle {
            texture: sprites.mage_tex.clone(),
            transform: Transform::from_translation(position).with_scale(Vec3::splat(1.5)),
            ..Default::default()
        }),
        HeroClass::Rogue => commands.spawn_bundle(SpriteBundle {
            texture: sprites.rogue_tex.clone(),
            transform: Transform::from_translation(position).with_scale(Vec3::splat(1.5)),
            ..Default::default()
        }),
        HeroClass::Healer => commands.spawn_bundle(SpriteBundle {
            texture: sprites.healer_tex.clone(),
            transform: Transform::from_translation(position).with_scale(Vec3::splat(1.5)),
            ..Default::default()
        }),
    }
    .insert(hero)
    .insert(stats)
    .insert(HeroState::Idle)
    .insert(HeroDecisionTimer::default())
    .insert(AttackCooldown::default())
    .insert(HeroEquipment::default())
    .id()
}

pub fn spawn_enemy_with_sprite(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    enemy_type: EnemyType,
    position: Vec3,
) -> Entity {
    let stats = enemy_type.stats();

    match enemy_type {
        EnemyType::Goblin => commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: sprites.goblin_atlas.clone(),
            transform: Transform::from_translation(position).with_scale(Vec3::splat(1.5)),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        }),
        EnemyType::Bandit => commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: sprites.bandit_atlas.clone(),
            transform: Transform::from_translation(position).with_scale(Vec3::splat(1.5)),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        }),
        EnemyType::Troll => commands.spawn_bundle(SpriteBundle {
            texture: sprites.troll_tex.clone(),
            transform: Transform::from_translation(position).with_scale(Vec3::splat(2.0)),
            ..Default::default()
        }),
        EnemyType::GoblinElite => commands.spawn_bundle(SpriteBundle {
            texture: sprites.goblin_elite_tex.clone(),
            transform: Transform::from_translation(position).with_scale(Vec3::splat(1.8)),
            ..Default::default()
        }),
        EnemyType::BossWarlord => commands.spawn_bundle(SpriteBundle {
            texture: sprites.boss_tex.clone(),
            transform: Transform::from_translation(position).with_scale(Vec3::splat(3.0)),
            ..Default::default()
        }),
        EnemyType::Werewolf => commands.spawn_bundle(SpriteBundle {
            texture: sprites.troll_tex.clone(),
            sprite: Sprite {
                color: Color::rgb(0.6, 0.4, 0.8),
                ..Default::default()
            },
            transform: Transform::from_translation(position).with_scale(Vec3::splat(1.8)),
            ..Default::default()
        }),
        EnemyType::ShadowBandit => commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: sprites.bandit_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                color: Color::rgb(0.3, 0.2, 0.4),
                ..Default::default()
            },
            transform: Transform::from_translation(position).with_scale(Vec3::splat(1.5)),
            ..Default::default()
        }),
    }
    .insert(Enemy { enemy_type })
    .insert(stats)
    .insert(EnemyAi::default())
    .insert(AttackCooldown {
        timer: 0.0,
        interval: 1.5,
    })
    .id()
}

pub fn spawn_building_with_sprite(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    building_type: BuildingType,
    position: Vec3,
) -> Entity {
    let building = Building::new(building_type);
    let tier = building.tier;

    commands
        .spawn_bundle(SpriteBundle {
            texture: building_texture_for_tier(sprites, building_type, tier),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(building_scale_for_tier(building_type, tier))),
            ..Default::default()
        })
        .insert(building)
        .insert(BuildingVisualTier { tier })
        .id()
}

/// Keep building sprites in sync with current building tier.
pub fn sync_building_tier_visuals(
    sprites: Res<SpriteAssets>,
    mut query: Query<(
        &Building,
        &mut Handle<Image>,
        &mut Transform,
        &mut BuildingVisualTier,
    )>,
) {
    for (building, mut texture, mut transform, mut visual_tier) in query.iter_mut() {
        if visual_tier.tier == building.tier {
            continue;
        }

        *texture = building_texture_for_tier(&sprites, building.building_type, building.tier);
        transform.scale = Vec3::splat(building_scale_for_tier(building.building_type, building.tier));
        visual_tier.tier = building.tier;
    }
}

/// Keep monster den sprites in sync with threat tier.
pub fn sync_monster_den_tier_visuals(
    sprites: Res<SpriteAssets>,
    mut query: Query<(
        &MonsterDen,
        &mut Handle<Image>,
        &mut Transform,
        &mut MonsterDenVisualTier,
    )>,
) {
    for (den, mut texture, mut transform, mut visual_tier) in query.iter_mut() {
        if visual_tier.tier == den.threat_tier {
            continue;
        }

        *texture = monster_den_texture_for_tier(&sprites, den.threat_tier);
        transform.scale = Vec3::splat(monster_den_scale_for_tier(den.threat_tier));
        visual_tier.tier = den.threat_tier;
    }
}

pub fn spawn_ground_tiles(mut commands: Commands, sprites: Res<SpriteAssets>) {
    let tile_size = 8.0 * 3.0;
    let half_map = 600.0;

    let mut x = -half_map;
    while x < half_map {
        let mut y = -half_map;
        while y < half_map {
            let index = ((rand::random::<f32>() * 4.0) as usize).min(3);

            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.grass_atlas.clone(),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)).with_scale(Vec3::splat(3.0)),
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

pub fn spawn_trees(mut commands: Commands, sprites: Res<SpriteAssets>) {
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
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 3.0)).with_scale(Vec3::splat(1.5)),
            sprite: TextureAtlasSprite {
                index: tree_variant,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
