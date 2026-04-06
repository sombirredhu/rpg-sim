use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use crate::components::*;

// ============================================================
// MAIN MENU SCREEN
// ============================================================

#[derive(Default, Eq, PartialEq, Debug, Clone)]
pub enum GameMenuState {
    #[default]
    MainMenu,
    Playing,
    Settings,
}

/// Resource tracking the current menu state
#[derive(Default)]
pub struct MenuState {
    pub current: GameMenuState,
}

/// Marker components for main menu buttons
#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct SettingsMenuRoot;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct ResumeGameButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct BackButton;

#[derive(Component)]
pub struct SettingsVolumeText;

#[derive(Component)]
pub struct SfxVolumeControl;

#[derive(Component)]
pub struct MusicVolumeControl;

#[derive(Component)]
pub struct CameraSpeedControl;

#[derive(Component)]
pub struct SettingToggleVisual;

// ============================================================
// Startup: Create Main Menu UI
// ============================================================
pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: UiColor(Color::rgb(0.08, 0.11, 0.09)),
            ..Default::default()
        })
        .insert(MainMenuRoot)
        .with_children(|parent| {
            // Title
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Realm of Bounties",
                    TextStyle {
                        font: font.clone(),
                        font_size: 48.0,
                        color: Color::rgb(0.9, 0.8, 0.2),
                    },
                    TextAlignment::default(),
                ),
                style: Style {
                    margin: Rect {
                        bottom: Val::Px(12.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            });

            // Subtitle
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "A Kingdom Simulation",
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::rgb(0.5, 0.5, 0.45),
                    },
                    TextAlignment::default(),
                ),
                style: Style {
                    margin: Rect {
                        bottom: Val::Px(40.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            });

            // Button container
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: UiColor(Color::NONE),
                ..Default::default()
            })
            .with_children(|btn_container| {
                // Start New Game button
                spawn_menu_button(
                    btn_container,
                    "Start New Game",
                    font.clone(),
                    StartGameButton,
                );

                // Resume Game button
                spawn_menu_button(
                    btn_container,
                    "Resume Game",
                    font.clone(),
                    ResumeGameButton,
                );

                // Settings button
                spawn_menu_button(
                    btn_container,
                    "Settings",
                    font.clone(),
                    SettingsButton,
                );

                // Quit button
                spawn_menu_button(
                    btn_container,
                    "Quit",
                    font.clone(),
                    QuitButton,
                );
            });
        });

    // Settings menu (hidden by default)
    spawn_settings_menu(&mut commands, font);
}

// ============================================================
// Button helper
// ============================================================
fn spawn_menu_button(
    parent: &mut ChildBuilder,
    label: &str,
    font: Handle<Font>,
    marker: impl Component,
) {
    parent.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(240.0), Val::Px(48.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: Rect {
                top: Val::Px(8.0),
                bottom: Val::Px(8.0),
                ..Default::default()
            },
            ..Default::default()
        },
        color: UiColor(Color::rgba(0.2, 0.3, 0.2, 0.9)),
        ..Default::default()
    })
    .insert(marker)
    .with_children(|btn| {
        btn.spawn_bundle(TextBundle {
            text: Text::with_section(
                label,
                TextStyle {
                    font,
                    font_size: 20.0,
                    color: Color::rgb(0.85, 0.85, 0.85),
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        });
    });
}

// ============================================================
// Settings menu
// ============================================================
fn spawn_settings_menu(commands: &mut Commands, font: Handle<Font>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::None,
                ..Default::default()
            },
            color: UiColor(Color::rgb(0.08, 0.11, 0.09)),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(SettingsMenuRoot)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Settings",
                    TextStyle {
                        font: font.clone(),
                        font_size: 36.0,
                        color: Color::rgb(0.9, 0.8, 0.2),
                    },
                    TextAlignment::default(),
                ),
                style: Style {
                    margin: Rect {
                        bottom: Val::Px(30.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            });

            // Setting rows
            spawn_setting_row(
                parent,
                "Sound Effects: 100%",
                font.clone(),
                SfxVolumeControl,
            );

            spawn_setting_row(
                parent,
                "Music: 100%",
                font.clone(),
                MusicVolumeControl,
            );

            spawn_setting_row(
                parent,
                "Camera Speed: Normal",
                font.clone(),
                CameraSpeedControl,
            );

            // Back button
            spawn_menu_button(
                parent,
                "Back",
                font.clone(),
                BackButton,
            );
        });
}

