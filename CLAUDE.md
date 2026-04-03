# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Realm of Bounties** (package name: `rpg-sim`) is a 2D top-down mobile kingdom simulation inspired by *Majesty: The Fantasy Kingdom Sim*. The player rules as a king but never gives direct orders — instead, they build a town, attract autonomous heroes, and influence behaviour by placing gold bounties on threats, objectives, and exploration targets.

**Core Design Pillar:** You are the ruler, not the soldier. You govern by incentive, infrastructure, and resource management — never by direct command.

- **Genre:** 2D Mobile Kingdom Simulation / Indirect Strategy
- **Platform:** iOS & Android (mobile-first), portrait & landscape
- **Target Audience:** Casual to mid-core strategy players, ages 14+
- **Session Length:** 5–20 minutes per session
- **Monetisation:** Free-to-play with cosmetic-only IAP (no pay-to-win)

Built with **Bevy 0.6.1** on Rust 2021 edition.

## Game Design (from GDD v1.0)

### Core Gameplay Loop

1. **Build & Expand** — Construct/upgrade town buildings, plan road networks, expand map perimeter to reveal new zones.
2. **Attract Heroes** — Heroes arrive at the town gate when the right buildings exist (e.g., inn → warriors, wizard tower → mages).
3. **Set Bounties** — Place gold rewards on monster dens, ruins, resource nodes, or bosses via the Bounty Board. Heroes autonomously evaluate bounties based on risk tolerance, gold, and skill.
4. **Manage Economy** — Collect taxes, balance spending between bounties/upgrades/reserves. Avoid bankruptcy (idle heroes leave).
5. **Survive Threats** — Waves of goblins, bandits, and monsters attack (especially at night). Boss events require coordinated bounty investment.

### Indirect Control System

Heroes are never directly commanded. Their AI weighs: bounty value, perceived danger, current needs (hunger/injury/morale), personality traits (Brave ignores danger; Cautious demands higher pay), and distance to target. The Bounty Board is the player's primary tool — bounties are paid only on completion (no wasted gold).

**Bounty Types:** Monster Bounties, Exploration Bounties, Objective Bounties (escort/defend), Resource Bounties.

### Hero Classes

| Class | Role | Combat Style | Special Ability |
|-------|------|-------------|-----------------|
| Warrior | Front-line tank | Melee — charges, holds aggro | Fortify: damage reduction aura |
| Archer | Ranged DPS | Ranged — kites, hits flyers | Volley: AoE arrow rain |
| Mage | AoE damage | Spells — high burst | Arcane Surge: multi-target blast |
| Rogue | Scout/assassin | Stealth — unseen entry, priority kills | Backstab: crit on unaware enemies |
| Healer | Support/sustain | Passive — follows party, heals | Sanctuary: healing pulse + revive |

**Progression:** XP from bounties/combat → Perk Point every 5 levels → Legendary at level 10+ (golden border, stat boost). Death = incapacitation (not permanent) unless recovery bounty is neglected.

### Buildings & Upgrades

| Building | Cost | Function | Unlock |
|----------|------|----------|--------|
| Town Hall | Starting | Core hub, kingdom level cap, map expansions | Always |
| Inn | 150g | Attracts warriors & rogues, HP/morale rest | Town Hall T1 |
| Market | 200g | Passive tax income, merchant events | Town Hall T1 |
| Temple | 250g | Attracts healers, morale boost aura | Town Hall T2 |
| Guard Tower | 300g | Auto arrow defense, slows enemies | Inn T1 |
| Wizard Tower | 400g | Attracts mages, arcane research | Temple T1 |
| Blacksmith | 350g | Hero attack/armor boost, crafting | Inn T2 |
| Alchemist | 300g | Potions, reduced recovery time | Temple T2 |
| Barracks | 450g | Hero party cap, squad formation bounties | Guard Tower T2 |

**Upgrade Tiers:** T1 = improved base function, T2 = new passive ability, T3 = major transformation (e.g., Temple T3 → Cathedral: 50g/day pilgrim income).

**Roads:** Paved roads +30% movement speed. Roads between buildings improve supply chains (tax bonus, craft speed). Bridges cross rivers to open new sectors.

### Economy

Gold is the single resource. **Income:** property tax (per building tier), market trade (periodic merchants), bounty taxes (% returned to treasury), event rewards, resource nodes. **Expenditure:** building construction/upgrades, bounty payments, emergency repairs, hero recruitment bonuses.

**Rule:** Never let treasury reach zero — keep 200g reserve for emergency repairs after boss raids.

### Enemies & Threat System

| Threat | Description |
|--------|-------------|
| Goblin Rabble | Weak, large numbers from goblin camps. Good for low-level heroes |
| Bandit Gang | Organised, targets market/caravans. Mid-tier, needs warriors/rogues |
| Cave Troll | Tanky/slow, destroys guard towers. Needs mixed hero party |
| Monster Lair | Persistent spawn point, generates monsters nightly until cleared |
| Dungeon Lord | End-of-era boss siege. All bounties stack, all classes participate |

**Threat Escalation:** Unaddressed zones escalate weekly (goblin camp → Stronghold → Warlord that raids town). **Dynamic Spawning:** From map edges and existing dens; harder threats at night and during storms. Clearing a lair permanently removes it, but new lairs appear in explored zones.

### Day/Night Cycle

One in-game day = 8 minutes real-time. **Day:** Heroes most active, merchants visit, safe for expansion. **Night:** +50% threat spawn, night-only enemies (werewolves, shadow bandits), low-morale heroes refuse to leave inn without high bounties, torch defense bonus within town light radius.

