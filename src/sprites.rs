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
#[derive(Clone)]
pub struct SpriteAssets {
    // Heroes — walk atlases (LPC 9 cols × 4 rows, 64×64)
    pub warrior_atlas: Handle<TextureAtlas>,
    pub archer_atlas: Handle<TextureAtlas>,
    pub mage_atlas: Handle<TextureAtlas>,
    pub rogue_atlas: Handle<TextureAtlas>,
    pub healer_atlas: Handle<TextureAtlas>,
    pub civilian_atlas: Handle<TextureAtlas>,

    // Heroes — attack atlases (LPC directional, 64×64)
    pub warrior_attack_atlas: Handle<TextureAtlas>,   // slash  6 cols × 4 rows
    pub archer_attack_atlas: Handle<TextureAtlas>,    // bow   13 cols × 4 rows
    pub mage_attack_atlas: Handle<TextureAtlas>,      // spellcast 7 cols × 4 rows
    pub rogue_attack_atlas: Handle<TextureAtlas>,     // slash  6 cols × 4 rows
    pub healer_attack_atlas: Handle<TextureAtlas>,    // spellcast 7 cols × 4 rows

    // Heroes — hurt atlases (LPC 6 cols × 1 row, 64×64)
    pub warrior_hurt_atlas: Handle<TextureAtlas>,
    pub archer_hurt_atlas: Handle<TextureAtlas>,
    pub mage_hurt_atlas: Handle<TextureAtlas>,
    pub rogue_hurt_atlas: Handle<TextureAtlas>,
    pub healer_hurt_atlas: Handle<TextureAtlas>,

    // Enemies
    pub goblin_atlas: Handle<TextureAtlas>,
    pub bandit_atlas: Handle<TextureAtlas>,
    pub skeleton_atlas: Handle<TextureAtlas>,
    pub skeleton_attack_atlas: Handle<TextureAtlas>,  // slash 6 cols × 4 rows
    pub skeleton_hurt_atlas: Handle<TextureAtlas>,    // hurt  6 cols × 1 row
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

    // Environment — single large ground texture
    pub ground_large_tex: Handle<Image>,
    // Legacy tiles kept for road system
    pub grass_tile_tex: Handle<Image>,
    pub road_stone_tex: Handle<Image>,
    pub water_tile_tex: Handle<Image>,
    pub rock_tile_tex: Handle<Image>,
    pub rock_wet_tex: Handle<Image>,
    pub road_edge_tex: Handle<Image>,

    // HD Decorations
    pub deco_tree_big1: Handle<Image>,
    pub deco_tree_big2: Handle<Image>,
    pub deco_pine1: Handle<Image>,
    pub deco_pine2: Handle<Image>,
    pub deco_bush1: Handle<Image>,
    pub deco_bush2: Handle<Image>,
    pub deco_flowers1: Handle<Image>,
    pub deco_flowers2: Handle<Image>,
    pub deco_mushrooms: Handle<Image>,
    pub deco_standing_stone1: Handle<Image>,
    pub deco_standing_stone2: Handle<Image>,
    pub deco_stone_circle: Handle<Image>,

    // Legacy environment (kept for trees)
    pub grass_atlas: Handle<TextureAtlas>,
    pub trees_atlas: Handle<TextureAtlas>,
    pub shadow_tex: Handle<Image>,

    // Merchant / caravan
    pub caravan_sprites: BuildingSpriteSet,

    // UI / effects
    pub healthbar_tex: Handle<Image>,
    pub target_tex: Handle<Image>,

    // RPG Icons (from Medieval art pack)
    pub icon_gold_coin: Handle<Image>,
    pub icon_bounty_scroll: Handle<Image>,
    pub icon_bounty_monster: Handle<Image>,
    pub icon_bounty_explore: Handle<Image>,
    pub icon_bounty_objective: Handle<Image>,
    pub icon_bounty_resource: Handle<Image>,
    pub icon_potion_health: Handle<Image>,
    pub icon_potion_mana: Handle<Image>,
    pub icon_potion_speed: Handle<Image>,
    pub icon_weapon_sword: Handle<Image>,
    pub icon_weapon_bow: Handle<Image>,
    pub icon_weapon_staff: Handle<Image>,
    pub icon_weapon_dagger: Handle<Image>,
    pub icon_armor_leather: Handle<Image>,
    pub icon_armor_chain: Handle<Image>,
    pub icon_armor_plate: Handle<Image>,
    pub icon_skill_heal: Handle<Image>,
    pub icon_skill_fortify: Handle<Image>,
    pub icon_skill_stealth: Handle<Image>,
    pub icon_skill_fireball: Handle<Image>,
    pub icon_skill_volley: Handle<Image>,
    pub icon_food_bread: Handle<Image>,
    pub icon_torch: Handle<Image>,
    pub icon_chest: Handle<Image>,
    pub icon_medal: Handle<Image>,
    pub icon_clock: Handle<Image>,

    // Medieval terrain base
    pub grassland_tex: Handle<Image>,
    pub grass_overcast_tex: Handle<Image>,

    // Terrain overlay textures (semi-transparent, z=0.05-0.3)
    pub water_overlay_tex: Handle<Image>,
    pub rock_overlay_tex: Handle<Image>,
    pub rock_wet_overlay_tex: Handle<Image>,
    pub path_overlay_tex: Handle<Image>,
    pub path_overlay2_tex: Handle<Image>,
    pub bark_overlay_tex: Handle<Image>,

