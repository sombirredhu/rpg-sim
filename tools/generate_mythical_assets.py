from __future__ import annotations

import json
import math
import random
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Dict

from PIL import Image, ImageDraw, ImageFilter


OUTPUT_ROOT = Path("assets") / "Assets"
BUILDINGS_ROOT = OUTPUT_ROOT / "buildings"
UNITS_ROOT = OUTPUT_ROOT / "units"

BUILDINGS = [
    "TownHall",
    "Inn",
    "Market",
    "Temple",
    "GuardTower",
    "WizardTower",
    "Blacksmith",
    "Alchemist",
    "Barracks",
]

HEROES = ["warrior", "archer", "mage", "rogue", "healer"]
ENEMIES = [
    "goblin",
    "bandit",
    "troll",
    "goblin_elite",
    "boss_warlord",
    "werewolf",
    "shadow_bandit",
]

HERO_FRAME_COUNTS = {"idle": 2, "walk": 4, "attack": 4, "hit": 2, "death": 4}
ENEMY_FRAME_COUNTS = {"idle": 2, "walk": 4, "attack": 4, "hit": 2, "death": 4}

BUILDING_SIZE = (512, 512)
UNIT_SIZE = (192, 192)
PORTRAIT_SIZE = (512, 512)


@dataclass(frozen=True)
class Palette:
    primary: tuple[int, int, int]
    secondary: tuple[int, int, int]
    accent: tuple[int, int, int]
    glow: tuple[int, int, int]


BUILDING_PALETTES: Dict[str, Palette] = {
    "TownHall": Palette((141, 114, 90), (95, 82, 72), (183, 53, 32), (255, 204, 120)),
    "Inn": Palette((156, 114, 82), (102, 74, 57), (170, 44, 30), (255, 190, 114)),
    "Market": Palette((164, 123, 89), (112, 83, 63), (42, 132, 174), (255, 210, 130)),
    "Temple": Palette((143, 145, 151), (103, 106, 120), (214, 188, 116), (240, 228, 168)),
    "GuardTower": Palette((128, 130, 135), (86, 89, 100), (175, 36, 30), (255, 170, 120)),
    "WizardTower": Palette((122, 125, 167), (82, 84, 125), (81, 186, 247), (180, 225, 255)),
    "Blacksmith": Palette((98, 90, 84), (62, 58, 56), (230, 116, 34), (255, 195, 112)),
    "Alchemist": Palette((126, 145, 110), (83, 99, 73), (84, 196, 156), (185, 255, 220)),
    "Barracks": Palette((126, 108, 95), (80, 70, 63), (168, 42, 36), (255, 181, 114)),
}

UNIT_PALETTES: Dict[str, Palette] = {
    "warrior": Palette((70, 120, 184), (46, 77, 126), (233, 176, 86), (255, 220, 140)),
    "archer": Palette((73, 146, 113), (46, 96, 72), (205, 153, 90), (204, 255, 186)),
    "mage": Palette((117, 95, 188), (72, 57, 126), (109, 214, 245), (187, 241, 255)),
    "rogue": Palette((105, 109, 122), (69, 73, 84), (196, 62, 76), (255, 191, 201)),
    "healer": Palette((103, 158, 180), (62, 103, 122), (229, 209, 113), (255, 247, 182)),
    "goblin": Palette((109, 164, 94), (66, 104, 56), (194, 89, 41), (207, 245, 167)),
    "bandit": Palette((143, 103, 82), (95, 64, 50), (192, 59, 53), (255, 195, 165)),
    "troll": Palette((92, 121, 90), (56, 78, 54), (157, 129, 97), (203, 233, 182)),
    "goblin_elite": Palette((121, 156, 85), (73, 96, 52), (210, 163, 56), (253, 246, 162)),
    "boss_warlord": Palette((128, 77, 67), (83, 47, 43), (212, 62, 34), (255, 195, 156)),
    "werewolf": Palette((132, 132, 142), (84, 84, 95), (95, 155, 223), (188, 222, 255)),
    "shadow_bandit": Palette((82, 74, 112), (52, 45, 75), (128, 103, 190), (189, 170, 244)),
}


