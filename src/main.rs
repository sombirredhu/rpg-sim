mod art_catalog;

use bevy::prelude::*;
use bevy::sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite};
use art_catalog::{ArtCatalog, BuildingSpriteSpec, UnitSpriteSpec};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const DAY_LENGTH_SECONDS: f32 = 120.0;
const AI_INTERVAL_SECONDS: f32 = 2.0;
const SERVICE_INTERVAL_SECONDS: f32 = 7.0;
const CIVILIAN_INTERVAL_SECONDS: f32 = 5.0;
const MAP_WIDTH: usize = 20;
const MAP_HEIGHT: usize = 15;
const TILE_SIZE: f32 = 40.0;
const MAP_ORIGIN_X: f32 = -380.0;
const MAP_ORIGIN_Y: f32 = 260.0;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum TileKind {
    Grass,
    Road,
    Forest,
    Mountain,
    Water,
    Ruins,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct TileState {
    kind: TileKind,
    building: Option<BuildingKind>,
    bridge: bool,
}

#[derive(Default)]
struct PlacementState {
    building: Option<BuildingKind>,
    road_mode: bool,
    bridge_mode: bool,
}

struct MapState {
    tiles: Vec<TileState>,
}

#[derive(Component)]
struct MapTileVisual {
    x: usize,
    y: usize,
}

fn tile_to_world(x: usize, y: usize, z: f32) -> Vec3 {
    Vec3::new(
        MAP_ORIGIN_X + x as f32 * TILE_SIZE + TILE_SIZE * 0.5,
        MAP_ORIGIN_Y - y as f32 * TILE_SIZE - TILE_SIZE * 0.5,
        z,
    )
}

fn world_to_tile(world: Vec2) -> Option<(usize, usize)> {
    let x = ((world.x - MAP_ORIGIN_X) / TILE_SIZE).floor() as i32;
    let y = ((MAP_ORIGIN_Y - world.y) / TILE_SIZE).floor() as i32;
    if x >= 0 && y >= 0 && x < MAP_WIDTH as i32 && y < MAP_HEIGHT as i32 {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

fn tile_index(x: usize, y: usize) -> usize {
    y * MAP_WIDTH + x
}

fn map_sector_requirement(x: usize, y: usize) -> u32 {
    if x >= 15 || y <= 2 {
        4
    } else if x >= 13 || y >= 12 {
        3
    } else if x >= 11 || y <= 4 {
        2
    } else {
        1
    }
}

fn in_town_zone(x: usize, y: usize) -> bool {
    (4..=8).contains(&x) && (5..=9).contains(&y)
}

fn building_cost(kind: BuildingKind) -> i32 {
    match kind {
        BuildingKind::TownHall => 0,
        BuildingKind::Inn => 150,
        BuildingKind::Market => 200,
        BuildingKind::Temple => 250,
        BuildingKind::GuardTower => 300,
        BuildingKind::WizardTower => 400,
        BuildingKind::Blacksmith => 350,
        BuildingKind::Alchemist => 300,
        BuildingKind::Barracks => 450,
        BuildingKind::House => 120,
        BuildingKind::Farm => 90,
    }
}

fn create_map_state() -> MapState {
    let mut tiles = vec![
        TileState {
            kind: TileKind::Grass,
            building: None,
            bridge: false,
        };
        MAP_WIDTH * MAP_HEIGHT
    ];

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let idx = tile_index(x, y);
            let mut kind = TileKind::Grass;
            if x <= 1 {
                kind = TileKind::Water;
            } else if x >= 17 || y <= 1 || y >= 13 {
                kind = TileKind::Forest;
            } else if x >= 15 && y >= 10 {
                kind = TileKind::Mountain;
            }
            tiles[idx].kind = kind;
        }
    }

    for x in 4..=10 {
        tiles[tile_index(x, 7)].kind = TileKind::Road;
    }
    for y in 5..=9 {
        tiles[tile_index(6, y)].kind = TileKind::Road;
    }
    tiles[tile_index(14, 3)].kind = TileKind::Ruins;

    tiles[tile_index(6, 7)].building = Some(BuildingKind::TownHall);
    tiles[tile_index(4, 6)].building = Some(BuildingKind::House);
    tiles[tile_index(4, 10)].building = Some(BuildingKind::Farm);

    MapState { tiles }
}

fn blend_color(base: Color, highlight: Color, amount: f32) -> Color {
    Color::rgba(
        base.r() + (highlight.r() - base.r()) * amount,
        base.g() + (highlight.g() - base.g()) * amount,
        base.b() + (highlight.b() - base.b()) * amount,
        base.a() + (highlight.a() - base.a()) * amount,
    )
}

fn begin_build_placement(
    state: &mut ResMut<GameState>,
    placement: &mut ResMut<PlacementState>,
    kind: BuildingKind,
) {
    placement.building = Some(kind);
    placement.road_mode = false;
    placement.bridge_mode = false;
    push_event(
        state,
        format!("Placement mode: tap a valid tile to place {}.", kind.label()).as_str(),
    );
}

fn apply_building_state(state: &mut ResMut<GameState>, kind: BuildingKind) {
    state.gold -= building_cost(kind);
    match kind {
        BuildingKind::TownHall => {}
        BuildingKind::Inn => {
            state.has_inn = true;
            state.inn_tier = 1;
        }
        BuildingKind::Market => {
            state.has_market = true;
            state.market_tier = 1;
        }
        BuildingKind::Temple => {
            state.has_temple = true;
            state.temple_tier = 1;
        }
        BuildingKind::GuardTower => {
            state.has_guard_tower = true;
            state.tower_tier = 1;
        }
        BuildingKind::WizardTower => state.has_wizard_tower = true,
        BuildingKind::Blacksmith => state.has_blacksmith = true,
        BuildingKind::Alchemist => state.has_alchemist = true,
        BuildingKind::Barracks => {
            state.has_barracks = true;
            state.hero_capacity += 3;
        }
        BuildingKind::House => {
            state.houses += 1;
            state.hero_capacity += 2;
        }
        BuildingKind::Farm => state.farms += 1,
    }
    recalculate_income(state);
}

fn can_place_building(
    map_state: &MapState,
    state: &GameState,
    x: usize,
    y: usize,
    kind: BuildingKind,
) -> bool {
    if map_sector_requirement(x, y) > state.revealed_sectors {
        return false;
    }
    let tile = map_state.tiles[tile_index(x, y)];
    if tile.building.is_some() || tile.bridge {
        return false;
    }
    if !matches!(tile.kind, TileKind::Grass | TileKind::Road) {
        return false;
    }
    if matches!(kind, BuildingKind::House | BuildingKind::Farm) {
        true
    } else {
        in_town_zone(x, y)
    }
}

fn can_place_road(map_state: &MapState, x: usize, y: usize) -> bool {
    let tile = map_state.tiles[tile_index(x, y)];
    tile.building.is_none() && matches!(tile.kind, TileKind::Grass | TileKind::Forest)
}

fn can_place_bridge(map_state: &MapState, x: usize, y: usize) -> bool {
    let tile = map_state.tiles[tile_index(x, y)];
    tile.kind == TileKind::Water && !tile.bridge
}

fn movement_speed_bonus(map_state: &MapState, world: Vec3) -> f32 {
    if let Some((x, y)) = world_to_tile(world.truncate()) {
        let tile = map_state.tiles[tile_index(x, y)];
        if tile.bridge || tile.kind == TileKind::Road {
            1.3
        } else {
            1.0
        }
    } else {
        1.0
    }
}

fn update_map_visuals_system(
    state: Res<GameState>,
    map_state: Res<MapState>,
    placement: Res<PlacementState>,
    mut tile_query: Query<(&MapTileVisual, &mut Sprite)>,
) {
    for (tile_visual, mut sprite) in tile_query.iter_mut() {
        let idx = tile_index(tile_visual.x, tile_visual.y);
        let tile = map_state.tiles[idx];
        let mut color = match tile.kind {
            TileKind::Grass => Color::rgba(0.28, 0.43, 0.24, 0.55),
            TileKind::Road => Color::rgba(0.75, 0.64, 0.42, 0.82),
            TileKind::Forest => Color::rgba(0.12, 0.28, 0.14, 0.82),
            TileKind::Mountain => Color::rgba(0.36, 0.34, 0.38, 0.82),
            TileKind::Water => Color::rgba(0.22, 0.44, 0.72, 0.78),
            TileKind::Ruins => Color::rgba(0.56, 0.52, 0.66, 0.85),
        };
        if tile.bridge {
            color = Color::rgba(0.76, 0.63, 0.41, 0.90);
        }
        if tile.building.is_some() {
            color = blend_color(color, Color::rgba(0.89, 0.82, 0.65, 0.95), 0.22);
        }
        if map_sector_requirement(tile_visual.x, tile_visual.y) > state.revealed_sectors {
            color.set_a(0.18);
        }
        if let Some(kind) = placement.building {
            if can_place_building(&map_state, &state, tile_visual.x, tile_visual.y, kind) {
                color = blend_color(color, Color::rgba(0.93, 0.84, 0.42, color.a()), 0.35);
            }
        } else if placement.road_mode && can_place_road(&map_state, tile_visual.x, tile_visual.y) {
            color = blend_color(color, Color::rgba(0.92, 0.78, 0.50, color.a()), 0.25);
        } else if placement.bridge_mode && can_place_bridge(&map_state, tile_visual.x, tile_visual.y) {
            color = blend_color(color, Color::rgba(0.72, 0.88, 0.98, color.a()), 0.25);
        }
        sprite.color = color;
    }
}

fn map_input_system(
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    catalog: Res<ArtCatalog>,
    mut map_state: ResMut<MapState>,
    mut placement: ResMut<PlacementState>,
    mut state: ResMut<GameState>,
    camera_query: Query<(&OrthographicProjection, &Transform)>,
) {
    if !buttons.just_pressed(MouseButton::Left) || state.paused || state.kingdom_fallen {
        return;
    }

    let window = if let Some(window) = windows.get_primary() {
        window
    } else {
        return;
    };
    let cursor = if let Some(cursor) = window.cursor_position() {
        cursor
    } else {
        return;
    };
    let (projection, camera_transform) = if let Ok(value) = camera_query.get_single() {
        value
    } else {
        return;
    };

    let window_size = Vec2::new(window.width(), window.height());
    let screen_pos = cursor - window_size / 2.0;
    let world_pos = Vec2::new(
        camera_transform.translation.x + screen_pos.x * projection.scale,
        camera_transform.translation.y - screen_pos.y * projection.scale,
    );
    let (x, y) = if let Some(tile) = world_to_tile(world_pos) {
        tile
    } else {
        return;
    };
    if let Some(kind) = placement.building {
        if !can_place_building(&map_state, &state, x, y, kind) {
            push_event(&mut state, "That tile cannot hold this building.");
            placement.building = None;
            return;
        }
        if state.gold < building_cost(kind) {
            push_event(&mut state, "Not enough gold to complete that building.");
            placement.building = None;
            return;
        }

        let idx = tile_index(x, y);
        map_state.tiles[idx].building = Some(kind);
        apply_building_state(&mut state, kind);
        spawn_building(
            &mut commands,
            &asset_server,
            &catalog,
            kind,
            tile_to_world(x, y, 5.0),
            kind.label(),
        );
        placement.building = None;
        push_event(&mut state, format!("{} placed on the map.", kind.label()).as_str());
        return;
    }

    if placement.road_mode {
        if state.gold >= 8 && can_place_road(&map_state, x, y) {
            let idx = tile_index(x, y);
            map_state.tiles[idx].kind = TileKind::Road;
            state.gold -= 8;
            push_event(&mut state, "A new road section is laid.");
        } else {
            push_event(&mut state, "That tile cannot become a road.");
        }
        placement.road_mode = false;
        return;
    }

    if placement.bridge_mode {
        if state.gold >= 45 && can_place_bridge(&map_state, x, y) {
            let idx = tile_index(x, y);
            map_state.tiles[idx].bridge = true;
            map_state.tiles[idx].kind = TileKind::Road;
            state.gold -= 45;
            push_event(&mut state, "A bridge crosses the river and opens the route.");
        } else {
            push_event(&mut state, "Bridges must be placed on water.");
        }
        placement.bridge_mode = false;
    }
}

fn town_hall_pos() -> Vec3 {
    tile_to_world(6, 7, 5.0)
}

fn inn_pos() -> Vec3 {
    tile_to_world(8, 6, 5.0)
}

fn goblin_camp_pos() -> Vec3 {
    tile_to_world(17, 3, 5.0)
}

fn bandit_den_pos() -> Vec3 {
    tile_to_world(17, 7, 5.0)
}

fn troll_lair_pos() -> Vec3 {
    tile_to_world(15, 12, 5.0)
}

fn shadow_rift_pos() -> Vec3 {
    tile_to_world(13, 1, 5.0)
}

fn dungeon_lord_pos() -> Vec3 {
    tile_to_world(18, 6, 5.0)
}

fn patrol_pos() -> Vec3 {
    Vec3::new(15.0, -30.0, 5.0)
}

fn outer_patrol_pos() -> Vec3 {
    Vec3::new(125.0, 0.0, 5.0)
}

fn house_pos(index: u32) -> Vec3 {
    let row = index / 3;
    let col = index % 3;
    tile_to_world((3 + col) as usize, (5 + row) as usize, 5.0)
}

fn farm_pos(index: u32) -> Vec3 {
    let col = index % 4;
    tile_to_world((3 + col) as usize, 10, 5.0)
}

fn ruins_pos() -> Vec3 {
    tile_to_world(14, 3, 5.0)
}

fn resource_node_pos() -> Vec3 {
    tile_to_world(12, 12, 5.0)
}

fn defend_pos() -> Vec3 {
    tile_to_world(10, 7, 5.0)
}

fn merchant_entry_pos() -> Vec3 {
    tile_to_world(18, 8, 5.0)
}

fn market_pos() -> Vec3 {
    tile_to_world(5, 4, 5.0)
}

fn temple_pos() -> Vec3 {
    tile_to_world(7, 4, 5.0)
}

fn tower_pos() -> Vec3 {
    tile_to_world(8, 5, 5.0)
}

fn wizard_tower_pos() -> Vec3 {
    tile_to_world(9, 7, 5.0)
}

fn blacksmith_pos() -> Vec3 {
    tile_to_world(5, 10, 5.0)
}

fn alchemist_pos() -> Vec3 {
    tile_to_world(7, 10, 5.0)
}

fn barracks_pos() -> Vec3 {
    tile_to_world(9, 9, 5.0)
}

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
        .insert_resource(GameState::default())
        .insert_resource(create_map_state())
        .insert_resource(PlacementState::default())
        .insert_resource(AutoSaveState::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(button_interactions)
        .add_system(map_input_system)
        .add_system(update_map_visuals_system)
        .add_system(update_clock)
        .add_system(scheduled_threats_system)
        .add_system(kingdom_growth_system)
        .add_system(apply_recovery_payments_system)
        .add_system(merchant_event_system)
        .add_system(merchant_movement_system)
        .add_system(daily_realm_system)
        .add_system(zone_pressure_system)
        .add_system(tower_attack_system)
        .add_system(hero_ai_system)
        .add_system(hero_movement_system)
        .add_system(hero_bounty_resolution_system)
        .add_system(hero_service_economy_system)
        .add_system(civilian_ai_system)
        .add_system(civilian_movement_system)
        .add_system(update_fog_of_war_system)
        .add_system(update_day_night_overlay_system)
        .add_system(autosave_system)
        .add_system(new_era_system)
        .add_system(update_hero_labels_system)
        .add_system(update_zone_labels_system)
        .add_system(update_zone_markers_system)
        .add_system(update_ui_system)
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum HeroClass {
    Warrior,
    Archer,
    Mage,
    Rogue,
    Healer,
}

impl HeroClass {
    fn name(self) -> &'static str {
        match self {
            HeroClass::Warrior => "Warrior",
            HeroClass::Archer => "Archer",
            HeroClass::Mage => "Mage",
            HeroClass::Rogue => "Rogue",
            HeroClass::Healer => "Healer",
        }
    }

    fn max_hp(self) -> f32 {
        match self {
            HeroClass::Warrior => 120.0,
            HeroClass::Archer => 70.0,
            HeroClass::Mage => 60.0,
            HeroClass::Rogue => 80.0,
            HeroClass::Healer => 90.0,
        }
    }

    fn attack(self) -> f32 {
        match self {
            HeroClass::Warrior => 15.0,
            HeroClass::Archer => 20.0,
            HeroClass::Mage => 35.0,
            HeroClass::Rogue => 25.0,
            HeroClass::Healer => 6.0,
        }
    }

    fn speed(self) -> f32 {
        match self {
            HeroClass::Warrior => 42.0,
            HeroClass::Archer => 48.0,
            HeroClass::Mage => 36.0,
            HeroClass::Rogue => 58.0,
            HeroClass::Healer => 40.0,
        }
    }

    fn bravery(self) -> f32 {
        match self {
            HeroClass::Warrior => 1.0,
            HeroClass::Archer => 0.8,
            HeroClass::Mage => 0.75,
            HeroClass::Rogue => 0.65,
            HeroClass::Healer => 0.7,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum HeroPersonality {
    Brave,
    Cautious,
    Greedy,
    Dutiful,
    Wanderer,
}

impl HeroPersonality {
    fn name(self) -> &'static str {
        match self {
            HeroPersonality::Brave => "Brave",
            HeroPersonality::Cautious => "Cautious",
            HeroPersonality::Greedy => "Greedy",
            HeroPersonality::Dutiful => "Dutiful",
            HeroPersonality::Wanderer => "Wanderer",
        }
    }

    fn tolerance_bonus(self) -> f32 {
        match self {
            HeroPersonality::Brave => 1.0,
            HeroPersonality::Cautious => -0.6,
            HeroPersonality::Greedy => 0.2,
            HeroPersonality::Dutiful => 0.4,
            HeroPersonality::Wanderer => -0.1,
        }
    }

    fn bounty_multiplier(self) -> f32 {
        match self {
            HeroPersonality::Greedy => 1.25,
            HeroPersonality::Brave => 1.1,
            HeroPersonality::Dutiful => 1.0,
            HeroPersonality::Wanderer => 0.95,
            HeroPersonality::Cautious => 0.85,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum HeroGuild {
    FreeRoaming,
    Fighters,
    Rangers,
    Arcane,
    Faith,
}

impl HeroGuild {
    fn name(self) -> &'static str {
        match self {
            HeroGuild::FreeRoaming => "Free",
            HeroGuild::Fighters => "Fighters",
            HeroGuild::Rangers => "Rangers",
            HeroGuild::Arcane => "Arcane",
            HeroGuild::Faith => "Faith",
        }
    }
}

fn hero_task_name(task: HeroTask) -> &'static str {
    match task {
        HeroTask::Patrol => "Patrol",
        HeroTask::RecoverAtInn => "Recover",
        HeroTask::PrayAtTemple => "Pray",
        HeroTask::ClaimBounty => "Hunt",
        HeroTask::ExploreRuins => "Explore",
        HeroTask::GatherResource => "Gather",
        HeroTask::DefendTown => "Defend",
        HeroTask::EscortMerchant => "Escort",
        HeroTask::LayLow => "Lay Low",
        HeroTask::Incapacitated => "Recovering",
    }
}

fn hero_guild_for_class(class: HeroClass) -> HeroGuild {
    match class {
        HeroClass::Warrior => HeroGuild::Fighters,
        HeroClass::Archer => HeroGuild::Rangers,
        HeroClass::Mage => HeroGuild::Arcane,
        HeroClass::Rogue => HeroGuild::FreeRoaming,
        HeroClass::Healer => HeroGuild::Faith,
    }
}

fn hero_personality_for_class(class: HeroClass, seed: i32) -> HeroPersonality {
    let variants = match class {
        HeroClass::Warrior => [HeroPersonality::Brave, HeroPersonality::Dutiful, HeroPersonality::Greedy],
        HeroClass::Archer => [HeroPersonality::Cautious, HeroPersonality::Wanderer, HeroPersonality::Greedy],
        HeroClass::Mage => [HeroPersonality::Greedy, HeroPersonality::Cautious, HeroPersonality::Dutiful],
        HeroClass::Rogue => [HeroPersonality::Greedy, HeroPersonality::Wanderer, HeroPersonality::Cautious],
        HeroClass::Healer => [HeroPersonality::Dutiful, HeroPersonality::Brave, HeroPersonality::Cautious],
    };
    variants[(seed.rem_euclid(variants.len() as i32)) as usize]
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum HeroTask {
    Patrol,
    RecoverAtInn,
    PrayAtTemple,
    ClaimBounty,
    ExploreRuins,
    GatherResource,
    DefendTown,
    EscortMerchant,
    LayLow,
    Incapacitated,
}

#[derive(Component)]
struct Hero {
    class: HeroClass,
    personality: HeroPersonality,
    guild: HeroGuild,
    task: HeroTask,
    hp: f32,
    max_hp: f32,
    attack: f32,
    speed: f32,
    morale: f32,
    level: u32,
    xp: u32,
    perk_points: u32,
    legendary: bool,
    bonus_hp: f32,
    bonus_attack: f32,
    bonus_speed: f32,
    personal_gold: i32,
    incapacitated_until_day: u32,
    recovery_bounty_due: i32,
    recovery_bounty_paid: bool,
    days_without_work: u32,
    ability_cooldown: f32,
    preferred_zone: Option<ZoneKind>,
    decision_timer: Timer,
    service_timer: Timer,
}

#[derive(Component)]
struct MoveTarget(Vec3);

#[derive(Component)]
struct RecoveryPayment;

#[derive(Component)]
struct Building {
    kind: BuildingKind,
}

#[derive(Component)]
struct WorldEntity;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum BuildingKind {
    TownHall,
    Inn,
    Market,
    Temple,
    GuardTower,
    WizardTower,
    Blacksmith,
    Alchemist,
    Barracks,
    House,
    Farm,
}

impl BuildingKind {
    fn label(self) -> &'static str {
        match self {
            BuildingKind::TownHall => "King's Castle",
            BuildingKind::Inn => "Inn",
            BuildingKind::Market => "Market",
            BuildingKind::Temple => "Temple",
            BuildingKind::GuardTower => "Tower",
            BuildingKind::WizardTower => "Wizard Tower",
            BuildingKind::Blacksmith => "Blacksmith",
            BuildingKind::Alchemist => "Alchemist",
            BuildingKind::Barracks => "Barracks",
            BuildingKind::House => "Hero House",
            BuildingKind::Farm => "Field",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ZoneKind {
    GoblinCamp,
    BanditDen,
    TrollLair,
    ShadowRift,
    DungeonLord,
}

impl ZoneKind {
    fn name(self) -> &'static str {
        match self {
            ZoneKind::GoblinCamp => "Goblin Camp",
            ZoneKind::BanditDen => "Bandit Den",
            ZoneKind::TrollLair => "Troll Lair",
            ZoneKind::ShadowRift => "Shadow Rift",
            ZoneKind::DungeonLord => "Dungeon Lord",
        }
    }
}

fn zone_danger(kind: ZoneKind) -> f32 {
    match kind {
        ZoneKind::GoblinCamp => 1.0,
        ZoneKind::BanditDen => 2.0,
        ZoneKind::TrollLair => 3.5,
        ZoneKind::ShadowRift => 2.7,
        ZoneKind::DungeonLord => 6.0,
    }
}

#[derive(Component)]
struct MonsterZone {
    kind: ZoneKind,
    name: String,
    danger: f32,
    hp: f32,
    max_hp: f32,
    active_bounty: bool,
    cleared: bool,
    reward_gold: i32,
    reward_xp: u32,
    tier: u32,
    raid_timer: f32,
    last_escalation_day: u32,
    bounty_posted_day: Option<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CivilianRole {
    Farmer,
    Trader,
    Smith,
    Acolyte,
    Laborer,
}

impl CivilianRole {
    fn name(self) -> &'static str {
        match self {
            CivilianRole::Farmer => "Farmer",
            CivilianRole::Trader => "Trader",
            CivilianRole::Smith => "Smith",
            CivilianRole::Acolyte => "Acolyte",
            CivilianRole::Laborer => "Laborer",
        }
    }
}

#[derive(Component)]
struct Civilian {
    role: CivilianRole,
    home_index: u32,
    decision_timer: Timer,
    speed: f32,
    at_work: bool,
}

#[derive(Component)]
struct CivilianTarget(Vec3);

#[derive(Component)]
struct Merchant {
    reward_gold: i32,
    escorted: bool,
}

#[derive(Component)]
struct TopBarText;

#[derive(Component)]
struct HintText;

#[derive(Component)]
struct EventLogText;

#[derive(Component)]
struct WorldLabel;

#[derive(Component)]
struct HeroStatusLabel;

#[derive(Component)]
struct ZoneStatusLabel;

#[derive(Component)]
struct ZoneTargetMarker;

#[derive(Component)]
struct ZoneHealthBarDecor;

#[derive(Component)]
struct FogSector {
    required_sectors: u32,
}

#[derive(Component)]
struct DayNightOverlay;

#[derive(Component)]
struct ActionButton(ActionButtonKind);

#[derive(Clone, Copy)]
enum ActionButtonKind {
    BuildInn,
    BuildMarket,
    BuildTemple,
    BuildTower,
    BuildWizardTower,
    BuildBlacksmith,
    BuildAlchemist,
    BuildBarracks,
    BuildRoad,
    BuildBridge,
    RecruitWarrior,
    RecruitArcher,
    RecruitRogue,
    RecruitMage,
    RecruitHealer,
    UpgradeHall,
    UpgradeInn,
    UpgradeMarket,
    UpgradeTemple,
    RepairKeep,
    FundRecovery,
    EscortMerchant,
    PostExploreBounty,
    PostResourceBounty,
    PostDefenseBounty,
    PostNextBounty,
    NewEra,
    SpeedNormal,
    SpeedFast,
    Pause,
}

struct GameState {
    gold: i32,
    income_per_day: i32,
    day: u32,
    day_timer: f32,
    speed: f32,
    paused: bool,
    era: u32,
    legacy_points: u32,
    kingdom_fallen: bool,
    castle_hp: f32,
    castle_max_hp: f32,
    has_inn: bool,
    has_market: bool,
    has_temple: bool,
    has_guard_tower: bool,
    has_wizard_tower: bool,
    has_blacksmith: bool,
    has_alchemist: bool,
    has_barracks: bool,
    town_hall_tier: u32,
    inn_tier: u32,
    market_tier: u32,
    temple_tier: u32,
    tower_tier: u32,
    hero_capacity: u32,
    houses: u32,
    civilians: u32,
    farms: u32,
    kingdom_rank: u32,
    ruins_bounty_active: bool,
    ruins_bounty_posted_day: Option<u32>,
    ruins_revealed: bool,
    resource_bounty_active: bool,
    resource_bounty_posted_day: Option<u32>,
    defense_bounty_active: bool,
    defense_bounty_posted_day: Option<u32>,
    merchant_bounty_active: bool,
    merchant_bounty_posted_day: Option<u32>,
    defense_days_remaining: u32,
    resource_income_days: u32,
    revealed_sectors: u32,
    spawned_bandits: bool,
    spawned_trolls: bool,
    spawned_shadows: bool,
    spawned_dungeon_lord: bool,
    era_complete: bool,
    new_era_requested: bool,
    events: Vec<String>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            gold: 500,
            income_per_day: 23,
            day: 1,
            day_timer: 0.0,
            speed: 1.0,
            paused: false,
            era: 1,
            legacy_points: 0,
            kingdom_fallen: false,
            castle_hp: 320.0,
            castle_max_hp: 320.0,
            has_inn: false,
            has_market: false,
            has_temple: false,
            has_guard_tower: false,
            has_wizard_tower: false,
            has_blacksmith: false,
            has_alchemist: false,
            has_barracks: false,
            town_hall_tier: 1,
            inn_tier: 0,
            market_tier: 0,
            temple_tier: 0,
            tower_tier: 0,
            hero_capacity: 2,
            houses: 1,
            civilians: 2,
            farms: 1,
            kingdom_rank: 1,
            ruins_bounty_active: false,
            ruins_bounty_posted_day: None,
            ruins_revealed: false,
            resource_bounty_active: false,
            resource_bounty_posted_day: None,
            defense_bounty_active: false,
            defense_bounty_posted_day: None,
            merchant_bounty_active: false,
            merchant_bounty_posted_day: None,
            defense_days_remaining: 0,
            resource_income_days: 0,
            revealed_sectors: 1,
            spawned_bandits: false,
            spawned_trolls: false,
            spawned_shadows: false,
            spawned_dungeon_lord: false,
            era_complete: false,
            new_era_requested: false,
            events: vec![
                "Place an Inn to attract more heroes, then post a bounty on the Goblin Camp.".to_string(),
            ],
        }
    }
}

struct AutoSaveState {
    timer: Timer,
}

impl Default for AutoSaveState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(30.0, true),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SaveSnapshot {
    version: u32,
    saved_unix_secs: u64,
    state: SaveStateSnapshot,
    tiles: Vec<TileState>,
    heroes: Vec<HeroSnapshot>,
    civilians: Vec<CivilianSnapshot>,
    zones: Vec<ZoneSnapshot>,
}

#[derive(Serialize, Deserialize)]
struct SaveStateSnapshot {
    gold: i32,
    income_per_day: i32,
    day: u32,
    day_timer: f32,
    speed: f32,
    paused: bool,
    era: u32,
    legacy_points: u32,
    kingdom_fallen: bool,
    castle_hp: f32,
    castle_max_hp: f32,
    has_inn: bool,
    has_market: bool,
    has_temple: bool,
    has_guard_tower: bool,
    has_wizard_tower: bool,
    has_blacksmith: bool,
    has_alchemist: bool,
    has_barracks: bool,
    town_hall_tier: u32,
    inn_tier: u32,
    market_tier: u32,
    temple_tier: u32,
    tower_tier: u32,
    hero_capacity: u32,
    houses: u32,
    civilians: u32,
    farms: u32,
    kingdom_rank: u32,
    ruins_bounty_active: bool,
    ruins_bounty_posted_day: Option<u32>,
    ruins_revealed: bool,
    resource_bounty_active: bool,
    resource_bounty_posted_day: Option<u32>,
    defense_bounty_active: bool,
    defense_bounty_posted_day: Option<u32>,
    merchant_bounty_active: bool,
    merchant_bounty_posted_day: Option<u32>,
    defense_days_remaining: u32,
    resource_income_days: u32,
    revealed_sectors: u32,
    spawned_bandits: bool,
    spawned_trolls: bool,
    spawned_shadows: bool,
    spawned_dungeon_lord: bool,
    era_complete: bool,
    events: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct HeroSnapshot {
    class: HeroClass,
    personality: HeroPersonality,
    guild: HeroGuild,
    task: HeroTask,
    hp: f32,
    morale: f32,
    level: u32,
    xp: u32,
    perk_points: u32,
    legendary: bool,
    bonus_hp: f32,
    bonus_attack: f32,
    bonus_speed: f32,
    personal_gold: i32,
    incapacitated_until_day: u32,
    recovery_bounty_due: i32,
    recovery_bounty_paid: bool,
    days_without_work: u32,
    ability_cooldown: f32,
    preferred_zone: Option<ZoneKind>,
    position: [f32; 3],
}

#[derive(Serialize, Deserialize)]
struct CivilianSnapshot {
    role: CivilianRole,
    home_index: u32,
    at_work: bool,
    position: [f32; 3],
}

#[derive(Serialize, Deserialize)]
struct ZoneSnapshot {
    kind: ZoneKind,
    hp: f32,
    max_hp: f32,
    active_bounty: bool,
    cleared: bool,
    reward_gold: i32,
    reward_xp: u32,
    tier: u32,
    raid_timer: f32,
    last_escalation_day: u32,
    bounty_posted_day: Option<u32>,
}

fn autosave_path() -> PathBuf {
    PathBuf::from("realm_autosave.json")
}

fn current_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn snapshot_game_state(state: &GameState) -> SaveStateSnapshot {
    SaveStateSnapshot {
        gold: state.gold,
        income_per_day: state.income_per_day,
        day: state.day,
        day_timer: state.day_timer,
        speed: state.speed,
        paused: state.paused,
        era: state.era,
        legacy_points: state.legacy_points,
        kingdom_fallen: state.kingdom_fallen,
        castle_hp: state.castle_hp,
        castle_max_hp: state.castle_max_hp,
        has_inn: state.has_inn,
        has_market: state.has_market,
        has_temple: state.has_temple,
        has_guard_tower: state.has_guard_tower,
        has_wizard_tower: state.has_wizard_tower,
        has_blacksmith: state.has_blacksmith,
        has_alchemist: state.has_alchemist,
        has_barracks: state.has_barracks,
        town_hall_tier: state.town_hall_tier,
        inn_tier: state.inn_tier,
        market_tier: state.market_tier,
        temple_tier: state.temple_tier,
        tower_tier: state.tower_tier,
        hero_capacity: state.hero_capacity,
        houses: state.houses,
        civilians: state.civilians,
        farms: state.farms,
        kingdom_rank: state.kingdom_rank,
        ruins_bounty_active: state.ruins_bounty_active,
        ruins_bounty_posted_day: state.ruins_bounty_posted_day,
        ruins_revealed: state.ruins_revealed,
        resource_bounty_active: state.resource_bounty_active,
        resource_bounty_posted_day: state.resource_bounty_posted_day,
        defense_bounty_active: state.defense_bounty_active,
        defense_bounty_posted_day: state.defense_bounty_posted_day,
        merchant_bounty_active: state.merchant_bounty_active,
        merchant_bounty_posted_day: state.merchant_bounty_posted_day,
        defense_days_remaining: state.defense_days_remaining,
        resource_income_days: state.resource_income_days,
        revealed_sectors: state.revealed_sectors,
        spawned_bandits: state.spawned_bandits,
        spawned_trolls: state.spawned_trolls,
        spawned_shadows: state.spawned_shadows,
        spawned_dungeon_lord: state.spawned_dungeon_lord,
        era_complete: state.era_complete,
        events: state.events.clone(),
    }
}

fn autosave_system(
    time: Res<Time>,
    mut autosave: ResMut<AutoSaveState>,
    state: Res<GameState>,
    map_state: Res<MapState>,
    hero_query: Query<(&Hero, &Transform)>,
    civilian_query: Query<(&Civilian, &Transform)>,
    zone_query: Query<&MonsterZone>,
) {
    autosave.timer.tick(time.delta());
    if !autosave.timer.finished() {
        return;
    }

    let snapshot = SaveSnapshot {
        version: 1,
        saved_unix_secs: current_unix_secs(),
        state: snapshot_game_state(&state),
        tiles: map_state.tiles.clone(),
        heroes: hero_query
            .iter()
            .map(|(hero, transform)| HeroSnapshot {
                class: hero.class,
                personality: hero.personality,
                guild: hero.guild,
                task: hero.task,
                hp: hero.hp,
                morale: hero.morale,
                level: hero.level,
                xp: hero.xp,
                perk_points: hero.perk_points,
                legendary: hero.legendary,
                bonus_hp: hero.bonus_hp,
                bonus_attack: hero.bonus_attack,
                bonus_speed: hero.bonus_speed,
                personal_gold: hero.personal_gold,
                incapacitated_until_day: hero.incapacitated_until_day,
                recovery_bounty_due: hero.recovery_bounty_due,
                recovery_bounty_paid: hero.recovery_bounty_paid,
                days_without_work: hero.days_without_work,
                ability_cooldown: hero.ability_cooldown,
                preferred_zone: hero.preferred_zone,
                position: [
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                ],
            })
            .collect(),
        civilians: civilian_query
            .iter()
            .map(|(civilian, transform)| CivilianSnapshot {
                role: civilian.role,
                home_index: civilian.home_index,
                at_work: civilian.at_work,
                position: [
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                ],
            })
            .collect(),
        zones: zone_query
            .iter()
            .map(|zone| ZoneSnapshot {
                kind: zone.kind,
                hp: zone.hp,
                max_hp: zone.max_hp,
                active_bounty: zone.active_bounty,
                cleared: zone.cleared,
                reward_gold: zone.reward_gold,
                reward_xp: zone.reward_xp,
                tier: zone.tier,
                raid_timer: zone.raid_timer,
                last_escalation_day: zone.last_escalation_day,
                bounty_posted_day: zone.bounty_posted_day,
            })
            .collect(),
    };

    if let Ok(json) = serde_json::to_string_pretty(&snapshot) {
        let _ = fs::write(autosave_path(), json);
    }
}

fn load_autosave_snapshot() -> Option<SaveSnapshot> {
    let path = autosave_path();
    let text = fs::read_to_string(path).ok()?;
    serde_json::from_str(&text).ok()
}

fn apply_snapshot_to_state(state: &mut ResMut<GameState>, snapshot: &SaveStateSnapshot) {
    state.gold = snapshot.gold;
    state.income_per_day = snapshot.income_per_day;
    state.day = snapshot.day;
    state.day_timer = snapshot.day_timer;
    state.speed = snapshot.speed;
    state.paused = snapshot.paused;
    state.era = snapshot.era;
    state.legacy_points = snapshot.legacy_points;
    state.kingdom_fallen = snapshot.kingdom_fallen;
    state.castle_hp = snapshot.castle_hp;
    state.castle_max_hp = snapshot.castle_max_hp;
    state.has_inn = snapshot.has_inn;
    state.has_market = snapshot.has_market;
    state.has_temple = snapshot.has_temple;
    state.has_guard_tower = snapshot.has_guard_tower;
    state.has_wizard_tower = snapshot.has_wizard_tower;
    state.has_blacksmith = snapshot.has_blacksmith;
    state.has_alchemist = snapshot.has_alchemist;
    state.has_barracks = snapshot.has_barracks;
    state.town_hall_tier = snapshot.town_hall_tier;
    state.inn_tier = snapshot.inn_tier;
    state.market_tier = snapshot.market_tier;
    state.temple_tier = snapshot.temple_tier;
    state.tower_tier = snapshot.tower_tier;
    state.hero_capacity = snapshot.hero_capacity;
    state.houses = snapshot.houses;
    state.civilians = snapshot.civilians;
    state.farms = snapshot.farms;
    state.kingdom_rank = snapshot.kingdom_rank;
    state.ruins_bounty_active = snapshot.ruins_bounty_active;
    state.ruins_bounty_posted_day = snapshot.ruins_bounty_posted_day;
    state.ruins_revealed = snapshot.ruins_revealed;
    state.resource_bounty_active = snapshot.resource_bounty_active;
    state.resource_bounty_posted_day = snapshot.resource_bounty_posted_day;
    state.defense_bounty_active = snapshot.defense_bounty_active;
    state.defense_bounty_posted_day = snapshot.defense_bounty_posted_day;
    state.merchant_bounty_active = snapshot.merchant_bounty_active;
    state.merchant_bounty_posted_day = snapshot.merchant_bounty_posted_day;
    state.defense_days_remaining = snapshot.defense_days_remaining;
    state.resource_income_days = snapshot.resource_income_days;
    state.revealed_sectors = snapshot.revealed_sectors;
    state.spawned_bandits = snapshot.spawned_bandits;
    state.spawned_trolls = snapshot.spawned_trolls;
    state.spawned_shadows = snapshot.spawned_shadows;
    state.spawned_dungeon_lord = snapshot.spawned_dungeon_lord;
    state.era_complete = snapshot.era_complete;
    state.new_era_requested = false;
    state.events = snapshot.events.clone();
}

fn apply_offline_progress(state: &mut ResMut<GameState>, saved_unix_secs: u64) {
    let now = current_unix_secs();
    if now <= saved_unix_secs {
        return;
    }
    let offline_days = (((now - saved_unix_secs) as f32) / DAY_LENGTH_SECONDS)
        .floor()
        .clamp(0.0, 5.0) as u32;
    if offline_days == 0 {
        return;
    }

    let mut earned = 0;
    for _ in 0..offline_days {
        state.day += 1;
        earned += state.income_per_day;
        if state.resource_income_days > 0 {
            earned += 18;
            state.resource_income_days -= 1;
        }
        if state.defense_days_remaining > 0 {
            state.defense_days_remaining -= 1;
        }
    }
    state.gold += earned;
    push_event(
        state,
        format!(
            "While you were away, the kingdom advanced {} day(s) and earned {} gold.",
            offline_days, earned
        )
        .as_str(),
    );
}

fn spawn_world_from_snapshot(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    catalog: &ArtCatalog,
    snapshot: &SaveSnapshot,
) {
    spawn_world_markers(commands, asset_server);
    spawn_buildings_from_state(commands, asset_server, catalog, &GameState {
        gold: snapshot.state.gold,
        income_per_day: snapshot.state.income_per_day,
        day: snapshot.state.day,
        day_timer: snapshot.state.day_timer,
        speed: snapshot.state.speed,
        paused: snapshot.state.paused,
        era: snapshot.state.era,
        legacy_points: snapshot.state.legacy_points,
        kingdom_fallen: snapshot.state.kingdom_fallen,
        castle_hp: snapshot.state.castle_hp,
        castle_max_hp: snapshot.state.castle_max_hp,
        has_inn: snapshot.state.has_inn,
        has_market: snapshot.state.has_market,
        has_temple: snapshot.state.has_temple,
        has_guard_tower: snapshot.state.has_guard_tower,
        has_wizard_tower: snapshot.state.has_wizard_tower,
        has_blacksmith: snapshot.state.has_blacksmith,
        has_alchemist: snapshot.state.has_alchemist,
        has_barracks: snapshot.state.has_barracks,
        town_hall_tier: snapshot.state.town_hall_tier,
        inn_tier: snapshot.state.inn_tier,
        market_tier: snapshot.state.market_tier,
        temple_tier: snapshot.state.temple_tier,
        tower_tier: snapshot.state.tower_tier,
        hero_capacity: snapshot.state.hero_capacity,
        houses: snapshot.state.houses,
        civilians: snapshot.state.civilians,
        farms: snapshot.state.farms,
        kingdom_rank: snapshot.state.kingdom_rank,
        ruins_bounty_active: snapshot.state.ruins_bounty_active,
        ruins_bounty_posted_day: snapshot.state.ruins_bounty_posted_day,
        ruins_revealed: snapshot.state.ruins_revealed,
        resource_bounty_active: snapshot.state.resource_bounty_active,
        resource_bounty_posted_day: snapshot.state.resource_bounty_posted_day,
        defense_bounty_active: snapshot.state.defense_bounty_active,
        defense_bounty_posted_day: snapshot.state.defense_bounty_posted_day,
        merchant_bounty_active: snapshot.state.merchant_bounty_active,
        merchant_bounty_posted_day: snapshot.state.merchant_bounty_posted_day,
        defense_days_remaining: snapshot.state.defense_days_remaining,
        resource_income_days: snapshot.state.resource_income_days,
        revealed_sectors: snapshot.state.revealed_sectors,
        spawned_bandits: snapshot.state.spawned_bandits,
        spawned_trolls: snapshot.state.spawned_trolls,
        spawned_shadows: snapshot.state.spawned_shadows,
        spawned_dungeon_lord: snapshot.state.spawned_dungeon_lord,
        era_complete: snapshot.state.era_complete,
        new_era_requested: false,
        events: snapshot.state.events.clone(),
    });

    for hero_snapshot in &snapshot.heroes {
        let entity = spawn_hero(
            commands,
            asset_server,
            catalog,
            hero_snapshot.class,
            Vec3::new(
                hero_snapshot.position[0],
                hero_snapshot.position[1],
                hero_snapshot.position[2],
            ),
        );
        commands.entity(entity).insert(Hero {
            class: hero_snapshot.class,
            personality: hero_snapshot.personality,
            guild: hero_snapshot.guild,
            task: hero_snapshot.task,
            hp: hero_snapshot.hp,
            max_hp: hero_snapshot.class.max_hp() + hero_snapshot.bonus_hp,
            attack: hero_snapshot.class.attack() + hero_snapshot.bonus_attack,
            speed: hero_snapshot.class.speed() + hero_snapshot.bonus_speed,
            morale: hero_snapshot.morale,
            level: hero_snapshot.level,
            xp: hero_snapshot.xp,
            perk_points: hero_snapshot.perk_points,
            legendary: hero_snapshot.legendary,
            bonus_hp: hero_snapshot.bonus_hp,
            bonus_attack: hero_snapshot.bonus_attack,
            bonus_speed: hero_snapshot.bonus_speed,
            personal_gold: hero_snapshot.personal_gold,
            incapacitated_until_day: hero_snapshot.incapacitated_until_day,
            recovery_bounty_due: hero_snapshot.recovery_bounty_due,
            recovery_bounty_paid: hero_snapshot.recovery_bounty_paid,
            days_without_work: hero_snapshot.days_without_work,
            ability_cooldown: hero_snapshot.ability_cooldown,
            preferred_zone: hero_snapshot.preferred_zone,
            decision_timer: Timer::from_seconds(AI_INTERVAL_SECONDS, true),
            service_timer: Timer::from_seconds(SERVICE_INTERVAL_SECONDS, true),
        });
    }

    for civilian_snapshot in &snapshot.civilians {
        let entity = spawn_civilian(
            commands,
            asset_server,
            catalog,
            civilian_snapshot.role,
            civilian_snapshot.home_index,
        );
        commands.entity(entity).insert(Transform::from_xyz(
            civilian_snapshot.position[0],
            civilian_snapshot.position[1],
            civilian_snapshot.position[2],
        ));
    }

    for zone_snapshot in &snapshot.zones {
        let entity = spawn_monster_zone(commands, asset_server, catalog, zone_snapshot.kind);
        commands.entity(entity).insert(MonsterZone {
            kind: zone_snapshot.kind,
            name: zone_snapshot.kind.name().to_string(),
            danger: zone_danger(zone_snapshot.kind),
            hp: zone_snapshot.hp,
            max_hp: zone_snapshot.max_hp,
            active_bounty: zone_snapshot.active_bounty,
            cleared: zone_snapshot.cleared,
            reward_gold: zone_snapshot.reward_gold,
            reward_xp: zone_snapshot.reward_xp,
            tier: zone_snapshot.tier,
            raid_timer: zone_snapshot.raid_timer,
            last_escalation_day: zone_snapshot.last_escalation_day,
            bounty_posted_day: zone_snapshot.bounty_posted_day,
        });
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut state: ResMut<GameState>,
    mut map_state: ResMut<MapState>,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.84;
    camera.transform = Transform::from_xyz(35.0, -10.0, 1000.0);
    commands.spawn_bundle(camera);

    let catalog = ArtCatalog::build(&asset_server, &mut texture_atlases);

    spawn_background(&mut commands, &catalog);
    spawn_map_tiles(&mut commands);
    spawn_environment_decor(&mut commands, &catalog);
    spawn_world_overlays(&mut commands);
    if let Some(snapshot) = load_autosave_snapshot() {
        apply_snapshot_to_state(&mut state, &snapshot.state);
        *map_state = MapState {
            tiles: snapshot.tiles.clone(),
        };
        apply_offline_progress(&mut state, snapshot.saved_unix_secs);
        spawn_world_from_snapshot(&mut commands, &asset_server, &catalog, &snapshot);
    } else {
        spawn_world_entities(&mut commands, &asset_server, &catalog);
    }
    spawn_ui(&mut commands, &asset_server);
    commands.insert_resource(catalog);

    state
        .events
        .push("A lone Warrior and a few settlers gather beneath the King's Castle.".to_string());
}

fn spawn_background(commands: &mut Commands, catalog: &ArtCatalog) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.14, 0.19, 0.13),
            custom_size: Some(Vec2::new(1400.0, 900.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(40.0, -10.0, -20.0),
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: catalog.map_overlay_texture.clone(),
        transform: Transform {
            translation: Vec3::new(250.0, 80.0, -18.0),
            scale: Vec3::splat(1.15),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgba(0.20, 0.24, 0.20, 0.12),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_map_tiles(commands: &mut Commands) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.18, 0.24, 0.18, 0.7),
                        custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(tile_to_world(x, y, -8.0)),
                    ..Default::default()
                })
                .insert(MapTileVisual { x, y });
        }
    }
}

fn spawn_environment_decor(commands: &mut Commands, catalog: &ArtCatalog) {
    let grass = catalog.grass_texture.clone();
    let brick = catalog.brick_texture.clone();
    let water = catalog.water_texture.clone();
    let rock = catalog.rock_texture.clone();
    let nav_tile = catalog.nav_tile_texture.clone();

    for x in -5..=5 {
        for y in -3..=3 {
            commands.spawn_bundle(SpriteBundle {
                texture: grass.clone(),
                transform: Transform {
                    translation: Vec3::new(x as f32 * 82.0 + 25.0, y as f32 * 68.0 - 10.0, -6.0),
                    scale: Vec3::splat(1.65),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgba(0.84, 0.95, 0.82, 0.40),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }

    for x in -2..=2 {
        for y in -1..=1 {
            commands.spawn_bundle(SpriteBundle {
                texture: brick.clone(),
                transform: Transform {
                    translation: Vec3::new(x as f32 * 60.0 + 15.0, y as f32 * 44.0 - 5.0, -2.0),
                    scale: Vec3::new(1.35, 1.05, 1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgba(1.0, 0.95, 0.86, 0.82),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }

    for offset in [-130.0, -70.0, -10.0, 50.0, 110.0, 170.0] {
        commands.spawn_bundle(SpriteBundle {
            texture: brick.clone(),
            transform: Transform {
                translation: Vec3::new(offset, -22.0, -2.0),
                scale: Vec3::new(1.15, 0.95, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgba(0.98, 0.92, 0.82, 0.75),
                ..Default::default()
            },
            ..Default::default()
        });
    }

    for y in [-108.0, 55.0] {
        for x in -1..=2 {
            commands.spawn_bundle(SpriteBundle {
                texture: nav_tile.clone(),
                transform: Transform {
                    translation: Vec3::new(x as f32 * 76.0 + 6.0, y, -1.5),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgba(0.95, 0.86, 0.62, 0.35),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }

    for x in -5..=-4 {
        for y in -3..=3 {
            commands.spawn_bundle(SpriteBundle {
                texture: water.clone(),
                transform: Transform {
                    translation: Vec3::new(x as f32 * 92.0 - 45.0, y as f32 * 70.0 - 15.0, -4.5),
                    scale: Vec3::splat(1.7),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgba(0.72, 0.90, 1.0, 0.6),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }

    for (x, y) in [
        (-305.0, 145.0),
        (-285.0, 60.0),
        (-260.0, -20.0),
        (285.0, 150.0),
        (330.0, 90.0),
        (300.0, 10.0),
        (255.0, -135.0),
        (170.0, 195.0),
        (90.0, 215.0),
    ] {
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: catalog.tree_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: if x > 0.0 { 1 } else { 0 },
                color: Color::rgba(0.92, 1.0, 0.92, 0.95),
                custom_size: Some(Vec2::new(36.0, 52.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 2.0),
            ..Default::default()
        });
    }

    for (x, y) in [
        (-210.0, -145.0),
        (-145.0, -145.0),
        (-85.0, -145.0),
        (160.0, -145.0),
        (220.0, -145.0),
        (280.0, -145.0),
        (260.0, 165.0),
        (335.0, -115.0),
    ] {
        commands.spawn_bundle(SpriteBundle {
            texture: rock.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 3.0),
                scale: Vec3::splat(0.8),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgba(0.95, 0.95, 0.95, 0.95),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn spawn_world_overlays(commands: &mut Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.98, 0.84, 0.42, 0.03),
                custom_size: Some(Vec2::new(1450.0, 920.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(35.0, -10.0, 60.0),
            ..Default::default()
        })
        .insert(DayNightOverlay);

    for (position, size, required_sectors) in [
        (Vec3::new(245.0, 145.0, 55.0), Vec2::new(300.0, 220.0), 2),
        (Vec3::new(300.0, -25.0, 55.0), Vec2::new(230.0, 260.0), 3),
        (Vec3::new(165.0, -175.0, 55.0), Vec2::new(420.0, 135.0), 4),
        (Vec3::new(-280.0, -15.0, 55.0), Vec2::new(165.0, 420.0), 5),
    ] {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.02, 0.05, 0.04, 0.88),
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .insert(FogSector { required_sectors });
    }
}

fn spawn_world_markers(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    spawn_marker(commands, asset_server, ruins_pos(), Color::rgb(0.75, 0.75, 0.88), "Ruins");
    spawn_marker(
        commands,
        asset_server,
        resource_node_pos(),
        Color::rgb(0.84, 0.93, 0.56),
        "Resource Node",
    );
    spawn_marker(
        commands,
        asset_server,
        defend_pos(),
        Color::rgb(0.96, 0.82, 0.48),
        "Town Perimeter",
    );
}

fn spawn_buildings_from_state(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    catalog: &ArtCatalog,
    state: &GameState,
) {
    spawn_building(
        commands,
        asset_server,
        catalog,
        BuildingKind::TownHall,
        town_hall_pos(),
        BuildingKind::TownHall.label(),
    );
    spawn_building(
        commands,
        asset_server,
        catalog,
        BuildingKind::House,
        house_pos(0),
        BuildingKind::House.label(),
    );
    if state.has_inn {
        spawn_building(commands, asset_server, catalog, BuildingKind::Inn, inn_pos(), BuildingKind::Inn.label());
    }
    if state.has_market {
        spawn_building(commands, asset_server, catalog, BuildingKind::Market, market_pos(), BuildingKind::Market.label());
    }
    if state.has_temple {
        spawn_building(commands, asset_server, catalog, BuildingKind::Temple, temple_pos(), BuildingKind::Temple.label());
    }
    if state.has_guard_tower {
        spawn_building(
            commands,
            asset_server,
            catalog,
            BuildingKind::GuardTower,
            tower_pos(),
            BuildingKind::GuardTower.label(),
        );
    }
    if state.has_wizard_tower {
        spawn_building(
            commands,
            asset_server,
            catalog,
            BuildingKind::WizardTower,
            wizard_tower_pos(),
            BuildingKind::WizardTower.label(),
        );
    }
    if state.has_blacksmith {
        spawn_building(
            commands,
            asset_server,
            catalog,
            BuildingKind::Blacksmith,
            blacksmith_pos(),
            BuildingKind::Blacksmith.label(),
        );
    }
    if state.has_alchemist {
        spawn_building(
            commands,
            asset_server,
            catalog,
            BuildingKind::Alchemist,
            alchemist_pos(),
            BuildingKind::Alchemist.label(),
        );
    }
    if state.has_barracks {
        spawn_building(
            commands,
            asset_server,
            catalog,
            BuildingKind::Barracks,
            barracks_pos(),
            BuildingKind::Barracks.label(),
        );
    }
    for i in 1..state.houses {
        spawn_building(commands, asset_server, catalog, BuildingKind::House, house_pos(i), BuildingKind::House.label());
    }
    for i in 0..state.farms {
        spawn_building(commands, asset_server, catalog, BuildingKind::Farm, farm_pos(i), BuildingKind::Farm.label());
    }
}

fn spawn_world_entities(commands: &mut Commands, asset_server: &Res<AssetServer>, catalog: &ArtCatalog) {
    let default_state = GameState::default();
    spawn_world_markers(commands, asset_server);
    spawn_buildings_from_state(commands, asset_server, catalog, &default_state);
    spawn_hero(
        commands,
        asset_server,
        catalog,
        HeroClass::Warrior,
        town_hall_pos() + Vec3::new(50.0, -20.0, 5.0),
    );
    spawn_civilian(commands, asset_server, catalog, CivilianRole::Farmer, 0);
    spawn_civilian(commands, asset_server, catalog, CivilianRole::Laborer, 1);
    spawn_monster_zone(commands, asset_server, catalog, ZoneKind::GoblinCamp);
}

fn spawn_building(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    catalog: &ArtCatalog,
    kind: BuildingKind,
    position: Vec3,
    label: &str,
) {
    let tint = match kind {
        BuildingKind::TownHall => Color::WHITE,
        BuildingKind::Inn => Color::rgb(0.93, 0.85, 0.80),
        BuildingKind::Market => Color::rgb(0.95, 0.82, 0.55),
        BuildingKind::Temple => Color::rgb(0.84, 0.88, 0.96),
        BuildingKind::GuardTower => Color::rgb(0.82, 0.72, 0.72),
        BuildingKind::WizardTower => Color::rgb(0.79, 0.68, 0.92),
        BuildingKind::Blacksmith => Color::rgb(0.73, 0.73, 0.78),
        BuildingKind::Alchemist => Color::rgb(0.68, 0.90, 0.76),
        BuildingKind::Barracks => Color::rgb(0.85, 0.70, 0.52),
        BuildingKind::House => Color::rgb(0.88, 0.76, 0.68),
        BuildingKind::Farm => Color::rgb(0.70, 0.88, 0.48),
    };
    let spec = building_sprite_spec(catalog, kind);

    let building_entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spec.atlas,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(spec.scale),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: spec.index,
                color: tint,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Building { kind })
        .insert(WorldEntity)
        .id();

    let icon_texture = match kind {
        BuildingKind::Inn => Some(catalog.beer_icon.clone()),
        BuildingKind::Blacksmith => Some(catalog.axe_icon.clone()),
        BuildingKind::Alchemist => Some(catalog.potion_icon.clone()),
        BuildingKind::Barracks => Some(catalog.helm_icon.clone()),
        BuildingKind::Temple => Some(catalog.firework_texture.clone()),
        BuildingKind::WizardTower => Some(catalog.target_texture.clone()),
        _ => None::<Handle<Image>>,
    };

    if let Some(icon_texture) = icon_texture {
        let icon_entity = commands
            .spawn_bundle(SpriteBundle {
                texture: icon_texture,
                transform: Transform {
                    translation: Vec3::new(0.0, 12.0, 10.0),
                    scale: Vec3::splat(0.26),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();
        commands.entity(building_entity).push_children(&[icon_entity]);
    }

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                label,
                TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: if matches!(kind, BuildingKind::House | BuildingKind::Farm) {
                        0.0
                    } else {
                        13.0
                    },
                    color: Color::rgb(0.96, 0.92, 0.78),
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            transform: Transform::from_xyz(position.x, position.y - 48.0, 20.0),
            ..Default::default()
        })
        .insert(WorldEntity)
        .insert(WorldLabel);
}

fn spawn_hero(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    catalog: &ArtCatalog,
    class: HeroClass,
    position: Vec3,
) -> Entity {
    let tint = match class {
        HeroClass::Warrior => Color::rgb(0.92, 0.74, 0.74),
        HeroClass::Archer => Color::rgb(0.70, 0.92, 0.70),
        HeroClass::Mage => Color::rgb(0.80, 0.70, 0.95),
        HeroClass::Rogue => Color::rgb(0.72, 0.72, 0.76),
        HeroClass::Healer => Color::rgb(0.95, 0.95, 0.95),
    };
    let spec = hero_sprite_spec(catalog, class);
    let personality_seed = position.x as i32 + position.y as i32 + class as i32 * 13;

    let hero_entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spec.atlas,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(spec.scale),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: spec.index,
                color: tint,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Hero {
            class,
            personality: hero_personality_for_class(class, personality_seed),
            guild: hero_guild_for_class(class),
            task: HeroTask::Patrol,
            hp: class.max_hp(),
            max_hp: class.max_hp(),
            attack: class.attack(),
            speed: class.speed(),
            morale: 100.0,
            level: 1,
            xp: 0,
            perk_points: 0,
            legendary: false,
            bonus_hp: 0.0,
            bonus_attack: 0.0,
            bonus_speed: 0.0,
            personal_gold: 0,
            incapacitated_until_day: 0,
            recovery_bounty_due: 0,
            recovery_bounty_paid: false,
            days_without_work: 0,
            ability_cooldown: 0.0,
            preferred_zone: None,
            decision_timer: Timer::from_seconds(AI_INTERVAL_SECONDS, true),
            service_timer: Timer::from_seconds(SERVICE_INTERVAL_SECONDS, true),
        })
        .insert(MoveTarget(patrol_pos()))
        .insert(WorldEntity)
        .id();

    let shadow_entity = commands
        .spawn_bundle(SpriteBundle {
            texture: catalog.shadow_texture.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 14.0, -1.0),
                scale: Vec3::splat(0.30),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 0.0, 0.45),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let label_entity = commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                format!("{}\nPatrol", class.name()),
                TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: 10.0,
                    color: Color::rgb(0.98, 0.97, 0.90),
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            transform: Transform::from_xyz(0.0, -22.0, 20.0),
            ..Default::default()
        })
        .insert(HeroStatusLabel)
        .id();

    commands.entity(hero_entity).push_children(&[shadow_entity, label_entity]);
    hero_entity
}

fn spawn_civilian(
    commands: &mut Commands,
    _asset_server: &Res<AssetServer>,
    catalog: &ArtCatalog,
    role: CivilianRole,
    home_index: u32,
) -> Entity {
    let color = match role {
        CivilianRole::Farmer => Color::rgb(0.82, 0.92, 0.52),
        CivilianRole::Trader => Color::rgb(0.95, 0.83, 0.58),
        CivilianRole::Smith => Color::rgb(0.75, 0.75, 0.80),
        CivilianRole::Acolyte => Color::rgb(0.88, 0.88, 0.98),
        CivilianRole::Laborer => Color::rgb(0.88, 0.78, 0.68),
    };
    let position = house_pos(home_index / 3) + Vec3::new((home_index % 3) as f32 * 10.0 - 8.0, -12.0, 5.0);
    let spec = civilian_sprite_spec(catalog, role);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spec.atlas,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(spec.scale),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: spec.index,
                color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Civilian {
            role,
            home_index,
            decision_timer: Timer::from_seconds(CIVILIAN_INTERVAL_SECONDS, true),
            speed: 26.0,
            at_work: false,
        })
        .insert(CivilianTarget(position))
        .insert(WorldEntity)
        .id()
}

fn spawn_merchant(commands: &mut Commands, catalog: &ArtCatalog) -> Entity {
    let spec = civilian_sprite_spec(catalog, CivilianRole::Trader);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spec.atlas,
            transform: Transform {
                translation: merchant_entry_pos(),
                scale: Vec3::splat(spec.scale * 1.05),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: spec.index,
                color: Color::rgb(0.98, 0.86, 0.60),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Merchant {
            reward_gold: 55,
            escorted: false,
        })
        .insert(WorldEntity)
        .id()
}

fn spawn_monster_zone(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    catalog: &ArtCatalog,
    kind: ZoneKind,
) -> Entity {
    let (position, hp, reward_gold, reward_xp, danger, tint) = match kind {
        ZoneKind::GoblinCamp => (
            goblin_camp_pos(),
            80.0,
            120,
            60,
            1.0,
            Color::rgb(1.0, 0.82, 0.82),
        ),
        ZoneKind::BanditDen => (
            bandit_den_pos(),
            150.0,
            180,
            110,
            2.0,
            Color::rgb(0.98, 0.68, 0.48),
        ),
        ZoneKind::TrollLair => (
            troll_lair_pos(),
            320.0,
            280,
            220,
            3.5,
            Color::rgb(0.82, 0.52, 0.42),
        ),
        ZoneKind::ShadowRift => (
            shadow_rift_pos(),
            220.0,
            240,
            170,
            2.7,
            Color::rgb(0.78, 0.62, 0.92),
        ),
        ZoneKind::DungeonLord => (
            dungeon_lord_pos(),
            950.0,
            700,
            600,
            6.0,
            Color::rgb(0.95, 0.40, 0.35),
        ),
    };
    let spec = zone_sprite_spec(catalog, kind);

    let zone_entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spec.atlas,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(spec.scale),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: spec.index,
                color: tint,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MonsterZone {
            kind,
            name: kind.name().to_string(),
            danger,
            hp,
            max_hp: hp,
            active_bounty: false,
            cleared: false,
            reward_gold,
            reward_xp,
            tier: 1,
            raid_timer: 0.0,
            last_escalation_day: 1,
            bounty_posted_day: None,
        })
        .insert(WorldEntity)
        .id();

    let label_entity = commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                format!("{}\nHP {} | T1", kind.name(), hp as i32),
                TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: 11.0,
                    color: Color::rgb(0.95, 0.82, 0.58),
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            transform: Transform::from_xyz(0.0, -28.0, 20.0),
            ..Default::default()
        })
        .insert(ZoneStatusLabel)
        .id();

    let target_marker = commands
        .spawn_bundle(SpriteBundle {
            texture: catalog.target_texture.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 12.0, 12.0),
                    scale: Vec3::splat(0.22),
                    ..Default::default()
                },
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ZoneTargetMarker)
        .id();

    let health_bar = commands
        .spawn_bundle(SpriteBundle {
            texture: catalog.health_bar_texture.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, -16.0, 11.0),
                    scale: Vec3::splat(0.26),
                    ..Default::default()
                },
            ..Default::default()
        })
        .insert(ZoneHealthBarDecor)
        .id();

    commands.entity(zone_entity).push_children(&[label_entity, target_marker, health_bar]);
    zone_entity
}

fn spawn_marker(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    color: Color,
    label: &str,
) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(18.0, 18.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(position.x, position.y, 10.0),
        ..Default::default()
    })
    .insert(WorldEntity);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                label,
                TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: 10.0,
                    color: Color::rgb(0.98, 0.95, 0.84),
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            transform: Transform::from_xyz(position.x, position.y - 22.0, 20.0),
            ..Default::default()
        })
        .insert(WorldEntity)
        .insert(WorldLabel);
}

fn spawn_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font = asset_server.load("fonts/arial.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|root| {
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(460.0), Val::Px(140.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(20.0),
                        top: Val::Px(20.0),
                        ..Default::default()
                    },
                    padding: Rect::all(Val::Px(14.0)),
                    ..Default::default()
                },
                color: UiColor(Color::rgba(0.07, 0.10, 0.08, 0.92)),
                ..Default::default()
            })
            .with_children(|panel| {
                panel
                    .spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font: font.clone(),
                                font_size: 28.0,
                                color: Color::rgb(0.96, 0.92, 0.82),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    })
                    .insert(TopBarText);
            });

            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(380.0), Val::Px(320.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        right: Val::Px(20.0),
                        top: Val::Px(20.0),
                        ..Default::default()
                    },
                    padding: Rect::all(Val::Px(14.0)),
                    ..Default::default()
                },
                color: UiColor(Color::rgba(0.07, 0.10, 0.08, 0.92)),
                ..Default::default()
            })
            .with_children(|panel| {
                panel
                    .spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font: font.clone(),
                                font_size: 22.0,
                                color: Color::rgb(0.88, 0.89, 0.84),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    })
                    .insert(EventLogText);
            });

            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(680.0), Val::Px(100.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(20.0),
                        bottom: Val::Px(160.0),
                        ..Default::default()
                    },
                    padding: Rect::all(Val::Px(14.0)),
                    ..Default::default()
                },
                color: UiColor(Color::rgba(0.07, 0.10, 0.08, 0.90)),
                ..Default::default()
            })
            .with_children(|panel| {
                panel
                    .spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font: font.clone(),
                                font_size: 22.0,
                                color: Color::rgb(0.98, 0.85, 0.50),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    })
                    .insert(HintText);
            });

            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(130.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        ..Default::default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_wrap: FlexWrap::Wrap,
                    ..Default::default()
                },
                color: UiColor(Color::rgba(0.06, 0.08, 0.07, 0.95)),
                ..Default::default()
            })
            .with_children(|bar| {
                for (label, kind) in [
                    ("Build Inn", ActionButtonKind::BuildInn),
                    ("Build Market", ActionButtonKind::BuildMarket),
                    ("Build Temple", ActionButtonKind::BuildTemple),
                    ("Build Tower", ActionButtonKind::BuildTower),
                    ("Build Wizard", ActionButtonKind::BuildWizardTower),
                    ("Build Smith", ActionButtonKind::BuildBlacksmith),
                    ("Build Alchemist", ActionButtonKind::BuildAlchemist),
                    ("Build Barracks", ActionButtonKind::BuildBarracks),
                    ("Build Road", ActionButtonKind::BuildRoad),
                    ("Build Bridge", ActionButtonKind::BuildBridge),
                    ("Recruit Warrior", ActionButtonKind::RecruitWarrior),
                    ("Recruit Archer", ActionButtonKind::RecruitArcher),
                    ("Recruit Rogue", ActionButtonKind::RecruitRogue),
                    ("Recruit Mage", ActionButtonKind::RecruitMage),
                    ("Recruit Healer", ActionButtonKind::RecruitHealer),
                    ("Upgrade Hall", ActionButtonKind::UpgradeHall),
                    ("Upgrade Inn", ActionButtonKind::UpgradeInn),
                    ("Upgrade Market", ActionButtonKind::UpgradeMarket),
                    ("Upgrade Temple", ActionButtonKind::UpgradeTemple),
                    ("Repair Keep", ActionButtonKind::RepairKeep),
                    ("Fund Recovery", ActionButtonKind::FundRecovery),
                    ("Escort Merchant", ActionButtonKind::EscortMerchant),
                    ("Scout Ruins", ActionButtonKind::PostExploreBounty),
                    ("Resource Run", ActionButtonKind::PostResourceBounty),
                    ("Defend Town", ActionButtonKind::PostDefenseBounty),
                    ("Post Next Bounty", ActionButtonKind::PostNextBounty),
                    ("New Era", ActionButtonKind::NewEra),
                    ("Speed 1x", ActionButtonKind::SpeedNormal),
                    ("Speed 2x", ActionButtonKind::SpeedFast),
                    ("Pause", ActionButtonKind::Pause),
                ] {
                    spawn_button(bar, font.clone(), label, kind);
                }
            });
        });
}