    // Additional decoration types (existing but unused assets)
    pub deco_tree_oak1: Handle<Image>,
    pub deco_tree_oak2: Handle<Image>,
    pub deco_tree_dead1: Handle<Image>,
    pub deco_tree_dead2: Handle<Image>,
    pub deco_tree_bush1: Handle<Image>,
    pub deco_tree_bush2: Handle<Image>,
    pub deco_tree_bush3: Handle<Image>,
    pub deco_pine3: Handle<Image>,
    pub deco_pine4: Handle<Image>,
    pub deco_pine5: Handle<Image>,
    pub deco_pine6: Handle<Image>,
    pub deco_plant1: Handle<Image>,
    pub deco_plant2: Handle<Image>,
    pub deco_rock_small1: Handle<Image>,
    pub deco_rock_small2: Handle<Image>,
    pub deco_rock_small3: Handle<Image>,
    pub deco_rock_flat: Handle<Image>,
    pub deco_ruin_arch1: Handle<Image>,
    pub deco_ruin_arch2: Handle<Image>,
    pub deco_ruin_pillar1: Handle<Image>,
    pub deco_ruin_pillar2: Handle<Image>,
    pub deco_ruin_wall1: Handle<Image>,
    pub deco_ruin_wall2: Handle<Image>,
    pub deco_temple_ruin: Handle<Image>,
    pub deco_fence1: Handle<Image>,
    pub deco_fence2: Handle<Image>,
    pub deco_fence_post: Handle<Image>,
    pub deco_stone_block1: Handle<Image>,
    pub deco_stone_block2: Handle<Image>,
    pub deco_stone_step: Handle<Image>,
    pub deco_cave1: Handle<Image>,
    pub deco_cave2: Handle<Image>,
    pub deco_cave3: Handle<Image>,
    pub deco_cabin_wall1: Handle<Image>,
    pub deco_cabin_wall2: Handle<Image>,
    pub deco_cabin_wall3: Handle<Image>,
    pub deco_water_tile1: Handle<Image>,
    pub deco_water_tile2: Handle<Image>,
    pub deco_bridge1: Handle<Image>,
    pub deco_bush_1: Handle<Image>,
    pub deco_bush_2: Handle<Image>,
    pub deco_bush_3: Handle<Image>,
    pub deco_bush_4: Handle<Image>,
}

fn load_building_set(asset_server: &Res<AssetServer>, folder: &str) -> BuildingSpriteSet {
    BuildingSpriteSet {
        lvl1: asset_server.load(&format!("GameplayAssetsV2/buildings/{}/lvl1.png", folder)),
        lvl2: asset_server.load(&format!("GameplayAssetsV2/buildings/{}/lvl2.png", folder)),
        lvl3: asset_server.load(&format!("GameplayAssetsV2/buildings/{}/lvl3.png", folder)),
    }
}