def _mix(c1: tuple[int, int, int], c2: tuple[int, int, int], t: float) -> tuple[int, int, int]:
    t = max(0.0, min(1.0, t))
    return (
        int(c1[0] + (c2[0] - c1[0]) * t),
        int(c1[1] + (c2[1] - c1[1]) * t),
        int(c1[2] + (c2[2] - c1[2]) * t),
    )


def _draw_soft_shadow(draw: ImageDraw.ImageDraw, cx: int, cy: int, rx: int, ry: int, alpha: int) -> None:
    draw.ellipse((cx - rx, cy - ry, cx + rx, cy + ry), fill=(0, 0, 0, alpha))


def _draw_banner(draw: ImageDraw.ImageDraw, x: int, y: int, color: tuple[int, int, int], side: int = 1) -> None:
    pole_h = 58
    draw.line((x, y - pole_h, x, y + 4), fill=(50, 38, 26, 255), width=4)
    flag = [(x, y - pole_h + 6), (x + (24 * side), y - pole_h + 14), (x, y - pole_h + 22)]
    draw.polygon(flag, fill=(*color, 220))


def _draw_building(building: str, level: int, path: Path) -> None:
    palette = BUILDING_PALETTES[building]
    rng = random.Random(f"{building}-{level}-mythic")
    img = Image.new("RGBA", BUILDING_SIZE, (0, 0, 0, 0))
    draw = ImageDraw.Draw(img, "RGBA")

    w, h = BUILDING_SIZE
    cx = w // 2
    ground_y = int(h * 0.77)
    scale = {1: 0.72, 2: 0.88, 3: 1.0}[level]
    wall_w = int((180 + rng.randint(-10, 14)) * scale)
    wall_h = int((106 + rng.randint(-7, 8)) * scale)
    x0 = cx - wall_w // 2
    x1 = cx + wall_w // 2
    y1 = ground_y
    y0 = y1 - wall_h

    _draw_soft_shadow(draw, cx, ground_y + 18, int(wall_w * 0.56), 26, 95)

    wall_color = _mix(palette.primary, (136, 136, 142), (level - 1) / 2.0)
    roof_color = _mix(palette.secondary, (74, 74, 84), (level - 1) / 2.0)

    draw.rounded_rectangle((x0, y0, x1, y1), radius=18, fill=(*wall_color, 255), outline=(42, 32, 26, 255), width=4)
    draw.rectangle((x0 + 8, y0 + 12, x1 - 8, y0 + 22), fill=(255, 255, 255, 18))

    roof_h = int((62 + rng.randint(-8, 9)) * scale)
    roof = [(x0 - 16, y0 + 18), (x1 + 16, y0 + 18), (cx, y0 - roof_h)]
    draw.polygon(roof, fill=(*roof_color, 255), outline=(36, 24, 20, 255))

    door_w = int(34 * scale)
    door_h = int(52 * scale)
    draw.rounded_rectangle((cx - door_w // 2, y1 - door_h, cx + door_w // 2, y1), radius=6, fill=(54, 39, 28, 255))

    window_count = 2 + (2 * level)
    for i in range(window_count):
        wx = int(x0 + 22 + i * (wall_w - 44) / max(1, window_count - 1))
        wy = int(y0 + wall_h * (0.44 if i % 2 == 0 else 0.57))
        ww, wh = int(14 * scale), int(16 * scale)
        draw.rounded_rectangle((wx - ww // 2, wy - wh // 2, wx + ww // 2, wy + wh // 2), radius=3, fill=(255, 215, 129, 210))
        draw.rectangle((wx - ww // 2, wy - 1, wx + ww // 2, wy + 1), fill=(108, 83, 66, 200))
        draw.rectangle((wx - 1, wy - wh // 2, wx + 1, wy + wh // 2), fill=(108, 83, 66, 200))

    if level >= 2:
        side_w = int(52 * scale)
        side_h = int(74 * scale)
        for side in (-1, 1):
            sx0 = cx + side * (wall_w // 2 - 14)
            rect = (
                sx0 - (side_w if side < 0 else 0),
                y1 - side_h,
                sx0 + (0 if side < 0 else side_w),
                y1,
            )
            draw.rounded_rectangle(rect, radius=12, fill=(*_mix(wall_color, palette.secondary, 0.15), 255), outline=(39, 30, 24, 255), width=3)
            _draw_banner(draw, int((rect[0] + rect[2]) / 2), int(rect[1] + 6), palette.accent, side=side)

    if level == 3:
        for side in (-1, 1):
            tx = cx + side * int(wall_w * 0.46)
            tw, th = int(34 * scale), int(122 * scale)
            draw.rounded_rectangle((tx - tw // 2, y1 - th, tx + tw // 2, y1), radius=12, fill=(*_mix(wall_color, (112, 116, 126), 0.35), 255), outline=(35, 28, 22, 255), width=3)
            torch_y = y0 + 36
            draw.ellipse((tx - 8, torch_y - 8, tx + 8, torch_y + 8), fill=(*palette.glow, 200))
            draw.ellipse((tx - 4, torch_y - 4, tx + 4, torch_y + 4), fill=(255, 245, 200, 230))

    emblem_y = y0 + 24
    emblem_w = int(36 * scale)
    draw.rounded_rectangle((cx - emblem_w // 2, emblem_y, cx + emblem_w // 2, emblem_y + int(28 * scale)), radius=6, fill=(*palette.accent, 230), outline=(38, 28, 24, 255), width=2)

    if building in {"WizardTower", "Alchemist"}:
        orb_x = cx + int(wall_w * 0.3)
        orb_y = y0 + 24
        draw.ellipse((orb_x - 16, orb_y - 16, orb_x + 16, orb_y + 16), fill=(*palette.glow, 180))
        draw.ellipse((orb_x - 8, orb_y - 8, orb_x + 8, orb_y + 8), fill=(255, 255, 255, 220))
    if building == "Blacksmith":
        anvil_x = cx - int(wall_w * 0.32)
        anvil_y = y1 - 20
        draw.polygon(
            [(anvil_x - 16, anvil_y), (anvil_x + 14, anvil_y), (anvil_x + 20, anvil_y + 8), (anvil_x - 20, anvil_y + 8)],
            fill=(72, 74, 82, 255),
            outline=(30, 30, 35, 255),
        )
    if building == "Market":
        awning_y = y0 + int(wall_h * 0.32)
        draw.polygon(
            [(x0 + 10, awning_y), (x1 - 10, awning_y), (x1 - 32, awning_y + 28), (x0 + 32, awning_y + 28)],
            fill=(*_mix(palette.accent, (255, 255, 255), 0.2), 220),
            outline=(46, 32, 24, 220),
        )

    img = img.filter(ImageFilter.UnsharpMask(radius=1.2, percent=140, threshold=2))
    path.parent.mkdir(parents=True, exist_ok=True)
    img.save(path)


def _pose_for_state(state: str, frame: int, total: int) -> dict[str, float]:
    t = frame / max(1, total - 1)
    phase = math.sin(t * math.tau)
    pose = {"bob": 0.0, "swing": 0.0, "lean": 0.0, "weapon": 0.0, "collapse": 0.0}

    if state == "idle":
        pose["bob"] = phase * 2.0
        pose["swing"] = phase * 4.0
    elif state == "walk":
        pose["bob"] = abs(phase) * 4.0
        pose["swing"] = phase * 14.0
    elif state == "attack":
        pose["bob"] = -abs(phase) * 2.0
        pose["weapon"] = 24.0 + (math.sin(t * math.pi) * 48.0)
        pose["lean"] = -8.0
    elif state == "hit":
        pose["lean"] = -16.0 + (8.0 * t)
        pose["bob"] = 3.0
    elif state == "death":
        pose["collapse"] = min(1.0, (frame + 1) / total)
        pose["lean"] = -70.0 * pose["collapse"]
        pose["bob"] = 6.0 * pose["collapse"]
    return pose


def _draw_character(unit: str, state: str, frame: int, total: int, is_enemy: bool, path: Path) -> None:
    palette = UNIT_PALETTES[unit]
    pose = _pose_for_state(state, frame, total)
    rng = random.Random(f"{unit}-{state}-{frame}-mythic")

    canvas = Image.new("RGBA", UNIT_SIZE, (0, 0, 0, 0))
    draw = ImageDraw.Draw(canvas, "RGBA")
    cw, ch = UNIT_SIZE
    cx = cw // 2
    floor_y = int(ch * 0.82)
    _draw_soft_shadow(draw, cx, floor_y + 3, 24 + (7 if unit in {"troll", "boss_warlord"} else 0), 10, 86)

    layer = Image.new("RGBA", UNIT_SIZE, (0, 0, 0, 0))
    ld = ImageDraw.Draw(layer, "RGBA")

    bob = pose["bob"]
    body_scale = 1.24 if unit in {"troll", "boss_warlord", "werewolf"} else 1.0
    torso_w = int(30 * body_scale)
    torso_h = int(38 * body_scale)
    torso_top = floor_y - int(58 * body_scale) + int(bob)
    torso_bottom = torso_top + torso_h

    leg_len = int(24 * body_scale)
    swing = pose["swing"]
    lean = pose["lean"]
    hip_y = torso_bottom - 2

    ld.line((cx - 8, hip_y, cx - 8 - swing * 0.25, hip_y + leg_len), fill=(40, 31, 27, 255), width=5)
    ld.line((cx + 8, hip_y, cx + 8 + swing * 0.25, hip_y + leg_len), fill=(40, 31, 27, 255), width=5)
    ld.ellipse((cx - 14, floor_y - 2, cx - 2, floor_y + 6), fill=(34, 26, 23, 255))
    ld.ellipse((cx + 2, floor_y - 2, cx + 14, floor_y + 6), fill=(34, 26, 23, 255))

    cloak_color = _mix(palette.secondary, (30, 30, 40), 0.2 if is_enemy else 0.05)
    ld.polygon(
        [
            (cx - torso_w // 2 - 4, torso_top + 8),
            (cx + torso_w // 2 + 4, torso_top + 8),
            (cx + torso_w // 2 + 10, torso_bottom + 14),
            (cx - torso_w // 2 - 10, torso_bottom + 14),
        ],
        fill=(*cloak_color, 226),
    )

    ld.rounded_rectangle((cx - torso_w // 2, torso_top, cx + torso_w // 2, torso_bottom), radius=10, fill=(*palette.primary, 255), outline=(28, 24, 26, 255), width=3)
    ld.rectangle((cx - torso_w // 2, torso_top + 12, cx + torso_w // 2, torso_top + 16), fill=(255, 255, 255, 34))
    ld.rectangle((cx - torso_w // 2, torso_bottom - 8, cx + torso_w // 2, torso_bottom - 4), fill=(*palette.accent, 232))

    head_r = int(12 * body_scale)
    head_y = torso_top - head_r - 7
    skin = (225, 191, 156) if not is_enemy else _mix((156, 205, 129), (179, 179, 188), 0.35 if unit in {"werewolf", "shadow_bandit"} else 0.0)
    ld.ellipse((cx - head_r, head_y - head_r, cx + head_r, head_y + head_r), fill=(*skin, 255), outline=(34, 28, 26, 255), width=2)

    eye_y = head_y - 1
    eye_color = (20, 20, 20, 255) if not is_enemy else (*palette.glow, 255)
    ld.ellipse((cx - 6, eye_y - 2, cx - 2, eye_y + 2), fill=eye_color)
    ld.ellipse((cx + 2, eye_y - 2, cx + 6, eye_y + 2), fill=eye_color)

    if unit in {"goblin_elite", "boss_warlord"}:
        ld.polygon([(cx - 10, head_y - head_r + 4), (cx - 4, head_y - head_r - 8), (cx - 1, head_y - head_r + 6)], fill=(216, 196, 142, 255))
        ld.polygon([(cx + 10, head_y - head_r + 4), (cx + 4, head_y - head_r - 8), (cx + 1, head_y - head_r + 6)], fill=(216, 196, 142, 255))
    if unit == "werewolf":
        ld.polygon([(cx - 10, head_y - head_r + 2), (cx - 15, head_y - head_r - 8), (cx - 6, head_y - head_r + 1)], fill=(82, 82, 89, 255))
        ld.polygon([(cx + 10, head_y - head_r + 2), (cx + 15, head_y - head_r - 8), (cx + 6, head_y - head_r + 1)], fill=(82, 82, 89, 255))

    shoulder_y = torso_top + 12
    arm_len = int(22 * body_scale)
    ld.line((cx - torso_w // 2, shoulder_y, cx - torso_w // 2 - arm_len, shoulder_y + swing * 0.22), fill=(42, 31, 24, 255), width=5)
    ld.line((cx + torso_w // 2, shoulder_y, cx + torso_w // 2 + arm_len, shoulder_y - swing * 0.22), fill=(42, 31, 24, 255), width=5)

    weapon_len = int(28 * body_scale)
    weapon_angle = -35 + pose["weapon"]
    wx = cx + torso_w // 2 + arm_len
    wy = shoulder_y - swing * 0.22
    tip_x = wx + weapon_len * math.cos(math.radians(weapon_angle))
    tip_y = wy + weapon_len * math.sin(math.radians(weapon_angle))
    ld.line((wx, wy, tip_x, tip_y), fill=(220, 220, 225, 255), width=4)
    ld.ellipse((tip_x - 4, tip_y - 4, tip_x + 4, tip_y + 4), fill=(*palette.accent, 224))

    if state == "attack":
        arc_color = (*palette.glow, 110)
        for i in range(3):
            radius = 14 + i * 5
            ld.arc((tip_x - radius, tip_y - radius, tip_x + radius, tip_y + radius), start=200, end=320, fill=arc_color, width=2)

    if state == "death":
        layer = layer.rotate(lean, center=(cx, floor_y - 8), resample=Image.Resampling.BICUBIC)
        alpha = layer.getchannel("A").point(lambda a: int(a * (1.0 - 0.26 * pose["collapse"])))
        layer.putalpha(alpha)

    if unit == "shadow_bandit":
        tint = Image.new("RGBA", UNIT_SIZE, (52, 34, 80, 60))
        layer = Image.alpha_composite(layer, tint)

    canvas.alpha_composite(layer)
    canvas = canvas.filter(ImageFilter.UnsharpMask(radius=0.8, percent=130, threshold=1))
    path.parent.mkdir(parents=True, exist_ok=True)
    canvas.save(path)


def _draw_portrait(hero: str, legendary: bool, path: Path) -> None:
    palette = UNIT_PALETTES[hero]
    img = Image.new("RGBA", PORTRAIT_SIZE, (0, 0, 0, 0))
    draw = ImageDraw.Draw(img, "RGBA")
    w, h = PORTRAIT_SIZE
    cx, cy = w // 2, h // 2

    bg_top = _mix(palette.secondary, (12, 17, 28), 0.45)
    bg_bottom = _mix(palette.primary, (22, 26, 36), 0.52)
    for y in range(h):
        t = y / max(1, h - 1)
        col = _mix(bg_top, bg_bottom, t)
        draw.line((0, y, w, y), fill=(*col, 255))

    glow = palette.glow if legendary else _mix(palette.glow, (255, 255, 255), 0.4)
    draw.ellipse((cx - 154, cy - 186, cx + 154, cy + 122), fill=(*glow, 42 if not legendary else 84))

    bust_top = cy - 40
    bust_bottom = cy + 190
    draw.rounded_rectangle((cx - 128, bust_top, cx + 128, bust_bottom), radius=42, fill=(*palette.primary, 240), outline=(20, 20, 24, 255), width=4)
    draw.rectangle((cx - 128, bust_top + 52, cx + 128, bust_top + 64), fill=(255, 255, 255, 30))
    draw.rectangle((cx - 128, bust_top + 160, cx + 128, bust_top + 172), fill=(*palette.accent, 210))

    head_r = 72
    draw.ellipse((cx - head_r, cy - 170, cx + head_r, cy - 26), fill=(232, 201, 164, 255), outline=(25, 21, 18, 255), width=3)
    draw.ellipse((cx - 32, cy - 112, cx - 12, cy - 92), fill=(18, 18, 18, 255))
    draw.ellipse((cx + 12, cy - 112, cx + 32, cy - 92), fill=(18, 18, 18, 255))
    draw.arc((cx - 26, cy - 84, cx + 26, cy - 62), start=8, end=172, fill=(90, 48, 40, 255), width=3)

    if legendary:
        crown_y = cy - 198
        draw.polygon(
            [
                (cx - 70, crown_y + 34),
                (cx - 44, crown_y - 2),
                (cx - 16, crown_y + 24),
                (cx, crown_y - 14),
                (cx + 16, crown_y + 24),
                (cx + 44, crown_y - 2),
                (cx + 70, crown_y + 34),
                (cx + 70, crown_y + 52),
                (cx - 70, crown_y + 52),
            ],
            fill=(231, 198, 89, 255),
            outline=(120, 92, 44, 255),
        )
        for offset in (-36, 0, 36):
            draw.ellipse((cx + offset - 8, crown_y + 22, cx + offset + 8, crown_y + 38), fill=(*palette.glow, 255))
    else:
        draw.polygon(
            [(cx - 62, cy - 194), (cx + 62, cy - 194), (cx + 52, cy - 164), (cx - 52, cy - 164)],
            fill=(*_mix(palette.secondary, (255, 255, 255), 0.2), 210),
            outline=(24, 24, 28, 220),
        )

    frame_color = (239, 218, 146) if legendary else (174, 188, 204)
    draw.rounded_rectangle((18, 18, w - 18, h - 18), radius=26, outline=(*frame_color, 235), width=10)
    draw.rounded_rectangle((36, 36, w - 36, h - 36), radius=22, outline=(24, 24, 30, 182), width=3)

    path.parent.mkdir(parents=True, exist_ok=True)
    img.save(path)


def generate() -> dict:
    manifest = {
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "output_root": str(OUTPUT_ROOT),
        "buildings": {},
        "heroes": {},
        "enemies": {},
    }

    building_count = 0
    for building in BUILDINGS:
        entries = []
        for level in (1, 2, 3):
            out = BUILDINGS_ROOT / building / f"lvl{level}.png"
            _draw_building(building, level, out)
            entries.append(str(out))
            building_count += 1
        manifest["buildings"][building] = entries

    hero_count = 0
    for hero in HEROES:
        hero_dir = UNITS_ROOT / "heroes" / hero
        entries = []
        for state, count in HERO_FRAME_COUNTS.items():
            for idx in range(1, count + 1):
                out = hero_dir / f"{state}_{idx:02d}.png"
                _draw_character(hero, state, idx - 1, count, is_enemy=False, path=out)
                entries.append(str(out))
                hero_count += 1
        normal = hero_dir / "portrait_normal.png"
        legendary = hero_dir / "portrait_legendary.png"
        _draw_portrait(hero, legendary=False, path=normal)
        _draw_portrait(hero, legendary=True, path=legendary)
        entries.extend([str(normal), str(legendary)])
        hero_count += 2
        manifest["heroes"][hero] = entries

    enemy_count = 0
    for enemy in ENEMIES:
        enemy_dir = UNITS_ROOT / "enemies" / enemy
        entries = []
        for state, count in ENEMY_FRAME_COUNTS.items():
            for idx in range(1, count + 1):
                out = enemy_dir / f"{state}_{idx:02d}.png"
                _draw_character(enemy, state, idx - 1, count, is_enemy=True, path=out)
                entries.append(str(out))
                enemy_count += 1
        manifest["enemies"][enemy] = entries

    manifest["counts"] = {
        "building_images": building_count,
        "hero_images_including_portraits": hero_count,
        "enemy_images": enemy_count,
        "total_images": building_count + hero_count + enemy_count,
    }

    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)
    (OUTPUT_ROOT / "manifest.json").write_text(json.dumps(manifest, indent=2), encoding="utf-8")
    (OUTPUT_ROOT / "README.txt").write_text(
        "\n".join(
            [
                "Mythical Age Asset Pack",
                "Generated for game usage from ASSET_IMAGE_REQUIREMENTS.md",
                "",
                f"Building images: {building_count}",
                f"Hero images (including portraits): {hero_count}",
                f"Enemy images: {enemy_count}",
                f"Total images: {building_count + hero_count + enemy_count}",
                "",
                "Folder layout:",
                "  - buildings/<BuildingName>/lvl1.png .. lvl3.png",
                "  - units/heroes/<hero>/{idle,walk,attack,hit,death}_NN.png",
                "  - units/heroes/<hero>/portrait_normal.png",
                "  - units/heroes/<hero>/portrait_legendary.png",
                "  - units/enemies/<enemy>/{idle,walk,attack,hit,death}_NN.png",
                "",
                "This pack is deterministic; rerun the script to regenerate the same base style.",
            ]
        ),
        encoding="utf-8",
    )
    return manifest["counts"]


if __name__ == "__main__":
    counts = generate()
    print(json.dumps(counts, indent=2))