fn spawn_button(parent: &mut ChildBuilder, font: Handle<Font>, label: &str, kind: ActionButtonKind) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(180.0), Val::Px(52.0)),
                margin: Rect::all(Val::Px(8.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: UiColor(Color::rgb(0.42, 0.31, 0.18)),
            ..Default::default()
        })
        .insert(ActionButton(kind))
        .with_children(|button| {
            button.spawn_bundle(TextBundle {
                text: Text::with_section(
                    label,
                    TextStyle {
                        font,
                        font_size: 20.0,
                        color: Color::rgb(0.96, 0.92, 0.84),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

fn button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &ActionButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    catalog: Res<ArtCatalog>,
    mut state: ResMut<GameState>,
    _map_state: ResMut<MapState>,
    mut placement: ResMut<PlacementState>,
    mut zone_query: Query<&mut MonsterZone>,
    hero_query: Query<(Entity, &Hero)>,
    merchant_query: Query<Entity, With<Merchant>>,
) {
    for (interaction, mut color, action) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = UiColor(Color::rgb(0.76, 0.54, 0.24));
                match action.0 {
                    ActionButtonKind::BuildInn => {
                        if state.has_inn {
                            push_event(&mut state, "The kingdom already has an Inn.");
                        } else if state.gold < 150 {
                            push_event(&mut state, "Not enough gold to build an Inn.");
                        } else {
                            begin_build_placement(&mut state, &mut placement, BuildingKind::Inn);
                        }
                    }
                    ActionButtonKind::BuildMarket => {
                        if state.has_market {
                            push_event(&mut state, "The kingdom already has a Market.");
                        } else if state.gold < 200 {
                            push_event(&mut state, "Not enough gold to build a Market.");
                        } else {
                            begin_build_placement(&mut state, &mut placement, BuildingKind::Market);
                        }
                    }
                    ActionButtonKind::BuildTemple => {
                        if state.has_temple {
                            push_event(&mut state, "The kingdom already has a Temple.");
                        } else if state.gold < 250 {
                            push_event(&mut state, "Not enough gold to build a Temple.");
                        } else {
                            begin_build_placement(&mut state, &mut placement, BuildingKind::Temple);
                        }
                    }
                    ActionButtonKind::BuildTower => {
                        if state.has_guard_tower {
                            push_event(&mut state, "A Guard Tower already protects the town.");
                        } else if state.gold < 300 {
                            push_event(&mut state, "Not enough gold to build a Guard Tower.");
                        } else {
                            begin_build_placement(&mut state, &mut placement, BuildingKind::GuardTower);
                        }
                    }
                    ActionButtonKind::BuildWizardTower => {
                        if state.has_wizard_tower {
                            push_event(&mut state, "The kingdom already has a Wizard Tower.");
                        } else if !state.has_temple {
                            push_event(&mut state, "Build a Temple first to unlock arcane study.");
                        } else if state.gold < 400 {
                            push_event(&mut state, "Not enough gold to build a Wizard Tower.");
                        } else {
                            begin_build_placement(&mut state, &mut placement, BuildingKind::WizardTower);
                        }
                    }
                    ActionButtonKind::BuildBlacksmith => {
                        if state.has_blacksmith {
                            push_event(&mut state, "The kingdom already has a Blacksmith.");
                        } else if !state.has_inn {
                            push_event(&mut state, "Build an Inn before opening a Blacksmith.");
                        } else if state.gold < 350 {
                            push_event(&mut state, "Not enough gold to build a Blacksmith.");
                        } else {
                            begin_build_placement(&mut state, &mut placement, BuildingKind::Blacksmith);
                        }
                    }
                    ActionButtonKind::BuildAlchemist => {
                        if state.has_alchemist {
                            push_event(&mut state, "The kingdom already has an Alchemist.");
                        } else if !state.has_temple {
                            push_event(&mut state, "Build a Temple before inviting an Alchemist.");
                        } else if state.gold < 300 {
                            push_event(&mut state, "Not enough gold to build an Alchemist.");
                        } else {
                            begin_build_placement(&mut state, &mut placement, BuildingKind::Alchemist);
                        }
                    }
                    ActionButtonKind::BuildBarracks => {
                        if state.has_barracks {
                            push_event(&mut state, "The kingdom already has Barracks.");
                        } else if !state.has_guard_tower {
                            push_event(&mut state, "Raise a Guard Tower before expanding into Barracks.");
                        } else if state.gold < 450 {
                            push_event(&mut state, "Not enough gold to build Barracks.");
                        } else {
                            begin_build_placement(&mut state, &mut placement, BuildingKind::Barracks);
                        }
                    }
                    ActionButtonKind::BuildRoad => {
                        if state.gold < 8 {
                            push_event(&mut state, "Not enough gold to lay a road.");
                        } else {
                            placement.building = None;
                            placement.bridge_mode = false;
                            placement.road_mode = true;
                            push_event(&mut state, "Road placement mode active. Tap a grass or forest tile.");
                        }
                    }
                    ActionButtonKind::BuildBridge => {
                        if state.gold < 45 {
                            push_event(&mut state, "Not enough gold to build a bridge.");
                        } else {
                            placement.building = None;
                            placement.road_mode = false;
                            placement.bridge_mode = true;
                            push_event(&mut state, "Bridge placement mode active. Tap a river tile.");
                        }
                    }
                    ActionButtonKind::RecruitWarrior => {
                        if !state.has_inn {
                            push_event(&mut state, "Build an Inn first to attract Warriors.");
                        } else if hero_query.iter().count() as u32 >= state.hero_capacity {
                            push_event(
                                &mut state,
                                "Hero housing is full. More heroes will settle once new houses rise.",
                            );
                        } else if state.gold < 70 {
                            push_event(&mut state, "Not enough gold to recruit a Warrior.");
                        } else {
                            state.gold -= 70;
                            let hero_count = hero_query.iter().count() as f32;
                            spawn_hero(
                                &mut commands,
                                &asset_server,
                                &catalog,
                                HeroClass::Warrior,
                                town_hall_pos() + Vec3::new(60.0 + hero_count * 18.0, -24.0, 5.0),
                            );
                            push_event(&mut state, "A Warrior joined the kingdom.");
                        }
                    }
                    ActionButtonKind::RecruitArcher => {
                        if !state.has_inn {
                            push_event(&mut state, "Build an Inn first to attract Archers.");
                        } else if hero_query.iter().count() as u32 >= state.hero_capacity {
                            push_event(
                                &mut state,
                                "Hero housing is full. Expand the settlement first.",
                            );
                        } else if state.gold < 80 {
                            push_event(&mut state, "Not enough gold to recruit an Archer.");
                        } else {
                            state.gold -= 80;
                            let hero_count = hero_query.iter().count() as f32;
                            spawn_hero(
                                &mut commands,
                                &asset_server,
                                &catalog,
                                HeroClass::Archer,
                                town_hall_pos() + Vec3::new(60.0 + hero_count * 18.0, -24.0, 5.0),
                            );
                            push_event(&mut state, "An Archer joined the kingdom.");
                        }
                    }
                    ActionButtonKind::RecruitRogue => {
                        if !state.has_inn {
                            push_event(&mut state, "Build an Inn first to attract Rogues.");
                        } else if hero_query.iter().count() as u32 >= state.hero_capacity {
                            push_event(&mut state, "Hero housing is full. Build into the town first.");
                        } else if state.gold < 85 {
                            push_event(&mut state, "Not enough gold to recruit a Rogue.");
                        } else {
                            state.gold -= 85;
                            let hero_count = hero_query.iter().count() as f32;
                            spawn_hero(
                                &mut commands,
                                &asset_server,
                                &catalog,
                                HeroClass::Rogue,
                                town_hall_pos() + Vec3::new(60.0 + hero_count * 18.0, -24.0, 5.0),
                            );
                            push_event(&mut state, "A Rogue joined the kingdom.");
                        }
                    }
                    ActionButtonKind::RecruitMage => {
                        if !state.has_wizard_tower {
                            push_event(&mut state, "Build a Wizard Tower first to attract Mages.");
                        } else if hero_query.iter().count() as u32 >= state.hero_capacity {
                            push_event(&mut state, "Hero housing is full. Expand before hiring more heroes.");
                        } else if state.gold < 110 {
                            push_event(&mut state, "Not enough gold to recruit a Mage.");
                        } else {
                            state.gold -= 110;
                            let hero_count = hero_query.iter().count() as f32;
                            spawn_hero(
                                &mut commands,
                                &asset_server,
                                &catalog,
                                HeroClass::Mage,
                                town_hall_pos() + Vec3::new(60.0 + hero_count * 18.0, -24.0, 5.0),
                            );
                            push_event(&mut state, "A Mage joined the kingdom.");
                        }
                    }
                    ActionButtonKind::RecruitHealer => {
                        if !state.has_temple {
                            push_event(&mut state, "Build a Temple first to attract Healers.");
                        } else if hero_query.iter().count() as u32 >= state.hero_capacity {
                            push_event(
                                &mut state,
                                "Hero housing is full. Safer towns naturally grow more homes.",
                            );
                        } else if state.gold < 90 {
                            push_event(&mut state, "Not enough gold to recruit a Healer.");
                        } else {
                            state.gold -= 90;
                            let hero_count = hero_query.iter().count() as f32;
                            spawn_hero(
                                &mut commands,
                                &asset_server,
                                &catalog,
                                HeroClass::Healer,
                                town_hall_pos() + Vec3::new(60.0 + hero_count * 18.0, -24.0, 5.0),
                            );
                            push_event(&mut state, "A Healer joined the kingdom.");
                        }
                    }
                    ActionButtonKind::UpgradeHall => {
                        if state.town_hall_tier >= 3 {
                            push_event(&mut state, "The King's Castle already stands at its highest tier.");
                        } else {
                            let cost = upgrade_cost(250, state.town_hall_tier);
                            if state.gold < cost {
                                push_event(&mut state, "Not enough gold to upgrade the King's Castle.");
                            } else {
                                state.gold -= cost;
                                state.town_hall_tier += 1;
                                state.revealed_sectors += 1;
                                state.hero_capacity += 1;
                                state.castle_max_hp += 45.0;
                                state.castle_hp = (state.castle_hp + 45.0).min(state.castle_max_hp);
                                recalculate_income(&mut state);
                                let tier = state.town_hall_tier;
                                push_event(
                                    &mut state,
                                    format!(
                                        "The King's Castle reaches tier {} and reveals more of the realm.",
                                        tier
                                    )
                                    .as_str(),
                                );
                            }
                        }
                    }
                    ActionButtonKind::UpgradeInn => {
                        if !state.has_inn {
                            push_event(&mut state, "Build an Inn before upgrading it.");
                        } else if state.inn_tier >= 3 {
                            push_event(&mut state, "The Inn has already reached tier 3.");
                        } else {
                            let cost = upgrade_cost(150, state.inn_tier.max(1));
                            if state.gold < cost {
                                push_event(&mut state, "Not enough gold to upgrade the Inn.");
                            } else {
                                state.gold -= cost;
                                state.inn_tier += 1;
                                state.hero_capacity += 1;
                                recalculate_income(&mut state);
                                push_event(&mut state, "The Inn grows warmer and faster at restoring heroes.");
                            }
                        }
                    }
                    ActionButtonKind::UpgradeMarket => {
                        if !state.has_market {
                            push_event(&mut state, "Build a Market before upgrading it.");
                        } else if state.market_tier >= 3 {
                            push_event(&mut state, "The Market has already reached tier 3.");
                        } else {
                            let cost = upgrade_cost(200, state.market_tier.max(1));
                            if state.gold < cost {
                                push_event(&mut state, "Not enough gold to upgrade the Market.");
                            } else {
                                state.gold -= cost;
                                state.market_tier += 1;
                                recalculate_income(&mut state);
                                push_event(&mut state, "The Market expands and draws richer trade caravans.");
                            }
                        }
                    }
                    ActionButtonKind::UpgradeTemple => {
                        if !state.has_temple {
                            push_event(&mut state, "Build a Temple before upgrading it.");
                        } else if state.temple_tier >= 3 {
                            push_event(&mut state, "The Temple has already reached tier 3.");
                        } else {
                            let cost = upgrade_cost(250, state.temple_tier.max(1));
                            if state.gold < cost {
                                push_event(&mut state, "Not enough gold to upgrade the Temple.");
                            } else {
                                state.gold -= cost;
                                state.temple_tier += 1;
                                recalculate_income(&mut state);
                                push_event(&mut state, "The Temple deepens its morale aura across the town.");
                            }
                        }
                    }
                    ActionButtonKind::RepairKeep => {
                        let missing_hp = (state.castle_max_hp - state.castle_hp).max(0.0);
                        if missing_hp <= 0.5 {
                            push_event(&mut state, "The King's Castle is already fully repaired.");
                        } else {
                            let repair_cost = 40 + state.town_hall_tier as i32 * 20;
                            let repair_amount = 70.0 + state.town_hall_tier as f32 * 25.0;
                            if state.gold < repair_cost {
                                push_event(&mut state, "Not enough gold to repair the King's Castle.");
                            } else {
                                state.gold -= repair_cost;
                                state.castle_hp =
                                    (state.castle_hp + repair_amount).min(state.castle_max_hp);
                                push_event(&mut state, "Stone masons repair the King's Castle walls.");
                            }
                        }
                    }
                    ActionButtonKind::FundRecovery => {
                        let mut funded = None;
                        for (entity, hero) in hero_query.iter() {
                            if hero.task == HeroTask::Incapacitated
                                && hero.recovery_bounty_due > 0
                                && !hero.recovery_bounty_paid
                            {
                                funded = Some((entity, hero.recovery_bounty_due, hero.class.name()));
                                break;
                            }
                        }

                        if let Some((entity, cost, class_name)) = funded {
                            if state.gold < cost {
                                push_event(&mut state, "Not enough gold to fund the next hero recovery bounty.");
                            } else {
                                state.gold -= cost;
                                commands.entity(entity).insert(RecoveryPayment);
                                push_event(
                                    &mut state,
                                    format!("The recovery bounty is funded for the fallen {}.", class_name).as_str(),
                                );
                            }
                        } else {
                            push_event(&mut state, "No unpaid recovery bounty is waiting for the kingdom.");
                        }
                    }
                    ActionButtonKind::EscortMerchant => {
                        if !state.has_market {
                            push_event(&mut state, "Build a Market first to attract merchant caravans.");
                        } else if merchant_query.iter().next().is_none() {
                            push_event(&mut state, "No merchant caravan is waiting for an escort right now.");
                        } else if state.merchant_bounty_active {
                            push_event(&mut state, "A merchant escort bounty is already active.");
                        } else {
                            state.merchant_bounty_active = true;
                            state.merchant_bounty_posted_day = Some(state.day);
                            push_event(&mut state, "An escort bounty is posted for the incoming merchant caravan.");
                        }
                    }
                    ActionButtonKind::PostExploreBounty => {
                        if state.ruins_revealed {
                            push_event(&mut state, "The nearby ruins have already been explored.");
                        } else if state.ruins_bounty_active {
                            push_event(&mut state, "An exploration bounty is already posted on the ruins.");
                        } else {
                            state.ruins_bounty_active = true;
                            state.ruins_bounty_posted_day = Some(state.day);
                            push_event(&mut state, "A scout bounty is posted to reveal the old ruins.");
                        }
                    }
                    ActionButtonKind::PostResourceBounty => {
                        if state.resource_bounty_active {
                            push_event(&mut state, "A resource expedition is already underway.");
                        } else {
                            state.resource_bounty_active = true;
                            state.resource_bounty_posted_day = Some(state.day);
                            push_event(&mut state, "A resource bounty is posted on the frontier node.");
                        }
                    }
                    ActionButtonKind::PostDefenseBounty => {
                        if state.defense_bounty_active {
                            push_event(&mut state, "A defense bounty is already active around the town core.");
                        } else {
                            state.defense_bounty_active = true;
                            state.defense_bounty_posted_day = Some(state.day);
                            push_event(&mut state, "A defense bounty calls heroes to guard the kingdom perimeter.");
                        }
                    }
                    ActionButtonKind::PostNextBounty => {
                        let mut posted = None;
                        for mut zone in zone_query.iter_mut() {
                            if !zone.cleared && !zone.active_bounty {
                                zone.active_bounty = true;
                                zone.bounty_posted_day = Some(state.day);
                                posted = Some((zone.name.clone(), zone.reward_gold * zone.tier as i32));
                                break;
                            }
                        }
                        if let Some((name, reward)) = posted {
                            push_event(
                                &mut state,
                                format!("A {}g bounty was posted on {}.", reward, name).as_str(),
                            );
                        } else {
                            push_event(&mut state, "No uncleared threat is available for a new bounty.");
                        }
                    }
                    ActionButtonKind::NewEra => {
                        state.new_era_requested = true;
                    }
                    ActionButtonKind::SpeedNormal => {
                        state.speed = 1.0;
                        state.paused = false;
                        push_event(&mut state, "Time speed set to 1x.");
                    }
                    ActionButtonKind::SpeedFast => {
                        state.speed = 2.0;
                        state.paused = false;
                        push_event(&mut state, "Time speed set to 2x.");
                    }
                    ActionButtonKind::Pause => {
                        state.paused = !state.paused;
                        let message = if state.paused {
                            "Kingdom paused."
                        } else {
                            "Kingdom resumed."
                        };
                        push_event(&mut state, message);
                    }
                }
            }
            Interaction::Hovered => {
                *color = UiColor(Color::rgb(0.58, 0.42, 0.22));
            }
            Interaction::None => {
                *color = UiColor(Color::rgb(0.42, 0.31, 0.18));
            }
        }
    }
}

fn update_clock(time: Res<Time>, mut state: ResMut<GameState>) {
    if state.paused || state.era_complete || state.kingdom_fallen {
        return;
    }

    state.day_timer += time.delta_seconds() * state.speed;
    if state.day_timer >= DAY_LENGTH_SECONDS {
        state.day_timer -= DAY_LENGTH_SECONDS;
        state.day += 1;
        recalculate_income(&mut state);
        let mut income = state.income_per_day;
        if state.resource_income_days > 0 {
            income += 18;
            state.resource_income_days -= 1;
        }
        if state.has_market && state.day % 3 == 0 {
            income += 30 + state.kingdom_rank as i32 * 10 + state.market_tier as i32 * 8;
            push_event(&mut state, "A merchant caravan pays rich trade fees to the market.");
        }
        if state.defense_days_remaining > 0 {
            state.defense_days_remaining -= 1;
            if state.defense_days_remaining == 0 {
                state.defense_bounty_active = false;
            }
        }
        state.gold += income;
        if state.day == 7 {
            push_event(
                &mut state,
                "Day 7 arrives. The frontier wakes up, and new monster dens begin to appear.",
            );
        }
        push_event(
            &mut state,
            format!("A new day begins. The treasury collects {} gold.", income).as_str(),
        );
    }
}

fn scheduled_threats_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    catalog: Res<ArtCatalog>,
    mut state: ResMut<GameState>,
    mut zone_query: Query<&mut MonsterZone>,
) {
    if state.era_complete || state.kingdom_fallen {
        return;
    }

    if state.day >= 7 && !state.spawned_bandits {
        spawn_monster_zone(&mut commands, &asset_server, &catalog, ZoneKind::BanditDen);
        state.spawned_bandits = true;
        push_event(&mut state, "A Bandit Den forms at the frontier.");
    }
    if state.day >= 14 && !state.spawned_trolls {
        spawn_monster_zone(&mut commands, &asset_server, &catalog, ZoneKind::TrollLair);
        state.spawned_trolls = true;
        push_event(&mut state, "A Troll Lair is discovered in the hills.");
    }
    if state.day >= 21 && !state.spawned_shadows {
        spawn_monster_zone(&mut commands, &asset_server, &catalog, ZoneKind::ShadowRift);
        state.spawned_shadows = true;
        push_event(&mut state, "A Shadow Rift opens and dark creatures seep into the realm.");
    }
    if state.day >= 30 && !state.spawned_dungeon_lord {
        spawn_monster_zone(&mut commands, &asset_server, &catalog, ZoneKind::DungeonLord);
        state.spawned_dungeon_lord = true;
        push_event(&mut state, "The Dungeon Lord launches the final siege of this era.");
    }

    for mut zone in zone_query.iter_mut() {
        if zone.cleared || state.day < 7 {
            continue;
        }
        if state.day >= zone.last_escalation_day + 7 {
            zone.last_escalation_day = state.day;
            zone.tier += 1;
            zone.max_hp *= 1.5;
            zone.hp = zone.max_hp;
            zone.reward_gold += 40;
            zone.reward_xp += 35;
            zone.danger += 0.5;
            push_event(
                &mut state,
                format!(
                    "{} escalates to tier {} and sends tougher raiders.",
                    zone.name, zone.tier
                )
                .as_str(),
            );
        }
    }
}

fn kingdom_growth_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    catalog: Res<ArtCatalog>,
    mut state: ResMut<GameState>,
    hero_query: Query<Entity, With<Hero>>,
    civilian_query: Query<Entity, With<Civilian>>,
    mut last_processed_day: Local<u32>,
) {
    if state.era_complete || state.kingdom_fallen {
        return;
    }

    if *last_processed_day == state.day {
        return;
    }
    *last_processed_day = state.day;

    let hero_count = hero_query.iter().count() as u32;
    let civilian_count = civilian_query.iter().count() as u32;
    state.civilians = civilian_count;

    if hero_count >= state.hero_capacity {
        spawn_building(
            &mut commands,
            &asset_server,
            &catalog,
            BuildingKind::House,
            house_pos(state.houses),
            BuildingKind::House.label(),
        );
        state.houses += 1;
        state.hero_capacity += 2;
        push_event(&mut state, "New hero houses rise as the kingdom's security improves.");
    }

    let desired_civilians = (hero_count * 3).max(2);
    if state.civilians < desired_civilians && state.civilians < state.houses * 3 {
        let role = next_civilian_role(&state);
        spawn_civilian(&mut commands, &asset_server, &catalog, role, state.civilians);
        state.civilians += 1;
        push_event(
            &mut state,
            format!("A {} settles in town, strengthening the local economy.", role.name()).as_str(),
        );
    }

    if state.civilians > state.farms * 3 {
        spawn_building(
            &mut commands,
            &asset_server,
            &catalog,
            BuildingKind::Farm,
            farm_pos(state.farms),
            BuildingKind::Farm.label(),
        );
        state.farms += 1;
        push_event(&mut state, "Settlers clear a new field and start farming.");
    }

    state.kingdom_rank = compute_rank(&state);
    recalculate_income(&mut state);
}

