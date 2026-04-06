# CLAUDE.md

## Project Overview

**Realm of Bounties** (`realm-of-bounties`) — 2D top-down mobile kingdom simulation inspired by *Majesty: The Fantasy Kingdom Sim*. Player governs by incentive (bounties, buildings, economy), never by direct command.

Built with **Bevy 0.6.1**, Rust 2021 edition. No automated tests.

## Tech Stack

- **Engine:** Bevy 0.6.1 (not modern Bevy — see Bevy 0.6 Notes below)
- **Language:** Rust 2021
- **Deps:** `bevy 0.6.1`, `bevy_ecs_ldtk 0.2.0`, `rand 0.8.5`, `serde 1.0`, `serde_json 1.0`
- **Sprites:** LPC-style 4-directional character sheets, per-tier building PNGs
- **Lint:** `clippy.toml` — `too-many-arguments-threshold = 10`, `type-complexity-threshold = 1000`

## Commands

```bash
cargo run              # Build and run (dev profile, opt-level=1)
cargo run --release    # Release build
cargo check            # Type-check without building
cargo clippy           # Lint
cargo build            # Build only
```

## Git Workflow

- **NEVER merge branches into main unless the user explicitly asks you to.**
- Work on feature branches when needed.
- Always commit changes before switching branches.

## Project Structure

### Source Files (`src/`, ~6,750 lines)

| File | Lines | Purpose |
|------|-------|---------|
| `main.rs` | ~177 | App setup, system registration, startup functions |
| `components.rs` | ~1,490 | All ECS components, resources, events, enums |
| `hero.rs` | ~544 | Hero AI decisions, movement, leveling |
| `combat.rs` | ~285 | Hero/enemy attacks, healing, equipment damage |
| `enemy.rs` | ~261 | Enemy spawning, AI, threat escalation, boss raids |
| `building.rs` | ~206 | Building placement, upgrades, repairs, guard towers |
| `economy.rs` | ~130 | Tax collection, bounty payouts, kingdom progression |
| `features.rs` | ~1,483 | Roads, merchants, caravans, night enemies, buffs, animations, fog of war |
| `ui.rs` | ~724 | HUD setup and per-frame UI updates |
| `sprites.rs` | ~912 | Sprite asset loading, entity spawning with visuals |
| `day_night.rs` | ~100 | Day/night cycle, speed control, night overlay |
| `camera.rs` | ~56 | Camera panning and zooming |
| `audio.rs` | ~59 | Audio asset loading and SFX event system |
| `art_catalog.rs` | ~320 | Legacy art asset specifications (mostly unused) |

### Key Asset Paths

- `assets/Character/LPC/{class}/` — Hero walk/attack/hurt sprite sheets (9×4 grids)
- `assets/Character/LPC/skeleton/` — Enemy skeleton animations
- `assets/GameplayAssetsV2/buildings/{type}/lvl{1,2,3}.png` — Tiered building sprites
- `assets/Audio/Music/`, `assets/Audio/SFX/` — Background music and sound effects
- `assets/Level/Ground/` — Tile textures (grass, stone, water, rock, road)
- `assets/fonts/FiraSans-Bold.ttf` — UI font

### Key ECS Types (all in `components.rs`)

**Resources:** `GameEconomy`, `BountyBoard`, `GameTime`, `KingdomState`, `RoadNetwork`, `FogOfWar`, `GamePhase`, `GameAlerts`, `ActiveBuffs`, `BuildingBonuses`, `EraState`, `LegacyUpgrades`, `Milestones`

**Components:** `Hero`, `HeroStats`, `HeroEquipment`, `Building`, `Enemy`, `EnemyAi`, `MonsterDen`, `Merchant`, `TradeCaravan`, `Road`, `SpriteAnimation`, `AnimationSet`, `AttackCooldown`

**Events:** `BountyCompletedEvent`, `HeroDeathEvent`, `BuildingDestroyedEvent`, `EnemyDeathEvent`, `ThreatEscalationEvent`, `HeroSpawnEvent`, `SfxEvent`

**Key enums:** `HeroClass` (5 classes), `HeroState` (10 states), `HeroPersonality` (4 types), `BuildingType` (9 types), `EnemyType` (7 types), `BountyType` (4 types), `KingdomRank` (5 ranks), `TimeOfDay` (4 phases)

## Bevy 0.6 Notes

This is **not** modern Bevy. Key API differences:
- `bevy::sprite::Rect` (not `bevy::math::Rect`)
- `WindowDescriptor` resource for window config
- `Windows` resource (not `Window` component)
- `Input<MouseButton>` resource for input
- `.add_system()` (not `.add_systems()`)
- No `App::new().run()` pattern
- Startup stages: `StartupStage::Startup` (highest priority) then `StartupStage::PostStartup`

## Map Constants

- 20×15 tile grid, `TILE_SIZE = 40.0`, origin at `(-380, 260)` world space
- Town zone: `x ∈ [4,8], y ∈ [5,9]`
- 4 sectors; higher sectors require more `revealed_sectors`

## Game Design Reference

The full Game Design Document (GDD v1.0) is at [Realm_of_Bounties_GDD.txt](Realm_of_Bounties_GDD.txt). It covers hero classes, buildings, economy rules, enemies, day/night cycle, kingdom progression, and mobile UI design. Core concepts:
- 5 hero classes (Warrior/Archer/Mage/Rogue/Healer) with personality-driven AI
- 9 building types with 3 upgrade tiers each
- Gold-only economy with tax/bounty/merchant income
- Day/night cycle (8 min real-time), night = +50% threat spawn
- Kingdom ranks: Hamlet → Village → Town → City → Kingdom

## Feature Tracking

All features and their current implementation status are tracked in [PENDING_FEATURES.md](PENDING_FEATURES.md). Check it before working on any feature to understand what's implemented, partial, or missing.

## Additional Documentation

Check these files for detailed patterns when working on related code:

- [Architectural Patterns](.claude/docs/architectural_patterns.md) — ECS patterns, state management, system communication, asset loading, and combat model conventions used across the codebase

## Workflow: Troubleshoot After Every Task

After completing every task, you **must** troubleshoot the game by running it:

1. Run `cargo run` to build and launch the game
2. Verify the game starts and runs smoothly without errors or crashes
3. If the game fails to compile, crashes, or exhibits broken behavior:
   - Read the error output carefully to identify the root cause
   - Fix the issue(s) in the code
   - Re-run `cargo run` to verify the fix
4. Repeat step 3 until the game runs smoothly
5. After testing alwasy close the game app (which was created with `cargo run`)

Do not consider a task complete until the game runs without issues after your changes.