fn spawn_setting_row(
    parent: &mut ChildBuilder,
    label: &str,
    font: Handle<Font>,
    marker: impl Component,
) {
    parent.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(280.0), Val::Px(40.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: Rect {
                top: Val::Px(6.0),
                bottom: Val::Px(6.0),
                ..Default::default()
            },
            ..Default::default()
        },
        color: UiColor(Color::rgba(0.15, 0.15, 0.15, 0.9)),
        ..Default::default()
    })
    .insert(SettingToggleVisual)
    .insert(marker)
    .with_children(|btn| {
        btn.spawn_bundle(TextBundle {
            text: Text::with_section(
                label,
                TextStyle {
                    font,
                    font_size: 18.0,
                    color: Color::rgb(0.8, 0.8, 0.8),
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        });
    });
}

// ============================================================
// Per-button click systems
// ============================================================

pub fn start_game_button_system(
    mut interaction_query: Query<&Interaction, (With<StartGameButton>, Changed<Interaction>)>,
    mut menu_state: ResMut<MenuState>,
    mut main_menu: Query<(&mut Style, &mut Visibility), With<MainMenuRoot>>,
    mut game_phase: ResMut<GamePhase>,
    mut game_time: ResMut<GameTime>,
    mut game_ui_query: Query<&mut Visibility, (With<GameUiRoot>, Without<MainMenuRoot>, Without<SettingsMenuRoot>)>,
) {
    if menu_state.current != GameMenuState::MainMenu { return; }
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Clicked {
            menu_state.current = GameMenuState::Playing;
            for (mut style, mut vis) in main_menu.iter_mut() {
                style.display = Display::None;
                vis.is_visible = false;
            }
            // Show in-game UI
            for mut vis in game_ui_query.iter_mut() {
                vis.is_visible = true;
            }
            game_phase.game_started = true;
            game_time.is_paused = false;
            game_time.speed_multiplier = 1.0;
        }
    }
}

pub fn resume_game_button_system(
    mut interaction_query: Query<&Interaction, (With<ResumeGameButton>, Changed<Interaction>)>,
    mut menu_state: ResMut<MenuState>,
    mut main_menu: Query<(&mut Style, &mut Visibility), With<MainMenuRoot>>,
    mut game_phase: ResMut<GamePhase>,
    mut game_time: ResMut<GameTime>,
    mut load_request: ResMut<crate::save::LoadRequest>,
    mut alerts: ResMut<GameAlerts>,
    mut game_ui_query: Query<&mut Visibility, (With<GameUiRoot>, Without<MainMenuRoot>, Without<SettingsMenuRoot>)>,
) {
    if menu_state.current != GameMenuState::MainMenu { return; }
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Clicked {
            if !crate::save::has_save() {
                alerts.push("No save found — start a new game instead.".to_string());
                return;
            }
            menu_state.current = GameMenuState::Playing;
            for (mut style, mut vis) in main_menu.iter_mut() {
                style.display = Display::None;
                vis.is_visible = false;
            }
            for mut vis in game_ui_query.iter_mut() {
                vis.is_visible = true;
            }
            game_phase.game_started = true;
            load_request.pending = true;
            game_time.is_paused = false;
            if game_time.speed_multiplier == 0.0 {
                game_time.speed_multiplier = 1.0;
            }
        }
    }
}

pub fn settings_button_system(
    mut interaction_query: Query<&Interaction, (With<SettingsButton>, Changed<Interaction>)>,
    mut menu_state: ResMut<MenuState>,
    mut main_menu: Query<(&mut Style, &mut Visibility), (With<MainMenuRoot>, Without<SettingsMenuRoot>)>,
    mut settings_menu: Query<(&mut Style, &mut Visibility), (With<SettingsMenuRoot>, Without<MainMenuRoot>)>,
) {
    if menu_state.current != GameMenuState::MainMenu { return; }
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Clicked {
            menu_state.current = GameMenuState::Settings;
            for (mut style, mut vis) in main_menu.iter_mut() {
                style.display = Display::None;
                vis.is_visible = false;
            }
            for (mut style, mut vis) in settings_menu.iter_mut() {
                style.display = Display::Flex;
                vis.is_visible = true;
            }
        }
    }
}

pub fn back_button_system(
    mut interaction_query: Query<&Interaction, (With<BackButton>, Changed<Interaction>)>,
    mut menu_state: ResMut<MenuState>,
    mut main_menu: Query<(&mut Style, &mut Visibility), (With<MainMenuRoot>, Without<SettingsMenuRoot>)>,
    mut settings_menu: Query<(&mut Style, &mut Visibility), (With<SettingsMenuRoot>, Without<MainMenuRoot>)>,
) {
    if menu_state.current != GameMenuState::Settings { return; }
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Clicked {
            menu_state.current = GameMenuState::MainMenu;
            for (mut style, mut vis) in main_menu.iter_mut() {
                style.display = Display::Flex;
                vis.is_visible = true;
            }
            for (mut style, mut vis) in settings_menu.iter_mut() {
                style.display = Display::None;
                vis.is_visible = false;
            }
        }
    }
}

