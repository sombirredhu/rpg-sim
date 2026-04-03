# Asset Image Requirements (Buildings and Units)

This file lists the image assets needed so:
- Every building level looks clearly different.
- Units (heroes and enemies) have complete visual sets.

## 1) Building Images (Level-Based)

Goal: each building must have a clear progression from simple to advanced.
- Level 1: basic, small, low-detail.
- Level 2: larger silhouette, clearer materials, added function props.
- Level 3: biggest and strongest look, fortified and premium detail.

Recommended naming:
- `assets/buildings/<building_name>/lvl1.png`
- `assets/buildings/<building_name>/lvl2.png`
- `assets/buildings/<building_name>/lvl3.png`
- Optional: `damaged.png`, `ruined.png`

Building list (from game code):
- TownHall (castle core)
- Inn
- Market
- Temple
- GuardTower
- WizardTower
- Blacksmith
- Alchemist
- Barracks

Required level variants:
- TownHall: `lvl1`, `lvl2`, `lvl3`
- Inn: `lvl1`, `lvl2`, `lvl3`
- Market: `lvl1`, `lvl2`, `lvl3`
- Temple: `lvl1`, `lvl2`, `lvl3`
- GuardTower: `lvl1`, `lvl2`, `lvl3`
- WizardTower: `lvl1`, `lvl2`, `lvl3`
- Blacksmith: `lvl1`, `lvl2`, `lvl3`
- Alchemist: `lvl1`, `lvl2`, `lvl3`
- Barracks: `lvl1`, `lvl2`, `lvl3`

Minimum building image count:
- 9 buildings x 3 levels = 27 building images

Recommended extra building images:
- `damaged.png` for each building (9)
- `ruined.png` for each building (9)
- `construction_stage1.png` and `construction_stage2.png` for each building (18)

## 2) Unit Images (Heroes and Enemies)

Recommended naming:
- `assets/units/<faction>/<unit_name>/<state>_<frame>.png`
- Example: `assets/units/heroes/warrior/walk_01.png`

### Heroes (required)
- Warrior
- Archer
- Mage
- Rogue
- Healer

Hero image set per class:
- Idle: 2 frames
- Walk: 4 to 8 frames
- Attack: 4 to 6 frames
- Hit/React: 2 frames
- Death/Downed: 4 frames
- Portrait normal: 1 image
- Portrait legendary: 1 image

Minimum hero deliverables:
- 5 hero classes with full animation strips
- 10 portraits total (normal + legendary)

### Enemies (required)
- Goblin
- Bandit
- Troll
- GoblinElite
- BossWarlord
- Werewolf
- ShadowBandit

Enemy image set per type:
- Idle: 2 frames
- Walk: 4 to 8 frames
- Attack: 4 to 6 frames
- Hit/React: 2 frames
- Death: 4 frames

Minimum enemy deliverables:
- 7 enemy types with full animation strips

## 3) Visual Progression Rules (Important)

Use these rules so upgrades feel meaningful:
- Level 1 buildings should be narrow/simple, wood and cloth heavy.
- Level 2 buildings should add stone sections, secondary roof pieces, and more props.
- Level 3 buildings should have thicker walls, reinforced corners, banners/lights, and stronger silhouettes.
- Keep footprint readable at gameplay zoom levels.
- Keep color families consistent per building type across all levels.

## 4) Optional But High Value Images

- Resource node upgrades:
  - Mine `lvl1/lvl2/lvl3`
  - LumberMill `lvl1/lvl2/lvl3`
- Merchant caravan variants: cart `lvl1/lvl2/lvl3`
- Night lighting overlays per building level:
  - `torch_glow_lvl1/lvl2/lvl3`
- UI building icons per level for build/upgrade menus

## 5) Summary Count (Core Scope)

Core minimum to start:
- Buildings: 27 images
- Heroes: 5 full unit sets + 10 portraits
- Enemies: 7 full unit sets

If you also include damaged/ruined building states:
- +18 building images (9 damaged + 9 ruined)

