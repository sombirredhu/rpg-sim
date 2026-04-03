# Realm of Bounties - Pending Features vs GDD

## Legend
- [x] = Implemented
- [ ] = Not yet implemented / Partial

---

## 1. Core Gameplay Loop (GDD Section 2)
- [x] Build & expand town buildings
- [x] Attract heroes via buildings
- [x] Set bounties to influence hero behavior
- [x] Manage gold economy (income vs. expenditure)
- [x] Survive threats (enemy waves, boss raids)
- [ ] Plan road networks connecting buildings (roads exist but no connection bonus)
- [x] Expand map perimeter to reveal new zones (E key, gold cost, rank-gated)

## 2. Indirect Control System (GDD Section 3)
- [x] Hero AI evaluates bounty value, danger, needs, personality, distance
- [x] Bounty Board with 4 types: Monster, Exploration, Objective, Resource
- [x] Auto-bounty creation for monster dens
- [x] Manual bounty placement (Q key)
- [ ] Bounty Board UI panel (currently keyboard-only, no visual list)
- [ ] Dynamic bounty amount adjustment UI
- [ ] Bounty pay-on-completion visibility / ROI display

## 3. Hero Classes & Abilities (GDD Section 4)
- [x] All 5 classes: Warrior, Archer, Mage, Rogue, Healer
- [x] Unique stats per class
- [x] Hero personalities: Brave, Cautious, Greedy, Loyal
- [x] Hero XP progression & leveling
- [x] Perk points every 5 levels (class-specific upgrades)
- [x] Legendary heroes at level 10+ (stat boost, golden border concept)
- [x] Rogue Backstab (critical hit on attack)
- [x] Archer Volley (bonus damage at range)
- [x] Mage Arcane Surge (bonus damage)
- [x] Warrior Fortify (consistent damage bonus)
- [x] Healer healing nearby allies
- [ ] Warrior Fortify as damage reduction AURA for nearby allies
- [ ] Archer Volley as actual AoE arrow rain on clustered enemies
- [ ] Mage Arcane Surge as channelled multi-target blast
- [ ] Healer Sanctuary ability (healing pulse that revives fallen heroes)
- [ ] Rogue stealth mechanic (enters camps unseen, priority kills)
- [ ] Hero death with permanent loss if recovery bounty is neglected (partially done - respawn timer exists but no permanent death)

## 4. Town Buildings & Structures (GDD Section 5)
- [x] All 9 buildings: TownHall, Inn, Market, Temple, GuardTower, WizardTower, Blacksmith, Alchemist, Barracks
- [x] Building costs matching GDD
- [x] Building upgrade tiers (0-3)
- [x] Building destruction and repair
- [x] Guard Tower auto-attack
- [x] Building unlock restrictions by kingdom rank

### Building Tier Abilities
- [x] Inn: faster healing at tier 1+
- [x] Market: trade bonus at tier 1+
- [x] Temple: morale aura at tier 1+
- [x] Temple Tier 3: Cathedral with pilgrim income
- [x] Blacksmith: ATK/DEF bonus for heroes
- [x] Alchemist: faster hero recovery
- [x] Barracks: extra hero cap
- [x] Wizard Tower: mage damage bonus
- [ ] Market Tier 2: trade caravans with rare items
- [ ] Equipment crafting at Blacksmith
- [ ] Potion crafting at Alchemist
- [ ] Barracks squad formation bounties

### Road Network
- [x] Road placement (R + click)
- [x] 30% speed boost on roads
- [ ] Road connection bonuses (market tax boost, blacksmith craft speed)
- [ ] Bridge building to cross rivers/open new map sectors
- [ ] Drag-and-drop road placement (snap-to-grid)

## 5. Economy & Resources (GDD Section 6)
- [x] Property tax from buildings
- [x] Market trade (merchant caravans)
- [x] Bounty tax (10% return to treasury)
- [x] Resource nodes (Mine, Lumber Mill) with passive income
- [x] Milestone gold rewards (event rewards)
- [x] Building construction/upgrade spending
- [x] Bounty payment spending
- [x] Emergency repair costs
- [ ] Signing bonus for high-tier hero recruitment
- [ ] Manual repair trigger UI (currently auto-repair only)
- [ ] Treasury reserve warning (below 200 gold alert)
- [ ] Income-per-minute breakdown UI

## 6. Enemies & Threat System (GDD Section 7)
- [x] Goblin Rabble (weak, spawn in numbers)
- [x] Bandit Gang (target market/roads)
- [x] Cave Troll (tanky, slow)
- [x] Goblin Elite (escalated goblins)
- [x] Boss Warlord (boss raid events)
- [x] Werewolf (night-only)
- [x] Shadow Bandit (night-only)
- [x] Monster dens as persistent spawn points
- [x] Destroyable dens
- [x] New dens spawn in explored zones
- [x] Threat escalation (dens get stronger over time)
- [x] Goblin camp → Goblin Stronghold with elite units
- [ ] Goblin Warlord tier 3 (stronghold → launches direct town raids)
- [ ] Dynamic spawning from map edges based on difficulty
- [ ] Weather/storm events increasing spawn difficulty

