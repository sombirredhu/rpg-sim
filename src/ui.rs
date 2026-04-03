use bevy::prelude::*;
use crate::components::*;

/// Startup: Create the HUD UI
pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // UI Camera
    commands.spawn_bundle(UiCameraBundle::default());

    // Root UI node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            // ===== TOP BAR =====
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(40.0)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: Rect::all(Val::Px(8.0)),
                    ..Default::default()
                },
                color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.7)),
                ..Default::default()
            })
            .with_children(|top_bar| {
                // Gold display
                top_bar.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Gold: 500",
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::rgb(1.0, 0.85, 0.0),
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(GoldText);

                // Day/Night clock
                top_bar.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Day 1 - Dawn",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::WHITE,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(DayNightText);

                // Kingdom rank
                top_bar.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Hamlet",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::rgb(0.7, 0.9, 1.0),
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(KingdomRankText);

                // Speed display
                top_bar.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "1x",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(SpeedText);
            });

            // ===== HERO PANEL (right side) =====
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(220.0), Val::Auto),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        right: Val::Px(5.0),
                        top: Val::Px(50.0),
                        ..Default::default()
                    },
                    padding: Rect::all(Val::Px(8.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.6)),
                ..Default::default()
            })
            .with_children(|panel| {
                panel.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Heroes: 0",
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(HeroPanelText);
            });

            // ===== BOUNTY BOARD PANEL (left side) =====
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(260.0), Val::Auto),
                    max_size: Size::new(Val::Px(260.0), Val::Px(400.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(5.0),
                        top: Val::Px(50.0),
                        ..Default::default()
                    },
                    padding: Rect::all(Val::Px(8.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.75)),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(BountyBoardUi)
            .with_children(|panel| {
                panel.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Bounty Board",
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            color: Color::rgb(1.0, 0.85, 0.0),
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(BountyBoardText);
            });

            // ===== ALERT TEXT (bottom center) =====
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Val::Px(60.0),
                        ..Default::default()
                    },
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color: UiColor(Color::NONE),
                ..Default::default()
            })
            .with_children(|alert_area| {
                alert_area.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::rgb(1.0, 1.0, 0.5),
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(AlertText);
            });

            // ===== BOTTOM BAR (controls help) =====
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(30.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Val::Px(5.0),
                        ..Default::default()
                    },
                    padding: Rect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.5)),
                ..Default::default()
            })
            .with_children(|bar| {
                bar.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "WASD:Move | Scroll:Zoom | B:Build | U:Upgrade | E:Expand | 1/2/3:Speed | Space:Pause | Q:Bounty",
                        TextStyle {
                            font: font.clone(),
                            font_size: 13.0,
                            color: Color::rgb(0.7, 0.7, 0.7),
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                });
            });
        });
}

/// System: Update gold display
pub fn update_gold_ui(
    economy: Res<GameEconomy>,
    mut text_query: Query<&mut Text, With<GoldText>>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!(
            "Gold: {:.0}  (+{:.1}/min)",
            economy.gold, economy.income_per_minute
        );
    }
}

/// System: Update day/night display
pub fn update_day_night_ui(
    game_time: Res<GameTime>,
    mut text_query: Query<&mut Text, With<DayNightText>>,
) {
    let time_name = match game_time.time_of_day {
        TimeOfDay::Dawn => "Dawn",
        TimeOfDay::Day => "Day",
        TimeOfDay::Dusk => "Dusk",
        TimeOfDay::Night => "Night",
    };

    let paused = if game_time.is_paused { " [PAUSED]" } else { "" };

    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!(
            "Day {} - {}{}",
            game_time.current_day, time_name, paused
        );

        // Color based on time
        text.sections[0].style.color = match game_time.time_of_day {
            TimeOfDay::Dawn => Color::rgb(1.0, 0.8, 0.5),
            TimeOfDay::Day => Color::WHITE,
            TimeOfDay::Dusk => Color::rgb(1.0, 0.5, 0.3),
            TimeOfDay::Night => Color::rgb(0.5, 0.5, 0.9),
        };
    }
}

