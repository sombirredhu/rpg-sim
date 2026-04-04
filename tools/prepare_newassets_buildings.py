from __future__ import annotations

import json
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Dict, List
from collections import deque

from PIL import Image


SOURCE_ROOT = Path("assets") / "spritesheet_images" / "Building"
FALLBACK_ROOT = Path("assets") / "Assets" / "buildings"
OUTPUT_ROOT = Path("assets") / "GameplayAssetsV2"
BUILDINGS_ROOT = OUTPUT_ROOT / "buildings"


@dataclass(frozen=True)
class SheetMapping:
    source_file: str
    output_folder: str


PRIMARY_MAPPING: Dict[str, SheetMapping] = {
    "TownHall": SheetMapping("Medieval castle progression sprite sheet.png", "townhall"),
    "Inn": SheetMapping("Medieval taverns through time.png", "inn"),
    "Market": SheetMapping("Medieval market progression in isometric style.png", "market"),
    "Temple": SheetMapping("Temple of Godess.png", "temple"),
    "GuardTower": SheetMapping("Evolution of guard towers.png", "guard_tower"),
    "WizardTower": SheetMapping("Gothic dark mage academies in isometric view.png", "wizard_tower"),
    "Blacksmith": SheetMapping("Armor smithy evolution in three stages.png", "blacksmith"),
    "Alchemist": SheetMapping("Alchemy shops through the ages.png", "alchemist"),
    "Barracks": SheetMapping("Medieval Army barracks through the ages.png", "barracks"),
    "MonsterDen": SheetMapping("Fortress of the bandit dens.png", "monster_den"),
    "Caravan": SheetMapping("Magical trade caravans in progression.png", "caravan"),
}


ALTERNATE_SHEETS = [
    "Archery barracks progression in fantasy style.png",
    "Medieval archer towers progression.png",
    "Magical mage towers at each level.png",
    "Medieval weapon smithies progression.png",
    "Progressive towers of the Black Mage.png",
    "Viking temples through the ages.png",
]


def split_sheet_into_three_levels(source_path: Path, output_dir: Path) -> List[str]:
    image = Image.open(source_path).convert("RGBA")
    width, height = image.size
    output_dir.mkdir(parents=True, exist_ok=True)

    # Pre-clean entire sheet first so boundary seam cutting can break accidental
    # cross-level connections (common in tightly packed sprite sheets).
    image = remove_border_connected_checkerboard(image)
    image = remove_small_alpha_islands(image, min_area=120)
    split_1, split_2 = choose_split_boundaries(image)
    image = cut_vertical_level_seams(image, seams=(split_1, split_2), seam_half_width=4)
    ranges = [(0, split_1), (split_1, split_2), (split_2, width)]

    written: List[str] = []
    for idx, (left, right) in enumerate(ranges):
        cell = image.crop((left, 0, right, height)).convert("RGBA")

        cell = keep_primary_building_component(cell)
        cell = remove_small_alpha_islands(cell, min_area=120)
        cell = trim_alpha_outlier_columns(cell, low_q=0.005, high_q=0.983)

        # Trim transparent borders, then normalize into a 512x512 sheet for consistent world scale.
        alpha_bbox = cell.getchannel("A").getbbox()
        if alpha_bbox:
            cell = cell.crop(alpha_bbox)

        canvas = Image.new("RGBA", (512, 512), (0, 0, 0, 0))
        max_w, max_h = 472, 472
        scale = min(max_w / max(1, cell.width), max_h / max(1, cell.height))
        new_size = (max(1, int(cell.width * scale)), max(1, int(cell.height * scale)))
        resized = cell.resize(new_size, Image.Resampling.LANCZOS)

        # Bottom-align so world placement feels grounded.
        paste_x = (canvas.width - resized.width) // 2
        paste_y = canvas.height - resized.height - 10
        canvas.paste(resized, (paste_x, paste_y), resized)
        canvas = remove_small_alpha_islands(canvas, min_area=10)
        canvas = clean_alpha_halos(canvas)

        out_path = output_dir / f"lvl{idx + 1}.png"
        canvas.save(out_path)
        written.append(str(out_path))
    return written


def clean_alpha_halos(img: Image.Image) -> Image.Image:
    """Remove semi-transparent halo pixels that cause white edges in-game."""
    w, h = img.size
    px = img.load()
    for y in range(h):
        for x in range(w):
            r, g, b, a = px[x, y]
            if a == 0 and (r != 0 or g != 0 or b != 0):
                px[x, y] = (0, 0, 0, 0)
            elif 0 < a < 8:
                px[x, y] = (0, 0, 0, 0)
    return img


