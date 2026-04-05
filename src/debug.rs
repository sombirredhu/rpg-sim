#![allow(dead_code)]

use bevy::prelude::*;
use std::collections::VecDeque;

use crate::components::*;
use crate::sprites::SpriteAssets;

#[derive(Default)]
pub struct DebugConsole {
    pub active: bool,
    pub input_buffer: String,
    pub output: VecDeque<String>,
    pub cursor_visible: bool,
    pub cursor_timer: f32,
    pub pending_command: String,
}

#[derive(Default)]
pub struct DebugCommandHistory {
    pub entries: Vec<String>,
    pub index: usize,
    pub temp_input: String,
}

#[derive(Component)]
pub struct DebugConsoleRoot;

#[derive(Component)]
pub struct DebugConsoleInput;

#[derive(Component)]
pub struct DebugConsoleOutput;

// ============================================================
// SETUP
// ============================================================

pub fn setup_debug_console(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(260.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Column,
                padding: Rect::all(Val::Px(8.0)),
                ..Default::default()
            },
            color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.85)),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(DebugConsoleRoot)
        .with_children(|parent| {
            // Output area
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        overflow: Overflow::Hidden,
                        flex_grow: 1.0,
                        ..Default::default()
                    },
                    color: UiColor(Color::NONE),
                    ..Default::default()
                })
                .insert(DebugConsoleOutput)
                .with_children(|output| {
                    output.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font_size: 13.0,
                                color: Color::rgb(0.55, 1.0, 0.55),
                                font: font.clone(),
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Bottom,
                                horizontal: HorizontalAlign::Left,
                            },
                        ),
                        ..Default::default()
                    });
                });

            // Input row
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: UiColor(Color::NONE),
                    ..Default::default()
                })
                .insert(DebugConsoleInput)
                .with_children(|input| {
                    input.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "> ",
                            TextStyle {
                                font_size: 13.0,
                                color: Color::YELLOW,
                                font: font.clone(),
                            },
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    });
                    input.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font_size: 13.0,
                                color: Color::WHITE,
                                font,
                            },
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    });
                });
        });
}

// ============================================================
// INPUT
// ============================================================

