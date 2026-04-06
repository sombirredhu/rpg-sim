use bevy::prelude::*;
use crate::components::*;
use crate::sprites::SpriteAssets;
use crate::camera::cursor_to_world_2d;

/// Startup: Create the HUD UI
pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprites: Res<SpriteAssets>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.insert_resource(UiFont(font.clone()));

    // UI Camera
    commands.spawn_bundle(UiCameraBundle::default());

    // Root UI node — the in-game HUD
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(GameUiRoot)
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
                // Gold display (icon + text row)
                top_bar.spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: UiColor(Color::NONE),
                    ..Default::default()
                })
                .with_children(|gold_row| {
                    gold_row.spawn_bundle(ImageBundle {
                        image: UiImage(sprites.icon_gold_coin.clone()),
                        style: Style {
                            size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                            margin: Rect {
                                right: Val::Px(4.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    gold_row.spawn_bundle(TextBundle {
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
                });

                // Day/Night clock (icon + text)
                top_bar.spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: UiColor(Color::NONE),
                    ..Default::default()
                })
                .with_children(|clock_row| {
                    clock_row.spawn_bundle(ImageBundle {
                        image: UiImage(sprites.icon_clock.clone()),
                        style: Style {
                            size: Size::new(Val::Px(20.0), Val::Px(20.0)),
                            margin: Rect {
                                right: Val::Px(4.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    clock_row.spawn_bundle(TextBundle {
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
                });

                // Kingdom rank (medal icon + text)
                top_bar.spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: UiColor(Color::NONE),
                    ..Default::default()
                })
                .with_children(|rank_row| {
                    rank_row.spawn_bundle(ImageBundle {
                        image: UiImage(sprites.icon_medal.clone()),
                        style: Style {
                            size: Size::new(Val::Px(20.0), Val::Px(20.0)),
                            margin: Rect {
                                right: Val::Px(4.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    rank_row.spawn_bundle(TextBundle {
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
                });

                // Speed + Action buttons row
                top_bar.spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: UiColor(Color::NONE),
                    ..Default::default()
                })
                .with_children(|btn_row| {
                    // Speed button (clickable)
                    btn_row.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(40.0), Val::Px(28.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: Rect { right: Val::Px(4.0), ..Default::default() },
                            ..Default::default()
                        },
                        color: UiColor(Color::rgba(0.3, 0.3, 0.3, 0.8)),
                        ..Default::default()
                    })
                    .insert(SpeedButton)
                    .with_children(|btn| {
                        btn.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "1x",
                                TextStyle { font: font.clone(), font_size: 16.0, color: Color::rgb(0.8, 0.8, 0.8) },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(SpeedText);
                    });

                    // Pause button (clickable)
                    btn_row.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(32.0), Val::Px(28.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: Rect { right: Val::Px(4.0), ..Default::default() },
                            ..Default::default()
                        },
                        color: UiColor(Color::rgba(0.3, 0.3, 0.3, 0.8)),
                        ..Default::default()
                    })
                    .insert(PauseButton)
                    .with_children(|btn| {
                        btn.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "||",
                                TextStyle { font: font.clone(), font_size: 16.0, color: Color::rgb(0.8, 0.8, 0.8) },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        });
                    });

                    // Build button
                    btn_row.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(32.0), Val::Px(28.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: Rect { right: Val::Px(4.0), ..Default::default() },
                            ..Default::default()
                        },
                        color: UiColor(Color::rgba(0.2, 0.5, 0.2, 0.8)),
                        ..Default::default()
                    })
                    .insert(BuildButton)
                    .with_children(|btn| {
                        btn.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "B",
                                TextStyle { font: font.clone(), font_size: 16.0, color: Color::rgb(0.6, 1.0, 0.6) },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        });
                    });

                    // Bounty board button
                    btn_row.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(32.0), Val::Px(28.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: Rect { right: Val::Px(4.0), ..Default::default() },
                            ..Default::default()
                        },
                        color: UiColor(Color::rgba(0.5, 0.4, 0.1, 0.8)),
                        ..Default::default()
                    })
                    .insert(BountyButton)
                    .with_children(|btn| {
                        btn.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Q",
                                TextStyle { font: font.clone(), font_size: 16.0, color: Color::rgb(1.0, 0.85, 0.4) },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        });
                    });

                    // Expand button
                    btn_row.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(32.0), Val::Px(28.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: Rect { right: Val::Px(4.0), ..Default::default() },
                            ..Default::default()
                        },
                        color: UiColor(Color::rgba(0.2, 0.2, 0.5, 0.8)),
                        ..Default::default()
                    })
                    .insert(ExpandButton)
                    .with_children(|btn| {
                        btn.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "E",
                                TextStyle { font: font.clone(), font_size: 16.0, color: Color::rgb(0.6, 0.6, 1.0) },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        });
                    });

                    // Road tool button
                    btn_row.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(32.0), Val::Px(28.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        color: UiColor(Color::rgba(0.3, 0.2, 0.1, 0.8)),
                        ..Default::default()
                    })
                    .insert(RoadToolButton)
                    .with_children(|btn| {
                        btn.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "R",
                                TextStyle { font: font.clone(), font_size: 16.0, color: Color::rgb(1.0, 0.7, 0.4) },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        });
                    });

                    // Economy breakdown button
                    btn_row.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(32.0), Val::Px(28.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: Rect { right: Val::Px(4.0), ..Default::default() },
                            ..Default::default()
                        },
                        color: UiColor(Color::rgba(0.2, 0.6, 0.2, 0.8)),
                        ..Default::default()
                    })
                    .insert(EconomyButton)
                    .with_children(|btn| {
                        btn.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "$",
                                TextStyle { font: font.clone(), font_size: 16.0, color: Color::rgb(0.6, 1.0, 0.6) },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        });
                    });
                });
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
            .insert(HeroPanelUi)
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

            // ===== ECONOMY BREAKDOWN PANEL (hidden by default) =====
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Auto),
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
                color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.6)),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(EconomyBreakdownPanel)
            .with_children(|panel| {
                panel.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Income Breakdown",
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            color: Color::rgb(0.8, 0.9, 1.0),
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                });
                panel.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Tax: 0/min",
                        TextStyle {
                            font: font.clone(),
                            font_size: 14.0,
                            color: Color::WHITE,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(EconIncomeLine::Tax);
                panel.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Trade: 0/min",
                        TextStyle {
                            font: font.clone(),
                            font_size: 14.0,
                            color: Color::WHITE,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(EconIncomeLine::Trade);
                panel.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Bounties: 0/min",
                        TextStyle {
                            font: font.clone(),
                            font_size: 14.0,
                            color: Color::WHITE,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(EconIncomeLine::Bounty);
                panel.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Total: 0/min",
                        TextStyle {
                            font: font.clone(),
                            font_size: 15.0,
                            color: Color::rgb(1.0, 0.85, 0.0),
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(EconIncomeLine::Total);
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
                // Header row with scroll icon
                panel.spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: Rect {
                            bottom: Val::Px(4.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    color: UiColor(Color::NONE),
                    ..Default::default()
                })
                .with_children(|header| {
                    header.spawn_bundle(ImageBundle {
                        image: UiImage(sprites.icon_bounty_scroll.clone()),
                        style: Style {
                            size: Size::new(Val::Px(22.0), Val::Px(22.0)),
                            margin: Rect {
                                right: Val::Px(6.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    header.spawn_bundle(TextBundle {
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
                    });
                });

                // Bounty list text
                panel.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 13.0,
                            color: Color::rgb(0.9, 0.9, 0.85),
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

            // ===== BUILDING MENU PANEL (left side, below bounty board) =====
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Auto),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(5.0),
                        top: Val::Px(320.0), // Below bounty board
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
            .insert(BuildingMenuUi);

            // ===== BUILDING INFO PANEL (right side, below hero panel) =====
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(220.0), Val::Auto),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        right: Val::Px(5.0),
                        top: Val::Px(320.0), // Below hero panel
                        ..Default::default()
                    },
                    padding: Rect::all(Val::Px(8.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.6)),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(BuildingInfoUi)
            .with_children(|panel| {
                panel.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 14.0,
                            color: Color::WHITE,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(BuildingInfoText);

                // Repair button (hidden by default)
                panel.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(180.0), Val::Px(28.0)),
                        margin: Rect { top: Val::Px(4.0), ..Default::default() },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: UiColor(Color::rgba(0.2, 0.5, 0.2, 0.8)),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(RepairButton)
                .with_children(|btn| {
                    btn.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Repair (0g)",
                            TextStyle {
                                font: font.clone(),
                                font_size: 14.0,
                                color: Color::rgb(0.8, 1.0, 0.8),
                            },
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    })
                    .insert(RepairButtonText);
                });
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
                        "LeftClick:Select | RightDrag:Pan | Scroll:Zoom | WASD:Move | TopBarButtons:Speed/Pause/Build/Bounty/Expand/Road",
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
    game_phase: Res<GamePhase>,
    heroes: Query<(&Hero, &HeroStats, &HeroState)>,
    mut text_query: Query<&mut Text, With<HeroPanelText>>,
    mut panel_visibility: Query<&mut Visibility, With<HeroPanelUi>>,
) {
    for mut vis in panel_visibility.iter_mut() {
        vis.is_visible = game_phase.hero_panel_open;
    }
    if !game_phase.hero_panel_open {
        return;
    }
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
    if class_counts[0] > 0 { info.push_str(&format!("  [Sword] Warriors: {}\n", class_counts[0])); }
    if class_counts[1] > 0 { info.push_str(&format!("  [Bow] Archers: {}\n", class_counts[1])); }
    if class_counts[2] > 0 { info.push_str(&format!("  [Staff] Mages: {}\n", class_counts[2])); }
    if class_counts[3] > 0 { info.push_str(&format!("  [Dagger] Rogues: {}\n", class_counts[3])); }
    if class_counts[4] > 0 { info.push_str(&format!("  [Heart] Healers: {}\n", class_counts[4])); }

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
            HeroState::Casting { .. } => "Casting",
        };
        let legendary_prefix = if hero.is_legendary { "[LEG] " } else { "" };
        info.push_str(&format!(
            "  {}{}Lv{} HP:{:.0}/{:.0} [{}]\n",
            legendary_prefix,
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
    mut panel_query: Query<&mut Visibility, (With<BountyBoardUi>, Without<BountyBoardText>)>,
    mut text_visibility_query: Query<&mut Visibility, (With<BountyBoardText>, Without<BountyBoardUi>)>,
    economy: Res<GameEconomy>,
    mut text_query: Query<&mut Text, With<BountyBoardText>>,
) {
    // Toggle panel visibility
    for mut vis in panel_query.iter_mut() {
        vis.is_visible = game_phase.bounty_board_open;
    }
    for mut vis in text_visibility_query.iter_mut() {
        vis.is_visible = game_phase.bounty_board_open;
    }

    if !game_phase.bounty_board_open {
        for mut text in text_query.iter_mut() {
            text.sections[0].value.clear();
        }
        return;
    }

    // Build bounty list text
    let mut info = String::from("=== BOUNTY BOARD ===\n\n");

    let active: Vec<_> = bounty_board.bounties.iter().filter(|b| !b.is_completed).collect();

    if active.is_empty() {
        info.push_str("  No active bounties.\n");
    } else {
        for bounty in &active {
            let (type_icon, type_name) = match bounty.bounty_type {
                BountyType::Monster => ("<<Sword>>", "Monster"),
                BountyType::Exploration => ("<<Map>>", "Explore"),
                BountyType::Objective => ("<<Shield>>", "Objective"),
                BountyType::Resource => ("<<Pick>>", "Resource"),
            };
            let status = if bounty.assigned_hero.is_some() {
                ">> Active"
            } else {
                "Open"
            };
            let danger_stars: String = (0..bounty.danger_level).map(|_| '*').collect();
            info.push_str(&format!(
                " {} {} - {:.0}g\n   Risk:{} | {}\n",
                type_icon, type_name, bounty.gold_reward, danger_stars, status
            ));
        }
    }

    let available_count = active.iter().filter(|b| b.assigned_hero.is_none()).count();
    let in_progress_count = active.iter().filter(|b| b.assigned_hero.is_some()).count();
    info.push_str(&format!(
        "\nTotal: {} | Avail: {} | Active: {}\n",
        active.len(), available_count, in_progress_count
    ));

    // ROI display: show lifetime bounty stats
    let completed = bounty_board.total_bounties_completed;
    let paid = bounty_board.total_bounty_gold_paid;
    let returned = bounty_board.total_bounty_tax_returned;
    let net_cost = paid - returned;
    if completed > 0 {
        let avg_cost = net_cost / completed as f32;
        info.push_str(&format!(
            "\n--- BOUNTY ROI ---\n Completed: {} | Paid: {:.0}g\n Tax back: +{:.0}g | Net: {:.0}g\n Avg/bounty: {:.0}g\n",
            completed, paid, returned, net_cost, avg_cost
        ));
    } else {
        info.push_str("\n--- BOUNTY ROI ---\n No bounties completed yet.\n");
    }

    // Show adjustable bounty amount with affordability
    let amount = game_phase.manual_bounty_amount;
    let can_afford = if economy.gold >= amount { "OK" } else { "!!" };
    info.push_str(&format!(
        "\n--- Place Bounty: {:.0}g [{}] ---\n",
        amount, can_afford
    ));
    info.push_str("+/-10g: Up/Down | +/-50g: Shift+Up/Down\n");
    info.push_str("Click map to place");

    for mut text in text_query.iter_mut() {
        text.sections[0].value = info.clone();
    }
}

/// System: Update building menu panel visibility and spawn button list
pub fn update_building_menu_ui(
    game_phase: Res<GamePhase>,
    kingdom: Res<KingdomState>,
    ui_font: Res<UiFont>,
    mut commands: Commands,
    mut panel_query: Query<(&mut Visibility, Entity), With<BuildingMenuUi>>,
    children_query: Query<&Children>,
) {
    for (mut vis, panel_entity) in panel_query.iter_mut() {
        vis.is_visible = game_phase.show_build_menu;

        // If menu is visible and no buttons spawned yet, create them
        if game_phase.show_build_menu {
            // Check if buttons already exist (children count > 0)
            let has_children = children_query
                .get(panel_entity)
                .map(|c| !c.is_empty())
                .unwrap_or(false);
            if !has_children {
                // Spawn a button for each available building
                let available = kingdom.rank.available_buildings();
                for building_type in available.iter() {
                    let button_entity = commands
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(180.0), Val::Px(28.0)),
                                margin: Rect { top: Val::Px(4.0), ..Default::default() },
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: UiColor(Color::rgba(0.2, 0.5, 0.2, 0.8)),
                            ..Default::default()
                        })
                        .insert(BuildingMenuItem(building_type.clone()))
                        .with_children(|btn| {
                            btn.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    format!("{} ({:.0}g)", building_type.display_name(), building_type.cost()),
                                    TextStyle {
                                        font: ui_font.0.clone(),
                                        font_size: 14.0,
                                        color: Color::rgb(0.9, 0.9, 0.85),
                                    },
                                    TextAlignment::default(),
                                ),
                                ..Default::default()
                            });
                        })
                        .id();

                    commands.entity(panel_entity).add_child(button_entity);
                }
            }
        } else {
            // Despawn children to clean up; next open will rebuild fresh
            if let Ok(children) = children_query.get(panel_entity) {
                for &child in children.iter() {
                    commands.entity(child).despawn();
                }
            }
        }
    }
}