pub fn building_texture_for_tier(
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

pub fn building_scale_for_tier(building_type: BuildingType, tier: u32) -> f32 {
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
    // Hero LPC animated sprite sheets (64×64 grid, 9 cols × 4 rows)
    let warrior_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/warrior/walkcycle.png"),
        Vec2::new(64.0, 64.0), 9, 4,
    ));
    let archer_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/archer/walkcycle.png"),
        Vec2::new(64.0, 64.0), 9, 4,
    ));
    let mage_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/mage/walkcycle.png"),
        Vec2::new(64.0, 64.0), 9, 4,
    ));
    let rogue_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/rogue/walkcycle.png"),
        Vec2::new(64.0, 64.0), 9, 4,
    ));
    let healer_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/healer/walkcycle.png"),
        Vec2::new(64.0, 64.0), 9, 4,
    ));
    let civilian_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/civilian/walkcycle.png"),
        Vec2::new(64.0, 64.0), 9, 4,
    ));

    // Hero attack atlases (LPC directional, 64×64)
    let warrior_attack_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/warrior/slash.png"),
        Vec2::new(64.0, 64.0), 6, 4,
    ));
    let archer_attack_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/archer/bow.png"),
        Vec2::new(64.0, 64.0), 13, 4,
    ));
    let mage_attack_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/mage/spellcast.png"),
        Vec2::new(64.0, 64.0), 7, 4,
    ));
    let rogue_attack_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/rogue/slash.png"),
        Vec2::new(64.0, 64.0), 6, 4,
    ));
    let healer_attack_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/healer/spellcast.png"),
        Vec2::new(64.0, 64.0), 7, 4,
    ));

    // Hero hurt atlases (LPC 6 cols × 1 row, 64×64)
    let warrior_hurt_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/warrior/hurt.png"),
        Vec2::new(64.0, 64.0), 6, 1,
    ));
    let archer_hurt_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/archer/hurt.png"),
        Vec2::new(64.0, 64.0), 6, 1,
    ));
    let mage_hurt_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/mage/hurt.png"),
        Vec2::new(64.0, 64.0), 6, 1,
    ));
    let rogue_hurt_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/rogue/hurt.png"),
        Vec2::new(64.0, 64.0), 6, 1,
    ));
    let healer_hurt_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/healer/hurt.png"),
        Vec2::new(64.0, 64.0), 6, 1,
    ));

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

    // Skeleton enemy (LPC, 9 cols × 4 rows)
    let skeleton_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/skeleton/walkcycle.png"),
        Vec2::new(64.0, 64.0), 9, 4,
    ));
    let skeleton_attack_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/skeleton/slash.png"),
        Vec2::new(64.0, 64.0), 6, 4,
    ));
    let skeleton_hurt_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("Character/LPC/skeleton/hurt.png"),
        Vec2::new(64.0, 64.0), 6, 1,
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

    // Environment — medieval grassland replaces single ground texture
    let grassland_tex: Handle<Image> = asset_server.load("Level/Ground/grassland.png");
    let grass_overcast_tex: Handle<Image> = asset_server.load("Level/Ground/grass_overcast.png");
    // Medieval terrain overlays
    let water_overlay_tex: Handle<Image> = asset_server.load("Level/Ground/water_overlay.png");
    let rock_overlay_tex: Handle<Image> = asset_server.load("Level/Ground/rock_overlay.png");
    let rock_wet_overlay_tex: Handle<Image> = asset_server.load("Level/Ground/rock_wet_overlay.png");
    let path_overlay_tex: Handle<Image> = asset_server.load("Level/Ground/path_overlay.png");
    let path_overlay2_tex: Handle<Image> = asset_server.load("Level/Ground/path_overlay2.png");
    let bark_overlay_tex: Handle<Image> = asset_server.load("Level/Ground/bark_overlay.png");

    // Legacy ground kept for road system compatibility
    let ground_large_tex: Handle<Image> = asset_server.load("Level/Ground/ground_large.png");
    let grass_tile_tex: Handle<Image> = asset_server.load("Level/Ground/grass_new.png");
    let road_stone_tex: Handle<Image> = asset_server.load("Level/Ground/road_stone.png");
    let water_tile_tex: Handle<Image> = asset_server.load("Level/Ground/water_new.png");
    let rock_tile_tex: Handle<Image> = asset_server.load("Level/Ground/rock_new.png");
    let rock_wet_tex: Handle<Image> = asset_server.load("Level/Ground/rock_wet.png");
    let road_edge_tex: Handle<Image> = asset_server.load("Level/Ground/road_edge.png");

    // HD Decorations
    let deco = |name: &str| -> Handle<Image> { asset_server.load(&format!("Level/Decorations/{}", name)) };
    let deco_tree_big1 = deco("tree_big1_hd.png");
    let deco_tree_big2 = deco("tree_big2_hd.png");
    let deco_pine1 = deco("pine_hd1.png");
    let deco_pine2 = deco("pine_hd2.png");
    let deco_bush1 = deco("bush_hd1.png");
    let deco_bush2 = deco("bush_hd2.png");
    let deco_flowers1 = deco("flowers1_hd.png");
    let deco_flowers2 = deco("flowers2_hd.png");
    let deco_mushrooms = deco("mushrooms_hd.png");
    let deco_standing_stone1 = deco("standing_stone_hd.png");
    let deco_standing_stone2 = deco("standing_stone2_hd.png");
    let deco_stone_circle = deco("stone_circle_hd.png");

    // Medieval terrain overlay textures (will be used as semi-transparent overlays)
    // Already loaded above: water_overlay_tex, rock_overlay_tex, etc.

    // Additional decoration types (existing but previously unused from Decorations/)
    let deco_tree_oak1 = deco("tree_oak1.png");
    let deco_tree_oak2 = deco("tree_oak2.png");
    let deco_tree_dead1 = deco("tree_dead1.png");
    let deco_tree_dead2 = deco("tree_dead2.png");
    let deco_tree_bush1 = deco("tree_bush1.png");
    let deco_tree_bush2 = deco("tree_bush2.png");
    let deco_tree_bush3 = deco("tree_bush3.png");
    let deco_pine3 = deco("pine3.png");
    let deco_pine4 = deco("pine4.png");
    let deco_pine5 = deco("pine5.png");
    let deco_pine6 = deco("pine6.png");
    let deco_plant1 = deco("plant1.png");
    let deco_plant2 = deco("plant2.png");
    let deco_rock_small1 = deco("rock_small1.png");
    let deco_rock_small2 = deco("rock_small2.png");
    let deco_rock_small3 = deco("rock_small3.png");
    let deco_rock_flat = deco("rock_flat.png");
    let deco_ruin_arch1 = deco("ruin_arch1.png");
    let deco_ruin_arch2 = deco("ruin_arch2.png");
    let deco_ruin_pillar1 = deco("ruin_pillar1.png");
    let deco_ruin_pillar2 = deco("ruin_pillar2.png");
    let deco_ruin_wall1 = deco("ruin_wall1.png");
    let deco_ruin_wall2 = deco("ruin_wall2.png");
    let deco_temple_ruin = deco("temple_ruin.png");
    let deco_fence1 = deco("fence1.png");
    let deco_fence2 = deco("fence2.png");
    let deco_fence_post = deco("fence_post.png");
    let deco_stone_block1 = deco("stone_block1.png");
    let deco_stone_block2 = deco("stone_block2.png");
    let deco_stone_step = deco("stone_step.png");
    let deco_cave1 = deco("cave_entrance1.png");
    let deco_cave2 = deco("cave_entrance2.png");
    let deco_cave3 = deco("cave_entrance3.png");
    let deco_cabin_wall1 = deco("cabin_wall1.png");
    let deco_cabin_wall2 = deco("cabin_wall2.png");
    let deco_cabin_wall3 = deco("cabin_wall3.png");
    let deco_water_tile1 = deco("water_tile1.png");
    let deco_water_tile2 = deco("water_tile2.png");
    let deco_bridge1 = deco("bridge1.png");
    let deco_bush_1 = deco("bush1.png");
    let deco_bush_2 = deco("bush2.png");
    let deco_bush_3 = deco("bush3.png");
    let deco_bush_4 = deco("bush4.png");

    // Legacy grass atlas (kept for fallback)
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

    // Merchant / caravan
    let caravan_sprites = load_building_set(&asset_server, "caravan");

    // UI / effects
    let healthbar_tex: Handle<Image> = asset_server.load("HealthBar/HealthBar.png");
    let target_tex: Handle<Image> = asset_server.load("Effects/target.png");

    // RPG Icons
    let icon = |name: &str| -> Handle<Image> { asset_server.load(&format!("Icons/RPG/{}", name)) };
    let icon_gold_coin = icon("gold_coin.png");
    let icon_bounty_scroll = icon("bounty_scroll.png");
    let icon_bounty_monster = icon("bounty_monster.png");
    let icon_bounty_explore = icon("exploration_map.png");
    let icon_bounty_objective = icon("bounty_objective.png");
    let icon_bounty_resource = icon("bounty_resource.png");
    let icon_potion_health = icon("potion_health.png");
    let icon_potion_mana = icon("potion_mana.png");
    let icon_potion_speed = icon("potion_speed.png");
    let icon_weapon_sword = icon("weapon_sword_t1.png");
    let icon_weapon_bow = icon("weapon_bow_t1.png");
    let icon_weapon_staff = icon("weapon_staff_t1.png");
    let icon_weapon_dagger = icon("weapon_dagger_t1.png");
    let icon_armor_leather = icon("armor_leather.png");
    let icon_armor_chain = icon("armor_chain.png");
    let icon_armor_plate = icon("armor_plate.png");
    let icon_skill_heal = icon("skill_heal.png");
    let icon_skill_fortify = icon("skill_fortify.png");
    let icon_skill_stealth = icon("skill_stealth.png");
    let icon_skill_fireball = icon("skill_fireball.png");
    let icon_skill_volley = icon("skill_volley.png");
    let icon_food_bread = icon("food_bread.png");
    let icon_torch = icon("torch.png");
    let icon_chest = icon("chest.png");
    let icon_medal = icon("medal_hero.png");
    let icon_clock = icon("clock.png");

    commands.insert_resource(SpriteAssets {
        warrior_atlas,
        archer_atlas,
        mage_atlas,
        rogue_atlas,
        healer_atlas,
        civilian_atlas,
        warrior_attack_atlas,
        archer_attack_atlas,
        mage_attack_atlas,
        rogue_attack_atlas,
        healer_attack_atlas,
        warrior_hurt_atlas,
        archer_hurt_atlas,
        mage_hurt_atlas,
        rogue_hurt_atlas,
        healer_hurt_atlas,
        goblin_atlas,
        bandit_atlas,
        skeleton_atlas,
        skeleton_attack_atlas,
        skeleton_hurt_atlas,
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
        ground_large_tex,
        grass_tile_tex,
        road_stone_tex,
        water_tile_tex,
        rock_tile_tex,
        rock_wet_tex,
        road_edge_tex,
        deco_tree_big1,
        deco_tree_big2,
        deco_pine1,
        deco_pine2,
        deco_bush1,
        deco_bush2,
        deco_flowers1,
        deco_flowers2,
        deco_mushrooms,
        deco_standing_stone1,
        deco_standing_stone2,
        deco_stone_circle,
        grass_atlas,
        trees_atlas,
        shadow_tex,
        caravan_sprites,
        healthbar_tex,
        target_tex,
        icon_gold_coin,
        icon_bounty_scroll,
        icon_bounty_monster,
        icon_bounty_explore,
        icon_bounty_objective,
        icon_bounty_resource,
        icon_potion_health,
        icon_potion_mana,
        icon_potion_speed,
        icon_weapon_sword,
        icon_weapon_bow,
        icon_weapon_staff,
        icon_weapon_dagger,
        icon_armor_leather,
        icon_armor_chain,
        icon_armor_plate,
        icon_skill_heal,
        icon_skill_fortify,
        icon_skill_stealth,
        icon_skill_fireball,
        icon_skill_volley,
        icon_food_bread,
        icon_torch,
        icon_chest,
        icon_medal,
        icon_clock,

        // Medieval terrain base
        grassland_tex,
        grass_overcast_tex,
        // Terrain overlays
        water_overlay_tex,
        rock_overlay_tex,
        rock_wet_overlay_tex,
        path_overlay_tex,
        path_overlay2_tex,
        bark_overlay_tex,
        // Additional decorations
        deco_tree_oak1,
        deco_tree_oak2,
        deco_tree_dead1,
        deco_tree_dead2,
        deco_tree_bush1,
        deco_tree_bush2,
        deco_tree_bush3,
        deco_pine3,
        deco_pine4,
        deco_pine5,
        deco_pine6,
        deco_plant1,
        deco_plant2,
        deco_rock_small1,
        deco_rock_small2,
        deco_rock_small3,
        deco_rock_flat,
        deco_ruin_arch1,
        deco_ruin_arch2,
        deco_ruin_pillar1,
        deco_ruin_pillar2,
        deco_ruin_wall1,
        deco_ruin_wall2,
        deco_temple_ruin,
        deco_fence1,
        deco_fence2,
        deco_fence_post,
        deco_stone_block1,
        deco_stone_block2,
        deco_stone_step,
        deco_cave1,
        deco_cave2,
        deco_cave3,
        deco_cabin_wall1,
        deco_cabin_wall2,
        deco_cabin_wall3,
        deco_water_tile1,
        deco_water_tile2,
        deco_bridge1,
        deco_bush_1,
        deco_bush_2,
        deco_bush_3,
        deco_bush_4,
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

    let (walk_atlas, attack_atlas, hurt_atlas, attack_frames) = match class {
        HeroClass::Warrior => (
            sprites.warrior_atlas.clone(),
            sprites.warrior_attack_atlas.clone(),
            sprites.warrior_hurt_atlas.clone(),
            6,  // slash
        ),
        HeroClass::Archer => (
            sprites.archer_atlas.clone(),
            sprites.archer_attack_atlas.clone(),
            sprites.archer_hurt_atlas.clone(),
            13, // bow
        ),
        HeroClass::Mage => (
            sprites.mage_atlas.clone(),
            sprites.mage_attack_atlas.clone(),
            sprites.mage_hurt_atlas.clone(),
            7,  // spellcast
        ),
        HeroClass::Rogue => (
            sprites.rogue_atlas.clone(),
            sprites.rogue_attack_atlas.clone(),
            sprites.rogue_hurt_atlas.clone(),
            6,  // slash
        ),
        HeroClass::Healer => (
            sprites.healer_atlas.clone(),
            sprites.healer_attack_atlas.clone(),
            sprites.healer_hurt_atlas.clone(),
            7,  // spellcast
        ),
    };

    // Use walk atlas as idle atlas (slower frame rate will differentiate it in-game)
    let rest_atlas = hurt_atlas.clone();

    let anim_set = AnimationSet {
        walk_atlas: walk_atlas.clone(),
        walk_frames: 9,
        // Use walk atlas as idle atlas (slower frame rate differentiates it)
        idle_atlas: walk_atlas.clone(),
        idle_frames: 9,
        // Rest uses hurt sheet (static pose for sleeping)
        rest_atlas,
        rest_frames: 6,
        attack_atlas,
        attack_frames,
        hurt_atlas,
        hurt_frames: 6,
        hurt_rows: 1,
        current_mode: AnimMode::Walk,
    };

    // LPC walk animation: 9 frames per row, 4 directions, start facing down (row 2)
    let anim = SpriteAnimation::new_directional(9, 8.0);
    let start_index = anim.atlas_index();

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: walk_atlas,
            transform: Transform::from_translation(position).with_scale(Vec3::splat(0.7)),
            sprite: TextureAtlasSprite {
                index: start_index,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(hero)
        .insert(stats)
        .insert(HeroState::Idle)
        .insert(HeroDecisionTimer::default())
        .insert(AttackCooldown::default())
        .insert(HeroEquipment::default())
        .insert(anim)
        .insert(anim_set)
        .insert(ArcaneSurgeCooldown::default())
        .id()
}

pub fn spawn_enemy_with_sprite(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    enemy_type: EnemyType,
    position: Vec3,
) -> Entity {
    let stats = enemy_type.stats();

    // Skeleton-based enemies use LPC animated sheets
    let use_skeleton = matches!(
        enemy_type,
        EnemyType::Troll | EnemyType::GoblinElite | EnemyType::BossWarlord | EnemyType::Werewolf
    );

    if use_skeleton {
        let anim = SpriteAnimation::new_directional(9, 6.0);
        let start_index = anim.atlas_index();
        let (scale, tint) = match enemy_type {
            EnemyType::Troll => (1.0, Color::rgb(0.7, 1.0, 0.7)),
            EnemyType::GoblinElite => (0.8, Color::rgb(1.0, 0.8, 0.4)),
            EnemyType::BossWarlord => (1.3, Color::rgb(1.0, 0.3, 0.3)),
            EnemyType::Werewolf => (0.9, Color::rgb(0.6, 0.4, 0.8)),
            _ => (0.7, Color::WHITE),
        };

        let anim_set = AnimationSet {
            walk_atlas: sprites.skeleton_atlas.clone(),
            walk_frames: 9,
            idle_atlas: sprites.skeleton_atlas.clone(),
            idle_frames: 9,
            rest_atlas: sprites.skeleton_hurt_atlas.clone(),
            rest_frames: 6,
            attack_atlas: sprites.skeleton_attack_atlas.clone(),
            attack_frames: 6,
            hurt_atlas: sprites.skeleton_hurt_atlas.clone(),
            hurt_frames: 6,
            hurt_rows: 1,
            current_mode: AnimMode::Walk,
        };

        return commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.skeleton_atlas.clone(),
                transform: Transform::from_translation(position).with_scale(Vec3::splat(scale)),
                sprite: TextureAtlasSprite {
                    index: start_index,
                    color: tint,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy { enemy_type })
            .insert(stats)
            .insert(EnemyAi::default())
            .insert(AttackCooldown { timer: 0.0, interval: 1.5 })
            .insert(anim)
            .insert(anim_set)
            .id();
    }

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
        _ => unreachable!(),
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

/// World-based enemy spawn for the save/load system (no Commands needed).
pub fn spawn_enemy_with_sprite_world(
    world: &mut World,
    sprites: &SpriteAssets,
    enemy_type: EnemyType,
    stats: EnemyStats,
    position: Vec3,
) -> Entity {
    let use_skeleton = matches!(
        enemy_type,
        EnemyType::Troll | EnemyType::GoblinElite | EnemyType::BossWarlord | EnemyType::Werewolf
    );

    if use_skeleton {
        let anim = SpriteAnimation::new_directional(9, 6.0);
        let start_index = anim.atlas_index();
        let (scale, tint) = match enemy_type {
            EnemyType::Troll => (1.0, Color::rgb(0.7, 1.0, 0.7)),
            EnemyType::GoblinElite => (0.8, Color::rgb(1.0, 0.8, 0.4)),
            EnemyType::BossWarlord => (1.3, Color::rgb(1.0, 0.3, 0.3)),
            EnemyType::Werewolf => (0.9, Color::rgb(0.6, 0.4, 0.8)),
            _ => (0.7, Color::WHITE),
        };

        let anim_set = AnimationSet {
            walk_atlas: sprites.skeleton_atlas.clone(),
            walk_frames: 9,
            idle_atlas: sprites.skeleton_atlas.clone(),
            idle_frames: 9,
            rest_atlas: sprites.skeleton_hurt_atlas.clone(),
            rest_frames: 6,
            attack_atlas: sprites.skeleton_attack_atlas.clone(),
            attack_frames: 6,
            hurt_atlas: sprites.skeleton_hurt_atlas.clone(),
            hurt_frames: 6,
            hurt_rows: 1,
            current_mode: AnimMode::Walk,
        };

        return world.spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: sprites.skeleton_atlas.clone(),
                transform: Transform::from_translation(position).with_scale(Vec3::splat(scale)),
                sprite: TextureAtlasSprite {
                    index: start_index,
                    color: tint,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy { enemy_type })
            .insert(stats)
            .insert(EnemyAi::default())
            .insert(AttackCooldown { timer: 0.0, interval: 1.5 })
            .insert(anim)
            .insert(anim_set)
            .id();
    }

    let entity = match enemy_type {
        EnemyType::Goblin => world.spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: sprites.goblin_atlas.clone(),
                transform: Transform::from_translation(position).with_scale(Vec3::splat(1.5)),
                sprite: TextureAtlasSprite { index: 0, ..Default::default() },
                ..Default::default()
            })
            .id(),
        EnemyType::Bandit | EnemyType::ShadowBandit => {
            let tint = if enemy_type == EnemyType::ShadowBandit {
                Color::rgb(0.3, 0.2, 0.4)
            } else {
                Color::WHITE
            };
            world.spawn()
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: sprites.bandit_atlas.clone(),
                    transform: Transform::from_translation(position).with_scale(Vec3::splat(1.5)),
                    sprite: TextureAtlasSprite { index: 0, color: tint, ..Default::default() },
                    ..Default::default()
                })
                .id()
        }
        _ => world.spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: sprites.goblin_atlas.clone(),
                transform: Transform::from_translation(position).with_scale(Vec3::splat(1.5)),
                sprite: TextureAtlasSprite { index: 0, ..Default::default() },
                ..Default::default()
            })
            .id(),
    };

    world.entity_mut(entity)
        .insert(Enemy { enemy_type })
        .insert(stats)
        .insert(EnemyAi::default())
        .insert(AttackCooldown { timer: 0.0, interval: 1.5 });

    entity
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
    // Tile the medieval grassland texture in a 3×3 grid to cover the ±1200 area.
    // grassland.png is ~512×512, scale each tile to cover ~800 world units.
    let tile_world_size = 800.0;
    for gx in -1..=1 {
        for gy in -1..=1 {
            commands.spawn_bundle(SpriteBundle {
                texture: sprites.grassland_tex.clone(),
                transform: Transform::from_translation(Vec3::new(
                    gx as f32 * tile_world_size,
                    gy as f32 * tile_world_size,
                    0.0,
                )).with_scale(Vec3::splat(1.5)),
                ..Default::default()
            });
        }
    }
}