pub fn debug_console_input(
    keyboard: Res<Input<KeyCode>>,
    mut debug_console: ResMut<DebugConsole>,
    mut command_history: ResMut<DebugCommandHistory>,
    mut game_phase: ResMut<GamePhase>,
    mut query_input: Query<&mut Text, With<DebugConsoleInput>>,
    mut query_root: Query<&mut Visibility, With<DebugConsoleRoot>>,
) {
    if keyboard.just_pressed(KeyCode::Grave) {
        if debug_console.active {
            debug_console.active = false;
            debug_console.input_buffer.clear();
            update_input_text(query_input, &debug_console.input_buffer);
            if let Ok(mut vis) = query_root.get_single_mut() {
                *vis = Visibility { is_visible: false };
            }
        } else {
            debug_console.active = true;
            debug_console.input_buffer.clear();
            debug_console.cursor_visible = true;
            debug_console.cursor_timer = 0.0;
            game_phase.build_mode = false;
            game_phase.bounty_board_open = false;
            game_phase.show_build_menu = false;
            if let Ok(mut vis) = query_root.get_single_mut() {
                *vis = Visibility { is_visible: true };
            }
            update_input_text(query_input, &debug_console.input_buffer);
        }
        return;
    }

    if keyboard.just_pressed(KeyCode::Escape) && debug_console.active {
        debug_console.active = false;
        debug_console.input_buffer.clear();
        update_input_text(query_input, &debug_console.input_buffer);
        if let Ok(mut vis) = query_root.get_single_mut() {
            *vis = Visibility { is_visible: false };
        }
        return;
    }

    if !debug_console.active {
        return;
    }

    debug_console.cursor_timer += 0.016;
    if debug_console.cursor_timer > 0.5 {
        debug_console.cursor_timer = 0.0;
        debug_console.cursor_visible = !debug_console.cursor_visible;
        refresh_input_display(&mut query_input, &debug_console);
    }

    if keyboard.just_pressed(KeyCode::Up) {
        if !command_history.entries.is_empty() {
            if command_history.entries.len() == command_history.index {
                command_history.temp_input = debug_console.input_buffer.clone();
            }
            if command_history.index > 0 {
                command_history.index -= 1;
            }
            debug_console.input_buffer = command_history.entries[command_history.index].clone();
            update_input_text(query_input, &debug_console.input_buffer);
            return;
        }
    }
    if keyboard.just_pressed(KeyCode::Down) {
        if command_history.index < command_history.entries.len() {
            command_history.index += 1;
            if command_history.index == command_history.entries.len() {
                debug_console.input_buffer = command_history.temp_input.clone();
            } else {
                debug_console.input_buffer = command_history.entries[command_history.index].clone();
            }
            update_input_text(query_input, &debug_console.input_buffer);
            return;
        }
    }

    if keyboard.just_pressed(KeyCode::Back) && !debug_console.input_buffer.is_empty() {
        debug_console.input_buffer.pop();
        update_input_text(query_input, &debug_console.input_buffer);
        return;
    }

    if keyboard.just_pressed(KeyCode::Return) {
        let cmd = debug_console.input_buffer.clone();
        if !cmd.trim().is_empty() {
            command_history.entries.push(cmd.clone());
            command_history.index = command_history.entries.len();
            command_history.temp_input.clear();
            debug_console.output.push_back(format!("> {}", cmd));
            debug_console.pending_command = cmd.trim().to_string();
        }
        debug_console.input_buffer.clear();
        update_input_text(query_input, &debug_console.input_buffer);
        return;
    }

    let char_keys = [
        (KeyCode::Key1, '1'), (KeyCode::Key2, '2'), (KeyCode::Key3, '3'),
        (KeyCode::Key4, '4'), (KeyCode::Key5, '5'), (KeyCode::Key6, '6'),
        (KeyCode::Key7, '7'), (KeyCode::Key8, '8'), (KeyCode::Key9, '9'),
        (KeyCode::Key0, '0'),
        (KeyCode::A, 'a'), (KeyCode::B, 'b'), (KeyCode::C, 'c'),
        (KeyCode::D, 'd'), (KeyCode::E, 'e'), (KeyCode::F, 'f'),
        (KeyCode::G, 'g'), (KeyCode::H, 'h'), (KeyCode::I, 'i'),
        (KeyCode::J, 'j'), (KeyCode::K, 'k'), (KeyCode::L, 'l'),
        (KeyCode::M, 'm'), (KeyCode::N, 'n'), (KeyCode::O, 'o'),
        (KeyCode::P, 'p'), (KeyCode::Q, 'q'), (KeyCode::R, 'r'),
        (KeyCode::S, 's'), (KeyCode::T, 't'), (KeyCode::U, 'u'),
        (KeyCode::V, 'v'), (KeyCode::W, 'w'), (KeyCode::X, 'x'),
        (KeyCode::Y, 'y'), (KeyCode::Z, 'z'),
        (KeyCode::Numpad0, '0'), (KeyCode::Numpad1, '1'), (KeyCode::Numpad2, '2'),
        (KeyCode::Numpad3, '3'), (KeyCode::Numpad4, '4'), (KeyCode::Numpad5, '5'),
        (KeyCode::Numpad6, '6'), (KeyCode::Numpad7, '7'), (KeyCode::Numpad8, '8'),
        (KeyCode::Numpad9, '9'),
        (KeyCode::Minus, '-'), (KeyCode::Equals, '='),
        (KeyCode::Comma, ','), (KeyCode::Period, '.'), (KeyCode::Slash, '/'),
    ];
    let shift = keyboard.pressed(KeyCode::LShift) || keyboard.pressed(KeyCode::RShift);
    for (kc, ch) in char_keys.iter() {
        if keyboard.just_pressed(*kc) {
            let c = if shift {
                match ch {
                    'a'..='z' => ch.to_ascii_uppercase(),
                    '/' => '?',
                    '.' => '>',
                    ',' => '<',
                    '-' => '_',
                    '=' => '+',
                    _ => *ch,
                }
            } else {
                *ch
            };
            debug_console.input_buffer.push(c);
            update_input_text(query_input, &debug_console.input_buffer);
            return;
        }
    }
    if keyboard.just_pressed(KeyCode::Space) {
        debug_console.input_buffer.push(' ');
        update_input_text(query_input, &debug_console.input_buffer);
    }
}

fn update_input_text(mut query: Query<&mut Text, With<DebugConsoleInput>>, input: &str) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("{}\u{25AE}", input);
    }
}