/// System: Handle clicks on building menu buttons
pub fn building_menu_button_system(
    mut game_phase: ResMut<GamePhase>,
    mut interaction_query: Query<
        (Entity, &Interaction, &BuildingMenuItem),
        (Changed<Interaction>, With<Button>),
    >,
    mut alerts: ResMut<GameAlerts>,
) {
    for (_entity, interaction, menu_item) in interaction_query.iter_mut() {
        if *interaction == Interaction::Clicked {
            let building_type = menu_item.0;
            game_phase.selected_building = Some(building_type);
            game_phase.build_mode = true;
            game_phase.show_build_menu = false;
            game_phase.bounty_board_open = false;
            alerts.push(format!(
                "Selected {} - Click to build, Right-click to cancel",
                building_type.display_name()
            ));
        }
    }
}

/// System: Update building info panel content (visibility handled by mouse system)
pub fn update_building_info_ui(
    mut selected_building_info: ResMut<SelectedBuildingInfo>,
    buildings: Query<(&Building, &Transform)>,
    mut text_query: Query<&mut Text, With<BuildingInfoText>>,
    mut building_info_ui: Query<&mut Visibility, With<BuildingInfoUi>>,
) {
    // Always update text for the selected building (if any)
    if let Some(entity) = selected_building_info.entity {
        if let Ok((building, _transform)) = buildings.get(entity) {
            let mut info = String::new();
            info.push_str(&format!("{} Info\n", building.building_type.display_name()));
            info.push_str(&format!("Tier: {}\n", building.tier));
            info.push_str(&format!("HP: {:.0}/{:.0}\n", building.hp, building.max_hp));
            if building.is_destroyed {
                info.push_str("Status: DESTROYED\n");
            }
            info.push_str(&format!("Tax Income: {:.1}/min\n", building.building_type.tax_income(building.tier)));

            // Add specific info based on building type
            match building.building_type {
                BuildingType::GuardTower => {
                    let range = 150.0 + building.tier as f32 * 50.0;
                    let damage = 15.0 + building.tier as f32 * 10.0;
                    info.push_str(&format!("Attack Range: {:.0}\n", range));
                    info.push_str(&format!("Attack Damage: {:.0}\n", damage));
                }
                BuildingType::WizardTower => {
                    info.push_str(&format!("Spell Power: {:.0}\n", 20.0 + building.tier as f32 * 15.0));
                }
                BuildingType::Blacksmith => {
                    info.push_str(&format!("Weapon Quality: {}\n", ["Basic", "Iron", "Steel", "Magic"][building.tier as usize]));
                    info.push_str(&format!("Armor Quality: {}\n", ["Basic", "Leather", "Chain", "Plate"][building.tier as usize]));
                }
                BuildingType::Alchemist => {
                    info.push_str(&format!("Potion Variety: {}\n", ["Basic", "Common", "Uncommon", "Rare"][building.tier as usize]));
                }
                BuildingType::Barracks => {
                    info.push_str(&format!("Training Speed: {}\n", ["Slow", "Normal", "Fast", "Elite"][building.tier as usize]));
                    info.push_str(&format!("Unit Capacity: {}\n", [5, 10, 15, 20][building.tier as usize]));
                }
                BuildingType::TownHall => {
                    info.push_str(&format!("Kingdom Control Center\n"));
                }
                BuildingType::Inn => {
                    info.push_str(&format!("Hero Rest & Recovery\n"));
                    info.push_str(&format!("Attracts: Warriors, Rogues\n"));
                }
                BuildingType::Market => {
                    info.push_str(&format!("Trade & Commerce\n"));
                    info.push_str(&format!("Attracts: Rogues\n"));
                }
                BuildingType::Temple => {
                    info.push_str(&format!("Healing & Blessings\n"));
                    info.push_str(&format!("Attracts: Healers\n"));
                }
            }

            for mut text in text_query.iter_mut() {
                text.sections[0].value = info.clone();
            }
        } else {
            // Building entity no longer valid - clear selection and hide panel
            selected_building_info.entity = None;
            for mut vis in building_info_ui.iter_mut() {
                vis.is_visible = false;
            }
            for mut text in text_query.iter_mut() {
                text.sections[0].value.clear();
            }
        }
    } else {
        // No selection - ensure panel is hidden
        for mut vis in building_info_ui.iter_mut() {
            vis.is_visible = false;
        }
    }
}

