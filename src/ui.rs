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

                // Day/Night arc indicator (visual sun/moon progression)
                top_bar.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(60.0), Val::Px(20.0)),
                        position_type: PositionType::Relative,
                        margin: Rect {
                            left: Val::Px(8.0),
                            right: Val::Px(8.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    color: UiColor(Color::NONE),
                    ..Default::default()
                })
                .with_children(|arc_container| {
                    // Sun/Moon indicator (circle)
                    arc_container.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(12.0), Val::Px(12.0)),
                            position_type: PositionType::Absolute,
                            position: Rect {
                                left: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        color: UiColor(Color::rgb(1.0, 1.0, 0.0)), // initial sun color
                        ..Default::default()
                    })
                    .insert(DayNightArcIndicator);
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

                    // Legacy button (open Legacy Upgrades screen)
                    btn_row.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(32.0), Val::Px(28.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: Rect { right: Val::Px(4.0), ..Default::default() },
                            ..Default::default()
                        },
                        color: UiColor(Color::rgba(0.6, 0.4, 0.8, 0.8)),
                        ..Default::default()
                    })
                    .insert(LegacyButton)
                    .with_children(|btn| {
                        btn.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "L",
                                TextStyle { font: font.clone(), font_size: 16.0, color: Color::rgb(0.9, 0.7, 1.0) },
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
                // Era completion score screen (initially hidden)
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(300.0), Val::Auto),
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(50.0),
                            top: Val::Px(100.0),
                            ..Default::default()
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: Rect::all(Val::Px(16.0)),
                        ..Default::default()
                    },
                    color: UiColor(Color::rgba(0.1, 0.1, 0.1, 0.95)),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(EraScoreScreen)
                .with_children(|panel| {
                    // Title
                    panel.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "ERA COMPLETE",
                            TextStyle { font: font.clone(), font_size: 28.0, color: Color::rgb(0.9, 0.8, 0.2) },
                            TextAlignment::default(),
                        ),
                        style: Style { margin: Rect { top: Val::Px(0.0), bottom: Val::Px(12.0), left: Val::Px(0.0), right: Val::Px(0.0) }, ..Default::default() },
                        ..Default::default()
                    });

                    // Legacy points earned
                    panel.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Legacy Points: +0",
                            TextStyle { font: font.clone(), font_size: 20.0, color: Color::WHITE },
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    })
                    .insert(EraScoreLegacyText);

                    // Stats: Gold, Heroes, Buildings
                    panel.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Gold remaining: 0\nHeroes alive: 0\nBuildings standing: 0",
                            TextStyle { font: font.clone(), font_size: 18.0, color: Color::rgb(0.8, 0.8, 0.8) },
                            TextAlignment::default(),
                        ),
                        style: Style { margin: Rect { top: Val::Px(8.0), bottom: Val::Px(12.0), left: Val::Px(0.0), right: Val::Px(0.0) }, ..Default::default() },
                        ..Default::default()
                    })
                    .insert(EraScoreStatsText);

                    // Continue button
                    panel.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(120.0), Val::Px(40.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: Rect { top: Val::Px(8.0), bottom: Val::Px(0.0), left: Val::Px(0.0), right: Val::Px(0.0) },
                            ..Default::default()
                        },
                        color: UiColor(Color::rgba(0.3, 0.5, 0.2, 0.8)),
                        ..Default::default()
                    })
                    .with_children(|btn| {
                        btn.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Continue",
                                TextStyle { font: font.clone(), font_size: 20.0, color: Color::WHITE },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        });
                    })
                    .insert(EraContinueButton);
                });

                // Legacy Upgrades screen (initially hidden)
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(300.0), Val::Auto),
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(400.0),
                            top: Val::Px(80.0),
                            ..Default::default()
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: Rect::all(Val::Px(16.0)),
                        ..Default::default()
                    },
                    color: UiColor(Color::rgba(0.1, 0.1, 0.1, 0.95)),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(LegacyUpgradeScreen)
                .with_children(|panel| {
                    // Title
                    panel.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Legacy Upgrades",
                            TextStyle { font: font.clone(), font_size: 28.0, color: Color::rgb(0.9, 0.8, 0.2) },
                            TextAlignment::default(),
                        ),
                        style: Style { margin: Rect { bottom: Val::Px(12.0), ..Default::default() }, ..Default::default() },
                        ..Default::default()
                    });

                    // Legacy Points display
                    panel.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Legacy Points: 0",
                            TextStyle { font: font.clone(), font_size: 20.0, color: Color::WHITE },
                            TextAlignment::default(),
                        ),
                        style: Style { margin: Rect { bottom: Val::Px(12.0), ..Default::default() }, ..Default::default() },
                        ..Default::default()
                    })
                    .insert(LegacyPointsText);

                    // Row: Tax Bonus
                    panel.spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: Rect { top: Val::Px(6.0), bottom: Val::Px(6.0), ..Default::default() },
                            ..Default::default()
                        },
                        color: UiColor(Color::NONE),
                        ..Default::default()
                    })
                    .insert(TaxUpgradeRow)
                    .with_children(|row| {
                        row.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Tax Bonus: +0%",
                                TextStyle { font: font.clone(), font_size: 18.0, color: Color::rgb(0.8, 0.8, 0.8) },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(TaxUpgradeLabel);
                        row.spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            color: UiColor(Color::rgba(0.2, 0.6, 0.2, 0.8)),
                            ..Default::default()
                        })
                        .insert(TaxUpgradeButton)
                        .with_children(|btn| {
                            btn.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "+",
                                    TextStyle { font: font.clone(), font_size: 16.0, color: Color::WHITE },
                                    TextAlignment::default(),
                                ),
                                ..Default::default()
                            });
                        });
                    });

                    // Row: Hero Start Level
                    panel.spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: Rect { top: Val::Px(6.0), bottom: Val::Px(6.0), ..Default::default() },
                            ..Default::default()
                        },
                        color: UiColor(Color::NONE),
                        ..Default::default()
                    })
                    .insert(HeroStartUpgradeRow)
                    .with_children(|row| {
                        row.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Hero Start Level: 1",
                                TextStyle { font: font.clone(), font_size: 18.0, color: Color::rgb(0.8, 0.8, 0.8) },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(HeroStartUpgradeLabel);
                        row.spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            color: UiColor(Color::rgba(0.2, 0.6, 0.2, 0.8)),
                            ..Default::default()
                        })
                        .insert(HeroStartUpgradeButton)
                        .with_children(|btn| {
                            btn.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "+",
                                    TextStyle { font: font.clone(), font_size: 16.0, color: Color::WHITE },
                                    TextAlignment::default(),
                                ),
                                ..Default::default()
                            });
                        });
                    });

                    // Row: Building HP Bonus
                    panel.spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: Rect { top: Val::Px(6.0), bottom: Val::Px(6.0), ..Default::default() },
                            ..Default::default()
                        },
                        color: UiColor(Color::NONE),
                        ..Default::default()
                    })
                    .insert(BuildingHpUpgradeRow)
                    .with_children(|row| {
                        row.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Building HP Bonus: +0%",
                                TextStyle { font: font.clone(), font_size: 18.0, color: Color::rgb(0.8, 0.8, 0.8) },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(BuildingHpUpgradeLabel);
                        row.spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            color: UiColor(Color::rgba(0.2, 0.6, 0.2, 0.8)),
                            ..Default::default()
                        })
                        .insert(BuildingHpUpgradeButton)
                        .with_children(|btn| {
                            btn.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "+",
                                    TextStyle { font: font.clone(), font_size: 16.0, color: Color::WHITE },
                                    TextAlignment::default(),
                                ),
                                ..Default::default()
                            });
                        });
                    });

                    // Row: Bounty Cost Reduction
                    panel.spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: Rect { top: Val::Px(6.0), bottom: Val::Px(6.0), ..Default::default() },
                            ..Default::default()
                        },
                        color: UiColor(Color::NONE),
                        ..Default::default()
                    })
                    .insert(BountyCostUpgradeRow)
                    .with_children(|row| {
                        row.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Bounty Cost: 0% off",
                                TextStyle { font: font.clone(), font_size: 18.0, color: Color::rgb(0.8, 0.8, 0.8) },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(BountyCostUpgradeLabel);
                        row.spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            color: UiColor(Color::rgba(0.2, 0.6, 0.2, 0.8)),
                            ..Default::default()
                        })
                        .insert(BountyCostUpgradeButton)
                        .with_children(|btn| {
                            btn.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "+",
                                    TextStyle { font: font.clone(), font_size: 16.0, color: Color::WHITE },
                                    TextAlignment::default(),
                                ),
                                ..Default::default()
                            });
                        });
                    });

                    // Back button
                    panel.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(120.0), Val::Px(40.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: Rect { top: Val::Px(12.0), bottom: Val::Px(0.0), ..Default::default() },
                            ..Default::default()
                        },
                        color: UiColor(Color::rgba(0.3, 0.5, 0.2, 0.8)),
                        ..Default::default()
                    })
                    .with_children(|btn| {
                        btn.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Back",
                                TextStyle { font: font.clone(), font_size: 18.0, color: Color::WHITE },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        });
                    })
                    .insert(LegacyBackButton); // use separate marker for in-game legacy screen (will differentiate by context)
                });

                // Floating bounty board button (bottom-right)
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(48.0), Val::Px(48.0)),
                        position_type: PositionType::Absolute,
                        position: Rect {
                            right: Val::Px(10.0),
                            bottom: Val::Px(50.0), // above bottom bar
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    color: UiColor(Color::rgba(0.5, 0.4, 0.1, 0.8)),
                    ..Default::default()
                })
                .insert(BountyButton);

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