fn refresh_input_display(
    query: &mut Query<&mut Text, With<DebugConsoleInput>>,
    dc: &DebugConsole,
) {
    let cursor = if dc.cursor_visible { "\u{25AE}" } else { " " };
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("{}{}", dc.input_buffer, cursor);
    }
}

// ============================================================
// UI UPDATE
// ============================================================

pub fn debug_console_ui_update(
    mut debug_console: ResMut<DebugConsole>,
    mut query_output: Query<&mut Text, With<DebugConsoleOutput>>,
) {
    while debug_console.output.len() > 50 {
        debug_console.output.pop_front();
    }
    let lines: Vec<_> = debug_console.output.iter().cloned().collect();
    let display = lines.iter().rev().take(12).rev().cloned().collect::<Vec<_>>().join("\n");
    if let Ok(mut text_comp) = query_output.get_single_mut() {
        text_comp.sections[0].value = display;
    }
}

// ============================================================
// COMMAND PARSER + EXECUTOR
// ============================================================

pub fn debug_command_executor(
    mut debug_console: ResMut<DebugConsole>,
    mut economy: ResMut<GameEconomy>,
    mut game_time: ResMut<GameTime>,
    mut kingdom: ResMut<KingdomState>,
    _alerts: ResMut<GameAlerts>,
    mut bounty_board: ResMut<BountyBoard>,
    mut fog: ResMut<FogOfWar>,
    sprites: Option<Res<SpriteAssets>>,
    mut commands: Commands,
    heroes: Query<(Entity, &Hero, &HeroStats), Without<Building>>,
    enemies: Query<(Entity, &Enemy), Without<Hero>>,
    buildings: Query<(Entity, &Building, &Transform), Without<Enemy>>,
    cameras: Query<&Transform, With<MainCamera>>,
) {
    if debug_console.pending_command.trim().is_empty() {
        return;
    }

    let cmd_str = debug_console.pending_command.trim().to_string();
    debug_console.pending_command.clear();

    let parts: Vec<&str> = cmd_str.split_whitespace().collect();
    if parts.is_empty() { return; }
    let cmd = parts[0].trim_start_matches('/').to_lowercase();
    let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

    // Borrow checker: take output temporarily, put it back at end
    let output = std::mem::take(&mut debug_console.output);
    let dc = &mut DebugConsole {
        active: false,
        input_buffer: String::new(),
        output,
        cursor_visible: false,
        cursor_timer: 0.0,
        pending_command: String::new(),
    };

    match cmd.as_str() {
        "help" | "h" => {
            dc_push(dc, "--- Debug Console Commands ---");
            dc_push(dc, "  /gold [amount]          - Set treasury gold");
            dc_push(dc, "  /gold+ [amount]         - Add gold");
            dc_push(dc, "  /spawn_hero [class]     - warrior/archer/mage/rogue/healer");
            dc_push(dc, "  /spawn_enemy [type]     - goblin/bandit/skeleton/troll/boss");
            dc_push(dc, "  /kill_hero [idx/all]    - Kill hero by index or all");
            dc_push(dc, "  /kill_enemy [idx/all]   - Kill enemy by index or all");
            dc_push(dc, "  /list_heroes /heroes     - List heroes with HP/level");
            dc_push(dc, "  /list_enemies /enemies   - List active enemies");
            dc_push(dc, "  /list_buildings /builds  - List buildings with tier");
            dc_push(dc, "  /pause                   - Toggle pause");
            dc_push(dc, "  /speed [0/1/2/3]        - Set game speed");
            dc_push(dc, "  /time /day               - Current time info");
            dc_push(dc, "  /bounty [amount]        - Place bounty at town center");
            dc_push(dc, "  /teleport [id] [x] [y]  - Move entity by index");
            dc_push(dc, "  /rank [name]            - Set kingdom rank");
            dc_push(dc, "  /fog [radius]            - Set fog revealed radius");
            dc_push(dc, "  /stats /status           - Full game stats");
            dc_push(dc, "  /clear                   - Clear console");
        }

        "gold" | "g" => {
            if args.is_empty() {
                dc_push(dc, &format!("  Current: {:.0}g", economy.gold));
            } else if let Ok(v) = args[0].parse::<f32>() {
                economy.gold = v;
                dc_push(dc, &format!("  Gold set to {:.0}", v));
            } else {
                dc_push(dc, "  Invalid number");
            }
        }

        "gold+" | "addgold" => {
            if args.is_empty() { dc_push(dc, "  Usage: /gold+ <amount>"); return; }
            if let Ok(v) = args[0].parse::<f32>() {
                let gold_before = economy.gold;
                economy.gold += v;
                economy.total_earned += v;
                dc_push(dc, &format!("  Added {:.0}g (was {:.0}, now {:.0})", v, gold_before, economy.gold));
            } else {
                dc_push(dc, "  Invalid number");
            }
        }

        "clear" | "cls" => {
            dc.output.clear();
            dc_push(dc, "  Cleared");
        }

        "pause" | "p" => {
            game_time.is_paused = !game_time.is_paused;
            game_time.speed_multiplier = if game_time.is_paused { 0.0 } else { 1.0 };
            let state = if game_time.is_paused { "paused" } else { "running" };
            dc_push(dc, &format!("  Game {}", state));
        }

        "speed" | "s" => {
            if args.is_empty() { dc_push(dc, "  Usage: /speed <0|1|2|3>"); return; }
            match args[0].parse::<f32>() {
                Ok(v) if v == 0.0 || v == 1.0 || v == 2.0 || v == 3.0 => {
                    game_time.is_paused = v == 0.0;
                    game_time.speed_multiplier = v;
                    let display = if v == 0.0 { "paused" } else { &format!("{}x", v as i32) };
                    dc_push(dc, &format!("  Speed -> {}", display));
                }
                _ => { dc_push(dc, "  Must be 0, 1, 2, or 3"); }
            }
        }

        "time" | "day" => {
            dc_push(dc, &format!("  Day {} | {:?} | {:.1}s elapsed | day progress: {:.0}%",
                game_time.current_day, game_time.time_of_day, game_time.time_seconds,
                game_time.day_progress * 100.0));
        }

        "bounty" | "add_bounty" => {
            if args.is_empty() { dc_push(dc, "  Usage: /bounty <amount>"); return; }
            if let Ok(amount) = args[0].parse::<f32>() {
                if amount > economy.gold {
                    dc_push(dc, &format!("  Not enough gold (need {:.0}, have {:.0})", amount, economy.gold));
                    return;
                }
                economy.gold -= amount;
                let bounty = Bounty {
                    id: bounty_board.next_id,
                    bounty_type: BountyType::Objective,
                    gold_reward: amount,
                    location: Vec2::new(0.0, 0.0),  // town center
                    target_entity: None,
                    danger_level: 1,
                    is_completed: false,
                    assigned_hero: None,
                };
                bounty_board.next_id += 1;
                bounty_board.bounties.push(bounty);
                dc_push(dc, &format!("  Bounty placed: {:.0}g at center (id: {})", amount, bounty_board.next_id - 1));
            } else {
                dc_push(dc, "  Invalid amount");
            }
        }

        "stats" | "status" => {
            dc_push(dc, &format!("  === Kingdom State ==="));
            dc_push(dc, &format!("  Gold: {:.0}g (+{:.0}/min)", economy.gold, economy.income_per_minute));
            dc_push(dc, &format!("  Rank: {:?} | Score: {}", kingdom.rank, kingdom.score));
            dc_push(dc, &format!("  Era: {} (day {}) | Legacy pts: {}", kingdom.era, kingdom.era_day, kingdom.legacy_points));
            dc_push(dc, &format!("  Heroes: {} | Buildings: {}", kingdom.hero_count, kingdom.buildings_count));
            dc_push(dc, &format!("  Day {} | {:?} | Speed: {}x | Paused: {}",
                game_time.current_day, game_time.time_of_day,
                game_time.speed_multiplier as i32, game_time.is_paused));
            dc_push(dc, &format!("  Fog radius: {:.0} | Expansions: {}", fog.revealed_radius, fog.expansions));
            dc_push(dc, &format!("  Bounties completed: {}", bounty_board.total_bounties_completed));
            dc_push(dc, &format!("  Lifetime earned: {:.0} | spent: {:.0}", economy.total_earned, economy.total_spent));

            let hero_count = heroes.iter().count();
            let enemy_count = enemies.iter().count();
            let building_count = buildings.iter().count();
            dc_push(dc, &format!("  Active entities: {} heroes, {} enemies, {} buildings", hero_count, enemy_count, building_count));
        }

        "fog" | "fow" => {
            if args.is_empty() { dc_push(dc, &format!("  Current radius: {:.0}", fog.revealed_radius)); return; }
            if let Ok(r) = args[0].parse::<f32>() {
                fog.revealed_radius = r;
                dc_push(dc, &format!("  Fog radius -> {:.0}", r));
            } else {
                dc_push(dc, "  Invalid number");
            }
        }

        "rank" => {
            if args.is_empty() {
                dc_push(dc, &format!("  Current: {:?}. Options: Hamlet, Village, Town, City, Kingdom", kingdom.rank));
                return;
            }
            let rank_str = args.join(" ");
            let new_rank = match rank_str.to_lowercase().as_str() {
                "hamlet" => KingdomRank::Hamlet,
                "village" => KingdomRank::Village,
                "town" => KingdomRank::Town,
                "city" => KingdomRank::City,
                "kingdom" => KingdomRank::Kingdom,
                _ => { dc_push(dc, "  Unknown rank"); return; }
            };
            kingdom.rank = new_rank;
            dc_push(dc, &format!("  Rank set to {:?}", new_rank));
        }

        "teleport" | "tp" => {
            if args.len() < 3 { dc_push(dc, "  Usage: /teleport <idx> <x> <y>"); return; }
            if let (Ok(idx), Ok(x), Ok(y)) = (args[0].parse::<usize>(), args[1].parse::<f32>(), args[2].parse::<f32>()) {
                if let Some((entity, _, _)) = heroes.iter().nth(idx) {
                    commands.entity(entity).insert(Transform {
                        translation: Vec3::new(x, y, 10.0),
                        ..Default::default()
                    });
                    dc_push(dc, &format!("  Hero #{} moved to ({:.0}, {:.0})", idx, x, y));
                } else {
                    dc_push(dc, &format!("  No hero at index {}", idx));
                }
            } else {
                dc_push(dc, "  Invalid args");
            }
        }

        "spawn_hero" | "spawn" => {
            if args.is_empty() { dc_push(dc, "  Classes: warrior, archer, mage, rogue, healer"); return; }
            let class = match args[0].to_lowercase().as_str() {
                "warrior" => Some(HeroClass::Warrior),
                "archer" => Some(HeroClass::Archer),
                "mage" => Some(HeroClass::Mage),
                "rogue" => Some(HeroClass::Rogue),
                "healer" => Some(HeroClass::Healer),
                _ => None,
            };
            if let Some(hero_class) = class {
                if let Some(sprites) = sprites.as_ref() {
                    // Spawn near town center with slight randomness
                    let x = (rand::random::<f32>() - 0.5) * 100.0;
                    let y = (rand::random::<f32>() - 0.5) * 100.0;
                    let pos = Vec3::new(x, y, 10.0);
                    crate::sprites::spawn_hero_with_sprite(&mut commands, sprites, hero_class, pos);
                    dc_push(dc, &format!("  Spawned {:?} at ({:.0}, {:.0})", hero_class, x, y));
                } else {
                    dc_push(dc, "  Sprites not loaded yet");
                }
            } else {
                dc_push(dc, &format!("  Unknown class '{}'", args[0]));
            }
        }

        "spawn_enemy" | "spawn_en" => {
            if args.is_empty() { dc_push(dc, "  Types: goblin, bandit, skeleton, troll, werewolf, goblin_elite, boss"); return; }
            let etype = match args[0].to_lowercase().as_str() {
                "goblin" => Some(EnemyType::Goblin),
                "bandit" => Some(EnemyType::Bandit),
                "skeleton" => Some(EnemyType::Troll),
                "troll" => Some(EnemyType::Troll),
                "werewolf" => Some(EnemyType::Werewolf),
                "goblin_elite" => Some(EnemyType::GoblinElite),
                "boss" => Some(EnemyType::BossWarlord),
                _ => None,
            };
            if let Some(etype) = etype {
                if let Some(sprites) = sprites.as_ref() {
                    let angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
                    let dist = 300.0 + rand::random::<f32>() * 100.0;
                    let x = angle.cos() * dist;
                    let y = angle.sin() * dist;
                    let pos = Vec3::new(x, y, 10.0);
                    crate::sprites::spawn_enemy_with_sprite(&mut commands, sprites, etype, pos);
                    dc_push(dc, &format!("  Spawned {:?} at ({:.0}, {:.0})", etype, x, y));
                } else {
                    dc_push(dc, "  Sprites not loaded yet");
                }
            } else {
                dc_push(dc, &format!("  Unknown type '{}'", args[0]));
            }
        }

        "kill_hero" | "kill_heroes" => {
            if args.is_empty() { dc_push(dc, "  Usage: /kill_hero <idx|all>"); return; }
            if args[0].to_lowercase() == "all" {
                let count = heroes.iter().count();
                for (entity, _, _) in heroes.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                dc_push(dc, &format!("  Killed all {} heroes", count));
            } else if let Ok(idx) = args[0].parse::<usize>() {
                if let Some((entity, hero, _)) = heroes.iter().nth(idx) {
                    let class = hero.class;
                    commands.entity(entity).despawn_recursive();
                    dc_push(dc, &format!("  Killed {:?} #{}", class, idx));
                } else {
                    dc_push(dc, &format!("  No hero at index {}", idx));
                }
            } else {
                dc_push(dc, "  Invalid index");
            }
        }

        "kill_enemy" | "kill_enemies" => {
            if args.is_empty() { dc_push(dc, "  Usage: /kill_enemy <idx|all>"); return; }
            if args[0].to_lowercase() == "all" {
                let count = enemies.iter().count();
                for (entity, _) in enemies.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                dc_push(dc, &format!("  Killed all {} enemies", count));
            } else if let Ok(idx) = args[0].parse::<usize>() {
                if let Some((entity, enemy)) = enemies.iter().nth(idx) {
                    let etype = enemy.enemy_type.clone();
                    commands.entity(entity).despawn_recursive();
                    dc_push(dc, &format!("  Killed {:?} #{}", etype, idx));
                } else {
                    dc_push(dc, &format!("  No enemy at index {}", idx));
                }
            } else {
                dc_push(dc, "  Invalid index");
            }
        }

        "list_heroes" | "heroes" => {
            let heroes_vec: Vec<_> = heroes.iter().collect();
            if heroes_vec.is_empty() {
                dc_push(dc, "  No heroes active");
            } else {
                dc_push(dc, &format!("  === {} Active Heroes ===", heroes_vec.len()));
                for (i, (_entity, hero, stats)) in heroes_vec.iter().enumerate() {
                    dc_push(dc, &format!("  #{} {:?} Lvl {} HP: {:.0}/{} ATK: {:.0} DEF: {:.0} Morale: {:.1}",
                        i, hero.class, hero.level,
                        stats.hp, stats.max_hp, stats.attack, stats.defense,
                        hero.morale));
                }
            }
        }

        "list_enemies" | "enemies" => {
            let enemies_vec: Vec<_> = enemies.iter().collect();
            if enemies_vec.is_empty() {
                dc_push(dc, "  No enemies active");
            } else {
                dc_push(dc, &format!("  === {} Active Enemies ===", enemies_vec.len()));
                for (i, (_entity, enemy)) in enemies_vec.iter().enumerate() {
                    dc_push(dc, &format!("  #{} {:?}", i, enemy.enemy_type));
                }
            }
        }

        "list_buildings" | "buildings" | "builds" => {
            let bldgs: Vec<_> = buildings.iter().collect();
            if bldgs.is_empty() {
                dc_push(dc, "  No buildings");
            } else {
                dc_push(dc, &format!("  === {} Buildings ===", bldgs.len()));
                for (i, (_entity, bld, transform)) in bldgs.iter().enumerate() {
                    dc_push(dc, &format!("  #{} {:?} tier {} HP: {:.0} at ({:.0}, {:.0})",
                        i, bld.building_type, bld.tier, bld.hp,
                        transform.translation.x, transform.translation.y));
                }
            }
        }

        "pos" => {
            if let Ok(cam) = cameras.get_single() {
                dc_push(dc, &format!("  Camera at ({:.0}, {:.0}, {:.0})",
                    cam.translation.x, cam.translation.y, cam.translation.z));
            } else {
                dc_push(dc, "  Camera not found");
            }
        }

        _ => {
            dc_push(dc, &format!("  Unknown: '{}'. Type /help", cmd));
        }
    }

    // Restore output buffer back into the main debug_console
    debug_console.output = std::mem::take(&mut dc.output);
}

fn dc_push(dc: &mut DebugConsole, msg: &str) {
    dc.output.push_back(msg.to_string());
}
