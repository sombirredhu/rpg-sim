"""Clean up semi-transparent edge pixels in building PNGs.

LANCZOS resampling can create pixels with non-zero RGB but very low alpha,
which appear as faint white halos when rendered in-game. This script:
1. Sets fully transparent pixels (alpha=0) to have RGB=(0,0,0)
2. Removes pixels with very low alpha (< 8) that create halos
"""

from pathlib import Path
from PIL import Image

BUILDINGS_ROOT = Path("assets") / "GameplayAssetsV2" / "buildings"


def clean_alpha(img_path: Path) -> bool:
    img = Image.open(img_path).convert("RGBA")
    px = img.load()
    w, h = img.size
    changed = False

    for y in range(h):
        for x in range(w):
            r, g, b, a = px[x, y]
            if a == 0:
                if r != 0 or g != 0 or b != 0:
                    px[x, y] = (0, 0, 0, 0)
                    changed = True
            elif a < 8:
                # Very low alpha = halo artifact, remove it
                px[x, y] = (0, 0, 0, 0)
                changed = True

    if changed:
        img.save(img_path)
    return changed


def main():
    count = 0
    for png in sorted(BUILDINGS_ROOT.rglob("*.png")):
        if clean_alpha(png):
            print(f"Cleaned: {png}")
            count += 1
        else:
            print(f"OK:      {png}")
    print(f"\nCleaned {count} files")


if __name__ == "__main__":
    main()
