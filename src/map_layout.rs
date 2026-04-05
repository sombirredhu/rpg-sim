//! Structured map layout with intentional zone placement.
//!
//! 80×80 tile grid, TILE_SIZE = 37.5 world units per tile.
//! Total map: 3000×3000 world units (±1500 centered at origin).
//!
//! The core map is divided into geographic zones around the town center:
//! - Town Center (origin): buildings, paths, clean grass
//! - Gold Mine NE/SW/Outer: rocky terrain, mine resource nodes
//! - Forest NW/SE/Outer N: dense trees, darker grass, lumber nodes
//! - Quarry E/NE: rock outcrops, stone paths
//! - River (S-curve NW→SE crossing): water barriers west of town center
//! - Ruins (3 spots): exploration landmarks with debris
//! - Enemy zones (map edges): monster den locations
//!
//! Expansion zones (outside revealed radius) remain procedural for variety.

use bevy::prelude::Vec2;

/// World units per tile. 3000 / 80 = 37.5
pub const TILE_SIZE: f32 = 37.5;
/// Grid width in tiles
pub const GRID_W: usize = 80;
/// Grid height in tiles
pub const GRID_H: usize = 80;
/// Half-extent in world units
pub const MAP_HALF_EXTENT: f32 = 1500.0;

// ============================================================
// ZONE DEFINITIONS
// Coordinates stored as raw f32 because Vec2::new is not const fn.
// ============================================================

#[derive(Debug, Clone, Copy)]
pub enum ZoneTerrain {
    CleanGrass,
    ForestGrass,
    RockyDirt,
}

#[derive(Debug, Clone, Copy)]
pub struct ZoneConfig {
    pub cx: f32,
    pub cy: f32,
    pub radius: f32,
    pub terrain: ZoneTerrain,
}

impl ZoneConfig {
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.cx, self.cy)
    }
}

/// All core zones ordered roughly by importance
pub const CORE_ZONES: &[ZoneConfig] = &[
    // Town Center
    ZoneConfig { cx: 0.0,     cy: 0.0,    radius: 180.0, terrain: ZoneTerrain::CleanGrass  },
    // Town Outskirts
    ZoneConfig { cx: 0.0,     cy: 0.0,    radius: 300.0, terrain: ZoneTerrain::CleanGrass  },
    // Forest NW
    ZoneConfig { cx: -450.0,  cy: 280.0,  radius: 220.0, terrain: ZoneTerrain::ForestGrass },
    // Forest SE
    ZoneConfig { cx: 500.0,   cy: -320.0, radius: 200.0, terrain: ZoneTerrain::ForestGrass },
    // Gold Mine NE
    ZoneConfig { cx: 480.0,   cy: 280.0,  radius: 180.0, terrain: ZoneTerrain::RockyDirt   },
    // Gold Mine SW
    ZoneConfig { cx: -320.0,  cy: -280.0, radius: 160.0, terrain: ZoneTerrain::RockyDirt   },
    // Quarry E
    ZoneConfig { cx: 750.0,   cy: 0.0,    radius: 160.0, terrain: ZoneTerrain::RockyDirt   },
    // Quarry NE
    ZoneConfig { cx: 900.0,   cy: 400.0,  radius: 130.0, terrain: ZoneTerrain::RockyDirt   },
    // Gold Mine Outer
    ZoneConfig { cx: 700.0,   cy: 150.0,  radius: 140.0, terrain: ZoneTerrain::RockyDirt   },
    // Forest Outer N
    ZoneConfig { cx: -100.0,  cy: 650.0,  radius: 160.0, terrain: ZoneTerrain::ForestGrass },
];

// ============================================================
// RIVER — 8-segment S-curve NW to SE, passes west of town
// ============================================================
pub const RIVER_SEGMENTS: &[(f32; 4)] = &[
    (-1200.0, 1000.0,  -900.0,  700.0),
    ( -900.0,  700.0, -700.0,  500.0),
    ( -700.0,  500.0, -500.0,  350.0),
    ( -500.0,  350.0, -350.0,  200.0),
    ( -350.0,  200.0, -200.0,  -50.0),
    ( -200.0,  -50.0,  100.0, -300.0),
    (  100.0, -300.0,  400.0, -550.0),
    (  400.0, -550.0,  700.0, -800.0),
];

pub const RIVER_OVERLAY_SIZE: f32 = 180.0;
pub const RIVER_STEP: f32 = 80.0;

// ============================================================
// RUINS — exploration landmarks
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuinType { TempleRuin, ArchRuin, WallRuin }

pub const RUIN_POSITIONS: &[(f32, f32, RuinType)] = &[
    (-650.0,  550.0, RuinType::TempleRuin),
    ( 950.0, -500.0, RuinType::ArchRuin),
    (-750.0, -650.0, RuinType::WallRuin),
];

// ============================================================
// MONSTER DENS — fixed positions at map edges
// ============================================================
use crate::components::EnemyType;

pub const CORE_MONSTER_DENS: &[(f32, f32, EnemyType)] = &[
    // NE edge — goblins
    ( 650.0,  650.0, EnemyType::Goblin),
    // NW edge — bandits
    (-550.0,  650.0, EnemyType::Bandit),
    // SW edge — trolls
    (-750.0, -750.0, EnemyType::Troll),
    // SE edge — elite goblins
    ( 950.0, -650.0, EnemyType::GoblinElite),
];
