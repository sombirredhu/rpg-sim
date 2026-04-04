# Architectural Patterns

Patterns observed across multiple files in the Realm of Bounties codebase.

## 1. ECS Component/Resource Split

All ECS definitions live in `components.rs`. The codebase follows a strict convention:

- **Resources** = singleton game state (`GameEconomy`, `BountyBoard`, `GameTime`, etc.)
- **Components** = per-entity data (`Hero`, `Building`, `Enemy`, etc.)
- **Marker components** = zero-size structs for query filtering (`MainCamera`, `Road`, `GoldText`, `NightOverlay`, etc.)
- **Events** = inter-system communication (`BountyCompletedEvent`, `EnemyDeathEvent`, etc.)

Each enum type (e.g. `HeroClass`, `BuildingType`, `EnemyType`) defines its own `impl` block with factory methods for stats, costs, and display names — keeping game data co-located with the type definition rather than in systems.

## 2. Event-Driven System Communication

Systems do not call each other directly. Instead, they communicate through Bevy events:

1. Combat system kills enemy → emits `EnemyDeathEvent` (with xp_reward, gold_reward, killer entity)
2. Hero system completes bounty → emits `BountyCompletedEvent` (bounty_id, hero_entity, gold_reward)
3. Economy system reads these events → updates `GameEconomy`, pays heroes
4. Building destroyed → `BuildingDestroyedEvent` → triggers repair bounty logic
5. Threat escalation → `ThreatEscalationEvent` → updates den visuals/difficulty

**Pattern:** Producer system emits event → Consumer system reads with `EventReader<T>` → updates resources/components.

## 3. Timer-Based Decision Making

Multiple systems use cooldown timers to throttle expensive logic:

- `HeroDecisionTimer`: Hero AI re-evaluates every 2-3 seconds (`hero.rs`)
- `AttackCooldown`: Per-entity attack throttle with configurable interval (`combat.rs`)
- `MonsterDen.spawn_timer`: Enemy spawn intervals per den (`enemy.rs`)
- `Merchant.leave_timer`: Merchant departure countdown (`features.rs`)
- `GameAlerts.messages`: Alert text with 5-second TTL (`ui.rs`)

**Pattern:** Store `Timer` or `f32` on component/resource → decrement by `time.delta_seconds()` each frame → act when <= 0 → reset.

## 4. Scoring/Weighting for AI Decisions

Hero AI (`hero.rs`) uses a scoring formula to pick bounties:

```
score = base_gold - distance*0.1 + personality_modifier(danger) ± class_affinity
```

Personality modifiers:
- **Brave:** `+danger*2` (seeks danger)
- **Cautious:** `-danger*15` (avoids)
- **Greedy:** `0` (pure gold focus)
- **Loyal:** `-danger*5` (moderate caution)

Risk gate: if `danger > risk_tolerance*5+1`, score is multiplied by 0.1.

This pattern keeps all behavior variation in data (personality enum → modifier) rather than branching code paths.

## 5. Tier-Based Progression

Buildings, equipment, and enemy dens all use a numeric tier system:

- **Buildings** (`BuildingType`): tier 0-3, each tier changes cost/function via `upgrade_cost(tier)`, `tax_income(tier)`, unlocks abilities at specific tiers
- **Equipment** (`EquipmentTier`): Iron → Steel → Mithril → Legendary, created via `from_blacksmith_tier(building_tier)`
- **Monster Dens** (`MonsterDen`): `threat_tier` 0-3, escalates when unaddressed (interval shrinks, max_spawned increases)
- **Building Sprites**: `BuildingSpriteSet.for_tier()` maps tier → lvl1/lvl2/lvl3 texture handle

**Pattern:** Tier is a simple integer stored on the component. Tier-dependent behavior is computed via match/methods on the type, not stored separately.

## 6. Startup Stage Ordering

Asset loading must complete before entities spawn. The codebase enforces this via Bevy's startup stages:

- `StartupStage::Startup` — `load_sprite_assets` (loads all textures/atlases into `SpriteAssets` resource)
- `StartupStage::PostStartup` — `spawn_ground_tiles`, `spawn_heroes`, `setup_ui`, etc. (consume loaded handles)

**Pattern:** All asset handles stored in a single `SpriteAssets` resource → PostStartup systems read from it to create entities.

## 7. LPC Animation Convention

All characters use LPC-format sprite sheets:

- **Layout:** 4 rows (up/left/down/right) × N columns per animation
- **Walk:** 9 frames/row in `walkcycle.png`
- **Attack:** 6-13 frames/row in `slash.png` / `bow.png` / `spellcast.png`
- **Hurt:** 6 frames × 1 row in `hurt.png`

The `SpriteAnimation` component tracks `current_frame`, `row_offset` (direction), and `frame_timer`. Direction is updated by `apply_hero_facing()` based on movement vector. Mode switching (walk/attack/hurt) swaps the atlas handle via `AnimationSet`.

## 8. UI Marker Pattern

UI elements are identified by zero-size marker components, not by name or hierarchy:

- `GoldText`, `DayNightText`, `HeroPanelText`, `KingdomRankText`, `AlertText`, `SpeedText`
- `BountyBoardUi`, `BuildMenuUi`

**Pattern:** `setup_ui()` spawns text entities with marker components → `update_ui_system()` queries `(Text, With<GoldText>)` to find and update specific elements each frame.

## 9. Road Network Spatial Queries

`RoadNetwork` (`components.rs`) stores road tile positions as `Vec<Vec2>` and provides spatial methods:

- `is_on_road(pos)` — within 12px of any road tile
- `speed_multiplier(pos)` — returns 1.3x on road, 1.0x off
- `are_connected(from, to, radius)` — BFS through adjacent tiles (18px threshold)

Building bonuses check road connectivity: Market connection = +10% tax, Blacksmith connection = +15% craft speed.

## 10. Graceful Failure Convention

The codebase avoids panics. Missing data is handled with:

- `Option<T>` with `.unwrap_or_default()` or `.is_none()` checks
- `if let Some(x) = ...` guards before acting
- Systems silently skip when required resources/entities are missing
- Default personalities, positions, and enemy types as fallbacks

No `unwrap()` on fallible operations in gameplay systems.

## 11. Single-Resource Economy

Gold is the only currency. All economic operations go through `GameEconomy`:

- `gold: f32` — current treasury
- `income_per_minute: f32` — computed from buildings + road bonuses
- `total_earned / total_spent` — lifetime tracking

Bounty payout split: hero receives 90%, 10% returns as tax. All costs defined as methods on types (`BuildingType::cost()`, `BuildingType::upgrade_cost(tier)`, `RareItem::cost()`).