### Kingdom Progression

**Ranks:** Hamlet → Village → Town → City → Kingdom (each unlocks new zones, enemies, buildings).

**Era System:** Each run = 30–60 in-game days, ending with a Dungeon Lord siege. Score based on wealth/heroes/buildings.

**Roguelite Meta:** Era completion → Legacy Points for permanent bonuses (+10% tax, heroes start at level 2). Challenge Eras: "No Inns", "Double Bounties", "Permanent Death".

### Mobile UI Design

- One-handed play, 48×48pt minimum tap targets
- Gold counter with income-per-minute, day/night clock arc, collapsible hero panel, toast alerts
- Bounty Board via floating action button (most important screen)
- Drag-and-drop road placement with snap-to-grid
- Offline progress (passive income + hero work), optional push notifications
- Speed toggle: 1×, 2×, or pause

### Art & Audio Direction

- Hand-painted pixel art, warm storybook medieval palette
- 32×32 hero sprites with walk/attack/idle/rest animations
- Day/night via real-time colour overlay with torch halos
- Adaptive music: calm lute (day) → tense orchestral strings (night)
- Per-class sound effects, layered town ambience by building count
- Boss raid horn alert

## Commands

```bash
# Build and run
cargo run

# Run with faster compilation (dev profile already uses opt-level=1)
cargo run --release

# Check for errors without building
cargo check

# Lint
cargo clippy

# Build only
cargo build
```

There are no automated tests in the project. The `clippy.toml` raises thresholds: `too-many-arguments-threshold = 10`, `type-complexity-threshold = 1000`.

## Architecture

The entire game logic lives in two files:

- **`src/main.rs`** — All game systems, components, state, and ECS wiring (~1200+ lines)
- **`src/art_catalog.rs`** — `ArtCatalog` resource that loads all sprite atlases and textures at startup

### Key Data Model

| Type | Role |
|------|------|
| `GameState` (Resource) | Gold, day/era tracking, building flags, bounty states, castle HP |
| `MapState` (Resource) | 20×15 grid of `TileState` (kind, building, bridge) |
| `PlacementState` (Resource) | Transient state for building/road/bridge placement mode |
| `AutoSaveState` (Resource) | 30-second autosave timer |
| `Hero` (Component) | Class, personality, guild, task, stats, timers |
| `MonsterZone` (Component) | Threat zones with HP, bounty, raid timer |
| `Civilian` (Component) | Role, home, work state |
| `Merchant` (Component) | Escort mission entity |
| `ArtCatalog` (Resource) | All loaded texture atlas handles |

### Map Layout

- 20×15 tile grid, `TILE_SIZE = 40.0`, origin at `(-380, 260)` in world space
- Tile kinds: Grass, Road, Forest, Mountain, Water, Ruins
- Map is divided into 4 sectors; higher-numbered sectors require more `revealed_sectors` to build on
- Town zone for core buildings: `x ∈ [4,8], y ∈ [5,9]`

### Hero AI Loop

Heroes operate on `decision_timer` (every 2s) and `service_timer` (every 7s). Each tick, `hero_ai_system` assigns a `HeroTask` based on class, personality, morale, available bounties, and building presence. Heroes then move via `MoveTarget` component updated by `hero_movement_system`. Tasks include: `Patrol`, `RecoverAtInn`, `PrayAtTemple`, `ClaimBounty`, `ExploreRuins`, `GatherResource`, `DefendTown`, `EscortMerchant`, `LayLow`, `Incapacitated`.

### Bevy System Registration (in `main()`)

Systems registered in order: `button_interactions`, `map_input_system`, `update_map_visuals_system`, `update_clock`, `scheduled_threats_system`, `kingdom_growth_system`, `apply_recovery_payments_system`, `merchant_event_system`, `merchant_movement_system`, `daily_realm_system`, `zone_pressure_system`, `tower_attack_system`, `hero_ai_system`, `hero_movement_system`, `hero_bounty_resolution_system`, `hero_service_economy_system`, `civilian_ai_system`, `civilian_movement_system`, `update_fog_of_war_system`, `update_day_night_overlay_system`, `autosave_system`, `new_era_system`, `update_hero_labels_system`, `update_zone_labels_system`, `update_zone_markers_system`, `update_ui_system`.

### Save System

Game autosaves every 30 seconds (and on quit) to a JSON file via `serde`/`serde_json`. The `SaveSnapshot` struct captures full game state: `SaveStateSnapshot`, tile grid, hero snapshots, civilian snapshots, and zone snapshots. Save version field is present for future migrations.

### Assets

All assets are loaded from the `assets/` directory. Key paths:
- `Level/Building/BlueBuilding.png`, `RedBuilding.png` — building sprite sheets
- `Character/Leader/*/` — hero sprites (Daniel=Warrior, Robin=Archer, Raja=Mage, Rollo=Rogue, Sami=Healer, Maori=Farmer)
- `Character/Creep/MeleeCreep/`, `RangeCreep/` — enemy sprites
- `Level/Ground/` — tile textures (grass, brick, water)
- `Level/Tress/Trees.png` — forest decoration

### Bevy Version Notes

This uses **Bevy 0.6.1** (not current). Key differences from modern Bevy:
- `bevy::sprite::Rect` is used directly (not `bevy::math::Rect`)
- `WindowDescriptor` resource for window setup
- `Windows` resource (not `Window` component)
- `Input<MouseButton>` resource
- No `App::new().run()` — systems use `.add_system()` not `.add_systems()`