fn apply_recovery_payments_system(
    mut commands: Commands,
    mut hero_query: Query<(Entity, &mut Hero), With<RecoveryPayment>>,
) {
    for (entity, mut hero) in hero_query.iter_mut() {
        hero.recovery_bounty_paid = true;
        commands.entity(entity).remove::<RecoveryPayment>();
    }
}

fn merchant_event_system(
    mut commands: Commands,
    catalog: Res<ArtCatalog>,
    mut state: ResMut<GameState>,
    merchant_query: Query<Entity, With<Merchant>>,
    mut last_processed_day: Local<u32>,
) {
    if state.era_complete || state.kingdom_fallen || *last_processed_day == state.day {
        return;
    }
    *last_processed_day = state.day;

    if state.has_market && state.day % 3 == 0 && merchant_query.iter().next().is_none() {
        spawn_merchant(&mut commands, &catalog);
        push_event(
            &mut state,
            "A merchant caravan arrives at the frontier. Post an escort bounty to bring it in safely.",
        );
    }
}

fn merchant_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut state: ResMut<GameState>,
    map_state: Res<MapState>,
    hero_query: Query<&Transform, With<Hero>>,
    mut merchant_query: Query<(Entity, &mut Merchant, &mut Transform)>,
) {
    if state.paused || state.era_complete || state.kingdom_fallen {
        return;
    }

    for (entity, mut merchant, mut transform) in merchant_query.iter_mut() {
        if state.merchant_bounty_active {
            merchant.escorted = hero_query
                .iter()
                .any(|hero_transform| hero_transform.translation.distance(transform.translation) < 42.0);
        }

        let target = if merchant.escorted { market_pos() } else { defend_pos() };
        let step = if merchant.escorted { 24.0 } else { 10.0 }
            * movement_speed_bonus(&map_state, transform.translation)
            * state.speed
            * time.delta_seconds();
        let direction = target - transform.translation;
        let distance = direction.length();
        if distance > 1.0 {
            transform.translation += direction.normalize() * step.min(distance);
        }

        if merchant.escorted && transform.translation.distance(market_pos()) < 18.0 {
            let payout = merchant.reward_gold + state.market_tier as i32 * 12;
            state.gold += payout;
            state.merchant_bounty_active = false;
            state.merchant_bounty_posted_day = None;
            push_event(
                &mut state,
                format!(
                    "The escorted merchant reaches the Market and pays {} gold in trade fees.",
                    payout
                )
                .as_str(),
            );
            commands.entity(entity).despawn_recursive();
        } else if !merchant.escorted && transform.translation.distance(defend_pos()) < 14.0 {
            push_event(
                &mut state,
                "The merchant loses nerve and turns away at the perimeter. The escort chance is lost.",
            );
            state.merchant_bounty_active = false;
            state.merchant_bounty_posted_day = None;
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn daily_realm_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    catalog: Res<ArtCatalog>,
    mut state: ResMut<GameState>,
    mut zone_query: Query<&mut MonsterZone>,
    mut hero_query: Query<(Entity, &mut Hero)>,
    mut last_processed_day: Local<u32>,
) {
    if state.era_complete || state.kingdom_fallen || *last_processed_day == state.day {
        return;
    }
    *last_processed_day = state.day;

    let mut events = Vec::new();

    for mut zone in zone_query.iter_mut() {
        if zone.active_bounty {
            if let Some(posted_day) = zone.bounty_posted_day {
                if state.day >= posted_day + 5 {
                    zone.active_bounty = false;
                    zone.bounty_posted_day = None;
                    events.push(format!("The bounty on {} expires after going unanswered.", zone.name));
                }
            }
        }
    }

    if state.ruins_bounty_active {
        if let Some(posted_day) = state.ruins_bounty_posted_day {
            if state.day >= posted_day + 5 {
                state.ruins_bounty_active = false;
                state.ruins_bounty_posted_day = None;
                events.push("The scout bounty on the ruins expires.".to_string());
            }
        }
    }
    if state.resource_bounty_active {
        if let Some(posted_day) = state.resource_bounty_posted_day {
            if state.day >= posted_day + 5 {
                state.resource_bounty_active = false;
                state.resource_bounty_posted_day = None;
                events.push("The resource expedition bounty expires.".to_string());
            }
        }
    }
    if state.defense_bounty_active {
        if let Some(posted_day) = state.defense_bounty_posted_day {
            if state.day >= posted_day + 5 {
                state.defense_bounty_active = false;
                state.defense_bounty_posted_day = None;
                events.push("The defense bounty around town expires.".to_string());
            }
        }
    }
    if state.merchant_bounty_active {
        if let Some(posted_day) = state.merchant_bounty_posted_day {
            if state.day >= posted_day + 5 {
                state.merchant_bounty_active = false;
                state.merchant_bounty_posted_day = None;
                events.push("The merchant escort bounty expires.".to_string());
            }
        }
    }

    let has_active_work = zone_query.iter_mut().any(|zone| zone.active_bounty)
        || state.ruins_bounty_active
        || state.resource_bounty_active
        || state.defense_bounty_active
        || state.merchant_bounty_active;

    let mut hero_count = hero_query.iter_mut().count() as u32;
    for (entity, mut hero) in hero_query.iter_mut() {
        if hero.task == HeroTask::Incapacitated && state.day >= hero.incapacitated_until_day {
            if hero.recovery_bounty_paid {
                hero.task = HeroTask::RecoverAtInn;
                hero.incapacitated_until_day = 0;
                hero.recovery_bounty_due = 0;
                hero.recovery_bounty_paid = false;
                hero.hp = (hero.max_hp * 0.7).min(hero.max_hp);
                hero.morale = hero.morale.max(60.0);
                events.push(format!("{} returns from recovery and can fight again.", hero.class.name()));
            } else {
                hero_count = hero_count.saturating_sub(1);
                commands.entity(entity).despawn_recursive();
                events.push(format!(
                    "{} is lost because the kingdom failed to fund recovery in time.",
                    hero.class.name()
                ));
                continue;
            }
        }

        if has_active_work {
            hero.days_without_work = 0;
        } else {
            hero.days_without_work += 1;
            if hero.days_without_work >= 3 && hero_count > 1 {
                hero_count -= 1;
                commands.entity(entity).despawn_recursive();
                events.push(format!(
                    "{} leaves the realm after three quiet days without worthwhile work.",
                    hero.class.name()
                ));
            }
        }
    }

    if state.has_inn && hero_count < state.hero_capacity && state.day % 2 == 0 {
        let class = if state.has_wizard_tower && state.day % 6 == 0 {
            HeroClass::Mage
        } else if state.has_temple && state.day % 5 == 0 {
            HeroClass::Healer
        } else if state.day % 3 == 0 {
            HeroClass::Archer
        } else {
            HeroClass::Warrior
        };
        spawn_hero(
            &mut commands,
            &asset_server,
            &catalog,
            class,
            town_hall_pos() + Vec3::new(50.0 + hero_count as f32 * 18.0, -20.0, 5.0),
        );
        events.push(format!("A {} answers the call of the realm and settles in town.", class.name()));
    }

    for event in events {
        push_event(&mut state, &event);
    }
}

fn zone_pressure_system(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<GameState>,
    mut zone_query: Query<&mut MonsterZone>,
    building_query: Query<(Entity, &Building)>,
) {
    if state.paused || state.day < 7 || state.era_complete || state.kingdom_fallen {
        return;
    }

    let is_night = state.day_timer / DAY_LENGTH_SECONDS >= 0.5;
    if !is_night {
        return;
    }

    for mut zone in zone_query.iter_mut() {
        if zone.cleared {
            continue;
        }

        zone.raid_timer += time.delta_seconds() * state.speed;
        if zone.raid_timer >= (18.0 - zone.tier as f32 * 1.5).max(7.0) {
            zone.raid_timer = 0.0;
            let mut pressure = match zone.kind {
                ZoneKind::GoblinCamp => 4,
                ZoneKind::BanditDen => 7,
                ZoneKind::TrollLair => 10,
                ZoneKind::ShadowRift => 9,
                ZoneKind::DungeonLord => 18,
            } * zone.tier as i32;
            if state.has_guard_tower {
                pressure = (pressure - (3 + state.tower_tier as i32 * 2)).max(1);
            }
            if state.defense_days_remaining > 0 {
                pressure = (pressure - 4).max(0);
            }
            let stolen_gold = pressure.min(state.gold);
            state.gold -= stolen_gold;
            let overflow = pressure - stolen_gold;
            let direct_damage = (zone.tier as f32 * 2.0).max(1.0);
            state.castle_hp = (state.castle_hp - direct_damage).max(0.0);
            if overflow > 0 {
                state.castle_hp = (state.castle_hp - overflow as f32 * 4.0).max(0.0);
            }
            push_event(
                &mut state,
                format!(
                    "{} sends raiders in the dark, draining {} gold and {} castle damage.",
                    zone.name,
                    stolen_gold,
                    overflow * 4 + direct_damage as i32
                )
                .as_str(),
            );

            if overflow >= 2 {
                let target_kind = if matches!(zone.kind, ZoneKind::TrollLair | ZoneKind::DungeonLord) {
                    BuildingKind::House
                } else {
                    BuildingKind::Farm
                };

                for (entity, building) in building_query.iter() {
                    if building.kind != target_kind {
                        continue;
                    }

                    if target_kind == BuildingKind::House && state.houses <= 1 {
                        continue;
                    }
                    if target_kind == BuildingKind::Farm && state.farms == 0 {
                        continue;
                    }

                    commands.entity(entity).despawn_recursive();
                    if target_kind == BuildingKind::House {
                        state.houses = state.houses.saturating_sub(1);
                        state.hero_capacity = state.hero_capacity.saturating_sub(2);
                        push_event(
                            &mut state,
                            "Raiders burn down a hero house on the edge of town.",
                        );
                    } else {
                        state.farms = state.farms.saturating_sub(1);
                        push_event(&mut state, "Raiders torch one of the kingdom's outer fields.");
                    }
                    recalculate_income(&mut state);
                    break;
                }
            }
            if state.castle_hp <= 0.0 {
                state.kingdom_fallen = true;
                state.paused = true;
                push_event(&mut state, "The King's Castle falls. This era is lost.");
                break;
            }
        }
    }
}

fn tower_attack_system(
    time: Res<Time>,
    mut state: ResMut<GameState>,
    tower_query: Query<(&Building, &Transform)>,
    mut zone_query: Query<(&mut MonsterZone, &Transform, &mut TextureAtlasSprite)>,
) {
    if state.paused || state.era_complete || state.kingdom_fallen || !state.has_guard_tower {
        return;
    }

    let tower_positions: Vec<Vec3> = tower_query
        .iter()
        .filter_map(|(building, transform)| {
            if building.kind == BuildingKind::GuardTower {
                Some(transform.translation)
            } else {
                None
            }
        })
        .collect();

    if tower_positions.is_empty() {
        return;
    }

    let range = 340.0 + state.tower_tier as f32 * 20.0;
    let dps = 10.0 + state.tower_tier as f32 * 8.0;
    let mut tower_kills = Vec::new();

    for (mut zone, zone_transform, mut sprite) in zone_query.iter_mut() {
        if zone.cleared {
            continue;
        }

        let in_range = tower_positions
            .iter()
            .any(|tower_pos| tower_pos.distance(zone_transform.translation) <= range);
        if !in_range {
            continue;
        }

        zone.hp -= dps * time.delta_seconds() * state.speed;
        let health_ratio = (zone.hp / zone.max_hp).clamp(0.15, 1.0);
        sprite.color = Color::rgb(1.0, health_ratio, health_ratio);

        if zone.hp <= 0.0 {
            zone.cleared = true;
            zone.active_bounty = false;
            zone.bounty_posted_day = None;
            state.gold += (zone.reward_gold / 4).max(15);
            tower_kills.push(zone.name.clone());
            if zone.kind == ZoneKind::DungeonLord {
                state.era_complete = true;
                state.legacy_points += 10 + state.kingdom_rank;
            }
        }
    }

    for zone_name in tower_kills {
        push_event(
            &mut state,
            format!("The Guard Tower tears down {} before it can raid the town.", zone_name).as_str(),
        );
    }
}

fn new_era_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    catalog: Res<ArtCatalog>,
    mut state: ResMut<GameState>,
    mut map_state: ResMut<MapState>,
    mut placement: ResMut<PlacementState>,
    world_query: Query<Entity, With<WorldEntity>>,
) {
    if !state.new_era_requested {
        return;
    }

    let legacy_points = state.legacy_points;
    let next_era = state.era + 1;

    for entity in world_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    *state = GameState::default();
    *map_state = create_map_state();
    *placement = PlacementState::default();
    state.era = next_era;
    state.legacy_points = legacy_points;
    state.gold += legacy_points as i32 * 15;
    state.castle_max_hp += legacy_points as f32 * 6.0;
    state.castle_hp = state.castle_max_hp;
    recalculate_income(&mut state);
    spawn_world_entities(&mut commands, &asset_server, &catalog);
    let era_number = state.era;
    let bonus_gold = legacy_points as i32 * 15;
    push_event(
        &mut state,
        format!(
            "Era {} begins. Legacy points grant {} bonus starting gold.",
            era_number,
            bonus_gold
        )
        .as_str(),
    );
}

fn hero_ai_system(
    time: Res<Time>,
    state: Res<GameState>,
    zone_query: Query<(&Transform, &MonsterZone)>,
    merchant_query: Query<&Transform, With<Merchant>>,
    mut hero_query: Query<(&mut Hero, &mut MoveTarget, &Transform)>,
) {
    if state.paused || state.era_complete || state.kingdom_fallen {
        return;
    }

    for (mut hero, mut target, transform) in hero_query.iter_mut() {
        hero.decision_timer.tick(time.delta());
        hero.ability_cooldown = (hero.ability_cooldown - time.delta_seconds() * state.speed).max(0.0);
        if !hero.decision_timer.finished() {
            continue;
        }

        if hero.task == HeroTask::Incapacitated || state.day < hero.incapacitated_until_day {
            hero.task = HeroTask::Incapacitated;
            target.0 = house_pos(0);
            continue;
        }

        let is_night = state.day_timer / DAY_LENGTH_SECONDS >= 0.5;
        hero.morale = (hero.morale - if is_night { 2.0 } else { 0.8 }).max(10.0);

        if hero.hp < hero.max_hp * 0.5 && state.has_inn {
            hero.task = HeroTask::RecoverAtInn;
            target.0 = inn_pos();
            continue;
        }

        if hero.morale < 45.0 && state.has_temple {
            hero.task = HeroTask::PrayAtTemple;
            target.0 = town_hall_pos() + Vec3::new(20.0, 120.0, 5.0);
            continue;
        }

        let mut best_score = 0.0;
        let mut best_target = None;
        for (zone_transform, zone) in zone_query.iter() {
            if zone.cleared || !zone.active_bounty {
                continue;
            }

            let distance = transform.translation.distance(zone_transform.translation).max(25.0);
            let tolerance = hero.level as f32 * 1.5
                + hero.class.bravery() * 2.0
                + hero.personality.tolerance_bonus()
                + if hero.legendary { 1.0 } else { 0.0 };
            if zone.danger > tolerance {
                continue;
            }

            let mut score =
                (zone.reward_gold * zone.tier as i32) as f32 / (distance * zone.danger);
            score *= hero.personality.bounty_multiplier();
            if hero.preferred_zone == Some(zone.kind) {
                score *= 1.2;
            }
            score *= match hero.guild {
                HeroGuild::Fighters => {
                    if matches!(zone.kind, ZoneKind::GoblinCamp | ZoneKind::BanditDen | ZoneKind::TrollLair)
                    {
                        1.15
                    } else {
                        1.0
                    }
                }
                HeroGuild::Rangers => {
                    if matches!(zone.kind, ZoneKind::GoblinCamp | ZoneKind::BanditDen) {
                        1.18
                    } else {
                        0.95
                    }
                }
                HeroGuild::Arcane => {
                    if zone.tier >= 2 {
                        1.2
                    } else {
                        1.0
                    }
                }
                HeroGuild::Faith => 0.95,
                HeroGuild::FreeRoaming => 1.0,
            };
            if score > best_score {
                best_score = score;
                best_target = Some(zone_transform.translation);
            }
        }

        if let Some(best_target) = best_target {
            hero.task = HeroTask::ClaimBounty;
            target.0 = best_target;
            continue;
        }

        if state.ruins_bounty_active && !state.ruins_revealed {
            hero.task = HeroTask::ExploreRuins;
            target.0 = ruins_pos();
            continue;
        }

        if state.resource_bounty_active {
            if hero.guild == HeroGuild::Faith && hero.personality == HeroPersonality::Dutiful {
                continue;
            }
            hero.task = HeroTask::GatherResource;
            target.0 = resource_node_pos();
            continue;
        }

        if state.merchant_bounty_active {
            if let Some(merchant_transform) = merchant_query.iter().next() {
                hero.task = HeroTask::EscortMerchant;
                target.0 = merchant_transform.translation;
                continue;
            }
        }

        if state.defense_bounty_active {
            if matches!(hero.guild, HeroGuild::Fighters | HeroGuild::Faith) {
                hero.morale = (hero.morale + 4.0).min(100.0);
            }
            hero.task = HeroTask::DefendTown;
            target.0 = defend_pos();
            continue;
        }

        if is_night && hero.morale < 60.0 && state.has_inn {
            hero.task = HeroTask::LayLow;
            target.0 = inn_pos();
            continue;
        }

        hero.task = HeroTask::Patrol;
        if transform.translation.distance(patrol_pos()) < 20.0 {
            target.0 = outer_patrol_pos();
        } else {
            target.0 = patrol_pos();
        }
    }
}

fn hero_movement_system(
    time: Res<Time>,
    state: Res<GameState>,
    map_state: Res<MapState>,
    mut hero_query: Query<(&Hero, &MoveTarget, &mut Transform)>,
) {
    if state.paused || state.era_complete || state.kingdom_fallen {
        return;
    }

    for (hero, target, mut transform) in hero_query.iter_mut() {
        if hero.task == HeroTask::Incapacitated {
            continue;
        }
        let step = hero.speed
            * movement_speed_bonus(&map_state, transform.translation)
            * state.speed
            * time.delta_seconds();
        let direction = target.0 - transform.translation;
        let distance = direction.length();
        if distance > 1.0 {
            transform.translation += direction.normalize() * step.min(distance);
        }
    }
}

fn hero_bounty_resolution_system(
    time: Res<Time>,
    mut state: ResMut<GameState>,
    mut zone_query: Query<(&mut MonsterZone, &Transform, &mut TextureAtlasSprite)>,
    mut hero_query: Query<(&mut Hero, &Transform, &mut MoveTarget)>,
) {
    if state.paused || state.era_complete || state.kingdom_fallen {
        return;
    }

    for (mut hero, hero_transform, mut move_target) in hero_query.iter_mut() {
        if hero.task == HeroTask::Incapacitated || state.day < hero.incapacitated_until_day {
            continue;
        }

        if matches!(hero.task, HeroTask::RecoverAtInn | HeroTask::LayLow)
            && hero_transform.translation.distance(inn_pos()) < 16.0
        {
            let heal_rate = (if state.has_alchemist { 24.0 } else { 18.0 }) + state.inn_tier as f32 * 4.0;
            hero.hp = (hero.hp + heal_rate * time.delta_seconds() * state.speed).min(hero.max_hp);
            hero.morale = (hero.morale + 12.0 * time.delta_seconds() * state.speed).min(100.0);
            if hero.hp >= hero.max_hp * 0.9 && hero.morale >= 72.0 {
                hero.task = HeroTask::Patrol;
                move_target.0 = patrol_pos();
            }
        }

        if hero.task == HeroTask::PrayAtTemple
            && hero_transform
                .translation
                .distance(town_hall_pos() + Vec3::new(20.0, 120.0, 5.0))
                < 16.0
        {
            hero.morale = (hero.morale + (18.0 + state.temple_tier as f32 * 4.0) * time.delta_seconds() * state.speed).min(100.0);
            hero.hp = (hero.hp + 8.0 * time.delta_seconds() * state.speed).min(hero.max_hp);
            if hero.morale >= 80.0 {
                hero.task = HeroTask::Patrol;
                move_target.0 = outer_patrol_pos();
            }
        }

        if hero.task == HeroTask::ExploreRuins && hero_transform.translation.distance(ruins_pos()) < 20.0 {
            state.ruins_bounty_active = false;
            state.ruins_bounty_posted_day = None;
            state.ruins_revealed = true;
            state.revealed_sectors += 2;
            hero.xp += 45;
            hero.personal_gold += 60;
            hero.days_without_work = 0;
            state.gold = (state.gold - 60).max(0) + 3;
            update_hero_progression(&mut hero);
            push_event(&mut state, "A hero explores the ruins and reveals more of the realm.");
            hero.task = HeroTask::Patrol;
            move_target.0 = outer_patrol_pos();
        }

        if hero.task == HeroTask::GatherResource && hero_transform.translation.distance(resource_node_pos()) < 20.0 {
            state.resource_bounty_active = false;
            state.resource_bounty_posted_day = None;
            state.resource_income_days = 3 + state.town_hall_tier;
            hero.xp += 35;
            hero.personal_gold += 45;
            hero.days_without_work = 0;
            state.gold = (state.gold - 45).max(0) + 2;
            update_hero_progression(&mut hero);
            push_event(&mut state, "A resource convoy returns and boosts passive income for several days.");
            hero.task = HeroTask::Patrol;
            move_target.0 = patrol_pos();
        }

        if hero.task == HeroTask::DefendTown && hero_transform.translation.distance(defend_pos()) < 22.0 {
            state.defense_bounty_active = false;
            state.defense_bounty_posted_day = None;
            state.defense_days_remaining = 2 + state.town_hall_tier;
            hero.xp += 30;
            hero.personal_gold += 50;
            hero.morale = (hero.morale + 20.0).min(100.0);
            hero.days_without_work = 0;
            state.gold = (state.gold - 50).max(0) + 2;
            update_hero_progression(&mut hero);
            push_event(&mut state, "A hero fortifies the town perimeter under a defense bounty.");
            hero.task = HeroTask::Patrol;
            move_target.0 = defend_pos();
        }

        if hero.task != HeroTask::ClaimBounty {
            continue;
        }

        for (mut zone, zone_transform, mut sprite) in zone_query.iter_mut() {
            if zone.cleared || !zone.active_bounty {
                continue;
            }

            if hero_transform.translation.distance(zone_transform.translation) < 26.0 {
                let blacksmith_bonus = if state.has_blacksmith { 1.15 } else { 1.0 };
                let mut outgoing_damage =
                    hero.attack * blacksmith_bonus * time.delta_seconds() * state.speed;
                let mut incoming_damage = zone.danger * zone.tier as f32 * 1.4 * time.delta_seconds();

                if hero.ability_cooldown <= 0.0 {
                    match hero.class {
                        HeroClass::Warrior => {
                            incoming_damage *= 0.55;
                            hero.morale = (hero.morale + 6.0).min(100.0);
                            hero.ability_cooldown = 14.0;
                        }
                        HeroClass::Archer => {
                            outgoing_damage *= 2.0;
                            hero.ability_cooldown = 12.0;
                        }
                        HeroClass::Mage => {
                            outgoing_damage *= 2.6;
                            hero.ability_cooldown = 16.0;
                        }
                        HeroClass::Rogue => {
                            if zone.hp >= zone.max_hp * 0.75 {
                                outgoing_damage *= 2.8;
                            } else {
                                outgoing_damage *= 1.4;
                            }
                            hero.ability_cooldown = 10.0;
                        }
                        HeroClass::Healer => {
                            hero.hp = (hero.hp + 12.0).min(hero.max_hp);
                            hero.morale = (hero.morale + 8.0).min(100.0);
                            outgoing_damage *= 1.15;
                            hero.ability_cooldown = 15.0;
                        }
                    }
                }

                if hero.legendary {
                    outgoing_damage *= 1.2;
                    incoming_damage *= 0.85;
                }

                zone.hp -= outgoing_damage;
                hero.hp -= incoming_damage;
                let health_ratio = (zone.hp / zone.max_hp).clamp(0.2, 1.0);
                sprite.color = Color::rgb(1.0, health_ratio, health_ratio);

                if hero.hp <= 0.0 {
                    hero.task = HeroTask::Incapacitated;
                    hero.incapacitated_until_day = state.day + 1;
                    hero.recovery_bounty_due = 45 + hero.level as i32 * 8;
                    hero.recovery_bounty_paid = false;
                    hero.hp = 0.0;
                    hero.morale = (hero.morale - 25.0).max(20.0);
                    move_target.0 = house_pos(0);
                    push_event(
                        &mut state,
                        format!(
                            "{} is incapacitated while fighting {} and needs a day to recover.",
                            hero.class.name(),
                            zone.name
                        )
                        .as_str(),
                    );
                    break;
                }

                if zone.hp <= 0.0 {
                    zone.cleared = true;
                    zone.active_bounty = false;
                    zone.bounty_posted_day = None;
                    hero.xp += zone.reward_xp * zone.tier;
                    let scaled_reward = zone.reward_gold * zone.tier as i32;
                    hero.personal_gold += scaled_reward;
                    hero.days_without_work = 0;
                    hero.preferred_zone = Some(zone.kind);
                    state.gold = (state.gold - scaled_reward).max(0);
                    state.gold += ((scaled_reward as f32) * 0.05) as i32;
                    update_hero_progression(&mut hero);
                    push_event(
                        &mut state,
                        format!(
                            "{} cleared {} and earned {} gold.",
                            hero.class.name(),
                            zone.name,
                            scaled_reward
                        )
                        .as_str(),
                    );
                    if zone.kind == ZoneKind::DungeonLord {
                        state.era_complete = true;
                        state.legacy_points += 10 + state.kingdom_rank + state.houses / 2;
                        push_event(&mut state, "The Dungeon Lord falls and the era is won.");
                    }
                    move_target.0 = patrol_pos();
                    hero.task = HeroTask::Patrol;
                }
            }
        }
    }
}

fn hero_service_economy_system(
    time: Res<Time>,
    mut state: ResMut<GameState>,
    mut hero_query: Query<(&mut Hero, &Transform)>,
) {
    if state.paused || state.era_complete || state.kingdom_fallen {
        return;
    }

    for (mut hero, transform) in hero_query.iter_mut() {
        if hero.task == HeroTask::Incapacitated || state.day < hero.incapacitated_until_day {
            continue;
        }
        hero.service_timer.tick(time.delta());
        if !hero.service_timer.finished() {
            continue;
        }

        let mut charge = 0;
        if state.has_inn && transform.translation.distance(inn_pos()) < 36.0 {
            charge += 4;
        }
        if state.has_market {
            charge += 2;
        }
        if state.has_blacksmith && hero.level >= 2 {
            charge += 3;
        }
        if state.has_alchemist && hero.hp < hero.max_hp * 0.85 {
            charge += 3;
            hero.hp = (hero.hp + 5.0).min(hero.max_hp);
        }
        if state.has_temple && hero.morale < 75.0 {
            charge += 2;
        }

        let actual = charge.min(hero.personal_gold);
        hero.personal_gold -= actual;
        state.gold += actual;
    }
}

fn civilian_ai_system(
    time: Res<Time>,
    state: Res<GameState>,
    mut civilian_query: Query<(&mut Civilian, &mut CivilianTarget)>,
) {
    if state.paused || state.era_complete || state.kingdom_fallen {
        return;
    }

    for (mut civilian, mut target) in civilian_query.iter_mut() {
        civilian.decision_timer.tick(time.delta());
        if !civilian.decision_timer.finished() {
            continue;
        }

        civilian.at_work = !civilian.at_work;
        target.0 = if civilian.at_work {
            civilian_work_pos(civilian.role)
        } else {
            house_pos(civilian.home_index / 3)
        };
    }
}

fn civilian_movement_system(
    time: Res<Time>,
    state: Res<GameState>,
    map_state: Res<MapState>,
    mut civilian_query: Query<(&Civilian, &CivilianTarget, &mut Transform)>,
) {
    if state.paused || state.era_complete || state.kingdom_fallen {
        return;
    }

    for (civilian, target, mut transform) in civilian_query.iter_mut() {
        let step = civilian.speed
            * movement_speed_bonus(&map_state, transform.translation)
            * state.speed
            * time.delta_seconds();
        let direction = target.0 - transform.translation;
        let distance = direction.length();
        if distance > 1.0 {
            transform.translation += direction.normalize() * step.min(distance);
        }
    }
}

fn update_hero_labels_system(
    state: Res<GameState>,
    hero_query: Query<&Hero>,
    mut hero_label_query: Query<(&Parent, &mut Text), With<HeroStatusLabel>>,
) {
    for (parent, mut text) in hero_label_query.iter_mut() {
        if let Ok(hero) = hero_query.get(parent.0) {
            text.sections[0].value = format!(
                "{}{} L{}\n{} | HP {}{}",
                if hero.legendary { "Legendary " } else { "" },
                hero.class.name(),
                hero.level,
                hero_task_name(hero.task),
                hero.hp as i32,
                if hero.task == HeroTask::Incapacitated && state.day < hero.incapacitated_until_day {
                    format!(" | Due {}g | Day {}", hero.recovery_bounty_due, hero.incapacitated_until_day)
                } else {
                    String::new()
                }
            );
        }
    }
}

fn update_zone_labels_system(
    zone_query: Query<&MonsterZone>,
    mut zone_label_query: Query<(&Parent, &mut Text), With<ZoneStatusLabel>>,
) {
    for (parent, mut text) in zone_label_query.iter_mut() {
        if let Ok(zone) = zone_query.get(parent.0) {
            text.sections[0].value = format!(
                "{}\nHP {} | T{} | {}",
                zone.name,
                zone.hp.max(0.0) as i32,
                zone.tier,
                if zone.active_bounty { "Bounty" } else { "Idle" }
            );
        }
    }
}

fn update_zone_markers_system(
    zone_query: Query<&MonsterZone>,
    mut zone_marker_query: Query<(&Parent, &mut Sprite), With<ZoneTargetMarker>>,
) {
    for (parent, mut sprite) in zone_marker_query.iter_mut() {
        if let Ok(zone) = zone_query.get(parent.0) {
            sprite.color = if zone.active_bounty {
                Color::rgba(1.0, 1.0, 1.0, 0.95)
            } else {
                Color::rgba(1.0, 1.0, 1.0, 0.0)
            };
        }
    }
}

fn update_fog_of_war_system(
    state: Res<GameState>,
    mut fog_query: Query<(&FogSector, &mut Sprite)>,
) {
    for (fog, mut sprite) in fog_query.iter_mut() {
        let visible = state.revealed_sectors >= fog.required_sectors;
        sprite.color = if visible {
            Color::rgba(0.02, 0.05, 0.04, 0.0)
        } else {
            Color::rgba(0.02, 0.05, 0.04, 0.88)
        };
    }
}

fn update_day_night_overlay_system(
    state: Res<GameState>,
    mut overlay_query: Query<&mut Sprite, With<DayNightOverlay>>,
) {
    let cycle = state.day_timer / DAY_LENGTH_SECONDS;
    for mut sprite in overlay_query.iter_mut() {
        sprite.color = if cycle < 0.15 {
            Color::rgba(0.98, 0.84, 0.42, 0.08)
        } else if cycle < 0.5 {
            Color::rgba(1.0, 1.0, 1.0, 0.0)
        } else {
            let night_progress = ((cycle - 0.5) / 0.5).clamp(0.0, 1.0);
            Color::rgba(0.08, 0.16, 0.32, 0.18 + night_progress * 0.18)
        };
    }
}

fn update_ui_system(
    state: Res<GameState>,
    placement: Res<PlacementState>,
    hero_query: Query<&Hero>,
    civilian_query: Query<&Civilian>,
    zone_query: Query<&MonsterZone>,
    mut top_bar_query: Query<&mut Text, With<TopBarText>>,
    mut hint_query: Query<&mut Text, (With<HintText>, Without<TopBarText>)>,
    mut event_query: Query<&mut Text, (With<EventLogText>, Without<TopBarText>, Without<HintText>)>,
) {
    let heroes: Vec<&Hero> = hero_query.iter().collect();
    let hero_count = heroes.len();
    let civilian_count = civilian_query.iter().count();
    let active_zones: Vec<&MonsterZone> = zone_query.iter().filter(|zone| !zone.cleared).collect();
    let time_of_day = state.day_timer / DAY_LENGTH_SECONDS;
    let phase = if time_of_day < 0.5 { "Day" } else { "Night" };
    let phase_pct = ((time_of_day * 100.0) as i32).clamp(0, 100);

    if let Ok(mut text) = top_bar_query.get_single_mut() {
        text.sections[0].value = format!(
            "Era {}  |  Gold {}  |  Income/day {}  |  Castle {}/{}  |  Day {}  |  Rank {}  |  {} {}%  |  Heroes {}/{}  |  Civilians {}  |  Houses {}  |  Sectors {}  |  Legacy {}",
            state.era,
            state.gold,
            state.income_per_day,
            state.castle_hp.max(0.0) as i32,
            state.castle_max_hp as i32,
            state.day,
            rank_name(state.kingdom_rank),
            phase,
            phase_pct,
            hero_count,
            state.hero_capacity,
            civilian_count,
            state.houses,
            state.revealed_sectors,
            state.legacy_points
        );
    }

    if let Ok(mut text) = hint_query.get_single_mut() {
        text.sections[0].value = if let Some(kind) = placement.building {
            format!("Placing {}. Tap a valid grass tile inside the town zone.", kind.label())
        } else if placement.road_mode {
            "Road placement mode is active. Tap grass or forest to pave a route.".to_string()
        } else if placement.bridge_mode {
            "Bridge placement mode is active. Tap a river tile to open new passage.".to_string()
        } else if state.kingdom_fallen {
            "The kingdom has fallen. Press New Era to restart with your legacy points.".to_string()
        } else if state.era_complete {
            "The era is won. Press New Era to begin the next kingdom with legacy bonuses.".to_string()
        } else if heroes
            .iter()
            .any(|hero| hero.task == HeroTask::Incapacitated && !hero.recovery_bounty_paid)
        {
            "A hero is down. Use Fund Recovery before the due day or that hero is lost.".to_string()
        } else if !state.has_inn {
            "Use the bottom bar: Build Inn -> Recruit Hero -> Post Next Bounty. That is the first full kingdom loop.".to_string()
        } else if hero_count < 2 {
            "Hire another hero from the bottom bar. More security attracts settlers, houses, and taxes.".to_string()
        } else if state.day < 7 {
            "The frontier stays quiet until day 7. Upgrade buildings, post scout/resource bounties, and grow the town.".to_string()
        } else if state.castle_hp < state.castle_max_hp * 0.45 {
            "The castle is taking damage. Repair Keep buys time, but faster clears and stronger defenses matter more.".to_string()
        } else if state.gold < 100 {
            "Treasury is low. Lean on taxes, services, and quick threat clears before overbuilding.".to_string()
        } else if active_zones.iter().any(|zone| zone.active_bounty) {
            "Active bounties are pulling heroes outward. Keep healing, morale, and services ready.".to_string()
        } else if state.ruins_bounty_active || state.resource_bounty_active || state.defense_bounty_active || state.merchant_bounty_active {
            "Indirect objective bounties are active. Heroes will weigh them against danger and distance.".to_string()
        } else if !active_zones.is_empty() {
            "Post the next bounty to guide heroes toward the most urgent remaining threat.".to_string()
        } else if state.era_complete {
            "The Dungeon Lord has fallen. This era is complete.".to_string()
        } else {
            "The frontier is quiet again. Keep growing the kingdom before the next escalation.".to_string()
        };
    }

    if let Ok(mut text) = event_query.get_single_mut() {
        let mut lines = vec!["Realm Status".to_string()];
        for entry in state.events.iter().take(8) {
            lines.push(format!("- {}", entry));
        }
        lines.push("".to_string());
        lines.push("Hero Roster".to_string());
        for hero in heroes.iter().take(6) {
            lines.push(format!(
                "{}{} L{} | {} | HP {} | Morale {} | Purse {}g | {} {}",
                if hero.legendary { "Legendary " } else { "" },
                hero.class.name(),
                hero.level,
                hero_task_name(hero.task),
                hero.hp as i32,
                hero.morale as i32,
                hero.personal_gold,
                hero.personality.name(),
                hero.guild.name()
            ));
            if hero.task == HeroTask::Incapacitated {
                lines.push(format!(
                    "Recovery due: {}g | Paid: {} | Returns day {}",
                    hero.recovery_bounty_due,
                    if hero.recovery_bounty_paid { "Yes" } else { "No" },
                    hero.incapacitated_until_day
                ));
            }
        }
        lines.push("".to_string());
        lines.push("Objectives".to_string());
        lines.push(format!(
            "Monster bounty: {}",
            if active_zones.iter().any(|zone| zone.active_bounty) {
                "Active"
            } else {
                "Idle"
            }
        ));
        lines.push(format!(
            "Scout ruins: {}",
            if state.ruins_revealed {
                "Complete"
            } else if state.ruins_bounty_active {
                "Active"
            } else {
                "Available"
            }
        ));
        lines.push(format!(
            "Resource run: {}",
            if state.resource_bounty_active {
                "Active"
            } else if state.resource_income_days > 0 {
                "Paying Out"
            } else {
                "Available"
            }
        ));
        lines.push(format!(
            "Defense bounty: {}",
            if state.defense_bounty_active {
                "Active"
            } else if state.defense_days_remaining > 0 {
                "Guarded"
            } else {
                "Available"
            }
        ));
        lines.push(format!(
            "Escort merchant: {}",
            if state.merchant_bounty_active {
                "Active"
            } else {
                "Waiting"
            }
        ));
        lines.push("".to_string());
        lines.push("Threats".to_string());
        for zone in active_zones.iter().take(4) {
            lines.push(format!(
                "{} HP: {}/{}  |  Tier {}  |  Bounty {}",
                zone.name,
                zone.hp.max(0.0) as i32,
                zone.max_hp as i32,
                zone.tier,
                if zone.active_bounty { "Active" } else { "None" }
            ));
        }
        lines.push(format!(
            "Castle T{} | HP {}/{} | Inn T{} | Market T{} | Temple T{} | Defend {}d | Resource {}d",
            state.town_hall_tier,
            state.castle_hp.max(0.0) as i32,
            state.castle_max_hp as i32,
            state.inn_tier,
            state.market_tier,
            state.temple_tier,
            state.defense_days_remaining,
            state.resource_income_days
        ));
        text.sections[0].value = lines.join("\n");
    }
}

fn push_event(state: &mut ResMut<GameState>, message: &str) {
    state.events.insert(0, message.to_string());
    state.events.truncate(10);
}

fn building_sprite_spec(catalog: &ArtCatalog, kind: BuildingKind) -> BuildingSpriteSpec {
    match kind {
        BuildingKind::TownHall => catalog.building_townhall.clone(),
        BuildingKind::Inn => catalog.building_inn.clone(),
        BuildingKind::Market => catalog.building_market.clone(),
        BuildingKind::Temple => catalog.building_temple.clone(),
        BuildingKind::GuardTower => catalog.building_tower.clone(),
        BuildingKind::WizardTower => catalog.building_wizard.clone(),
        BuildingKind::Blacksmith => catalog.building_blacksmith.clone(),
        BuildingKind::Alchemist => catalog.building_alchemist.clone(),
        BuildingKind::Barracks => catalog.building_barracks.clone(),
        BuildingKind::House => catalog.building_house.clone(),
        BuildingKind::Farm => catalog.building_farm.clone(),
    }
}

fn hero_sprite_spec(catalog: &ArtCatalog, class: HeroClass) -> UnitSpriteSpec {
    match class {
        HeroClass::Warrior => catalog.warrior.clone(),
        HeroClass::Archer => catalog.archer.clone(),
        HeroClass::Mage => catalog.mage.clone(),
        HeroClass::Rogue => catalog.rogue.clone(),
        HeroClass::Healer => catalog.healer.clone(),
    }
}

fn civilian_sprite_spec(catalog: &ArtCatalog, role: CivilianRole) -> UnitSpriteSpec {
    match role {
        CivilianRole::Farmer => catalog.civilian_farmer.clone(),
        CivilianRole::Trader => catalog.civilian_trader.clone(),
        CivilianRole::Smith => catalog.civilian_smith.clone(),
        CivilianRole::Acolyte => catalog.civilian_acolyte.clone(),
        CivilianRole::Laborer => catalog.civilian_laborer.clone(),
    }
}

fn zone_sprite_spec(catalog: &ArtCatalog, kind: ZoneKind) -> UnitSpriteSpec {
    match kind {
        ZoneKind::GoblinCamp => catalog.goblin.clone(),
        ZoneKind::BanditDen => catalog.bandit.clone(),
        ZoneKind::TrollLair => catalog.troll.clone(),
        ZoneKind::ShadowRift => catalog.shadow.clone(),
        ZoneKind::DungeonLord => catalog.dungeon_lord.clone(),
    }
}

fn recalculate_income(state: &mut ResMut<GameState>) {
    let mut income = 10;
    if state.has_inn {
        income += 8 + (state.inn_tier.saturating_sub(1) as i32 * 4);
    }
    if state.has_market {
        income += 25 + (state.market_tier.saturating_sub(1) as i32 * 10);
    }
    if state.has_temple {
        income += 9 + (state.temple_tier.saturating_sub(1) as i32 * 4);
    }
    if state.has_guard_tower {
        income += 5 + (state.tower_tier.saturating_sub(1) as i32 * 3);
    }
    if state.has_wizard_tower {
        income += 12;
    }
    if state.has_blacksmith {
        income += 9;
    }
    if state.has_alchemist {
        income += 8;
    }
    if state.has_barracks {
        income += 7;
    }
    income += state.houses as i32 * (4 + state.town_hall_tier as i32);
    income += state.civilians as i32 * 2;
    income += state.farms as i32 * 5;
    state.income_per_day = income;
}

fn next_civilian_role(state: &ResMut<GameState>) -> CivilianRole {
    if state.farms <= state.civilians / 3 {
        CivilianRole::Farmer
    } else if state.has_blacksmith {
        CivilianRole::Smith
    } else if state.has_market {
        CivilianRole::Trader
    } else if state.has_temple {
        CivilianRole::Acolyte
    } else {
        CivilianRole::Laborer
    }
}

fn civilian_work_pos(role: CivilianRole) -> Vec3 {
    match role {
        CivilianRole::Farmer => farm_pos(0),
        CivilianRole::Trader => town_hall_pos() + Vec3::new(-20.0, 110.0, 5.0),
        CivilianRole::Smith => town_hall_pos() + Vec3::new(-40.0, -120.0, 5.0),
        CivilianRole::Acolyte => town_hall_pos() + Vec3::new(20.0, 120.0, 5.0),
        CivilianRole::Laborer => town_hall_pos() + Vec3::new(-30.0, -70.0, 5.0),
    }
}

fn compute_rank(state: &ResMut<GameState>) -> u32 {
    if state.has_barracks && state.has_wizard_tower && state.day >= 30 {
        5
    } else if state.town_hall_tier >= 3 && state.has_guard_tower && state.has_temple {
        4
    } else if state.has_wizard_tower || state.has_blacksmith || state.has_alchemist {
        4
    } else if state.has_temple || state.has_guard_tower {
        3
    } else if state.has_inn || state.has_market {
        2
    } else {
        1
    }
}

fn rank_name(rank: u32) -> &'static str {
    match rank {
        1 => "Hamlet",
        2 => "Village",
        3 => "Town",
        4 => "City",
        _ => "Kingdom",
    }
}