/// System: Update hero panel
pub fn update_hero_panel_ui(
    heroes: Query<(&Hero, &HeroStats, &HeroState)>,
    mut text_query: Query<&mut Text, With<HeroPanelText>>,
) {
    let mut info = String::new();
    let mut count = 0;
    let mut class_counts = [0u32; 5];

    for (hero, _stats, _state) in heroes.iter() {
        count += 1;
        match hero.class {
            HeroClass::Warrior => class_counts[0] += 1,
            HeroClass::Archer => class_counts[1] += 1,
            HeroClass::Mage => class_counts[2] += 1,
            HeroClass::Rogue => class_counts[3] += 1,
            HeroClass::Healer => class_counts[4] += 1,
        }
    }

    info.push_str(&format!("Heroes: {}\n", count));
    if class_counts[0] > 0 { info.push_str(&format!("  Warriors: {}\n", class_counts[0])); }
    if class_counts[1] > 0 { info.push_str(&format!("  Archers: {}\n", class_counts[1])); }
    if class_counts[2] > 0 { info.push_str(&format!("  Mages: {}\n", class_counts[2])); }
    if class_counts[3] > 0 { info.push_str(&format!("  Rogues: {}\n", class_counts[3])); }
    if class_counts[4] > 0 { info.push_str(&format!("  Healers: {}\n", class_counts[4])); }

    // Show first few hero details
    let mut shown = 0;
    for (hero, stats, state) in heroes.iter() {
        if shown >= 5 {
            info.push_str("  ...\n");
            break;
        }
        let state_str = match state {
            HeroState::Idle => "Idle",
            HeroState::MovingTo { .. } => "Moving",
            HeroState::AttackingEnemy { .. } => "Fighting",
            HeroState::PursuingBounty { .. } => "Bounty",
            HeroState::Resting => "Resting",
            HeroState::Shopping => "Shopping",
            HeroState::Dead { .. } => "Dead",
        };
        let legendary = if hero.is_legendary { "*" } else { "" };
        info.push_str(&format!(
            "  {}{}Lv{} HP:{:.0}/{:.0} [{}]\n",
            legendary,
            hero.class.display_name(),
            hero.level,
            stats.hp,
            stats.max_hp,
            state_str,
        ));
        shown += 1;
    }

    for mut text in text_query.iter_mut() {
        text.sections[0].value = info.clone();
    }
}

/// System: Update kingdom rank display
pub fn update_kingdom_rank_ui(
    kingdom: Res<KingdomState>,
    mut text_query: Query<&mut Text, With<KingdomRankText>>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!(
            "{} | Score: {} | Era {}",
            kingdom.rank.display_name(),
            kingdom.score,
            kingdom.era,
        );
    }
}

/// System: Update speed display
pub fn update_speed_ui(
    game_time: Res<GameTime>,
    mut text_query: Query<&mut Text, With<SpeedText>>,
) {
    for mut text in text_query.iter_mut() {
        if game_time.is_paused {
            text.sections[0].value = "||".to_string();
        } else {
            text.sections[0].value = format!("{}x", game_time.speed_multiplier as u32);
        }
    }
}

/// System: Show alert messages
pub fn update_alerts_ui(
    mut alerts: ResMut<GameAlerts>,
    mut text_query: Query<&mut Text, With<AlertText>>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    // Update timers and remove expired
    alerts.messages.retain_mut(|(_, timer)| {
        *timer -= dt;
        *timer > 0.0
    });

    // Show most recent alert
    let display = alerts.messages.last()
        .map(|(msg, _)| msg.clone())
        .unwrap_or_default();

    for mut text in text_query.iter_mut() {
        text.sections[0].value = display.clone();
    }
}

/// System: Update bounty board panel visibility and content
pub fn update_bounty_board_ui(
    game_phase: Res<GamePhase>,
    bounty_board: Res<BountyBoard>,
    mut panel_query: Query<&mut Visibility, With<BountyBoardUi>>,
    mut text_query: Query<&mut Text, With<BountyBoardText>>,
) {
    // Toggle panel visibility
    for mut vis in panel_query.iter_mut() {
        vis.is_visible = game_phase.bounty_board_open;
    }

    if !game_phase.bounty_board_open {
        return;
    }

    // Build bounty list text
    let mut info = String::from("=== BOUNTY BOARD ===\n\n");

    let active: Vec<_> = bounty_board.bounties.iter().filter(|b| !b.is_completed).collect();

    if active.is_empty() {
        info.push_str("  No active bounties.\n");
    } else {
        for bounty in &active {
            let type_icon = match bounty.bounty_type {
                BountyType::Monster => "[M]",
                BountyType::Exploration => "[E]",
                BountyType::Objective => "[O]",
                BountyType::Resource => "[R]",
            };
            let type_name = match bounty.bounty_type {
                BountyType::Monster => "Monster",
                BountyType::Exploration => "Explore",
                BountyType::Objective => "Objective",
                BountyType::Resource => "Resource",
            };
            let status = if bounty.assigned_hero.is_some() {
                "In Progress"
            } else {
                "Available"
            };
            let danger_str: String = (0..bounty.danger_level).map(|_| '!').collect();
            info.push_str(&format!(
                "{} {} {:.0}g {}\n   Danger:{} | {}\n",
                type_icon, type_name, bounty.gold_reward, danger_str, bounty.danger_level, status
            ));
        }
    }

    let available_count = active.iter().filter(|b| b.assigned_hero.is_none()).count();
    let in_progress_count = active.iter().filter(|b| b.assigned_hero.is_some()).count();
    info.push_str(&format!(
        "\nTotal: {} | Avail: {} | Active: {}\n",
        active.len(), available_count, in_progress_count
    ));
    info.push_str("Click map to place (30g)");

    for mut text in text_query.iter_mut() {
        text.sections[0].value = info.clone();
    }
}