def remove_border_connected_checkerboard(img: Image.Image) -> Image.Image:
    w, h = img.size
    px = img.load()

    def is_checker_candidate(r: int, g: int, b: int, a: int) -> bool:
        if a < 200:
            return False
        lo = min(r, g, b)
        hi = max(r, g, b)
        chroma = hi - lo
        # Checkerboard tones: bright near-neutral grays/whites.
        return lo >= 190 and chroma <= 20

    visited = [[False] * w for _ in range(h)]
    q = deque()

    def enqueue_if_candidate(x: int, y: int) -> None:
        if visited[y][x]:
            return
        r, g, b, a = px[x, y]
        if is_checker_candidate(r, g, b, a):
            visited[y][x] = True
            q.append((x, y))

    for x in range(w):
        enqueue_if_candidate(x, 0)
        enqueue_if_candidate(x, h - 1)
    for y in range(h):
        enqueue_if_candidate(0, y)
        enqueue_if_candidate(w - 1, y)

    while q:
        x, y = q.popleft()
        r, g, b, _ = px[x, y]
        px[x, y] = (r, g, b, 0)
        for nx, ny in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)):
            if 0 <= nx < w and 0 <= ny < h and not visited[ny][nx]:
                rr, gg, bb, aa = px[nx, ny]
                if is_checker_candidate(rr, gg, bb, aa):
                    visited[ny][nx] = True
                    q.append((nx, ny))

    return img


def remove_small_alpha_islands(img: Image.Image, min_area: int = 120) -> Image.Image:
    w, h = img.size
    px = img.load()
    visited = [[False] * w for _ in range(h)]

    for y in range(h):
        for x in range(w):
            if visited[y][x]:
                continue
            if px[x, y][3] == 0:
                visited[y][x] = True
                continue

            # Flood fill alpha-connected component.
            comp = []
            q = deque([(x, y)])
            visited[y][x] = True

            while q:
                cx, cy = q.popleft()
                comp.append((cx, cy))
                for nx, ny in ((cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)):
                    if 0 <= nx < w and 0 <= ny < h and not visited[ny][nx]:
                        visited[ny][nx] = True
                        if px[nx, ny][3] > 0:
                            q.append((nx, ny))

            if len(comp) < min_area:
                for cx, cy in comp:
                    r, g, b, _ = px[cx, cy]
                    px[cx, cy] = (r, g, b, 0)

    return img


def keep_primary_building_component(img: Image.Image) -> Image.Image:
    w, h = img.size
    px = img.load()
    visited = [[False] * w for _ in range(h)]
    components = []

    for y in range(h):
        for x in range(w):
            if visited[y][x]:
                continue
            visited[y][x] = True
            if px[x, y][3] == 0:
                continue

            q = deque([(x, y)])
            coords = []
            min_x = max_x = x
            min_y = max_y = y
            while q:
                cx, cy = q.popleft()
                coords.append((cx, cy))
                min_x = min(min_x, cx)
                max_x = max(max_x, cx)
                min_y = min(min_y, cy)
                max_y = max(max_y, cy)
                for nx, ny in ((cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)):
                    if 0 <= nx < w and 0 <= ny < h and not visited[ny][nx]:
                        visited[ny][nx] = True
                        if px[nx, ny][3] > 0:
                            q.append((nx, ny))

            area = len(coords)
            center_x = sum(c[0] for c in coords) / area
            components.append(
                {
                    "coords": coords,
                    "area": area,
                    "bbox": (min_x, min_y, max_x, max_y),
                    "center_x": center_x,
                }
            )

    if not components:
        return img

    # Prefer a large component near the center of the panel.
    panel_center_x = w / 2.0

    def score(comp: dict) -> float:
        dist_penalty = abs(comp["center_x"] - panel_center_x) * 1.2
        return comp["area"] - dist_penalty

    primary = max(components, key=score)
    pmin_x, pmin_y, pmax_x, pmax_y = primary["bbox"]
    primary_center_x = primary["center_x"]
    margin = 40
    expanded = (pmin_x - margin, pmin_y - margin, pmax_x + margin, pmax_y + margin)

    kept = []
    for comp in components:
        cmin_x, cmin_y, cmax_x, cmax_y = comp["bbox"]
        intersects_expanded = not (
            cmax_x < expanded[0]
            or cmin_x > expanded[2]
            or cmax_y < expanded[1]
            or cmin_y > expanded[3]
        )
        near_primary_center = abs(comp["center_x"] - primary_center_x) <= (w * 0.32)
        if comp is primary or (intersects_expanded and near_primary_center and comp["area"] >= 200):
            kept.append(comp)

    keep_set = set()
    for comp in kept:
        keep_set.update(comp["coords"])

    for y in range(h):
        for x in range(w):
            if px[x, y][3] == 0:
                continue
            if (x, y) not in keep_set:
                r, g, b, _ = px[x, y]
                px[x, y] = (r, g, b, 0)

    return img


def trim_alpha_outlier_columns(img: Image.Image, low_q: float = 0.005, high_q: float = 0.983) -> Image.Image:
    w, h = img.size
    px = img.load()
    alpha_counts = [0] * w

    total = 0
    for x in range(w):
        col = 0
        for y in range(h):
            if px[x, y][3] > 0:
                col += 1
        alpha_counts[x] = col
        total += col

    if total == 0:
        return img

    low_target = total * max(0.0, min(1.0, low_q))
    high_target = total * max(0.0, min(1.0, high_q))

    running = 0
    low_cut = 0
    for x in range(w):
        running += alpha_counts[x]
        if running >= low_target:
            low_cut = x
            break

    running = 0
    high_cut = w - 1
    for x in range(w):
        running += alpha_counts[x]
        if running >= high_target:
            high_cut = x
            break

    # Avoid over-cropping very compact or narrow shapes.
    if high_cut - low_cut < int(w * 0.35):
        return img

    for y in range(h):
        for x in range(0, low_cut):
            if px[x, y][3] > 0:
                r, g, b, _ = px[x, y]
                px[x, y] = (r, g, b, 0)
        for x in range(high_cut + 1, w):
            if px[x, y][3] > 0:
                r, g, b, _ = px[x, y]
                px[x, y] = (r, g, b, 0)

    return img


