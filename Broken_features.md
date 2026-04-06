# Broken Features - Realm of Bounties

This document lists features that are marked as implemented in PENDING_FEATURES.md but are either non-functional, incomplete, or contain critical bugs.

---

## 1. Blacksmith ATK/DEF Bonus Not Applied

**Feature:** Blacksmith building should provide attack and defense bonuses to heroes.

**Expected Behavior:** Heroes within range of a Blacksmith building should receive increased attack and defense stats based on building tier.

**Current Behavior:** The `building_bonuses_system` in `features.rs` (lines 645-670) correctly calculates `blacksmith_atk_bonus` and `blacksmith_def_bonus` values and stores them in the `BuildingBonuses` resource. However, these values are never used in any combat calculation.

**Root Cause:** 
- `hero_attack_system` calculates damage as `stats.attack + equipment.total_atk_bonus() + active_buffs.atk_bonus`
- `enemy_attack_system` calculates hero defense as `hero_stats.defense + hero_equipment.total_def_bonus() + active_buffs.def_bonus`
- Neither includes blacksmith building bonuses

**Required Fix:** Pass `BuildingBonuses` resource to `hero_attack_system` and `enemy_attack_system`, or apply bonuses via a dedicated system that updates hero stats each frame.

**Severity:** Critical

---

## 2. Wizard Tower Mage Damage Bonus Not Applied

**Feature:** Wizard Tower should provide spell damage bonus to mage heroes.

**Expected Behavior:** Mage heroes should deal increased damage when a Wizard Tower is present.

**Current Behavior:** `building_bonuses_system` sets `bonuses.wizard_research_bonus` (1.2x at tier 1, 1.5x at tier 2+) but this multiplier is never used in damage calculations.

**Root Cause:** No system applies `wizard_research_bonus` to mage damage output.

**Required Fix:** In `hero_attack_system` or arcane surge damage calculation, if hero class is Mage, multiply damage by `wizard_research_bonus`. Add `wizard_research_bonus` to `ActiveBuffs` or directly use the resource in combat systems.

**Severity:** Critical

---

## 3. Temple Morale Aura Not Applied

**Feature:** Temple should provide a morale aura that boosts hero morale recovery.

**Expected Behavior:** Heroes near a Temple should have higher morale (faster recovery or reduced decay).

**Current Behavior:** `building_bonuses_system` calculates `bonuses.temple_morale_aura` (2.0 at tier 1, 5.0 at tier 3) but this value is never applied to any hero morale calculations.

**Root Cause:** `hero_morale_system` does not incorporate `BuildingBonuses::temple_morale_aura`.

**Required Fix:** Pass `BuildingBonuses` to `hero_morale_system` and add the aura value to morale recovery rate (e.g., `hero.morale += temple_morale_aura * delta_time`).

**Severity:** Critical

---

## 4. Building Menu Mouse Selection Broken

**Feature:** Build menu should allow clicking to select buildings for placement.

**Expected Behavior:** Clicking on a building name in the build menu UI panel should select that building type and enter build mode.

**Current Behavior:** `map_click_system` in `mouse.rs` (lines 319-356) uses world coordinate math (`world.x < -300.0`, `world.y` approximation) to detect clicks on the menu. Since the build menu is a screen-space UI panel at absolute position left:5px, top:320px, world coordinates vary with camera position and zoom. The click detection almost never matches the visible menu.

**Root Cause:** Coordinate space mismatch - attempting to detect UI clicks using world coordinates instead of UI cursor/button interactions.

**Required Fix:** Replace the coordinate-based detection with proper Bevy UI button interactions. Each building menu item should be a `ButtonBundle` with a marker component (e.g., `BuildingMenuItem(index)`). Use a system that queries for `Interaction::Clicked` on these buttons to set `game_phase.selected_building`.

**Severity:** High

---

## 5. Building Highlight Z-Order Behind Building

**Feature:** Selected building should show a visible highlight ring.

**Expected Behavior:** When a building is clicked, a green highlight ring appears around it.

**Current Behavior:** Highlight ring is spawned at `Transform::from_translation(Vec3::new(pos.x, pos.y, 4.0))` in `mouse.rs:437`. Buildings are spawned at z=5.0 (`sprites.rs`). Lower z means behind in 2D, so the highlight is obscured by the building sprite.

**Required Fix:** Change highlight z-order to 6.0 or higher (e.g., `z = 10.0`) to ensure it renders on top.

**Severity:** Medium

---

## 6. Building Info Panel Shows Stale Data

**Feature:** Right-clicking a building shows an info panel with details.

