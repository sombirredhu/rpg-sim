use bevy::prelude::*;
use bevy::sprite::{Rect, TextureAtlas};

#[derive(Clone)]
pub struct BuildingSpriteSpec {
    pub atlas: Handle<TextureAtlas>,
    pub index: usize,
    pub scale: f32,
}

#[derive(Clone)]
pub struct UnitSpriteSpec {
    pub atlas: Handle<TextureAtlas>,
    pub index: usize,
    pub scale: f32,
}

#[derive(Clone)]
pub struct ArtCatalog {
    pub building_townhall: BuildingSpriteSpec,
    pub building_inn: BuildingSpriteSpec,
    pub building_market: BuildingSpriteSpec,
    pub building_temple: BuildingSpriteSpec,
    pub building_tower: BuildingSpriteSpec,
    pub building_wizard: BuildingSpriteSpec,
    pub building_blacksmith: BuildingSpriteSpec,
    pub building_alchemist: BuildingSpriteSpec,
    pub building_barracks: BuildingSpriteSpec,
    pub building_house: BuildingSpriteSpec,
    pub building_farm: BuildingSpriteSpec,
    pub warrior: UnitSpriteSpec,
    pub archer: UnitSpriteSpec,
    pub mage: UnitSpriteSpec,
    pub rogue: UnitSpriteSpec,
    pub healer: UnitSpriteSpec,
    pub civilian_farmer: UnitSpriteSpec,
    pub civilian_trader: UnitSpriteSpec,
    pub civilian_smith: UnitSpriteSpec,
    pub civilian_acolyte: UnitSpriteSpec,
    pub civilian_laborer: UnitSpriteSpec,
    pub goblin: UnitSpriteSpec,
    pub bandit: UnitSpriteSpec,
    pub troll: UnitSpriteSpec,
    pub shadow: UnitSpriteSpec,
    pub dungeon_lord: UnitSpriteSpec,
    pub tree_atlas: Handle<TextureAtlas>,
    pub shadow_texture: Handle<Image>,
    pub smoke_texture: Handle<Image>,
    pub target_texture: Handle<Image>,
    pub health_bar_texture: Handle<Image>,
    pub beer_icon: Handle<Image>,
    pub axe_icon: Handle<Image>,
    pub helm_icon: Handle<Image>,
    pub potion_icon: Handle<Image>,
    pub firework_texture: Handle<Image>,
    pub grass_texture: Handle<Image>,
    pub brick_texture: Handle<Image>,
    pub water_texture: Handle<Image>,
    pub nav_tile_texture: Handle<Image>,
    pub rock_texture: Handle<Image>,
    pub map_overlay_texture: Handle<Image>,
}