fn apply_hero_perks(hero: &mut Hero) {
    while hero.perk_points > 0 {
        match hero.class {
            HeroClass::Warrior | HeroClass::Healer => hero.bonus_hp += 18.0,
            HeroClass::Archer | HeroClass::Mage => hero.bonus_attack += 4.0,
            HeroClass::Rogue => hero.bonus_speed += 3.0,
        }
        hero.perk_points -= 1;
    }
}

fn update_hero_progression(hero: &mut Hero) {
    let previous_level = hero.level;
    hero.level = 1 + hero.xp / 100;
    let previous_perk_tier = previous_level / 5;
    let current_perk_tier = hero.level / 5;
    if current_perk_tier > previous_perk_tier {
        hero.perk_points += current_perk_tier - previous_perk_tier;
    }
    hero.legendary = hero.level >= 10;
    apply_hero_perks(hero);
    refresh_hero_stats(hero);
}

fn refresh_hero_stats(hero: &mut Hero) {
    let growth_stage = (hero.level / 5) as i32;
    let growth_multiplier = 2.0_f32.powi(growth_stage);
    hero.max_hp = hero.class.max_hp() * growth_multiplier
        + (hero.level.saturating_sub(1) as f32 * 6.0)
        + hero.bonus_hp;
    hero.attack = hero.class.attack() * growth_multiplier
        + (hero.level.saturating_sub(1) as f32 * 2.5)
        + hero.bonus_attack;
    hero.speed = hero.class.speed() + (hero.level.saturating_sub(1) as f32 * 1.0) + hero.bonus_speed;
    hero.hp = hero.hp.min(hero.max_hp).max(hero.max_hp * 0.45);
}

fn upgrade_cost(base: i32, current_tier: u32) -> i32 {
    let multiplier = match current_tier {
        0 | 1 => 1.5,
        2 => 2.25,
        _ => 3.0,
    };
    (base as f32 * multiplier) as i32
}