**Expected Behavior:** Info panel should show current building state and hide when building is no longer valid.

**Current Behavior:** `update_building_info_ui` in `ui.rs` only updates text when `selected_building_info.entity` is Some and the entity query succeeds. If the building entity is despawned (future feature) or loses its `Building` component, the panel remains visible (visibility controlled separately by mouse system) showing outdated info. No cleanup occurs.

**Required Fix:** After attempting to fetch building data, if query fails, clear `selected_building_info.entity` and hide the panel by setting `Visibility` of `BuildingInfoUi` to false. Also consider listening to despawn events.

**Severity:** Medium

---

## 7. Map Clicks Conflict with HUD Button Clicks

**Feature:** Mouse interactions should be contextually appropriate.

**Expected Behavior:** Clicking HUD buttons should not trigger world map interactions (building selection, etc.).

**Current Behavior:** `map_click_system` processes every left-click, even when clicking HUD buttons. Bevy's `Interaction` components on buttons are handled in separate systems, but `map_click_system` also fires and may interpret the same click as a building selection because world coordinates under the mouse could fall within UI region (e.g., build menu area hard-coded at world.x < -300).

**Root Cause:** No early-exit check for button interactions or HUD click zones.

**Required Fix:** At start of `map_click_system`, query for any `Interaction::Clicked` on HUD buttons and return early. Optionally also check screen bounds (ignore clicks in top/bottom UI regions).

**Severity:** Medium

---

## 8. Building Menu UI Panel Never Hidden by System

**Feature:** Build menu should show/hide based on `game_phase.show_build_menu`.

**Expected Behavior:** Panel visibility should be controlled by `update_building_menu_ui`.

**Current Behavior:** `update_building_menu_ui` in `ui.rs` correctly toggles `Visibility` based on `game_phase.show_build_menu`. However, the panel is also referenced in `map_click_system` via `mut building_menu_ui: Query<&mut Visibility, With<BuildingMenuUi>>` which is currently unused (the variable is never iterated). This is dead code.

**Required Fix:** Remove the unused `building_menu_ui` query parameter from `map_click_system` (line 261 in mouse.rs). Let `update_building_menu_ui` exclusively control visibility.

**Severity:** Low

---

## 9. Unused Variables and Imports

**Issues:**

- `src/ui.rs:826` - `mut alerts` parameter unused in `update_building_menu_ui`
- `src/mouse.rs:19` - `mut game_phase` used immutably; should be `Res<GamePhase>`
- `src/mouse.rs:46` - Same issue in `speed_button_click`
- `src/mouse.rs:286` - `building` variable in loop is unused; prefix with `_`
- `src/sprites.rs:1065` - Unused imports from `noise_map` module
- `src/noise_map.rs` - Multiple unused parameters in functions: `octaves`, `persistence`, `lacunarity`, `seed`, `town_radius_tiles`

**Required Fix:** Remove unused parameters, prefix with `_` where intentionally unused, clean up imports.

**Severity:** Low

---

## 10. Dead Code: `RoadToolActive` Struct

**File:** `src/components.rs:72`

**Issue:** `pub struct RoadToolActive(pub bool);` is defined but never inserted or used anywhere. Game uses `game_phase.road_tool_active` instead.

**Required Fix:** Delete the struct or refactor to replace `game_phase.road_tool_active` with this resource if intended.

**Severity:** Low

---

## 11. Building Info Panel Missing Destruction Status

**File:** `src/ui.rs:858-916` (`update_building_info_ui`)

**Issue:** The panel displays HP and tax income but does not indicate whether the building is destroyed. This could mislead players.

**Required Fix:** Include a "DESTROYED" indicator or red highlight when `building.is_destroyed` is true.

**Severity:** Low

---

## 12. Noise Terrain Generation Unused

**File:** `src/noise_map.rs`, `src/sprites.rs`

**Issue:** The `generate_terrain_noise` and related functions are implemented but the import in `sprites.rs` is unused. The terrain generation is not integrated into the map spawning.

**Required Fix:** Either integrate the noise-based terrain generation into `spawn_ground_tiles` or remove the module if it's exploratory work.

**Severity:** Low

---

## Summary

**Critical Issues (3):** Building bonus systems (Blacksmith, Wizard Tower, Temple) are calculated but never applied to gameplay.

**High Issues (1):** Build menu mouse selection is fundamentally broken due to coordinate space mismatch.

**Medium Issues (3):** Highlight z-order, building info stale data, map click conflicts.

**Low Issues (5):** Code hygiene, dead code, unused imports, minor UI omissions.

Total broken/incomplete features: **12** requiring attention before the game features are fully functional as intended.