/// System: Update repair button visibility and text based on selected building
pub fn update_repair_button_ui(
    selected_building_info: Res<SelectedBuildingInfo>,
    buildings: Query<&Building>,
    mut repair_btn_vis: Query<&mut Visibility, With<RepairButton>>,
    mut repair_btn_text: Query<&mut Text, With<RepairButtonText>>,
) {
    // By default hide the button
    for mut vis in repair_btn_vis.iter_mut() {
        vis.is_visible = false;
    }

    if let Some(entity) = selected_building_info.entity {
        if let Ok(building) = buildings.get(entity) {
            if building.is_destroyed {
                let repair_cost = building.building_type.cost() * 0.5;
                let label = format!("Repair ({:.0}g)", repair_cost);
                for mut vis in repair_btn_vis.iter_mut() {
                    vis.is_visible = true;
                }
                for mut text in repair_btn_text.iter_mut() {
                    text.sections[0].value = label.clone();
                }
            }
        }
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
            if !game_phase.show_build_menu {
                game_phase.bounty_board_open = false;
            }
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
            (KeyCode::Key1, 0), (KeyCode::Numpad1, 0),
            (KeyCode::Key2, 1), (KeyCode::Numpad2, 1),
            (KeyCode::Key3, 2), (KeyCode::Numpad3, 2),
            (KeyCode::Key4, 3), (KeyCode::Numpad4, 3),
            (KeyCode::Key5, 4), (KeyCode::Numpad5, 4),
            (KeyCode::Key6, 5), (KeyCode::Numpad6, 5),
            (KeyCode::Key7, 6), (KeyCode::Numpad7, 6),
            (KeyCode::Key8, 7), (KeyCode::Numpad8, 7),
            (KeyCode::Key9, 8), (KeyCode::Numpad9, 8),
        ];

        for (key, idx) in key_map {
            if keyboard.just_pressed(key) {
                if let Some(building_type) = available.get(idx) {
                    game_phase.selected_building = Some(*building_type);
                    game_phase.build_mode = true;
                    game_phase.show_build_menu = false;
                    game_phase.bounty_board_open = false;
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
            game_phase.build_mode = false;
            game_phase.show_build_menu = false;
            game_phase.selected_building = None;
            alerts.push(format!(
                "Bounty Board open - Up/Down to adjust amount ({:.0}g) - Click to place",
                game_phase.manual_bounty_amount
            ));
        } else {
            alerts.push("Bounty Board closed".to_string());
        }
    }

    // Adjust bounty amount with Up/Down arrows when bounty board is open
    if game_phase.bounty_board_open {
        let step = if keyboard.pressed(KeyCode::LShift) || keyboard.pressed(KeyCode::RShift) {
            50.0
        } else {
            10.0
        };

        if keyboard.just_pressed(KeyCode::Up) {
            game_phase.manual_bounty_amount = (game_phase.manual_bounty_amount + step).min(500.0);
            alerts.push(format!("Bounty amount: {:.0}g", game_phase.manual_bounty_amount));
        }
        if keyboard.just_pressed(KeyCode::Down) {
            game_phase.manual_bounty_amount = (game_phase.manual_bounty_amount - step).max(10.0);
            alerts.push(format!("Bounty amount: {:.0}g", game_phase.manual_bounty_amount));
        }
    }

    // Toggle hero panel with H
    if keyboard.just_pressed(KeyCode::H) && game_phase.game_started {
        game_phase.hero_panel_open = !game_phase.hero_panel_open;
        alerts.push(if game_phase.hero_panel_open {
            "Hero panel OPEN".to_string()
        } else {
            "Hero panel CLOSED".to_string()
        });
    }
}

/// System: Manual bounty placement
pub fn manual_bounty_system(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &Transform, &OrthographicProjection), With<MainCamera>>,
    game_phase: Res<GamePhase>,
    mut economy: ResMut<GameEconomy>,
    mut bounty_board: ResMut<BountyBoard>,
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
        if let Ok((_camera, camera_transform, projection)) = camera.get_single() {
            let world_pos = match cursor_to_world_2d(window, camera_transform, projection) {
                Some(pos) => pos,
                None => return,
            };

            let bounty_cost = game_phase.manual_bounty_amount;
            if economy.gold >= bounty_cost {
                economy.gold -= bounty_cost;
                economy.total_spent += bounty_cost;

                // Scale danger estimate based on bounty amount
                let danger = ((bounty_cost / 50.0) as u32).clamp(1, 5);
                bounty_board.add_bounty(
                    BountyType::Exploration,
                    bounty_cost,
                    world_pos,
                    None,
                    danger,
                );

                alerts.push(format!("Bounty placed at ({:.0}, {:.0}) for {:.0} gold!", world_pos.x, world_pos.y, bounty_cost));
            } else {
                alerts.push("Not enough gold for bounty!".to_string());
            }
        }
    }
}