/// System: Build menu keyboard shortcuts
pub fn build_menu_system(
    keyboard: Res<Input<KeyCode>>,
    mut game_phase: ResMut<GamePhase>,
    kingdom: Res<KingdomState>,
    mut alerts: ResMut<GameAlerts>,
) {
    // Toggle build mode with B
    if keyboard.just_pressed(KeyCode::B) {
        if game_phase.build_mode {
            game_phase.build_mode = false;
            game_phase.selected_building = None;
            alerts.push("Build mode OFF".to_string());
        } else {
            game_phase.show_build_menu = !game_phase.show_build_menu;
            if game_phase.show_build_menu {
                let available = kingdom.rank.available_buildings();
                let mut msg = "BUILD MENU:\n".to_string();
                for (i, bt) in available.iter().enumerate() {
                    msg.push_str(&format!("  {}: {} ({:.0}g)\n", i + 1, bt.display_name(), bt.cost()));
                }
                msg.push_str("Press number to select, then click to place");
                alerts.push(msg);
            }
        }
    }

    // Number keys to select building in build menu
    if game_phase.show_build_menu {
        let available = kingdom.rank.available_buildings();
        let key_map = [
            (KeyCode::Key1, 0), (KeyCode::Key2, 1), (KeyCode::Key3, 2),
            (KeyCode::Key4, 3), (KeyCode::Key5, 4), (KeyCode::Key6, 5),
            (KeyCode::Key7, 6),
        ];

        for (key, idx) in key_map {
            if keyboard.just_pressed(key) {
                if let Some(building_type) = available.get(idx) {
                    game_phase.selected_building = Some(*building_type);
                    game_phase.build_mode = true;
                    game_phase.show_build_menu = false;
                    alerts.push(format!(
                        "Placing {} - Click to build, Right-click to cancel",
                        building_type.display_name()
                    ));
                }
            }
        }
    }

    // Manual bounty placement with Q
    if keyboard.just_pressed(KeyCode::Q) {
        game_phase.bounty_board_open = !game_phase.bounty_board_open;
        if game_phase.bounty_board_open {
            alerts.push("Bounty Board open - Click on map to place bounty".to_string());
        } else {
            alerts.push("Bounty Board closed".to_string());
        }
    }
}

/// System: Manual bounty placement
pub fn manual_bounty_system(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &Transform), With<Camera>>,
    game_phase: Res<GamePhase>,
    mut bounty_board: ResMut<BountyBoard>,
    mut economy: ResMut<GameEconomy>,
    mut alerts: ResMut<GameAlerts>,
) {
    if !game_phase.bounty_board_open || game_phase.build_mode {
        return;
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        let window = match windows.get_primary() {
            Some(w) => w,
            None => return,
        };
        let cursor_pos = match window.cursor_position() {
            Some(p) => p,
            None => return,
        };

        if let Ok((_camera, camera_transform)) = camera.get_single() {
            let window_size = Vec2::new(window.width(), window.height());
            let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
            let world_pos = camera_transform.translation.truncate()
                + ndc * Vec2::new(window_size.x, window_size.y) * 0.3;

            let bounty_cost = 30.0;
            if economy.gold >= bounty_cost {
                economy.gold -= bounty_cost;
                economy.total_spent += bounty_cost;

                bounty_board.add_bounty(
                    BountyType::Exploration,
                    bounty_cost,
                    world_pos,
                    None,
                    1,
                );

                alerts.push(format!("Bounty placed at ({:.0}, {:.0}) for {:.0} gold!", world_pos.x, world_pos.y, bounty_cost));
            } else {
                alerts.push("Not enough gold for bounty!".to_string());
            }
        }
    }
}
