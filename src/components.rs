#![allow(dead_code)]

use bevy::prelude::*;

// ============================================================
// UI INTERACTION COMPONENTS — Marker components for mouse interaction
// ============================================================

/// Attached to the Speed display button in the HUD
#[derive(Component)]
pub struct SpeedButton;

/// Attached to the Pause button in the HUD
#[derive(Component)]
pub struct PauseButton;

/// Attached to the Build menu button in the HUD
#[derive(Component)]
pub struct BuildButton;

/// Attached to the Bounty board button in the HUD
#[derive(Component)]
pub struct BountyButton;

/// Attached to the Expand map button in the HUD
#[derive(Component)]
pub struct ExpandButton;

/// Attached to the Road tool toggle button in the HUD
#[derive(Component)]
pub struct RoadToolButton;

/// Visual highlight ring placed around a selected building
#[derive(Component)]
pub struct BuildingHighlight;

/// Resource tracking which building entity is currently selected on the map
#[derive(Default)]
pub struct SelectedBuilding {
    pub entity: Option<Entity>,
}

/// Resource tracking whether the road tool is currently active (click-paint mode)
#[derive(Default)]
pub struct RoadToolActive(pub bool);

// ============================================================
// HERO COMPONENTS
// ============================================================

/// The five hero classes from the GDD
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeroClass {
    Warrior, // Front-line tank, melee
    Archer,  // Ranged DPS
    Mage,    // AoE damage, spells
    Rogue,   // Scout/assassin, stealth
    Healer,  // Support/sustain
}

