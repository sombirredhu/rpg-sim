#![allow(dead_code)]

use bevy::prelude::*;

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
            },
            HeroClass::Archer => HeroStats {
                max_hp: 90.0,
                hp: 90.0,
                attack: 22.0,
                defense: 5.0,
                speed: 45.0,
                attack_range: 150.0,
                risk_tolerance: 0.5,
            },
            HeroClass::Mage => HeroStats {
                max_hp: 70.0,
                hp: 70.0,
                attack: 30.0,
                defense: 3.0,
                speed: 30.0,
                attack_range: 120.0,
                risk_tolerance: 0.4,
            },
            HeroClass::Rogue => HeroStats {
                max_hp: 80.0,
                hp: 80.0,
                attack: 25.0,
                defense: 6.0,
                speed: 55.0,
                attack_range: 25.0,
                risk_tolerance: 0.6,
            },
            HeroClass::Healer => HeroStats {
                max_hp: 85.0,
                hp: 85.0,
                attack: 8.0,
                defense: 7.0,
                speed: 35.0,
                attack_range: 100.0,
                risk_tolerance: 0.3,
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
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            EnemyType::Goblin => "Goblin",
            EnemyType::Bandit => "Bandit",
            EnemyType::Troll => "Troll",
            EnemyType::GoblinElite => "Goblin Elite",
            EnemyType::BossWarlord => "Boss Warlord",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            EnemyType::Goblin => Color::rgb(0.2, 0.6, 0.1),
            EnemyType::Bandit => Color::rgb(0.5, 0.3, 0.1),
            EnemyType::Troll => Color::rgb(0.3, 0.5, 0.3),
            EnemyType::GoblinElite => Color::rgb(0.1, 0.4, 0.0),
            EnemyType::BossWarlord => Color::rgb(0.8, 0.1, 0.1),
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

/// Monster den / spawn point
#[derive(Component)]
pub struct MonsterDen {
    pub enemy_type: EnemyType,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub max_spawned: u32,
    pub current_spawned: u32,
    pub threat_tier: u32,      // Escalates over time
    pub weeks_unaddressed: u32,
}

impl MonsterDen {
    pub fn new(enemy_type: EnemyType) -> Self {
        Self {
            enemy_type,
            spawn_timer: 0.0,
            spawn_interval: 30.0, // 30 seconds between spawns
            max_spawned: 3,
            current_spawned: 0,
            threat_tier: 1,
            weeks_unaddressed: 0,
        }
    }
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
}

impl Default for BountyBoard {
    fn default() -> Self {
        Self {
            bounties: Vec::new(),
            next_id: 1,
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

    pub fn complete_bounty(&mut self, id: u32) -> Option<f32> {
        if let Some(bounty) = self.bounties.iter_mut().find(|b| b.id == id) {
            bounty.is_completed = true;
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
pub struct BuildMenuUi;

#[derive(Component)]
pub struct NightOverlay;

#[derive(Component)]
pub struct SpeedText;

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
}

impl Default for GamePhase {
    fn default() -> Self {
        Self {
            build_mode: false,
            selected_building: None,
            bounty_board_open: false,
            show_build_menu: false,
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
