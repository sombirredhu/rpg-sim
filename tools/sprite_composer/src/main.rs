use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::path::{Path, PathBuf};

fn composite_layers(layers: &[PathBuf], output: &Path) {
    let first = image::open(&layers[0]).expect(&format!("Failed to open {:?}", layers[0]));
    let (w, h) = first.dimensions();
    let mut result: RgbaImage = ImageBuffer::new(w, h);

    for layer_path in layers {
        if !layer_path.exists() {
            eprintln!("  Warning: layer not found: {:?}, skipping", layer_path);
            continue;
        }
        let layer = image::open(layer_path)
            .expect(&format!("Failed to open {:?}", layer_path))
            .to_rgba8();

        for y in 0..h {
            for x in 0..w {
                let src = layer.get_pixel(x, y);
                let dst = result.get_pixel(x, y);
                let blended = alpha_blend(*dst, *src);
                result.put_pixel(x, y, blended);
            }
        }
        println!("  + {:?}", layer_path.file_name().unwrap());
    }

    result.save(output).expect("Failed to save output");
    println!("  => Saved {:?} ({}x{})", output, w, h);
}

fn alpha_blend(dst: Rgba<u8>, src: Rgba<u8>) -> Rgba<u8> {
    let sa = src[3] as f32 / 255.0;
    let da = dst[3] as f32 / 255.0;
    let out_a = sa + da * (1.0 - sa);
    if out_a == 0.0 {
        return Rgba([0, 0, 0, 0]);
    }
    let r = (src[0] as f32 * sa + dst[0] as f32 * da * (1.0 - sa)) / out_a;
    let g = (src[1] as f32 * sa + dst[1] as f32 * da * (1.0 - sa)) / out_a;
    let b = (src[2] as f32 * sa + dst[2] as f32 * da * (1.0 - sa)) / out_a;
    Rgba([r as u8, g as u8, b as u8, (out_a * 255.0) as u8])
}

/// Crop a region from a source image and save it
fn crop_and_save(src: &image::DynamicImage, x: u32, y: u32, w: u32, h: u32, output: &Path) {
    let cropped = src.crop_imm(x, y, w, h);
    cropped.save(output).expect("Failed to save crop");
    println!("  Cropped {}x{} at ({},{}) => {:?}", w, h, x, y, output.file_name().unwrap());
}

/// Create a scaled-down tile from a large seamless texture
fn tile_from_texture(src_path: &Path, tile_size: u32, output: &Path) {
    let img = image::open(src_path).expect(&format!("Failed to open {:?}", src_path));
    // Take a center crop and resize to tile_size
    let (w, h) = img.dimensions();
    let crop_size = w.min(h);
    let cx = (w - crop_size) / 2;
    let cy = (h - crop_size) / 2;
    let cropped = img.crop_imm(cx, cy, crop_size, crop_size);
    let resized = cropped.resize_exact(tile_size, tile_size, image::imageops::FilterType::Lanczos3);
    resized.save(output).expect("Failed to save tile");
    println!("  Tile {}x{} from {:?} => {:?}", tile_size, tile_size, src_path.file_name().unwrap(), output.file_name().unwrap());
}