impl HeroClass {
    pub fn base_stats(&self) -> HeroStats {
        match self {
            HeroClass::Warrior => HeroStats {
                max_hp: 150.0,
                hp: 150.0,
                attack: 18.0,
                defense: 12.0,
                speed: 35.0,
                attack_range: 30.0,
                risk_tolerance: 0.7,
                fortify_reduction: 0.0,
            },
            HeroClass::Archer => HeroStats {
                max_hp: 90.0,
                hp: 90.0,
                attack: 22.0,
                defense: 5.0,
                speed: 45.0,
                attack_range: 150.0,
                risk_tolerance: 0.5,
                fortify_reduction: 0.0,
            },
            HeroClass::Mage => HeroStats {
                max_hp: 70.0,
                hp: 70.0,
                attack: 30.0,
                defense: 3.0,
                speed: 30.0,
                attack_range: 120.0,
                risk_tolerance: 0.4,
                fortify_reduction: 0.0,
            },
            HeroClass::Rogue => HeroStats {
                max_hp: 80.0,
                hp: 80.0,
                attack: 25.0,
                defense: 6.0,
                speed: 55.0,
                attack_range: 25.0,
                risk_tolerance: 0.6,
                fortify_reduction: 0.0,
            },
            HeroClass::Healer => HeroStats {
                max_hp: 85.0,
                hp: 85.0,
                attack: 8.0,
                defense: 7.0,
                speed: 35.0,
                attack_range: 100.0,
                risk_tolerance: 0.3,
                fortify_reduction: 0.0,
            },
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            HeroClass::Warrior => "Warrior",
            HeroClass::Archer => "Archer",
            HeroClass::Mage => "Mage",
            HeroClass::Rogue => "Rogue",
            HeroClass::Healer => "Healer",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            HeroClass::Warrior => Color::rgb(0.8, 0.2, 0.2),   // Red
            HeroClass::Archer => Color::rgb(0.2, 0.8, 0.2),    // Green
            HeroClass::Mage => Color::rgb(0.3, 0.3, 0.9),      // Blue
            HeroClass::Rogue => Color::rgb(0.6, 0.2, 0.8),     // Purple
            HeroClass::Healer => Color::rgb(0.9, 0.9, 0.3),    // Yellow
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct HeroStats {
    pub max_hp: f32,
    pub hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub attack_range: f32,
    pub risk_tolerance: f32,
    /// Damage reduction from Warrior Fortify aura (0.0 = none, 0.2 = 20% reduction)
    pub fortify_reduction: f32,
}

#[derive(Component, Debug, Clone)]
pub struct Hero {
    pub class: HeroClass,
    pub level: u32,
    pub xp: f32,
    pub xp_to_next: f32,
    pub morale: f32,        // 0.0 - 100.0
    pub gold_carried: f32,
    pub personality: HeroPersonality,
    pub is_legendary: bool,
}

impl Hero {
    pub fn new(class: HeroClass) -> Self {
        let personality = HeroPersonality::random();
        Self {
            class,
            level: 1,
            xp: 0.0,
            xp_to_next: 100.0,
            morale: 80.0,
            gold_carried: 0.0,
            personality,
            is_legendary: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeroPersonality {
    Brave,     // Ignores danger penalties
    Cautious,  // Demands higher pay
    Greedy,    // Prioritises high-value bounties
    Loyal,     // Lower bounty threshold
}

impl HeroPersonality {
    pub fn random() -> Self {
        match (rand::random::<f32>() * 4.0) as u32 {
            0 => HeroPersonality::Brave,
            1 => HeroPersonality::Cautious,
            2 => HeroPersonality::Greedy,
            _ => HeroPersonality::Loyal,
        }
    }
}

/// Current action the hero is performing
#[derive(Component, Debug, Clone)]
pub enum HeroState {
    Idle,
    MovingTo { target: Vec2 },
    AttackingEnemy { target_entity: Entity },
    PursuingBounty { bounty_id: u32 },
    Resting,       // At inn, restoring HP
    Shopping,      // At market
    Dead { respawn_timer: f32 },
    Casting { channel_elapsed: f32, channel_duration: f32, focus_entity: Entity }, // Mages channel Arcane Surge
}

impl Default for HeroState {
    fn default() -> Self {
        HeroState::Idle
    }
}

#[derive(Component)]
pub struct HeroDecisionTimer(pub f32);

impl Default for HeroDecisionTimer {
    fn default() -> Self {
        Self(0.0)
    }
}

// ============================================================
// BUILDING COMPONENTS
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuildingType {
    TownHall,
    Inn,
    Market,
    Temple,
    GuardTower,
    WizardTower,
    Blacksmith,
    Alchemist,
    Barracks,
}

impl BuildingType {
    pub fn cost(&self) -> f32 {
        match self {
            BuildingType::TownHall => 0.0,
            BuildingType::Inn => 150.0,
            BuildingType::Market => 200.0,
            BuildingType::Temple => 250.0,
            BuildingType::GuardTower => 300.0,
            BuildingType::WizardTower => 400.0,
            BuildingType::Blacksmith => 350.0,
            BuildingType::Alchemist => 300.0,
            BuildingType::Barracks => 450.0,
        }
    }

    pub fn upgrade_cost(&self, tier: u32) -> f32 {
        self.cost() * (tier as f32 + 1.0) * 0.75
    }

    pub fn tax_income(&self, tier: u32) -> f32 {
        let base = match self {
            BuildingType::TownHall => 5.0,
            BuildingType::Inn => 8.0,
            BuildingType::Market => 15.0,
            BuildingType::Temple => 10.0,
            BuildingType::GuardTower => 3.0,
            BuildingType::WizardTower => 7.0,
            BuildingType::Blacksmith => 12.0,
            BuildingType::Alchemist => 9.0,
            BuildingType::Barracks => 5.0,
        };
        base * (1.0 + tier as f32 * 0.5)
    }

    pub fn display_name(&self) -> &str {
        match self {
            BuildingType::TownHall => "Town Hall",
            BuildingType::Inn => "Inn",
            BuildingType::Market => "Market",
            BuildingType::Temple => "Temple",
            BuildingType::GuardTower => "Guard Tower",
            BuildingType::WizardTower => "Wizard Tower",
            BuildingType::Blacksmith => "Blacksmith",
            BuildingType::Alchemist => "Alchemist",
            BuildingType::Barracks => "Barracks",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            BuildingType::TownHall => Color::rgb(0.8, 0.7, 0.3),
            BuildingType::Inn => Color::rgb(0.6, 0.4, 0.2),
            BuildingType::Market => Color::rgb(0.3, 0.7, 0.3),
            BuildingType::Temple => Color::rgb(0.9, 0.9, 0.9),
            BuildingType::GuardTower => Color::rgb(0.5, 0.5, 0.5),
            BuildingType::WizardTower => Color::rgb(0.4, 0.2, 0.8),
            BuildingType::Blacksmith => Color::rgb(0.4, 0.3, 0.2),
            BuildingType::Alchemist => Color::rgb(0.2, 0.7, 0.6),
            BuildingType::Barracks => Color::rgb(0.6, 0.3, 0.3),
        }
    }

    pub fn size(&self) -> Vec2 {
        match self {
            BuildingType::TownHall => Vec2::new(96.0, 96.0),
            BuildingType::GuardTower => Vec2::new(48.0, 72.0),
            _ => Vec2::new(64.0, 64.0),
        }
    }

    /// Which hero classes this building attracts
    pub fn attracts_heroes(&self) -> Vec<HeroClass> {
        match self {
            BuildingType::Inn => vec![HeroClass::Warrior, HeroClass::Rogue],
            BuildingType::Temple => vec![HeroClass::Healer],
            BuildingType::WizardTower => vec![HeroClass::Mage],
            BuildingType::Barracks => vec![HeroClass::Warrior, HeroClass::Archer],
            BuildingType::Market => vec![HeroClass::Rogue],
            _ => vec![],
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Building {
    pub building_type: BuildingType,
    pub tier: u32,        // 0 = base, 1, 2, 3
    pub hp: f32,
    pub max_hp: f32,
    pub is_destroyed: bool,
}

impl Building {
    pub fn new(building_type: BuildingType) -> Self {
        let max_hp = match building_type {
            BuildingType::TownHall => 500.0,
            BuildingType::GuardTower => 300.0,
            BuildingType::Barracks => 350.0,
            _ => 200.0,
        };
        Self {
            building_type,
            tier: 0,
            hp: max_hp,
            max_hp,
            is_destroyed: false,
        }
    }
}

/// Tracks which building-tier sprite is currently displayed on an entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct BuildingVisualTier {
    pub tier: u32,
}

// ============================================================
// ENEMY COMPONENTS
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnemyType {
    Goblin,
    Bandit,
    Troll,
    GoblinElite,
    BossWarlord,
    Werewolf,      // Night-only
    ShadowBandit,  // Night-only
}

impl EnemyType {
    pub fn stats(&self) -> EnemyStats {
        match self {
            EnemyType::Goblin => EnemyStats {
                max_hp: 30.0,
                hp: 30.0,
                attack: 8.0,
                defense: 2.0,
                speed: 30.0,
                attack_range: 25.0,
                threat_level: 1,
                xp_reward: 15.0,
                gold_reward: 10.0,
            },
            EnemyType::Bandit => EnemyStats {
                max_hp: 50.0,
                hp: 50.0,
                attack: 14.0,
                defense: 5.0,
                speed: 35.0,
                attack_range: 25.0,
                threat_level: 2,
                xp_reward: 30.0,
                gold_reward: 20.0,
            },
            EnemyType::Troll => EnemyStats {
                max_hp: 150.0,
                hp: 150.0,
                attack: 25.0,
                defense: 15.0,
                speed: 15.0,
                attack_range: 35.0,
                threat_level: 3,
                xp_reward: 60.0,
                gold_reward: 50.0,
            },
            EnemyType::GoblinElite => EnemyStats {
                max_hp: 70.0,
                hp: 70.0,
                attack: 18.0,
                defense: 8.0,
                speed: 30.0,
                attack_range: 25.0,
                threat_level: 2,
                xp_reward: 40.0,
                gold_reward: 30.0,
            },
            EnemyType::BossWarlord => EnemyStats {
                max_hp: 500.0,
                hp: 500.0,
                attack: 40.0,
                defense: 20.0,
                speed: 20.0,
                attack_range: 40.0,
                threat_level: 5,
                xp_reward: 200.0,
                gold_reward: 200.0,
            },
            EnemyType::Werewolf => EnemyStats {
                max_hp: 90.0,
                hp: 90.0,
                attack: 22.0,
                defense: 8.0,
                speed: 50.0,
                attack_range: 28.0,
                threat_level: 3,
                xp_reward: 50.0,
                gold_reward: 35.0,
            },
            EnemyType::ShadowBandit => EnemyStats {
                max_hp: 60.0,
                hp: 60.0,
                attack: 20.0,
                defense: 4.0,
                speed: 45.0,
                attack_range: 25.0,
                threat_level: 2,
                xp_reward: 35.0,
                gold_reward: 25.0,
            },
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            EnemyType::Goblin => "Goblin",
            EnemyType::Bandit => "Bandit",
            EnemyType::Troll => "Troll",
            EnemyType::GoblinElite => "Goblin Elite",
            EnemyType::BossWarlord => "Boss Warlord",
            EnemyType::Werewolf => "Werewolf",
            EnemyType::ShadowBandit => "Shadow Bandit",
        }
    }

    pub fn is_night_only(&self) -> bool {
        matches!(self, EnemyType::Werewolf | EnemyType::ShadowBandit)
    }

    pub fn color(&self) -> Color {
        match self {
            EnemyType::Goblin => Color::rgb(0.2, 0.6, 0.1),
            EnemyType::Bandit => Color::rgb(0.5, 0.3, 0.1),
            EnemyType::Troll => Color::rgb(0.3, 0.5, 0.3),
            EnemyType::GoblinElite => Color::rgb(0.1, 0.4, 0.0),
            EnemyType::BossWarlord => Color::rgb(0.8, 0.1, 0.1),
            EnemyType::Werewolf => Color::rgb(0.4, 0.3, 0.5),
            EnemyType::ShadowBandit => Color::rgb(0.2, 0.1, 0.3),
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct EnemyStats {
    pub max_hp: f32,
    pub hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub attack_range: f32,
    pub threat_level: u32,
    pub xp_reward: f32,
    pub gold_reward: f32,
}

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
}

#[derive(Component)]
pub struct EnemyAi {
    pub target: Option<Entity>,
    pub wander_angle: f32,
    pub wander_timer: f32,
}

impl Default for EnemyAi {
    fn default() -> Self {
        Self {
            target: None,
            wander_angle: rand::random::<f32>() * std::f32::consts::TAU,
            wander_timer: 0.0,
        }
    }
}

/// Monster den / spawn point — can be destroyed by heroes
#[derive(Component)]
pub struct MonsterDen {
    pub enemy_type: EnemyType,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub max_spawned: u32,
    pub current_spawned: u32,
    pub threat_tier: u32,
    pub weeks_unaddressed: u32,
    pub hp: f32,
    pub max_hp: f32,
}

impl MonsterDen {
    pub fn new(enemy_type: EnemyType) -> Self {
        let max_hp = match enemy_type {
            EnemyType::BossWarlord => 300.0,
            EnemyType::Troll => 200.0,
            _ => 120.0,
        };
        Self {
            enemy_type,
            spawn_timer: 0.0,
            spawn_interval: 30.0,
            max_spawned: 3,
            current_spawned: 0,
            threat_tier: 1,
            weeks_unaddressed: 0,
            hp: max_hp,
            max_hp,
        }
    }
}

/// Tracks which den-tier sprite is currently displayed on a den entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct MonsterDenVisualTier {
    pub tier: u32,
}

// ============================================================
// BOUNTY COMPONENTS
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BountyType {
    Monster,      // Kill enemy at location
    Exploration,  // Scout fog area
    Objective,    // Defend building, escort
    Resource,     // Gather from node
}

#[derive(Debug, Clone)]
pub struct Bounty {
    pub id: u32,
    pub bounty_type: BountyType,
    pub gold_reward: f32,
    pub location: Vec2,
    pub target_entity: Option<Entity>,
    pub danger_level: u32,  // 1-5
    pub is_completed: bool,
    pub assigned_hero: Option<Entity>,
}

// ============================================================
// ECONOMY / GAME STATE RESOURCES
// ============================================================

pub struct GameEconomy {
    pub gold: f32,
    pub income_per_minute: f32,
    pub total_earned: f32,
    pub total_spent: f32,
}

impl Default for GameEconomy {
    fn default() -> Self {
        Self {
            gold: 500.0, // Starting gold
            income_per_minute: 0.0,
            total_earned: 500.0,
            total_spent: 0.0,
        }
    }
}

pub struct BountyBoard {
    pub bounties: Vec<Bounty>,
    pub next_id: u32,
    pub total_bounties_completed: u32,
    pub total_bounty_gold_paid: f32,
    pub total_bounty_tax_returned: f32,
}

impl Default for BountyBoard {
    fn default() -> Self {
        Self {
            bounties: Vec::new(),
            next_id: 1,
            total_bounties_completed: 0,
            total_bounty_gold_paid: 0.0,
            total_bounty_tax_returned: 0.0,
        }
    }
}

impl BountyBoard {
    pub fn add_bounty(&mut self, bounty_type: BountyType, gold_reward: f32, location: Vec2, target_entity: Option<Entity>, danger_level: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.bounties.push(Bounty {
            id,
            bounty_type,
            gold_reward,
            location,
            target_entity,
            danger_level,
            is_completed: false,
            assigned_hero: None,
        });
        id
    }

    pub fn get_bounty(&self, id: u32) -> Option<&Bounty> {
        self.bounties.iter().find(|b| b.id == id && !b.is_completed)
    }

    pub fn assign_bounty(&mut self, id: u32, hero_entity: Entity) -> bool {
        if let Some(bounty) = self.bounties.iter_mut().find(|b| b.id == id && !b.is_completed) {
            if bounty.assigned_hero.is_none() || bounty.assigned_hero == Some(hero_entity) {
                bounty.assigned_hero = Some(hero_entity);
                return true;
            }
        }
        false
    }

    pub fn unassign_hero(&mut self, hero_entity: Entity) {
        for bounty in self.bounties.iter_mut() {
            if !bounty.is_completed && bounty.assigned_hero == Some(hero_entity) {
                bounty.assigned_hero = None;
            }
        }
    }

    pub fn complete_bounty(&mut self, id: u32) -> Option<f32> {
        if let Some(bounty) = self.bounties.iter_mut().find(|b| b.id == id) {
            bounty.is_completed = true;
            bounty.assigned_hero = None;
            Some(bounty.gold_reward)
        } else {
            None
        }
    }

    pub fn available_bounties(&self) -> Vec<&Bounty> {
        self.bounties.iter().filter(|b| !b.is_completed && b.assigned_hero.is_none()).collect()
    }

    pub fn cleanup_completed(&mut self) {
        self.bounties.retain(|b| !b.is_completed);
    }
}

// ============================================================
// DAY/NIGHT CYCLE
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeOfDay {
    Dawn,
    Day,
    Dusk,
    Night,
}

pub struct GameTime {
    pub time_seconds: f32,     // Total elapsed game-time seconds
    pub day_length: f32,       // Seconds per full day (480 = 8 minutes)
    pub current_day: u32,
    pub time_of_day: TimeOfDay,
    pub day_progress: f32,     // 0.0 to 1.0
    pub speed_multiplier: f32, // 1x, 2x, or 0 (pause)
    pub is_paused: bool,
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            time_seconds: 0.0,
            day_length: 480.0,  // 8 minutes per in-game day
            current_day: 1,
            time_of_day: TimeOfDay::Dawn,
            day_progress: 0.0,
            speed_multiplier: 1.0,
            is_paused: false,
        }
    }
}

impl GameTime {
    pub fn is_night(&self) -> bool {
        matches!(self.time_of_day, TimeOfDay::Night | TimeOfDay::Dusk)
    }

    pub fn threat_multiplier(&self) -> f32 {
        match self.time_of_day {
            TimeOfDay::Dawn => 1.0,
            TimeOfDay::Day => 1.0,
            TimeOfDay::Dusk => 1.25,
            TimeOfDay::Night => 1.5,
        }
    }

    pub fn ambient_color(&self) -> Color {
        match self.time_of_day {
            TimeOfDay::Dawn => Color::rgba(1.0, 0.9, 0.7, 0.15),
            TimeOfDay::Day => Color::rgba(1.0, 1.0, 1.0, 0.0),
            TimeOfDay::Dusk => Color::rgba(0.9, 0.5, 0.3, 0.2),
            TimeOfDay::Night => Color::rgba(0.1, 0.1, 0.3, 0.4),
        }
    }
}

// ============================================================
// KINGDOM PROGRESSION
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KingdomRank {
    Hamlet,   // Rank 1
    Village,  // Rank 2
    Town,     // Rank 3
    City,     // Rank 4
    Kingdom,  // Rank 5
}

impl KingdomRank {
    pub fn display_name(&self) -> &str {
        match self {
            KingdomRank::Hamlet => "Hamlet",
            KingdomRank::Village => "Village",
            KingdomRank::Town => "Town",
            KingdomRank::City => "City",
            KingdomRank::Kingdom => "Kingdom",
        }
    }

    pub fn max_heroes(&self) -> u32 {
        match self {
            KingdomRank::Hamlet => 5,
            KingdomRank::Village => 8,
            KingdomRank::Town => 12,
            KingdomRank::City => 18,
            KingdomRank::Kingdom => 25,
        }
    }

    /// Maximum number of map expansions allowed at this rank
    pub fn max_expansions(&self) -> u32 {
        match self {
            KingdomRank::Hamlet => 1,
            KingdomRank::Village => 2,
            KingdomRank::Town => 3,
            KingdomRank::City => 4,
            KingdomRank::Kingdom => 5,
        }
    }

    /// Gold cost for the next map expansion (scales with expansion count)
    pub fn expansion_cost(expansion_number: u32) -> f32 {
        100.0 + expansion_number as f32 * 75.0
    }

    pub fn available_buildings(&self) -> Vec<BuildingType> {
        match self {
            KingdomRank::Hamlet => vec![BuildingType::TownHall, BuildingType::Inn, BuildingType::Market],
            KingdomRank::Village => vec![BuildingType::TownHall, BuildingType::Inn, BuildingType::Market, BuildingType::Temple, BuildingType::GuardTower],
            KingdomRank::Town => vec![BuildingType::TownHall, BuildingType::Inn, BuildingType::Market, BuildingType::Temple, BuildingType::GuardTower, BuildingType::WizardTower, BuildingType::Blacksmith],
            KingdomRank::City | KingdomRank::Kingdom => vec![
                BuildingType::TownHall, BuildingType::Inn, BuildingType::Market,
                BuildingType::Temple, BuildingType::GuardTower, BuildingType::WizardTower,
                BuildingType::Blacksmith, BuildingType::Alchemist, BuildingType::Barracks,
            ],
        }
    }
}

pub struct KingdomState {
    pub rank: KingdomRank,
    pub era: u32,
    pub era_day: u32,
    pub legacy_points: u32,
    pub hero_count: u32,
    pub buildings_count: u32,
    pub score: u32,
}

impl Default for KingdomState {
    fn default() -> Self {
        Self {
            rank: KingdomRank::Hamlet,
            era: 1,
            era_day: 1,
            legacy_points: 0,
            hero_count: 0,
            buildings_count: 0,
            score: 0,
        }
    }
}

// ============================================================
// COMBAT COMPONENTS
// ============================================================

#[derive(Component)]
pub struct AttackCooldown {
    pub timer: f32,
    pub interval: f32,
}

impl Default for AttackCooldown {
    fn default() -> Self {
        Self {
            timer: 0.0,
            interval: 1.0, // 1 attack per second
        }
    }
}

#[derive(Component)]
pub struct HealthBar;

/// Cooldown timer for Mage's Arcane Surge ability
#[derive(Component)]
pub struct ArcaneSurgeCooldown {
    pub timer: f32,
    pub duration: f32, // 8 seconds
}

impl Default for ArcaneSurgeCooldown {
    fn default() -> Self {
        Self {
            timer: 0.0,
            duration: 8.0,
        }
    }
}

/// Visual marker for an active Arcane Surge blast (used for temporary effect sprite)
#[derive(Component)]
pub struct ArcaneSurgeEffect {
    pub timer: f32,
}

// ============================================================
// UI MARKER COMPONENTS
// ============================================================

#[derive(Component)]
pub struct GoldText;

#[derive(Component)]
pub struct DayNightText;

#[derive(Component)]
pub struct HeroPanelText;

#[derive(Component)]
pub struct KingdomRankText;

#[derive(Component)]
pub struct AlertText;

#[derive(Component)]
pub struct BountyBoardUi;

#[derive(Component)]
pub struct BountyBoardText;

#[derive(Component)]
pub struct BuildMenuUi;

#[derive(Component)]
pub struct NightOverlay;

#[derive(Component)]
pub struct SpeedText;

#[derive(Component)]
pub struct MainCamera;

// ============================================================
// GAME EVENTS
// ============================================================

pub struct BountyCompletedEvent {
    pub bounty_id: u32,
    pub hero_entity: Entity,
    pub gold_reward: f32,
}

pub struct HeroDeathEvent {
    pub hero_entity: Entity,
}

pub struct BuildingDestroyedEvent {
    pub building_entity: Entity,
}

pub struct EnemyDeathEvent {
    pub enemy_entity: Entity,
    pub xp_reward: f32,
    pub gold_reward: f32,
    pub killer: Option<Entity>,
}

pub struct ThreatEscalationEvent {
    pub den_entity: Entity,
    pub new_tier: u32,
}

pub struct HeroSpawnEvent {
    pub class: HeroClass,
}

// ============================================================
// GAME PHASE
// ============================================================

pub struct GamePhase {
    pub build_mode: bool,
    pub selected_building: Option<BuildingType>,
    pub bounty_board_open: bool,
    pub show_build_menu: bool,
    pub manual_bounty_amount: f32,
    pub road_tool_active: bool,
}

impl Default for GamePhase {
    fn default() -> Self {
        Self {
            build_mode: false,
            selected_building: None,
            bounty_board_open: false,
            show_build_menu: false,
            manual_bounty_amount: 30.0,
            road_tool_active: false,
        }
    }
}

// ============================================================
// GAME ALERTS
// ============================================================

pub struct GameAlerts {
    pub messages: Vec<(String, f32)>, // (message, time_remaining)
}

impl Default for GameAlerts {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

impl GameAlerts {
    pub fn push(&mut self, msg: String) {
        self.messages.push((msg, 5.0));
    }
}

// ============================================================
// ROAD NETWORK
// ============================================================

#[derive(Component)]
pub struct Road;

/// Marker for decoration sprites scattered on the map (rocks, bushes, ruins, etc.)
#[derive(Component)]
pub struct MapDecoration;

/// Resource tracking road tile positions for speed lookups
pub struct RoadNetwork {
    pub tiles: Vec<Vec2>,
}

impl Default for RoadNetwork {
    fn default() -> Self {
        Self { tiles: Vec::new() }
    }
}

impl RoadNetwork {
    /// Returns true if position is near a road (within 12px)
    pub fn is_on_road(&self, pos: Vec2) -> bool {
        self.tiles.iter().any(|t| (*t - pos).length() < 12.0)
    }

    /// Speed multiplier: 1.3 on road, 1.0 off road
    pub fn speed_multiplier(&self, pos: Vec2) -> f32 {
        if self.is_on_road(pos) { 1.3 } else { 1.0 }
    }

    /// Check if two positions are connected via a chain of road tiles.
    /// Uses BFS: starts from road tiles near `from`, floods through adjacent
    /// road tiles (within 18px of each other), and checks if any tile near `to`
    /// is reached. `radius` is how close a building must be to a road tile to
    /// count as "on the road network".
    pub fn are_connected(&self, from: Vec2, to: Vec2, radius: f32) -> bool {
        if self.tiles.is_empty() { return false; }

        // Find starting road tiles near `from`
        let start_indices: Vec<usize> = self.tiles.iter().enumerate()
            .filter(|(_, t)| (**t - from).length() < radius)
            .map(|(i, _)| i)
            .collect();

        if start_indices.is_empty() { return false; }

        // Check if any road tile is near `to`
        let has_dest = self.tiles.iter().any(|t| (*t - to).length() < radius);
        if !has_dest { return false; }

        // BFS through road tiles
        let mut visited = vec![false; self.tiles.len()];
        let mut queue = std::collections::VecDeque::new();
        for idx in start_indices {
            visited[idx] = true;
            queue.push_back(idx);
        }

        let chain_dist = 18.0; // Max distance between adjacent road tiles
        while let Some(current) = queue.pop_front() {
            let pos = self.tiles[current];
            // Check if we reached destination
            if (pos - to).length() < radius {
                return true;
            }
            // Expand neighbors
            for (i, tile) in self.tiles.iter().enumerate() {
                if !visited[i] && (*tile - pos).length() < chain_dist {
                    visited[i] = true;
                    queue.push_back(i);
                }
            }
        }
        false
    }
}

// ============================================================
// RESOURCE NODES
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Mine,
    LumberMill,
}

impl ResourceType {
    pub fn display_name(&self) -> &str {
        match self {
            ResourceType::Mine => "Mine",
            ResourceType::LumberMill => "Lumber Mill",
        }
    }

    pub fn income_per_tick(&self) -> f32 {
        match self {
            ResourceType::Mine => 3.0,
            ResourceType::LumberMill => 2.0,
        }
    }
}

#[derive(Component)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub is_active: bool,  // Needs hero to keep it safe / gather
    pub gather_timer: f32,
}

impl ResourceNode {
    pub fn new(resource_type: ResourceType) -> Self {
        Self {
            resource_type,
            is_active: false,
            gather_timer: 0.0,
        }
    }
}

// ============================================================
// MERCHANT CARAVANS
// ============================================================

#[derive(Component)]
pub struct Merchant {
    pub gold_value: f32,
    pub destination: Vec2,
    pub has_arrived: bool,
    pub leave_timer: f32,
}

// ============================================================
// TRADE CARAVANS (Market Tier 2+)
// ============================================================

/// Rare items carried by trade caravans — provide temporary kingdom-wide buffs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RareItem {
    EnchantedWeapons, // +5 ATK for all heroes for 2 game-days
    BlessedArmor,     // +4 DEF for all heroes for 2 game-days
    HealingElixirs,   // Restores 30% HP to all heroes on arrival
    SwiftBoots,       // +15% hero speed for 2 game-days
    MoraleBanner,     // +20 morale to all heroes on arrival
}

impl RareItem {
    pub fn random() -> Self {
        match (rand::random::<f32>() * 5.0) as u32 {
            0 => RareItem::EnchantedWeapons,
            1 => RareItem::BlessedArmor,
            2 => RareItem::HealingElixirs,
            3 => RareItem::SwiftBoots,
            _ => RareItem::MoraleBanner,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            RareItem::EnchantedWeapons => "Enchanted Weapons",
            RareItem::BlessedArmor => "Blessed Armor",
            RareItem::HealingElixirs => "Healing Elixirs",
            RareItem::SwiftBoots => "Swift Boots",
            RareItem::MoraleBanner => "Morale Banner",
        }
    }

    pub fn cost(&self) -> f32 {
        match self {
            RareItem::EnchantedWeapons => 80.0,
            RareItem::BlessedArmor => 80.0,
            RareItem::HealingElixirs => 60.0,
            RareItem::SwiftBoots => 50.0,
            RareItem::MoraleBanner => 40.0,
        }
    }

    /// Duration of the buff in game-time seconds (2 game-days = 960s)
    pub fn buff_duration(&self) -> f32 {
        match self {
            RareItem::HealingElixirs | RareItem::MoraleBanner => 0.0, // Instant effect
            _ => 960.0, // 2 game-days
        }
    }
}

/// Trade caravan spawned by Market Tier 2+ — carries a rare item for purchase
#[derive(Component)]
pub struct TradeCaravan {
    pub item: RareItem,
    pub destination: Vec2,
    pub has_arrived: bool,
    pub leave_timer: f32,
}

/// Tracks active rare-item buffs applied to the kingdom
pub struct ActiveBuffs {
    pub atk_bonus: f32,
    pub atk_timer: f32,
    pub def_bonus: f32,
    pub def_timer: f32,
    pub speed_bonus: f32,
    pub speed_timer: f32,
}

impl Default for ActiveBuffs {
    fn default() -> Self {
        Self {
            atk_bonus: 0.0,
            atk_timer: 0.0,
            def_bonus: 0.0,
            def_timer: 0.0,
            speed_bonus: 0.0,
            speed_timer: 0.0,
        }
    }
}

// ============================================================
// FOG OF WAR
// ============================================================

pub struct FogOfWar {
    pub revealed_radius: f32, // How far from town center is revealed
    pub explored_areas: Vec<Vec2>, // Additional explored locations
    pub expansions: u32, // Number of times the player has expanded the map
}

impl Default for FogOfWar {
    fn default() -> Self {
        Self {
            revealed_radius: 300.0,
            explored_areas: Vec::new(),
            expansions: 0,
        }
    }
}

#[derive(Component)]
pub struct FogTile;

// ============================================================
// SPRITE ANIMATION
// ============================================================

#[derive(Component)]
pub struct SpriteAnimation {
    pub frame_count: usize,
    pub frame_timer: f32,
    pub frame_duration: f32,
    pub current_frame: usize,
    /// Number of frames per directional row (for LPC-style sheets).
    pub frames_per_row: usize,
    /// Current direction row offset (0=up, 1=left, 2=down, 3=right).
    pub row_offset: usize,
}

impl SpriteAnimation {
    pub fn new(frame_count: usize, fps: f32) -> Self {
        Self {
            frame_count,
            frame_timer: 0.0,
            frame_duration: 1.0 / fps,
            current_frame: 0,
            frames_per_row: frame_count,
            row_offset: 2, // default facing down (toward camera)
        }
    }

    /// Create a directional animation (LPC-style 4-row sprite sheet).
    pub fn new_directional(frames_per_row: usize, fps: f32) -> Self {
        Self {
            frame_count: frames_per_row,
            frame_timer: 0.0,
            frame_duration: 1.0 / fps,
            current_frame: 0,
            frames_per_row,
            row_offset: 2, // default facing down
        }
    }

    /// Get the atlas index accounting for direction row.
    pub fn atlas_index(&self) -> usize {
        self.row_offset * self.frames_per_row + self.current_frame
    }
}

/// Which animation mode an entity is currently playing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimMode {
    Walk,
    Idle,
    Attack,
    Hurt,
    Rest,
}

/// Stores atlas handles for each animation mode so the sprite can be swapped at runtime.
#[derive(Component)]
pub struct AnimationSet {
    pub walk_atlas: Handle<TextureAtlas>,
    pub walk_frames: usize,
    pub idle_atlas: Handle<TextureAtlas>,
    pub idle_frames: usize,
    pub rest_atlas: Handle<TextureAtlas>,
    pub rest_frames: usize,
    pub attack_atlas: Handle<TextureAtlas>,
    pub attack_frames: usize,
    pub hurt_atlas: Handle<TextureAtlas>,
    pub hurt_frames: usize,
    /// How many directional rows the hurt sheet has (1 for single-row, 4 for full).
    pub hurt_rows: usize,
    pub current_mode: AnimMode,
}

// ============================================================
// TORCH HALO (night visual effect around buildings)
// ============================================================

#[derive(Component)]
pub struct TorchHalo {
    pub parent_building: Entity,
    pub pulse_timer: f32,
}

// ============================================================
// INSPECT SYSTEM
// ============================================================

#[derive(Component)]
pub struct InspectText;

pub struct InspectTarget {
    pub entity: Option<Entity>,
}

impl Default for InspectTarget {
    fn default() -> Self {
        Self { entity: None }
    }
}

// ============================================================
// MILESTONES & ERA
// ============================================================

pub struct Milestones {
    pub cleared_first_den: bool,
    pub reached_village: bool,
    pub reached_town: bool,
    pub reached_city: bool,
    pub first_legendary_hero: bool,
    pub killed_first_boss: bool,
    pub built_all_types: bool,
    pub ten_heroes: bool,
}

impl Default for Milestones {
    fn default() -> Self {
        Self {
            cleared_first_den: false,
            reached_village: false,
            reached_town: false,
            reached_city: false,
            first_legendary_hero: false,
            killed_first_boss: false,
            built_all_types: false,
            ten_heroes: false,
        }
    }
}

pub struct LegacyUpgrades {
    pub tax_bonus_pct: f32,        // +% tax income
    pub hero_start_level: u32,     // Heroes start at this level
    pub building_hp_bonus_pct: f32,
    pub bounty_cost_reduction: f32,
}

impl Default for LegacyUpgrades {
    fn default() -> Self {
        Self {
            tax_bonus_pct: 0.0,
            hero_start_level: 1,
            building_hp_bonus_pct: 0.0,
            bounty_cost_reduction: 0.0,
        }
    }
}

// ============================================================
// ERA SIEGE
// ============================================================

pub struct EraState {
    pub era_length_days: u32,    // 30-60 in-game days
    pub siege_active: bool,
    pub siege_waves_remaining: u32,
    pub siege_spawn_timer: f32,
}

impl Default for EraState {
    fn default() -> Self {
        Self {
            era_length_days: 45,
            siege_active: false,
            siege_waves_remaining: 0,
            siege_spawn_timer: 0.0,
        }
    }
}

// ============================================================
// EQUIPMENT / CRAFTING
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    Weapon,
    Armor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentTier {
    Iron,     // Blacksmith tier 0
    Steel,    // Blacksmith tier 1
    Mithril,  // Blacksmith tier 2
    Legendary, // Blacksmith tier 3
}

impl EquipmentTier {
    pub fn display_name(&self) -> &str {
        match self {
            EquipmentTier::Iron => "Iron",
            EquipmentTier::Steel => "Steel",
            EquipmentTier::Mithril => "Mithril",
            EquipmentTier::Legendary => "Legendary",
        }
    }

    pub fn craft_cost(&self) -> f32 {
        match self {
            EquipmentTier::Iron => 40.0,
            EquipmentTier::Steel => 80.0,
            EquipmentTier::Mithril => 150.0,
            EquipmentTier::Legendary => 300.0,
        }
    }

    /// Tier available at given blacksmith upgrade tier
    pub fn from_blacksmith_tier(tier: u32) -> Self {
        match tier {
            0 => EquipmentTier::Iron,
            1 => EquipmentTier::Steel,
            2 => EquipmentTier::Mithril,
            _ => EquipmentTier::Legendary,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Equipment {
    pub slot: EquipmentSlot,
    pub tier: EquipmentTier,
    pub atk_bonus: f32,
    pub def_bonus: f32,
}

impl Equipment {
    pub fn weapon(tier: EquipmentTier) -> Self {
        let (atk, def) = match tier {
            EquipmentTier::Iron => (4.0, 0.0),
            EquipmentTier::Steel => (8.0, 1.0),
            EquipmentTier::Mithril => (14.0, 2.0),
            EquipmentTier::Legendary => (22.0, 4.0),
        };
        Self { slot: EquipmentSlot::Weapon, tier, atk_bonus: atk, def_bonus: def }
    }

    pub fn armor(tier: EquipmentTier) -> Self {
        let (atk, def) = match tier {
            EquipmentTier::Iron => (0.0, 3.0),
            EquipmentTier::Steel => (1.0, 6.0),
            EquipmentTier::Mithril => (2.0, 10.0),
            EquipmentTier::Legendary => (4.0, 16.0),
        };
        Self { slot: EquipmentSlot::Armor, tier, atk_bonus: atk, def_bonus: def }
    }

    pub fn display_name(&self) -> String {
        let slot_name = match self.slot {
            EquipmentSlot::Weapon => "Weapon",
            EquipmentSlot::Armor => "Armor",
        };
        format!("{} {}", self.tier.display_name(), slot_name)
    }
}

/// Component attached to heroes tracking their equipment
#[derive(Component, Debug, Clone, Default)]
pub struct HeroEquipment {
    pub weapon: Option<Equipment>,
    pub armor: Option<Equipment>,
}

impl HeroEquipment {
    pub fn total_atk_bonus(&self) -> f32 {
        let w = self.weapon.as_ref().map_or(0.0, |e| e.atk_bonus);
        let a = self.armor.as_ref().map_or(0.0, |e| e.atk_bonus);
        w + a
    }

    pub fn total_def_bonus(&self) -> f32 {
        let w = self.weapon.as_ref().map_or(0.0, |e| e.def_bonus);
        let a = self.armor.as_ref().map_or(0.0, |e| e.def_bonus);
        w + a
    }

    /// Returns the best tier the hero currently has, or None
    pub fn best_tier(&self) -> Option<EquipmentTier> {
        let tiers: Vec<EquipmentTier> = [&self.weapon, &self.armor]
            .iter()
            .filter_map(|e| e.as_ref().map(|eq| eq.tier))
            .collect();
        // Higher enum variant = better tier
        tiers.into_iter().max_by_key(|t| *t as u32)
    }

    /// Check if hero needs an upgrade for the given slot at the given tier
    pub fn needs_upgrade(&self, slot: EquipmentSlot, available_tier: EquipmentTier) -> bool {
        let current = match slot {
            EquipmentSlot::Weapon => &self.weapon,
            EquipmentSlot::Armor => &self.armor,
        };
        match current {
            None => true,
            Some(eq) => (available_tier as u32) > (eq.tier as u32),
        }
    }
}

// ============================================================
// BUILDING TIER ABILITIES
// ============================================================

/// Tracks active tier-2/3 building bonuses applied globally
pub struct BuildingBonuses {
    pub inn_heal_speed: f32,           // Multiplier (1.0 = normal, 1.5 = tier 1)
    pub market_trade_bonus: f32,       // Extra gold from merchants
    pub temple_morale_aura: f32,       // Morale regen bonus
    pub blacksmith_atk_bonus: f32,     // Flat ATK bonus for all heroes
    pub blacksmith_def_bonus: f32,     // Flat DEF bonus for all heroes
    pub alchemist_recovery_speed: f32, // Reduces death timer
    pub barracks_hero_cap_bonus: u32,  // Extra hero slots
    pub wizard_research_bonus: f32,    // Mage damage multiplier
    pub temple_pilgrim_income: f32,    // Tier 3 cathedral income
    // Road connection bonuses
    pub road_tax_bonus_pct: f32,       // % tax boost from Market road connections
    pub road_craft_bonus_pct: f32,     // % craft/ATK boost from Blacksmith road connections
    pub road_connected_pairs: u32,     // Number of road-connected building pairs
}

impl Default for BuildingBonuses {
    fn default() -> Self {
        Self {
            inn_heal_speed: 1.0,
            market_trade_bonus: 0.0,
            temple_morale_aura: 0.0,
            blacksmith_atk_bonus: 0.0,
            blacksmith_def_bonus: 0.0,
            alchemist_recovery_speed: 1.0,
            barracks_hero_cap_bonus: 0,
            wizard_research_bonus: 1.0,
            temple_pilgrim_income: 0.0,
            road_tax_bonus_pct: 0.0,
            road_craft_bonus_pct: 0.0,
            road_connected_pairs: 0,
        }
    }
}
