from PIL import Image, ImageDraw, ImageFilter
import random
import math
import os

deco_dir = "C:/Users/Sombir/projects/Majesty Game/assets/Level/Decorations"

def make_flower_patch(size=64, seed=10):
    random.seed(seed)
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    flower_colors = [
        (220, 60, 80), (240, 200, 50), (180, 80, 220),
        (240, 140, 50), (230, 230, 240), (80, 140, 220),
    ]
    for _ in range(80):
        x = random.randint(8, size-8)
        y = random.randint(size//2, size-4)
        h = random.randint(4, 12)
        green = random.randint(80, 140)
        draw.line([(x, y), (x + random.randint(-2,2), y - h)], fill=(40, green, 30, 200), width=1)
    for _ in range(8):
        x = random.randint(12, size-12)
        y = random.randint(size//3, size-8)
        color = random.choice(flower_colors)
        r = random.randint(3, 5)
        for angle in range(0, 360, 72):
            px = x + int(r * math.cos(math.radians(angle)))
            py = y + int(r * math.sin(math.radians(angle)))
            draw.ellipse([px-2, py-2, px+2, py+2], fill=(*color, 230))
        draw.ellipse([x-2, y-2, x+2, y+2], fill=(240, 220, 50, 255))
    img = img.filter(ImageFilter.GaussianBlur(0.4))
    return img

def make_mushroom_cluster(size=48, seed=20):
    random.seed(seed)
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    positions = [(size//2, size-10), (size//2-10, size-6), (size//2+8, size-8)]
    cap_colors = [(180, 40, 30), (200, 50, 35), (170, 35, 25)]
    for i, (x, y) in enumerate(positions):
        h = random.randint(8, 14)
        cap_w = random.randint(6, 10)
        draw.rectangle([x-2, y-h, x+2, y], fill=(220, 210, 180, 240))
        draw.ellipse([x-cap_w, y-h-cap_w//2, x+cap_w, y-h+cap_w//2+2], fill=(*cap_colors[i], 240))
        for _ in range(3):
            sx = x + random.randint(-cap_w+2, cap_w-2)
            sy = y - h + random.randint(-cap_w//2+1, 1)
            draw.ellipse([sx-1, sy-1, sx+1, sy+1], fill=(255, 255, 240, 200))
    img = img.filter(ImageFilter.GaussianBlur(0.3))
    return img

def make_standing_stone(size=80, seed=30):
    random.seed(seed)
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    cx = size // 2
    base_y = size - 8
    stone_w = 12
    stone_h = 45
    points = [
        (cx - stone_w, base_y), (cx - stone_w - 2, base_y - stone_h//2),
        (cx - stone_w + 3, base_y - stone_h + 5), (cx - 3, base_y - stone_h - 5),
        (cx + 3, base_y - stone_h - 3), (cx + stone_w - 2, base_y - stone_h + 8),
        (cx + stone_w + 1, base_y - stone_h//2), (cx + stone_w, base_y),
    ]
    draw.polygon(points, fill=(140, 135, 125, 240))
    shade_points = points[:4] + [points[0]]
    draw.polygon(shade_points, fill=(120, 115, 105, 180))
    for _ in range(5):
        mx = cx + random.randint(-stone_w+2, stone_w-2)
        my = base_y - random.randint(5, stone_h - 10)
        draw.ellipse([mx-3, my-2, mx+3, my+2], fill=(60, 100, 45, 120))
    draw.ellipse([cx-stone_w-4, base_y-3, cx+stone_w+4, base_y+5], fill=(30, 50, 20, 80))
    img = img.filter(ImageFilter.GaussianBlur(0.5))
    return img

def make_stone_circle(size=96, seed=40):
    random.seed(seed)
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    cx, cy = size//2, size//2 + 8
    radius = 28
    for i in range(6):
        angle = i * 60 + random.randint(-10, 10)
        sx = cx + int(radius * math.cos(math.radians(angle)))
        sy = cy + int(radius * 0.6 * math.sin(math.radians(angle)))
        stone_h = random.randint(10, 18)
        stone_w = random.randint(5, 8)
        shade = random.randint(110, 150)
        draw.polygon([
            (sx - stone_w, sy), (sx - stone_w + 2, sy - stone_h),
            (sx + stone_w - 2, sy - stone_h + 2), (sx + stone_w, sy),
        ], fill=(shade, shade-5, shade-15, 230))
        draw.ellipse([sx-stone_w-2, sy-2, sx+stone_w+2, sy+3], fill=(30, 45, 20, 60))
    img = img.filter(ImageFilter.GaussianBlur(0.5))
    return img

def make_big_tree(size=128, seed=50):
    random.seed(seed)
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    cx = size // 2
    base_y = size - 10
    trunk_w = 8
    trunk_h = 40
    draw.polygon([(cx-trunk_w, base_y), (cx-trunk_w+2, base_y-trunk_h),
                  (cx+trunk_w-2, base_y-trunk_h), (cx+trunk_w, base_y)], fill=(95, 65, 35, 240))
    draw.polygon([(cx-trunk_w, base_y), (cx-trunk_w+2, base_y-trunk_h),
                  (cx, base_y-trunk_h), (cx-2, base_y)], fill=(75, 50, 25, 200))
    canopy_y = base_y - trunk_h - 10
    leaf_positions = [
        (cx, canopy_y, 30), (cx-18, canopy_y+8, 22), (cx+18, canopy_y+8, 22),
        (cx-8, canopy_y-15, 20), (cx+8, canopy_y-15, 20), (cx, canopy_y-22, 18),
    ]
    for lx, ly, lr in leaf_positions:
        draw.ellipse([lx-lr, ly-lr, lx+lr, ly+lr], fill=(35, 75, 25, 220))
    for lx, ly, lr in leaf_positions:
        r2 = int(lr * 0.85)
        draw.ellipse([lx-r2+2, ly-r2, lx+r2+2, ly+r2], fill=(50, 100, 35, 210))
    for lx, ly, lr in leaf_positions:
        r3 = int(lr * 0.5)
        draw.ellipse([lx-r3+3, ly-r3-2, lx+r3+3, ly+r3-2], fill=(65, 120, 45, 180))
    draw.ellipse([cx-25, base_y-3, cx+25, base_y+6], fill=(25, 40, 15, 70))
    img = img.filter(ImageFilter.GaussianBlur(0.6))
    return img

def make_pine_tree(size=128, seed=60):
    random.seed(seed)
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    cx = size // 2
    base_y = size - 10
    draw.polygon([(cx-5, base_y), (cx-3, base_y-25), (cx+3, base_y-25), (cx+5, base_y)], fill=(90, 60, 30, 240))
    layers = [(base_y-20, 35, 20), (base_y-38, 28, 18), (base_y-52, 22, 16), (base_y-64, 16, 14), (base_y-74, 10, 10)]
    for ly, w, h in layers:
        draw.polygon([(cx, ly-h), (cx-w, ly), (cx+w, ly)], fill=(25, 65, 20, 230))
        draw.polygon([(cx+2, ly-h+2), (cx-w+5, ly), (cx+w-2, ly)], fill=(35, 85, 30, 220))
        draw.polygon([(cx+3, ly-h+4), (cx-w+10, ly-2), (cx+w-6, ly-2)], fill=(50, 105, 40, 180))
    draw.ellipse([cx-18, base_y-3, cx+18, base_y+5], fill=(25, 40, 15, 60))
    img = img.filter(ImageFilter.GaussianBlur(0.5))
    return img

def make_bush_nice(size=48, seed=70):
    random.seed(seed)
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    cx, cy = size//2, size//2 + 8
    positions = [(cx, cy, 16), (cx-8, cy+3, 12), (cx+8, cy+3, 12), (cx, cy-6, 11)]
    for bx, by, br in positions:
        draw.ellipse([bx-br, by-br, bx+br, by+br], fill=(40, 80, 30, 220))
    for bx, by, br in positions:
        r2 = int(br * 0.8)
        draw.ellipse([bx-r2+1, by-r2-1, bx+r2+1, by+r2-1], fill=(55, 105, 40, 210))
    for bx, by, br in positions:
        r3 = int(br * 0.4)
        draw.ellipse([bx-r3+2, by-r3-2, bx+r3+2, by+r3-2], fill=(70, 125, 50, 170))
    draw.ellipse([cx-18, cy+10, cx+18, cy+16], fill=(25, 40, 15, 60))
    img = img.filter(ImageFilter.GaussianBlur(0.5))
    return img

print("Generating decorations...")
make_flower_patch(64, seed=10).save(os.path.join(deco_dir, "flowers1_hd.png"))
make_flower_patch(64, seed=15).save(os.path.join(deco_dir, "flowers2_hd.png"))
make_mushroom_cluster(48, seed=20).save(os.path.join(deco_dir, "mushrooms_hd.png"))
make_standing_stone(80, seed=30).save(os.path.join(deco_dir, "standing_stone_hd.png"))
make_standing_stone(80, seed=35).save(os.path.join(deco_dir, "standing_stone2_hd.png"))
make_stone_circle(96, seed=40).save(os.path.join(deco_dir, "stone_circle_hd.png"))
make_big_tree(128, seed=50).save(os.path.join(deco_dir, "tree_big1_hd.png"))
make_big_tree(128, seed=55).save(os.path.join(deco_dir, "tree_big2_hd.png"))
make_pine_tree(128, seed=60).save(os.path.join(deco_dir, "pine_hd1.png"))
make_pine_tree(128, seed=65).save(os.path.join(deco_dir, "pine_hd2.png"))
make_bush_nice(48, seed=70).save(os.path.join(deco_dir, "bush_hd1.png"))
make_bush_nice(48, seed=75).save(os.path.join(deco_dir, "bush_hd2.png"))
print("All decorations generated!")