/// System: Toggle economy breakdown panel visibility on button click
pub fn economy_button_click_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<EconomyButton>)>,
    mut panel_query: Query<&mut Visibility, With<EconomyBreakdownPanel>>,
) {
    for interaction in interaction_query.iter() {
        if let Interaction::Clicked = interaction {
            for mut visibility in panel_query.iter_mut() {
                visibility.is_visible = !visibility.is_visible;
            }
        }
    }
}

/// System: Update economy breakdown panel text with current income rates
pub fn update_economy_breakdown_ui(
    economy: Res<GameEconomy>,
    mut query: Query<(&mut Text, &EconIncomeLine)>,
) {
    for (mut text, line) in query.iter_mut() {
        let s = match line {
            EconIncomeLine::Tax => format!("Tax: {:.1}/min", economy.property_tax_income_per_minute),
            EconIncomeLine::Trade => format!("Trade: {:.1}/min", economy.merchant_trade_income_per_minute),
            EconIncomeLine::Bounty => format!("Bounties: {:.1}/min", economy.bounty_tax_income_per_minute),
            EconIncomeLine::Total => format!("Total: {:.1}/min", economy.income_per_minute),
        };
        text.sections[0].value = s;
    }
}

/// System: Handle manual building repair button click
pub fn repair_button_click_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RepairButton>)>,
    selected_building_info: Res<SelectedBuildingInfo>,
    mut buildings: Query<(&mut Building, &mut Visibility)>,
    mut economy: ResMut<GameEconomy>,
    mut alerts: ResMut<GameAlerts>,
) {
    for interaction in interaction_query.iter() {
        if let Interaction::Clicked = interaction {
            if let Some(entity) = selected_building_info.entity {
                if let Ok((mut building, mut vis)) = buildings.get_mut(entity) {
                    if building.is_destroyed {
                        let repair_cost = building.building_type.cost() * 0.5;
                        if economy.gold >= repair_cost {
                            economy.gold -= repair_cost;
                            economy.total_spent += repair_cost;
                            building.is_destroyed = false;
                            building.hp = building.max_hp * 0.5;
                            vis.is_visible = true;
                            alerts.push(format!("Repaired {} for {:.0}g", building.building_type.display_name(), repair_cost));
                        } else {
                            alerts.push(format!("Not enough gold to repair {} (need {:.0}g)", building.building_type.display_name(), repair_cost));
                        }
                    }
                }
            }
        }
    }
}