fn main() {
    let base = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let mode = std::env::args().nth(2).unwrap_or_else(|| "tiles".to_string());
    let base = Path::new(&base);

    let grassland = base.join("Medieval art/grassland_blend/grassland");
    let assets = base.join("assets");

    match mode.as_str() {
        "tiles" => {
            // Create tile textures from seamless textures
            let tile_dir = assets.join("Level/Ground");
            std::fs::create_dir_all(&tile_dir).unwrap();

            println!("=== Creating ground tiles ===");
            tile_from_texture(&grassland.join("grass_overcast.png"), 40, &tile_dir.join("grass_new.png"));
            tile_from_texture(&grassland.join("path.png"), 40, &tile_dir.join("road_stone.png"));
            tile_from_texture(&grassland.join("path2.png"), 40, &tile_dir.join("road_edge.png"));
            tile_from_texture(&grassland.join("water.png"), 40, &tile_dir.join("water_new.png"));
            tile_from_texture(&grassland.join("rock_diffuse.png"), 40, &tile_dir.join("rock_new.png"));
            tile_from_texture(&grassland.join("rock_diffuse_wet.png"), 40, &tile_dir.join("rock_wet.png"));

            println!("\n=== Slicing grassland.png master atlas ===");
            let atlas = image::open(grassland.join("grassland.png"))
                .expect("Failed to open grassland.png");
            let (aw, ah) = atlas.dimensions();
            println!("  Atlas size: {}x{}", aw, ah);

            let deco_dir = assets.join("Level/Decorations");
            std::fs::create_dir_all(&deco_dir).unwrap();

            // Row 1: Stone ruins and arches (top of image)
            // These are isometric renders, manually identified regions:

            // Stone pillars/ruins (top row, ~y=0..120)
            crop_and_save(&atlas, 0, 0, 80, 120, &deco_dir.join("ruin_pillar1.png"));
            crop_and_save(&atlas, 80, 0, 80, 120, &deco_dir.join("ruin_pillar2.png"));
            crop_and_save(&atlas, 160, 10, 70, 110, &deco_dir.join("ruin_arch1.png"));
            crop_and_save(&atlas, 230, 10, 70, 110, &deco_dir.join("ruin_arch2.png"));
            crop_and_save(&atlas, 310, 0, 90, 120, &deco_dir.join("ruin_wall1.png"));
            crop_and_save(&atlas, 410, 0, 90, 110, &deco_dir.join("ruin_wall2.png"));

            // Rocks and small props (y ~120..200)
            crop_and_save(&atlas, 0, 130, 50, 50, &deco_dir.join("rock_small1.png"));
            crop_and_save(&atlas, 55, 130, 50, 50, &deco_dir.join("rock_small2.png"));
            crop_and_save(&atlas, 110, 130, 50, 50, &deco_dir.join("rock_small3.png"));
            crop_and_save(&atlas, 165, 130, 50, 40, &deco_dir.join("rock_flat.png"));

            // Bushes and plants (y ~170..210)
            crop_and_save(&atlas, 0, 175, 50, 40, &deco_dir.join("bush1.png"));
            crop_and_save(&atlas, 55, 175, 50, 40, &deco_dir.join("bush2.png"));
            crop_and_save(&atlas, 110, 175, 50, 40, &deco_dir.join("bush3.png"));
            crop_and_save(&atlas, 165, 175, 60, 40, &deco_dir.join("bush4.png"));
            crop_and_save(&atlas, 230, 175, 50, 40, &deco_dir.join("plant1.png"));
            crop_and_save(&atlas, 285, 175, 50, 40, &deco_dir.join("plant2.png"));

            // Trees and large bushes (y ~195..240)
            crop_and_save(&atlas, 340, 168, 60, 50, &deco_dir.join("tree_bush1.png"));
            crop_and_save(&atlas, 400, 168, 60, 50, &deco_dir.join("tree_bush2.png"));
            crop_and_save(&atlas, 460, 168, 50, 50, &deco_dir.join("tree_bush3.png"));

            // Stone structures/walls middle rows (y ~210..320)
            crop_and_save(&atlas, 0, 215, 60, 55, &deco_dir.join("stone_block1.png"));
            crop_and_save(&atlas, 65, 215, 60, 55, &deco_dir.join("stone_block2.png"));
            crop_and_save(&atlas, 130, 215, 60, 55, &deco_dir.join("stone_step.png"));

            // Fences (y ~275..310)
            crop_and_save(&atlas, 290, 210, 60, 45, &deco_dir.join("fence1.png"));
            crop_and_save(&atlas, 355, 210, 60, 45, &deco_dir.join("fence2.png"));
            crop_and_save(&atlas, 420, 210, 50, 45, &deco_dir.join("fence_post.png"));

            // Water/bridge tiles (y ~330..400)
            crop_and_save(&atlas, 0, 330, 120, 60, &deco_dir.join("water_tile1.png"));
            crop_and_save(&atlas, 130, 330, 120, 60, &deco_dir.join("water_tile2.png"));
            crop_and_save(&atlas, 260, 330, 120, 60, &deco_dir.join("bridge1.png"));

            // Cabin/structures (y ~405..510)
            crop_and_save(&atlas, 0, 405, 80, 100, &deco_dir.join("cabin_wall1.png"));
            crop_and_save(&atlas, 85, 405, 80, 100, &deco_dir.join("cabin_wall2.png"));
            crop_and_save(&atlas, 170, 405, 80, 100, &deco_dir.join("cabin_wall3.png"));
            crop_and_save(&atlas, 310, 405, 150, 100, &deco_dir.join("temple_ruin.png"));

            // Cave entrances (y ~510..600)
            crop_and_save(&atlas, 0, 510, 110, 90, &deco_dir.join("cave_entrance1.png"));
            crop_and_save(&atlas, 115, 510, 110, 90, &deco_dir.join("cave_entrance2.png"));
            crop_and_save(&atlas, 235, 510, 110, 90, &deco_dir.join("cave_entrance3.png"));

            // Trees row (y ~600..720)
            crop_and_save(&atlas, 0, 600, 100, 120, &deco_dir.join("tree_oak1.png"));
            crop_and_save(&atlas, 105, 600, 100, 120, &deco_dir.join("tree_oak2.png"));
            crop_and_save(&atlas, 220, 600, 80, 120, &deco_dir.join("tree_dead1.png"));
            crop_and_save(&atlas, 310, 600, 80, 120, &deco_dir.join("tree_dead2.png"));

            // Pine trees (bottom row, y ~720..)
            crop_and_save(&atlas, 0, 725, 60, 80, &deco_dir.join("pine1.png"));
            crop_and_save(&atlas, 65, 725, 60, 80, &deco_dir.join("pine2.png"));
            crop_and_save(&atlas, 130, 725, 60, 80, &deco_dir.join("pine3.png"));
            crop_and_save(&atlas, 195, 725, 60, 80, &deco_dir.join("pine4.png"));
            crop_and_save(&atlas, 260, 725, 60, 80, &deco_dir.join("pine5.png"));
            crop_and_save(&atlas, 325, 725, 60, 80, &deco_dir.join("pine6.png"));

            println!("\nDone!");
        }
        _ => {
            eprintln!("Unknown mode: {}", mode);
        }
    }
}