pub fn spawn_terrain_overlays(mut commands: Commands, sprites: Res<SpriteAssets>) {
    use std::f32::consts::TAU;

    // Spawn a semi-transparent terrain overlay sprite.
    let spawn_overlay = |commands: &mut Commands, tex: Handle<Image>, pos: Vec2,
                         z: f32, size: f32, color: Color, rotation: f32| {
        commands.spawn_bundle(SpriteBundle {
            texture: tex,
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::splat(size)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, z))
                .with_rotation(Quat::from_rotation_z(rotation)),
            ..Default::default()
        });
    };

    // === Water ponds (z=0.2) — 4 ponds scattered across the map ===
    let water_positions = [
        Vec2::new(500.0, 500.0),
        Vec2::new(-600.0, 400.0),
        Vec2::new(400.0, -600.0),
        Vec2::new(-700.0, -500.0),
    ];
    for pos in &water_positions {
        let size = 200.0 + rand::random::<f32>() * 100.0;
        spawn_overlay(
            &mut commands, sprites.water_overlay_tex.clone(),
            *pos, 0.2, size,
            Color::rgba(0.3, 0.45, 0.65, 0.6),
            rand::random::<f32>() * TAU,
        );
    }

    // === Rocky areas (z=0.15) — scattered outcrops ===
    for _ in 0..15 {
        let angle = rand::random::<f32>() * TAU;
        let radius = 200.0 + rand::random::<f32>() * 900.0;
        let pos = Vec2::new(angle.cos() * radius, angle.sin() * radius);
        let size = 60.0 + rand::random::<f32>() * 80.0;
        let rock_tex = if rand::random::<bool>() {
            sprites.rock_overlay_tex.clone()
        } else {
            sprites.rock_wet_overlay_tex.clone()
        };
        spawn_overlay(
            &mut commands, rock_tex, pos, 0.15, size,
            Color::rgba(0.7, 0.65, 0.6, 0.5),
            rand::random::<f32>() * TAU,
        );
    }

    // === Dirt/bark patches (z=0.1) — ground variation ===
    for _ in 0..25 {
        let angle = rand::random::<f32>() * TAU;
        let radius = 100.0 + rand::random::<f32>() * 900.0;
        let pos = Vec2::new(angle.cos() * radius, angle.sin() * radius);
        let size = 50.0 + rand::random::<f32>() * 70.0;
        spawn_overlay(
            &mut commands, sprites.bark_overlay_tex.clone(),
            pos, 0.1, size,
            Color::rgba(0.7, 0.6, 0.4, 0.35),
            rand::random::<f32>() * TAU,
        );
    }

    // === Overcast grass patches (z=0.05) — darker grass areas ===
    for _ in 0..20 {
        let angle = rand::random::<f32>() * TAU;
        let radius = 100.0 + rand::random::<f32>() * 1000.0;
        let pos = Vec2::new(angle.cos() * radius, angle.sin() * radius);
        let size = 100.0 + rand::random::<f32>() * 150.0;
        spawn_overlay(
            &mut commands, sprites.grass_overcast_tex.clone(),
            pos, 0.05, size,
            Color::rgba(0.85, 0.8, 0.7, 0.25),
            rand::random::<f32>() * TAU,
        );
    }

    // === Path-like dirt trails (z=0.3) — connecting key points ===
    let path_segments = [
        (Vec2::new(0.0, 0.0), Vec2::new(500.0, 500.0)),
        (Vec2::new(0.0, 0.0), Vec2::new(-300.0, 200.0)),
        (Vec2::new(0.0, 0.0), Vec2::new(250.0, 150.0)),
        (Vec2::new(0.0, 0.0), Vec2::new(-200.0, -250.0)),
        (Vec2::new(250.0, 150.0), Vec2::new(500.0, 500.0)),
        (Vec2::new(-300.0, 200.0), Vec2::new(-600.0, 400.0)),
        (Vec2::new(0.0, 0.0), Vec2::new(400.0, -600.0)),
        (Vec2::new(0.0, 0.0), Vec2::new(-700.0, -500.0)),
    ];
    for (from, to) in &path_segments {
        for t in &[0.25, 0.5, 0.75] {
            let pos = *from + (*to - *from) * *t;
            let jitter = Vec2::new(
                (rand::random::<f32>() - 0.5) * 40.0,
                (rand::random::<f32>() - 0.5) * 40.0,
            );
            let path_tex = if rand::random::<bool>() {
                sprites.path_overlay_tex.clone()
            } else {
                sprites.path_overlay2_tex.clone()
            };
            spawn_overlay(
                &mut commands, path_tex,
                pos + jitter, 0.3,
                60.0 + rand::random::<f32>() * 40.0,
                Color::rgba(0.5, 0.4, 0.3, 0.45),
                (*to - *from).angle_between(Vec2::X) + (rand::random::<f32>() - 0.5) * 0.3,
            );
        }
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

/// Spawn map decorations: trees, bushes, flowers, mushrooms, and stone monuments.
pub fn spawn_map_decorations(mut commands: Commands, sprites: Res<SpriteAssets>) {
    use std::f32::consts::TAU;

    let spawn_deco = |commands: &mut Commands, tex: Handle<Image>, pos: Vec2, z: f32, scale: f32| {
        commands
            .spawn_bundle(SpriteBundle {
                texture: tex,
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, z))
                    .with_scale(Vec3::splat(scale)),
                ..Default::default()
            })
            .insert(MapDecoration);
    };

    let rand_pos = |min_r: f32, max_r: f32| -> Vec2 {
        let angle = rand::random::<f32>() * TAU;
        let radius = min_r + rand::random::<f32>() * (max_r - min_r);
        Vec2::new(angle.cos() * radius, angle.sin() * radius)
    };

    // === Big leafy trees — increased density, mid/outer rings ===
    let big_tree_tex = [
        sprites.deco_tree_big1.clone(),
        sprites.deco_tree_big2.clone(),
        sprites.deco_tree_oak1.clone(),
        sprites.deco_tree_oak2.clone(),
    ];
    // Spawn as clusters for forest feel
    for _ in 0..12 {
        let center = rand_pos(200.0, 900.0);
        let count = 2 + rand::random::<usize>() % 3; // 2-4 trees per cluster
        for _ in 0..count {
            let offset = Vec2::new(
                (rand::random::<f32>() - 0.5) * 50.0,
                (rand::random::<f32>() - 0.5) * 50.0,
            );
            let pos = center + offset;
            let idx = rand::random::<usize>() % big_tree_tex.len();
            let scale = 1.0 + rand::random::<f32>() * 0.6;
            spawn_deco(&mut commands, big_tree_tex[idx].clone(), pos, 4.0, scale);
        }
    }

    // === Pine trees — outer ring with more variety (6 variants) ===
    let pine_tex = [
        sprites.deco_pine1.clone(), sprites.deco_pine2.clone(),
        sprites.deco_pine3.clone(), sprites.deco_pine4.clone(),
        sprites.deco_pine5.clone(), sprites.deco_pine6.clone(),
    ];
    for _ in 0..10 {
        let center = rand_pos(400.0, 1300.0);
        let count = 2 + rand::random::<usize>() % 4; // 2-5 per cluster
        for _ in 0..count {
            let offset = Vec2::new(
                (rand::random::<f32>() - 0.5) * 40.0,
                (rand::random::<f32>() - 0.5) * 40.0,
            );
            let pos = center + offset;
            let idx = rand::random::<usize>() % pine_tex.len();
            let scale = 0.8 + rand::random::<f32>() * 0.6;
            spawn_deco(&mut commands, pine_tex[idx].clone(), pos, 4.0, scale);
        }
    }

    // === Dead/bare trees — far outer ring for wild frontier feel ===
    let dead_tree_tex = [
        sprites.deco_tree_dead1.clone(),
        sprites.deco_tree_dead2.clone(),
    ];
    for _ in 0..15 {
        let pos = rand_pos(600.0, 1400.0);
        let idx = rand::random::<usize>() % dead_tree_tex.len();
        let scale = 0.9 + rand::random::<f32>() * 0.5;
        spawn_deco(&mut commands, dead_tree_tex[idx].clone(), pos, 3.0, scale);
    }

    // === Bushes — all rings with 6 variants (HD + plain) ===
    let bush_tex = [
        sprites.deco_bush1.clone(), sprites.deco_bush2.clone(),
        sprites.deco_bush_1.clone(), sprites.deco_bush_2.clone(),
        sprites.deco_bush_3.clone(), sprites.deco_bush_4.clone(),
    ];
    for _ in 0..30 {
        let pos = rand_pos(80.0, 1100.0);
        let idx = rand::random::<usize>() % bush_tex.len();
        let scale = 0.7 + rand::random::<f32>() * 0.5;
        spawn_deco(&mut commands, bush_tex[idx].clone(), pos, 2.0, scale);
    }

    // === Tree bushes (undergrowth) — inner rings ===
    let tree_bush_tex = [
        sprites.deco_tree_bush1.clone(),
        sprites.deco_tree_bush2.clone(),
        sprites.deco_tree_bush3.clone(),
    ];
    for _ in 0..25 {
        let pos = rand_pos(50.0, 600.0);
        let idx = rand::random::<usize>() % tree_bush_tex.len();
        let scale = 0.8 + rand::random::<f32>() * 0.4;
        spawn_deco(&mut commands, tree_bush_tex[idx].clone(), pos, 1.5, scale);
    }

    // === Flower patches — near town for color ===
    let flower_textures = [
        sprites.deco_flowers1.clone(),
        sprites.deco_flowers2.clone(),
    ];
    for _ in 0..25 {
        let pos = rand_pos(50.0, 500.0);
        let idx = rand::random::<usize>() % flower_textures.len();
        spawn_deco(&mut commands, flower_textures[idx].clone(), pos, 1.5, 0.9);
    }

    // === Mushroom clusters ===
    for _ in 0..12 {
        let pos = rand_pos(150.0, 800.0);
        spawn_deco(&mut commands, sprites.deco_mushrooms.clone(), pos, 1.5, 1.0);
    }

    // === Small rocks — scattered everywhere ===
    let small_rock_tex = [
        sprites.deco_rock_small1.clone(),
        sprites.deco_rock_small2.clone(),
        sprites.deco_rock_small3.clone(),
    ];
    for _ in 0..40 {
        let pos = rand_pos(100.0, 1400.0);
        let idx = rand::random::<usize>() % small_rock_tex.len();
        let scale = 0.6 + rand::random::<f32>() * 0.5;
        spawn_deco(&mut commands, small_rock_tex[idx].clone(), pos, 1.0, scale);
    }

    // === Flat rocks — mid/outer ===
    for _ in 0..12 {
        let pos = rand_pos(200.0, 1000.0);
        let scale = 0.7 + rand::random::<f32>() * 0.4;
        spawn_deco(&mut commands, sprites.deco_rock_flat.clone(), pos, 0.8, scale);
    }

    // === Standing stones — landmark positions ===
    spawn_deco(&mut commands, sprites.deco_standing_stone1.clone(), Vec2::new(320.0, 280.0), 3.0, 1.2);
    spawn_deco(&mut commands, sprites.deco_standing_stone2.clone(), Vec2::new(-350.0, 310.0), 3.0, 1.2);
    spawn_deco(&mut commands, sprites.deco_standing_stone1.clone(), Vec2::new(-280.0, -320.0), 3.0, 1.0);
    spawn_deco(&mut commands, sprites.deco_standing_stone2.clone(), Vec2::new(380.0, -260.0), 3.0, 1.0);
    spawn_deco(&mut commands, sprites.deco_standing_stone1.clone(), Vec2::new(600.0, 100.0), 3.0, 1.1);
    spawn_deco(&mut commands, sprites.deco_standing_stone2.clone(), Vec2::new(-500.0, -200.0), 3.0, 1.1);

    // === Stone circle monuments ===
    spawn_deco(&mut commands, sprites.deco_stone_circle.clone(), Vec2::new(-380.0, 340.0), 3.5, 1.3);
    spawn_deco(&mut commands, sprites.deco_stone_circle.clone(), Vec2::new(400.0, -380.0), 3.5, 1.3);
    spawn_deco(&mut commands, sprites.deco_stone_circle.clone(), Vec2::new(700.0, 500.0), 3.5, 1.2);
    spawn_deco(&mut commands, sprites.deco_stone_circle.clone(), Vec2::new(-600.0, -500.0), 3.5, 1.2);

    // === Ruin arches — far ring ===
    let ruin_arch_tex = [sprites.deco_ruin_arch1.clone(), sprites.deco_ruin_arch2.clone()];
    for _ in 0..4 {
        let pos = rand_pos(500.0, 1200.0);
        let idx = rand::random::<usize>() % ruin_arch_tex.len();
        spawn_deco(&mut commands, ruin_arch_tex[idx].clone(), pos, 3.0, 1.0 + rand::random::<f32>() * 0.3);
    }

    // === Ruin pillars — far ring ===
    let ruin_pillar_tex = [sprites.deco_ruin_pillar1.clone(), sprites.deco_ruin_pillar2.clone()];
    for _ in 0..5 {
        let pos = rand_pos(500.0, 1200.0);
        let idx = rand::random::<usize>() % ruin_pillar_tex.len();
        spawn_deco(&mut commands, ruin_pillar_tex[idx].clone(), pos, 3.0, 0.9 + rand::random::<f32>() * 0.4);
    }

    // === Ruin walls — far ring accents ===
    let ruin_wall_tex = [sprites.deco_ruin_wall1.clone(), sprites.deco_ruin_wall2.clone()];
    for _ in 0..4 {
        let pos = rand_pos(600.0, 1100.0);
        let idx = rand::random::<usize>() % ruin_wall_tex.len();
        spawn_deco(&mut commands, ruin_wall_tex[idx].clone(), pos, 2.5, 0.8 + rand::random::<f32>() * 0.3);
    }

    // === Temple ruin — far landmark ===
    spawn_deco(&mut commands, sprites.deco_temple_ruin.clone(), Vec2::new(800.0, -300.0), 3.0, 1.1);
    spawn_deco(&mut commands, sprites.deco_temple_ruin.clone(), Vec2::new(-750.0, 150.0), 3.0, 1.0);

    // === Fence segments — near town area ===
    let fence_tex = [
        sprites.deco_fence1.clone(),
        sprites.deco_fence2.clone(),
        sprites.deco_fence_post.clone(),
    ];
    for _ in 0..15 {
        let pos = rand_pos(60.0, 200.0);
        let idx = rand::random::<usize>() % fence_tex.len();
        spawn_deco(&mut commands, fence_tex[idx].clone(), pos, 1.0, 0.7 + rand::random::<f32>() * 0.3);
    }

    // === Stone blocks — mid ring ===
    let stone_block_tex = [
        sprites.deco_stone_block1.clone(),
        sprites.deco_stone_block2.clone(),
        sprites.deco_stone_step.clone(),
    ];
    for _ in 0..8 {
        let pos = rand_pos(200.0, 800.0);
        let idx = rand::random::<usize>() % stone_block_tex.len();
        spawn_deco(&mut commands, stone_block_tex[idx].clone(), pos, 1.0, 0.6 + rand::random::<f32>() * 0.3);
    }

    // === Cave entrances — far ring landmarks ===
    let cave_tex = [
        sprites.deco_cave1.clone(),
        sprites.deco_cave2.clone(),
        sprites.deco_cave3.clone(),
    ];
    for i in 0..4 {
        let angle = (i as f32 / 4.0) * TAU + rand::random::<f32>() * 0.5;
        let radius = 700.0 + rand::random::<f32>() * 400.0;
        let pos = Vec2::new(angle.cos() * radius, angle.sin() * radius);
        let idx = rand::random::<usize>() % cave_tex.len();
        spawn_deco(&mut commands, cave_tex[idx].clone(), pos, 4.0, 0.9 + rand::random::<f32>() * 0.3);
    }

    // === Cabin walls — near town, medieval settlement feel ===
    let cabin_wall_tex = [
        sprites.deco_cabin_wall1.clone(),
        sprites.deco_cabin_wall2.clone(),
        sprites.deco_cabin_wall3.clone(),
    ];
    for _ in 0..6 {
        let pos = rand_pos(80.0, 250.0);
        let idx = rand::random::<usize>() % cabin_wall_tex.len();
        spawn_deco(&mut commands, cabin_wall_tex[idx].clone(), pos, 1.5, 0.5 + rand::random::<f32>() * 0.3);
    }

    // === Water tile decorations — near ponds ===
    let water_tile_tex = [sprites.deco_water_tile1.clone(), sprites.deco_water_tile2.clone()];
    for _ in 0..6 {
        let center = [Vec2::new(500.0, 500.0), Vec2::new(-600.0, 400.0), Vec2::new(400.0, -600.0), Vec2::new(-700.0, -500.0)][rand::random::<usize>() % 4];
        let offset = Vec2::new((rand::random::<f32>() - 0.5) * 100.0, (rand::random::<f32>() - 0.5) * 100.0);
        spawn_deco(&mut commands, water_tile_tex[rand::random::<usize>() % 2].clone(), center + offset, 0.5, 0.6 + rand::random::<f32>() * 0.3);
    }

    // === Bridge decorations — placed near water ===
    spawn_deco(&mut commands, sprites.deco_bridge1.clone(), Vec2::new(500.0, 580.0), 1.0, 0.8);
    spawn_deco(&mut commands, sprites.deco_bridge1.clone(), Vec2::new(-600.0, 320.0), 1.0, 0.7);

    // === Small plants — near water and paths ===
    let plant_tex = [sprites.deco_plant1.clone(), sprites.deco_plant2.clone()];
    for _ in 0..15 {
        let pos = rand_pos(100.0, 900.0);
        let idx = rand::random::<usize>() % plant_tex.len();
        spawn_deco(&mut commands, plant_tex[idx].clone(), pos, 0.8, 0.5 + rand::random::<f32>() * 0.4);
    }
}