/// System: Update day/night arc indicator position and icon
pub fn update_day_night_arc_system(
    game_time: Res<GameTime>,
    mut query: Query<(&mut Style, &mut UiColor), With<DayNightArcIndicator>>,
) {
    const ARC_CONTAINER_WIDTH: f32 = 60.0;
    const INDICATOR_SIZE: f32 = 12.0;
    let max_left = ARC_CONTAINER_WIDTH - INDICATOR_SIZE;

    for (mut style, mut ui_color) in query.iter_mut() {
        // Ensure absolute positioning
        style.position_type = PositionType::Absolute;
        // Update left position based on day_progress (0..1) - moves left to right
        let left = game_time.day_progress * max_left;
        style.position.left = Val::Px(left);

        // Color based on time of day (sun by day, moon by night)
        let color = match game_time.time_of_day {
            TimeOfDay::Dawn => Color::rgb(1.0, 0.6, 0.2), // orange dawn sun
            TimeOfDay::Day => Color::rgb(1.0, 1.0, 0.0), // yellow sun
            TimeOfDay::Dusk => Color::rgb(1.0, 0.4, 0.0), // red-orange dusk sun
            TimeOfDay::Night => Color::rgb(0.8, 0.8, 1.0), // light blue moon
        };
        *ui_color = UiColor(color);
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
            let required = bounty.required_heroes.max(1);
            let assigned = bounty.assigned_heroes.len();
            let status = if assigned == 0 {
                "Open"
            } else if assigned >= required as usize {
                ">> Active"
            } else {
                "Part" // partially filled
            };
            let danger_stars: String = (0..bounty.danger_level).map(|_| '*').collect();
            // Squad size info for squad bounties
            let squad_info = if required > 1 {
                format!("  Squad:{}/{}", assigned, required)
            } else {
                String::new()
            };
            info.push_str(&format!(
                " {} {} - {:.0}g{}\n   Risk:{} | {}\n",
                type_icon, type_name, bounty.gold_reward, squad_info, danger_stars, status
            ));
        }
    }

    let available_count = active.iter().filter(|b| b.assigned_heroes.len() < b.required_heroes.max(1) as usize).count();
    let in_progress_count = active.len() - available_count;
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
                BuildingType::Bridge => {
                    info.push_str("Bridge: Cross rivers\n");
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
    building_bonuses: Res<BuildingBonuses>,
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
                "Bounty Board open - Up/Down amount ({:.0}g) - Left/Right squad ({}) - Click to place",
                game_phase.manual_bounty_amount, game_phase.manual_bounty_squad_size
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

    // Adjust squad size with Left/Right arrows when bounty board is open
    if game_phase.bounty_board_open {
        let max_allowed = building_bonuses.max_squad_size.max(1);
        if keyboard.just_pressed(KeyCode::Left) {
            game_phase.manual_bounty_squad_size = (game_phase.manual_bounty_squad_size - 1).max(1);
            alerts.push(format!("Squad size: {}", game_phase.manual_bounty_squad_size));
        }
        if keyboard.just_pressed(KeyCode::Right) {
            game_phase.manual_bounty_squad_size = (game_phase.manual_bounty_squad_size + 1).min(max_allowed);
            alerts.push(format!("Squad size: {}", game_phase.manual_bounty_squad_size));
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
    building_bonuses: Res<BuildingBonuses>,
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
                // Determine required heroes for this manual bounty, respecting Barracks squad cap
                let max_allowed = building_bonuses.max_squad_size.max(1);
                let required_heroes = game_phase.manual_bounty_squad_size.min(max_allowed);
                bounty_board.add_bounty(
                    BountyType::Exploration,
                    bounty_cost,
                    world_pos,
                    None,
                    danger,
                    required_heroes,
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

// ============================================================
// LEGACY UPGRADE UI SYSTEMS
// ============================================================

/// System: Toggle the Legacy Upgrades screen visibility when the Legacy button is clicked
pub fn legacy_button_system(
    mut interaction_query: Query<&Interaction, (With<LegacyButton>, Changed<Interaction>)>,
    mut screen_query: Query<&mut Visibility, With<LegacyUpgradeScreen>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            for mut vis in screen_query.iter_mut() {
                vis.is_visible = !vis.is_visible;
            }
        }
    }
}

/// System: Close the Legacy Upgrades screen when its Back button is clicked
pub fn legacy_back_button_system(
    mut interaction_query: Query<&Interaction, (With<LegacyBackButton>, Changed<Interaction>)>,
    mut screen_query: Query<&mut Visibility, With<LegacyUpgradeScreen>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            for mut vis in screen_query.iter_mut() {
                vis.is_visible = false;
            }
        }
    }
}