## 7. Day/Night Cycle (GDD Section 8)
- [x] Full cycle: Dawn, Day, Dusk, Night
- [x] 8-minute day length
- [x] Night overlay visual
- [x] Increased threat spawn at night (50% boost)
- [x] Night-only enemies (Werewolf, Shadow Bandit)
- [x] Low morale heroes refuse to leave inn at night
- [x] Torch/light defense bonus near buildings at night
- [x] Merchant caravans only during daytime
- [ ] Torch halo visual effects around buildings at night
- [ ] Proper day/night lighting with real-time colour overlay

## 8. User Interface (GDD Section 9)
- [x] Gold counter with income-per-minute
- [x] Day/Night clock display
- [x] Hero panel (collapsible list of heroes)
- [x] Alert system (toast notifications)
- [x] Build menu (B key → number selection)
- [x] Speed toggle display (1x, 2x, 3x, pause)
- [x] Keyboard controls help bar
- [x] Click-to-inspect (I + click)
- [ ] Mobile-first UI (touch controls, portrait/landscape adaptive layout)
- [ ] 48x48pt tap targets for accessibility
- [ ] Long-press contextual info cards with quick-action buttons
- [ ] Floating action button for Bounty Board
- [ ] Visual Bounty Board panel with categorized bounty list
- [ ] Sun/moon arc day/night indicator
- [ ] Hero panel collapse/expand toggle

## 9. Session Design (GDD Section 9.3)
- [x] Speed toggle (1x, 2x, 3x)
- [x] Pause during planning
- [ ] Offline progress (kingdom earns passive income while away)
- [ ] Push notifications (boss raid warnings, building completions)
- [ ] Auto-save (every 30 seconds)
- [ ] Save/Load system of any kind

## 10. Kingdom Progression & Win Conditions (GDD Section 10)
- [x] 5 Kingdom ranks: Hamlet → Village → Town → City → Kingdom
- [x] Rank-based building unlocks
- [x] Rank-based hero cap
- [x] Era system with final siege
- [x] Legacy points awarded on era completion
- [x] Score calculation
- [ ] Map zone unlocks per rank (Forest, Mountain, Dungeon)
- [ ] Era completion score screen (wealth, heroes alive, buildings standing)
- [ ] Legacy Points spending UI for permanent bonuses (+10% tax, heroes start at level 2, etc.)
- [ ] Challenge Eras with modifiers ('No Inns', 'Double Bounties', 'Permanent Death')
- [ ] Roguelite fresh start with Legacy carry-forward

## 11. Art & Audio (GDD Section 11)
- [x] Sprite assets for heroes, enemies, buildings, environment
- [x] Grass ground tiles
- [x] Tree decoration
- [x] Basic sprite animation system
- [ ] Hero walk, attack, idle, rest animation sets (currently basic frame cycling)
- [ ] Animated building interiors (flickering fires, moving silhouettes)
- [ ] Golden border on Legendary hero portraits
- [ ] Adaptive music (calm lute during day, tense strings at night)
- [ ] Class-specific footstep and combat sound effects
- [ ] Town ambience (merchants calling, blacksmith hammering, temple bells)
- [ ] Boss raid siren horn effect
- [ ] Any audio at all

## 12. Monetization (GDD Section 12)
- [ ] Cosmetic hero skins
- [ ] Kingdom themes (desert, frost, volcanic)
- [ ] Speed-Up Pass (permanent 3x unlock)
- [ ] Battle Pass (seasonal cosmetic rewards)
- [ ] In-app purchase system

## 13. Emergent Gameplay (GDD Section 13)
- [x] Heroes have individual personalities affecting behavior
- [x] Heroes can refuse night missions (low morale)
- [x] Heroes leave kingdom if idle too long
- [ ] Heroes fall in love at the inn
- [ ] Heroes flee in panic during overwhelming battles
- [ ] Heroes have preferences beyond personality (e.g., favorite buildings)
- [ ] Emergent storytelling events

## 14. Other Missing Features
- [ ] Guild system (assign heroes to guilds or leave free-roaming)
- [ ] Hero Shopping state (exists in code but unused - heroes visit market)
- [ ] Escort merchant bounties (Objective bounty subtype)
- [ ] Map grid-based building placement (currently free placement)
- [ ] Building placement validation (overlap detection)
- [ ] Tutorial / onboarding (learn in 2 minutes per GDD)
- [ ] iOS & Android builds (currently desktop Bevy app)

---

## Summary

| Category | Implemented | Pending | Total |
|----------|------------|---------|-------|
| Core Loop | 6 | 1 | 7 |
| Indirect Control | 4 | 3 | 7 |
| Hero Classes | 12 | 6 | 18 |
| Buildings | 14 | 4 | 18 |
| Roads | 2 | 3 | 5 |
| Economy | 7 | 4 | 11 |
| Enemies | 12 | 3 | 15 |
| Day/Night | 8 | 2 | 10 |
| UI | 8 | 7 | 15 |
| Session Design | 2 | 4 | 6 |
| Progression | 6 | 5 | 11 |
| Art & Audio | 4 | 7 | 11 |
| Monetization | 0 | 5 | 5 |
| Emergent | 3 | 4 | 7 |
| Other | 0 | 7 | 7 |
| **TOTAL** | **~88** | **~65** | **~153** |

**Approximate completion: ~58% of GDD features implemented**
