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

- **NEVER merge branches into main** without explicit user permission.
- Work on feature branches. Always commit before switching branches.
- Do not interact with any branch other than current unless instructed.
- All testing must be performed only within the current branch.

## Execution Rules

### Core Constraints
- Never switch/merge/modify branches without explicit permission.
- All work, testing, execution stay strictly in current active branch.
- Only work on explicitly requested feature or task.
- Do not scan, analyze, or refactor entire codebase unless instructed.
- Limit file access to only those required for current task.

### Feature Workflow ("start creating pending feature")
1. Create/switch to dedicated feature branch (create if not exists).
2. Read **only** PENDING_FEATURES.md; select next feature.
3. Update the feature status in PENDING_FEATURES.md (change [ ] to [WIP] or add [WIP] marker).
4. Implement **only** that feature.
5. Validate alignment with GDD; adjust/redefine until aligned.
6. Test game strictly within current branch.
7. Run `cargo run` and verify smooth operation; fix only issues caused by this feature.
8. After testing, always close the game.
9. Stop after one feature unless explicitly instructed to continue.
10. Update the feature status in PENDING_FEATURES.md (change [WIP] to [x]).
11. Do not consider a task complete until the game runs without issues after your changes.

### Multiple Features
If instructed to create multiple/all pending features:
- Repeat full Feature Workflow sequentially.
- Complete one feature fully before starting next.

### Context & Optimization
- After completing a feature, **discard all feature-specific context**.
- Do not reuse previous feature context.
- Retain only: GDD, current feature requirements, core system rules.
- If full clearing impossible: strictly ignore previous feature details.
- Prefer minimal, direct code changes over large rewrites.
- Avoid re-reading unchanged files; avoid regenerating code.
- Avoid verbose explanations; summarize reasoning, don't expand.

### Testing & Errors
- Test only functionality impacted by current feature.
- Do not run full project tests unless explicitly required.
- Fix only errors related to current feature scope.
- Do not attempt global optimizations or unrelated bug fixes.

### Communication & Safety
- Respond with concise outputs; prefer code over explanations.
- Do not repeat previous outputs unless necessary.
- Do not modify main or shared branches.
- Do not overwrite unrelated code.
- Ensure changes isolated to current feature branch.

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
