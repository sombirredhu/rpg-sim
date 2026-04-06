use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};
use crate::map_layout::*;

/// Noise-based terrain types for procedural generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoiseTerrain {
    Water,
    Forest,
    Grass,
    Rocky,
}

/// Generate noise map for terrain types using the noise crate
pub fn generate_terrain_noise(
    width: usize,
    height: usize,
    scale: f64,
    _octaves: usize,
    _persistence: f64,
    _lacunarity: f64,
    _seed: u64,
) -> Vec<Vec<NoiseTerrain>> {
    // Create OpenSimplex noise generator
    let noise = OpenSimplex::new();

    // Convert noise values to terrain types
    let mut terrain_map = vec![vec![NoiseTerrain::Grass; height]; width];

    for x in 0..width {
        for y in 0..height {
            // Sample noise at this position
            let nx = x as f64 * scale;
            let ny = y as f64 * scale;
            let noise_value = noise.get([nx, ny]);

            // Map noise values (-1 to 1) to terrain types
            let terrain = match noise_value {
                n if n < -0.2 => NoiseTerrain::Water,
                n if n < 0.0 => NoiseTerrain::Forest,
                n if n < 0.3 => NoiseTerrain::Grass,
                _ => NoiseTerrain::Rocky,
            };

            terrain_map[x][y] = terrain;
        }
    }

    terrain_map
}

/// Apply core zones on top of noise-generated terrain to preserve important landmarks
pub fn apply_core_zones(
    terrain_map: &mut Vec<Vec<NoiseTerrain>>,
    offset_x: isize,
    offset_y: isize,
) {
    let width = terrain_map.len();
    let height = terrain_map[0].len();

    // Apply town center - force clean grass in center area
    for zone in CORE_ZONES.iter() {
        if zone.terrain == ZoneTerrain::CleanGrass && zone.radius >= 180.0 {
            let cx = (zone.cx / TILE_SIZE + offset_x as f32) as isize;
            let cy = (zone.cy / TILE_SIZE + offset_y as f32) as isize;
            let radius = (zone.radius / TILE_SIZE) as isize;

            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    let x = cx + dx;
                    let y = cy + dy;
                    if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                        let dist = ((dx * dx + dy * dy) as f32).sqrt();
                        if dist <= radius as f32 {
                            terrain_map[x as usize][y as usize] = NoiseTerrain::Grass;
                        }
                    }
                }
            }
        }
    }

    // Apply forest zones
    for zone in CORE_ZONES.iter() {
        if zone.terrain == ZoneTerrain::ForestGrass {
            let cx = (zone.cx / TILE_SIZE + offset_x as f32) as isize;
            let cy = (zone.cy / TILE_SIZE + offset_y as f32) as isize;
            let radius = (zone.radius / TILE_SIZE) as isize;

            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    let x = cx + dx;
                    let y = cy + dy;
                    if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                        let dist = ((dx * dx + dy * dy) as f32).sqrt();
                        if dist <= radius as f32 {
                            terrain_map[x as usize][y as usize] = NoiseTerrain::Forest;
                        }
                    }
                }
            }
        }
    }

    // Apply rocky zones (mines/quarries)
    for zone in CORE_ZONES.iter() {
        if zone.terrain == ZoneTerrain::RockyDirt {
            let cx = (zone.cx / TILE_SIZE + offset_x as f32) as isize;
            let cy = (zone.cy / TILE_SIZE + offset_y as f32) as isize;
            let radius = (zone.radius / TILE_SIZE) as isize;

            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    let x = cx + dx;
                    let y = cy + dy;
                    if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                        let dist = ((dx * dx + dy * dy) as f32).sqrt();
                        if dist <= radius as f32 {
                            terrain_map[x as usize][y as usize] = NoiseTerrain::Rocky;
                        }
                    }
                }
            }
        }
    }
}

/// Get world position from tile coordinates
pub fn tile_to_world(tile_x: usize, tile_y: usize, offset_x: f32, offset_y: f32) -> Vec2 {
    let world_x = tile_x as f32 * TILE_SIZE + offset_x;
    let world_y = tile_y as f32 * TILE_SIZE + offset_y;
    Vec2::new(world_x, world_y)
}

/// Get tile coordinates from world position
pub fn world_to_tile(world_pos: Vec2, offset_x: f32, offset_y: f32) -> (usize, usize) {
    let tile_x = ((world_pos.x - offset_x) / TILE_SIZE).round() as usize;
    let tile_y = ((world_pos.y - offset_y) / TILE_SIZE).round() as usize;
    (tile_x, tile_y)
}