pub fn quit_button_system(
    interaction_query: Query<&Interaction, (With<QuitButton>, Changed<Interaction>)>,
    menu_state: Res<MenuState>,
) {
    if menu_state.current != GameMenuState::MainMenu { return; }
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            std::process::exit(0);
        }
    }
}

// ============================================================
// System: Pause all game logic while main menu is showing
// ============================================================
pub fn menu_pause_system(
    menu_state: Res<MenuState>,
    mut game_time: ResMut<GameTime>,
) {
    match menu_state.current {
        GameMenuState::MainMenu | GameMenuState::Settings => {
            if !game_time.is_paused {
                game_time.is_paused = true;
            }
        }
        GameMenuState::Playing => {
            // Let speed control / space toggle decide the pause state during play
        }
    }
}

// ============================================================
// System: Button hover highlighting
// ============================================================
pub fn menu_button_hover_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        Or<(
            With<StartGameButton>,
            With<ResumeGameButton>,
            With<SettingsButton>,
            With<QuitButton>,
            With<BackButton>,
            With<SettingToggleVisual>,
        )>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = UiColor(Color::rgba(0.3, 0.5, 0.3, 0.95));
            }
            Interaction::None => {
                *color = UiColor(Color::rgba(0.2, 0.3, 0.2, 0.9));
            }
            Interaction::Clicked => {
                *color = UiColor(Color::rgba(0.25, 0.45, 0.25, 1.0));
            }
        }

        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(*child) {
                text.sections[0].style.color = match *interaction {
                    Interaction::Hovered => Color::rgb(1.0, 1.0, 0.8),
                    Interaction::Clicked => Color::rgb(0.6, 1.0, 0.6),
                    Interaction::None => Color::rgb(0.85, 0.85, 0.85),
                };
            }
        }
    }
}

// ============================================================
// Settings Sliders - Volume and Camera Speed Controls
// ============================================================

pub fn sfx_volume_control_system(
    mut interaction_query: Query<(&Children, &Interaction), (With<SfxVolumeControl>, Changed<Interaction>)>,
    mut text_query: Query<&mut Text>,
    mut volume: ResMut<SfxVolume>,
) {
    for (children, interaction) in interaction_query.iter_mut() {
        if let Interaction::Clicked = *interaction {
            let levels = [1.0, 0.75, 0.5, 0.25, 0.0];
            let mut next_index = 0;
            for (i, &lvl) in levels.iter().enumerate() {
                if (volume.0 - lvl).abs() < 0.01 {
                    next_index = (i + 1) % levels.len();
                    break;
                }
            }
            volume.0 = levels[next_index];
            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    let percent = (volume.0 * 100.0) as u32;
                    text.sections[0].value = format!("Sound Effects: {}%", percent);
                }
            }
        }
    }
}

pub fn music_volume_control_system(
    mut interaction_query: Query<(&Children, &Interaction), (With<MusicVolumeControl>, Changed<Interaction>)>,
    mut text_query: Query<&mut Text>,
    mut volume: ResMut<MusicVolume>,
) {
    for (children, interaction) in interaction_query.iter_mut() {
        if let Interaction::Clicked = *interaction {
            let levels = [1.0, 0.75, 0.5, 0.25, 0.0];
            let mut next_index = 0;
            for (i, &lvl) in levels.iter().enumerate() {
                if (volume.0 - lvl).abs() < 0.01 {
                    next_index = (i + 1) % levels.len();
                    break;
                }
            }
            volume.0 = levels[next_index];
            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    let percent = (volume.0 * 100.0) as u32;
                    text.sections[0].value = format!("Music: {}%", percent);
                }
            }
        }
    }
}

pub fn camera_speed_control_system(
    mut interaction_query: Query<(&Children, &Interaction), (With<CameraSpeedControl>, Changed<Interaction>)>,
    mut text_query: Query<&mut Text>,
    mut speed: ResMut<CameraSpeed>,
) {
    for (children, interaction) in interaction_query.iter_mut() {
        if let Interaction::Clicked = *interaction {
            let speeds = [(0.5, "Slow"), (1.0, "Normal"), (1.5, "Fast"), (2.0, "Ultra")];
            let mut next_index = 0;
            for (i, &(spd, _)) in speeds.iter().enumerate() {
                if (speed.0 - spd).abs() < 0.01 {
                    next_index = (i + 1) % speeds.len();
                    break;
                }
            }
            speed.0 = speeds[next_index].0;
            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.sections[0].value = format!("Camera Speed: {}", speeds[next_index].1);
                }
            }
        }
    }
}