def choose_split_boundaries(img: Image.Image) -> tuple[int, int]:
    w, h = img.size
    px = img.load()
    counts = [0] * w
    for x in range(w):
        col = 0
        for y in range(h):
            if px[x, y][3] > 0:
                col += 1
        counts[x] = col

    r1 = (int(w * 0.20), int(w * 0.45))
    r2 = (int(w * 0.55), int(w * 0.80))

    split_1 = min(range(r1[0], r1[1] + 1), key=lambda x: counts[x])
    split_2 = min(range(r2[0], r2[1] + 1), key=lambda x: counts[x])

    # Fallback to equal thirds if dynamic split looks invalid.
    if split_2 <= split_1 + int(w * 0.10):
        split_1 = w // 3
        split_2 = (2 * w) // 3

    return split_1, split_2


def cut_vertical_level_seams(img: Image.Image, seams: tuple[int, int], seam_half_width: int = 4) -> Image.Image:
    w, h = img.size
    px = img.load()
    for seam_x in seams:
        for x in range(max(0, seam_x - seam_half_width), min(w, seam_x + seam_half_width + 1)):
            for y in range(h):
                if px[x, y][3] == 0:
                    continue
                r, g, b, _ = px[x, y]
                px[x, y] = (r, g, b, 0)
    return img


def copy_fallback_levels(building_key: str, output_dir: Path) -> List[str]:
    fallback_dir = FALLBACK_ROOT / building_key
    output_dir.mkdir(parents=True, exist_ok=True)
    written: List[str] = []
    for level in (1, 2, 3):
        source = fallback_dir / f"lvl{level}.png"
        if not source.exists():
            raise FileNotFoundError(f"Fallback image missing: {source}")
        target = output_dir / f"lvl{level}.png"
        target.write_bytes(source.read_bytes())
        written.append(str(target))
    return written


def main() -> None:
    manifest = {
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "source_root": str(SOURCE_ROOT),
        "output_root": str(OUTPUT_ROOT),
        "mapped_buildings": {},
        "fallback_buildings": [],
        "alternate_sheets_available": [],
    }

    BUILDINGS_ROOT.mkdir(parents=True, exist_ok=True)
    for building_key, mapping in PRIMARY_MAPPING.items():
        output_dir = BUILDINGS_ROOT / mapping.output_folder
        source_path = SOURCE_ROOT / mapping.source_file

        if source_path.exists():
            files = split_sheet_into_three_levels(source_path, output_dir)
            manifest["mapped_buildings"][building_key] = {
                "source": str(source_path),
                "used_fallback": False,
                "files": files,
            }
        else:
            files = copy_fallback_levels(building_key, output_dir)
            manifest["mapped_buildings"][building_key] = {
                "source": None,
                "used_fallback": True,
                "files": files,
            }
            manifest["fallback_buildings"].append(building_key)

    manifest["alternate_sheets_available"] = [
        str(SOURCE_ROOT / name) for name in ALTERNATE_SHEETS if (SOURCE_ROOT / name).exists()
    ]

    expected_from_requirements = {
        "Buildings": [
            "TownHall",
            "Inn",
            "Market",
            "Temple",
            "GuardTower",
            "WizardTower",
            "Blacksmith",
            "Alchemist",
            "Barracks",
        ],
        "Building extras (optional, high value)": [
            "damaged.png x9",
            "ruined.png x9",
            "construction_stage1.png x9",
            "construction_stage2.png x9",
        ],
        "Heroes (still missing source sheets in NewAssets)": [
            "Warrior full set + portraits",
            "Archer full set + portraits",
            "Mage full set + portraits",
            "Rogue full set + portraits",
            "Healer full set + portraits",
        ],
        "Enemies (still missing source sheets in NewAssets)": [
            "Goblin full set",
            "Bandit full set",
            "Troll full set",
            "GoblinElite full set",
            "BossWarlord full set",
            "Werewolf full set",
            "ShadowBandit full set",
        ],
    }

    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)
    (OUTPUT_ROOT / "manifest.json").write_text(json.dumps(manifest, indent=2), encoding="utf-8")
    (OUTPUT_ROOT / "MISSING_IMAGE_REPORT.json").write_text(
        json.dumps(expected_from_requirements, indent=2),
        encoding="utf-8",
    )

    print(json.dumps({"processed_buildings": len(PRIMARY_MAPPING), "fallbacks_used": len(manifest["fallback_buildings"])}, indent=2))


if __name__ == "__main__":
    main()