impl ArtCatalog {
    pub fn build(asset_server: &Res<AssetServer>, atlases: &mut Assets<TextureAtlas>) -> Self {
        let blue_building_texture = asset_server.load("Level/Building/BlueBuilding.png");
        let red_building_texture = asset_server.load("Level/Building/RedBuilding.png");
        let mut blue_building_atlas = TextureAtlas::new_empty(blue_building_texture, Vec2::new(240.0, 128.0));
        blue_building_atlas.add_texture(Rect { min: Vec2::new(0.0, 2.0), max: Vec2::new(95.0, 116.0) });
        blue_building_atlas.add_texture(Rect { min: Vec2::new(96.0, 43.0), max: Vec2::new(127.0, 116.0) });
        blue_building_atlas.add_texture(Rect { min: Vec2::new(129.0, 64.0), max: Vec2::new(182.0, 116.0) });
        blue_building_atlas.add_texture(Rect { min: Vec2::new(184.0, 22.0), max: Vec2::new(237.0, 116.0) });
        let blue_building_atlas = atlases.add(blue_building_atlas);

        let mut red_building_atlas = TextureAtlas::new_empty(red_building_texture, Vec2::new(240.0, 128.0));
        red_building_atlas.add_texture(Rect { min: Vec2::new(0.0, 2.0), max: Vec2::new(95.0, 116.0) });
        red_building_atlas.add_texture(Rect { min: Vec2::new(96.0, 43.0), max: Vec2::new(127.0, 116.0) });
        red_building_atlas.add_texture(Rect { min: Vec2::new(129.0, 64.0), max: Vec2::new(182.0, 116.0) });
        red_building_atlas.add_texture(Rect { min: Vec2::new(184.0, 22.0), max: Vec2::new(237.0, 116.0) });
        let red_building_atlas = atlases.add(red_building_atlas);

        let warrior_atlas = atlases.add(single_sprite_atlas(
            asset_server.load("Character/Leader/Daniel/Daniel_Blue.png"),
            Vec2::new(40.0, 32.0),
            Rect { min: Vec2::new(11.0, 4.0), max: Vec2::new(31.0, 28.0) },
        ));
        let archer_atlas = atlases.add(single_sprite_atlas(
            asset_server.load("Character/Leader/Robin/Robin_Blue.png"),
            Vec2::new(32.0, 32.0),
            Rect { min: Vec2::new(1.0, 7.0), max: Vec2::new(28.0, 29.0) },
        ));
        let mage_atlas = atlases.add(single_sprite_atlas(
            asset_server.load("Character/Leader/Raja/Raja_Blue.png"),
            Vec2::new(32.0, 32.0),
            Rect { min: Vec2::new(2.0, 4.0), max: Vec2::new(28.0, 29.0) },
        ));
        let rogue_atlas = atlases.add(single_sprite_atlas(
            asset_server.load("Character/Leader/Rollo/Rollo_Blue.png"),
            Vec2::new(40.0, 32.0),
            Rect { min: Vec2::new(4.0, 7.0), max: Vec2::new(36.0, 29.0) },
        ));
        let healer_atlas = atlases.add(single_sprite_atlas(
            asset_server.load("Character/Leader/Sami/Sami_Blue.png"),
            Vec2::new(32.0, 32.0),
            Rect { min: Vec2::new(2.0, 2.0), max: Vec2::new(27.0, 29.0) },
        ));
        let farmer_atlas = atlases.add(single_sprite_atlas(
            asset_server.load("Character/Leader/Maori/Maori_Blue.png"),
            Vec2::new(40.0, 32.0),
            Rect { min: Vec2::new(1.0, 4.0), max: Vec2::new(37.0, 29.0) },
        ));

        let mut melee_red_atlas = TextureAtlas::new_empty(
            asset_server.load("Character/Creep/MeleeCreep/MeleeCreep_Red.png"),
            Vec2::new(96.0, 24.0),
        );
        melee_red_atlas.add_texture(Rect { min: Vec2::new(3.0, 2.0), max: Vec2::new(19.0, 22.0) });
        melee_red_atlas.add_texture(Rect { min: Vec2::new(50.0, 2.0), max: Vec2::new(69.0, 22.0) });
        melee_red_atlas.add_texture(Rect { min: Vec2::new(72.0, 2.0), max: Vec2::new(95.0, 21.0) });
        let melee_red_atlas = atlases.add(melee_red_atlas);

        let mut melee_blue_atlas = TextureAtlas::new_empty(
            asset_server.load("Character/Creep/MeleeCreep/MeleeCreep_Blue.png"),
            Vec2::new(96.0, 24.0),
        );
        melee_blue_atlas.add_texture(Rect { min: Vec2::new(3.0, 2.0), max: Vec2::new(19.0, 22.0) });
        melee_blue_atlas.add_texture(Rect { min: Vec2::new(50.0, 2.0), max: Vec2::new(69.0, 22.0) });
        melee_blue_atlas.add_texture(Rect { min: Vec2::new(72.0, 2.0), max: Vec2::new(95.0, 21.0) });
        let melee_blue_atlas = atlases.add(melee_blue_atlas);

        let mut range_red_atlas = TextureAtlas::new_empty(
            asset_server.load("Character/Creep/RangeCreep/RangeCreep_Red.png"),
            Vec2::new(128.0, 32.0),
        );
        range_red_atlas.add_texture(Rect { min: Vec2::new(1.0, 7.0), max: Vec2::new(28.0, 29.0) });
        range_red_atlas.add_texture(Rect { min: Vec2::new(33.0, 7.0), max: Vec2::new(55.0, 29.0) });
        let range_red_atlas = atlases.add(range_red_atlas);

        let mut range_blue_atlas = TextureAtlas::new_empty(
            asset_server.load("Character/Creep/RangeCreep/RangeCreep_Blue.png"),
            Vec2::new(128.0, 32.0),
        );
        range_blue_atlas.add_texture(Rect { min: Vec2::new(1.0, 7.0), max: Vec2::new(28.0, 29.0) });
        range_blue_atlas.add_texture(Rect { min: Vec2::new(33.0, 7.0), max: Vec2::new(55.0, 29.0) });
        let range_blue_atlas = atlases.add(range_blue_atlas);

        let dungeon_lord_atlas = atlases.add(single_sprite_atlas(
            asset_server.load("Character/Leader/Raja/Raja_Red.png"),
            Vec2::new(32.0, 32.0),
            Rect { min: Vec2::new(2.0, 4.0), max: Vec2::new(28.0, 29.0) },
        ));

        let tree_texture = asset_server.load("Level/Tress/Trees.png");
        let mut tree_atlas = TextureAtlas::new_empty(tree_texture, Vec2::new(80.0, 48.0));
        tree_atlas.add_texture(Rect {
            min: Vec2::new(9.0, 2.0),
            max: Vec2::new(38.0, 41.0),
        });
        tree_atlas.add_texture(Rect {
            min: Vec2::new(41.0, 2.0),
            max: Vec2::new(70.0, 41.0),
        });
        let tree_atlas = atlases.add(tree_atlas);

        Self {
            building_townhall: BuildingSpriteSpec {
                atlas: blue_building_atlas.clone(),
                index: 0,
                scale: 0.38,
            },
            building_market: BuildingSpriteSpec {
                atlas: blue_building_atlas.clone(),
                index: 1,
                scale: 0.34,
            },
            building_temple: BuildingSpriteSpec {
                atlas: blue_building_atlas.clone(),
                index: 2,
                scale: 0.34,
            },
            building_wizard: BuildingSpriteSpec {
                atlas: blue_building_atlas.clone(),
                index: 2,
                scale: 0.30,
            },
            building_alchemist: BuildingSpriteSpec {
                atlas: blue_building_atlas.clone(),
                index: 1,
                scale: 0.28,
            },
            building_inn: BuildingSpriteSpec {
                atlas: red_building_atlas.clone(),
                index: 1,
                scale: 0.34,
            },
            building_tower: BuildingSpriteSpec {
                atlas: red_building_atlas.clone(),
                index: 2,
                scale: 0.30,
            },
            building_blacksmith: BuildingSpriteSpec {
                atlas: red_building_atlas.clone(),
                index: 2,
                scale: 0.28,
            },
            building_barracks: BuildingSpriteSpec {
                atlas: red_building_atlas.clone(),
                index: 0,
                scale: 0.32,
            },
            building_house: BuildingSpriteSpec {
                atlas: red_building_atlas.clone(),
                index: 1,
                scale: 0.20,
            },
            building_farm: BuildingSpriteSpec {
                atlas: blue_building_atlas,
                index: 1,
                scale: 0.18,
            },
            warrior: UnitSpriteSpec {
                atlas: warrior_atlas.clone(),
                index: 0,
                scale: 0.9,
            },
            archer: UnitSpriteSpec {
                atlas: archer_atlas.clone(),
                index: 0,
                scale: 0.9,
            },
            mage: UnitSpriteSpec {
                atlas: mage_atlas.clone(),
                index: 0,
                scale: 0.9,
            },
            rogue: UnitSpriteSpec {
                atlas: rogue_atlas.clone(),
                index: 0,
                scale: 0.9,
            },
            healer: UnitSpriteSpec {
                atlas: healer_atlas.clone(),
                index: 0,
                scale: 0.9,
            },
            civilian_farmer: UnitSpriteSpec {
                atlas: farmer_atlas.clone(),
                index: 0,
                scale: 0.8,
            },
            civilian_trader: UnitSpriteSpec {
                atlas: mage_atlas.clone(),
                index: 0,
                scale: 0.8,
            },
            civilian_smith: UnitSpriteSpec {
                atlas: warrior_atlas.clone(),
                index: 0,
                scale: 0.8,
            },
            civilian_acolyte: UnitSpriteSpec {
                atlas: healer_atlas.clone(),
                index: 0,
                scale: 0.8,
            },
            civilian_laborer: UnitSpriteSpec {
                atlas: rogue_atlas.clone(),
                index: 0,
                scale: 0.8,
            },
            goblin: UnitSpriteSpec {
                atlas: melee_red_atlas,
                index: 0,
                scale: 1.0,
            },
            bandit: UnitSpriteSpec {
                atlas: range_red_atlas,
                index: 0,
                scale: 1.0,
            },
            troll: UnitSpriteSpec {
                atlas: melee_blue_atlas,
                index: 2,
                scale: 1.25,
            },
            shadow: UnitSpriteSpec {
                atlas: range_blue_atlas,
                index: 1,
                scale: 1.0,
            },
            dungeon_lord: UnitSpriteSpec {
                atlas: dungeon_lord_atlas,
                index: 0,
                scale: 1.4,
            },
            tree_atlas,
            shadow_texture: asset_server.load("Character/shadow.png"),
            smoke_texture: asset_server.load("Character/smoke.png"),
            target_texture: asset_server.load("Effects/target.png"),
            health_bar_texture: asset_server.load("HealthBar/HealthBar.png"),
            beer_icon: asset_server.load("Icons/Beer.png"),
            axe_icon: asset_server.load("Items/Axe.png"),
            helm_icon: asset_server.load("Items/Helm.png"),
            potion_icon: asset_server.load("Items/Red Potion 3.png"),
            firework_texture: asset_server.load("Effects/firework.png"),
            grass_texture: asset_server.load("Level/Ground/grass.png"),
            brick_texture: asset_server.load("Level/Ground/brick.png"),
            water_texture: asset_server.load("Level/Ground/water.png"),
            nav_tile_texture: asset_server.load("Level/NavTile/NavTile.png"),
            rock_texture: asset_server.load("Skills/Rock/rock0008.png"),
            map_overlay_texture: asset_server.load("Level/MapSketch.png"),
        }
    }
}

fn single_sprite_atlas(texture: Handle<Image>, size: Vec2, rect: Rect) -> TextureAtlas {
    let mut atlas = TextureAtlas::new_empty(texture, size);
    atlas.add_texture(rect);
    atlas
}