/// System: Update the Legacy Upgrades UI (points text, upgrade labels) and handle spending clicks
pub fn update_legacy_upgrades_ui_system(
    kingdom: Res<KingdomState>,
    legacy_upgrades: Res<LegacyUpgrades>,
    mut points_query: Query<&mut Text, (With<LegacyPointsText>, Without<TaxUpgradeLabel>, Without<HeroStartUpgradeLabel>, Without<BuildingHpUpgradeLabel>, Without<BountyCostUpgradeLabel>)>,
    mut tax_label: Query<&mut Text, (With<TaxUpgradeLabel>, Without<LegacyPointsText>, Without<HeroStartUpgradeLabel>, Without<BuildingHpUpgradeLabel>, Without<BountyCostUpgradeLabel>)>,
    mut hero_start_label: Query<&mut Text, (With<HeroStartUpgradeLabel>, Without<LegacyPointsText>, Without<TaxUpgradeLabel>, Without<BuildingHpUpgradeLabel>, Without<BountyCostUpgradeLabel>)>,
    mut building_hp_label: Query<&mut Text, (With<BuildingHpUpgradeLabel>, Without<LegacyPointsText>, Without<TaxUpgradeLabel>, Without<HeroStartUpgradeLabel>, Without<BountyCostUpgradeLabel>)>,
    mut bounty_cost_label: Query<&mut Text, (With<BountyCostUpgradeLabel>, Without<LegacyPointsText>, Without<TaxUpgradeLabel>, Without<HeroStartUpgradeLabel>, Without<BuildingHpUpgradeLabel>)>,
    tax_clicks: Query<&Interaction, (With<TaxUpgradeButton>, Changed<Interaction>)>,
    hero_start_clicks: Query<&Interaction, (With<HeroStartUpgradeButton>, Changed<Interaction>)>,
    building_hp_clicks: Query<&Interaction, (With<BuildingHpUpgradeButton>, Changed<Interaction>)>,
    bounty_cost_clicks: Query<&Interaction, (With<BountyCostUpgradeButton>, Changed<Interaction>)>,
    mut kingdom_res: ResMut<KingdomState>,
    mut legacy_res: ResMut<LegacyUpgrades>,
) {
    // Update points display
    for mut text in points_query.iter_mut() {
        text.sections[0].value = format!("Legacy Points: {}", kingdom.legacy_points);
    }

    // Constants
    const MAX_LEVEL_TAX: u32 = 10;
    const MAX_LEVEL_BUILDING_HP: u32 = 10;
    const MAX_LEVEL_BOUNTY_COST: u32 = 10;
    const MAX_LEVEL_HERO_START: u32 = 5;
    const COST_TAX: u32 = 5;
    const COST_BUILDING_HP: u32 = 5;
    const COST_BOUNTY_COST: u32 = 5;
    const COST_HERO_START: u32 = 10;

    // Tax Bonus
    let tax_level = (legacy_upgrades.tax_bonus_pct / 5.0).round() as u32;
    for mut text in tax_label.iter_mut() {
        if tax_level >= MAX_LEVEL_TAX {
            text.sections[0].value = format!("Tax Bonus: +{}% (MAX)", legacy_upgrades.tax_bonus_pct as u32);
        } else {
            text.sections[0].value = format!("Tax Bonus: +{}% (+5% for {} LP)", legacy_upgrades.tax_bonus_pct as u32, COST_TAX);
        }
    }
    if !tax_clicks.is_empty() && kingdom.legacy_points >= COST_TAX && tax_level < MAX_LEVEL_TAX {
        kingdom_res.legacy_points -= COST_TAX;
        legacy_res.tax_bonus_pct += 5.0;
    }

    // Hero Start Level
    let hero_level = legacy_upgrades.hero_start_level;
    for mut text in hero_start_label.iter_mut() {
        if hero_level >= MAX_LEVEL_HERO_START {
            text.sections[0].value = format!("Hero Start Level: {} (MAX)", hero_level);
        } else {
            let cost = COST_HERO_START * hero_level;
            text.sections[0].value = format!("Hero Start Level: {} (Next: {} LP)", hero_level, cost);
        }
    }
    for _ in hero_start_clicks.iter() {
        let cost = COST_HERO_START * hero_level;
        if kingdom.legacy_points >= cost && hero_level < MAX_LEVEL_HERO_START {
            kingdom_res.legacy_points -= cost;
            legacy_res.hero_start_level += 1;
        }
    }

    // Building HP Bonus
    let bh_level = (legacy_upgrades.building_hp_bonus_pct / 5.0).round() as u32;
    for mut text in building_hp_label.iter_mut() {
        if bh_level >= MAX_LEVEL_BUILDING_HP {
            text.sections[0].value = format!("Building HP Bonus: +{}% (MAX)", legacy_upgrades.building_hp_bonus_pct as u32);
        } else {
            text.sections[0].value = format!("Building HP Bonus: +{}% (+5% for {} LP)", legacy_upgrades.building_hp_bonus_pct as u32, COST_BUILDING_HP);
        }
    }
    for _ in building_hp_clicks.iter() {
        if kingdom.legacy_points >= COST_BUILDING_HP && bh_level < MAX_LEVEL_BUILDING_HP {
            kingdom_res.legacy_points -= COST_BUILDING_HP;
            legacy_res.building_hp_bonus_pct += 5.0;
        }
    }

    // Bounty Cost Reduction
    let bc_level = (legacy_upgrades.bounty_cost_reduction / 2.0).round() as u32;
    for mut text in bounty_cost_label.iter_mut() {
        if bc_level >= MAX_LEVEL_BOUNTY_COST {
            text.sections[0].value = format!("Bounty Cost: {}% off (MAX)", legacy_upgrades.bounty_cost_reduction as u32);
        } else {
            text.sections[0].value = format!("Bounty Cost: {}% off (+2% for {} LP)", legacy_upgrades.bounty_cost_reduction as u32, COST_BOUNTY_COST);
        }
    }
    for _ in bounty_cost_clicks.iter() {
        if kingdom.legacy_points >= COST_BOUNTY_COST && bc_level < MAX_LEVEL_BOUNTY_COST {
            kingdom_res.legacy_points -= COST_BOUNTY_COST;
            legacy_res.bounty_cost_reduction += 2.0;
        }
    }
}
