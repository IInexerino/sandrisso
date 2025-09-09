use bevy::{app::{AppExit, Plugin, Update}, color::{palettes::css::YELLOW, Color}, ecs::{ component::Component, entity::Entity, event::EventWriter, query::{Changed, With}, schedule::IntoScheduleConfigs, system::{Commands, Query, ResMut}}, prelude::{children, SpawnRelated}, state::{app::AppExtStates, condition::in_state, state::{NextState, OnEnter, OnExit}}, text::{TextColor, TextFont}, ui::{widget::{Button, Text}, AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Node, UiRect, Val}, utils::default};
use crate::{menu::MenuState, AppState};

const TEXT_COLOR: Color = Color::srgb(0., 0., 0.);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {

        app.init_state::<MenuState>();
        
        app.add_systems(
            OnEnter(AppState::MainMenu), 
            menu_setup
        )
        .add_systems(
            OnEnter(MenuState::Main), 
            setup_main_menu
        )
        .add_systems(
            OnExit(MenuState::Main), 
            despawn_screen::<MainMenuScreen>,
        )
        .add_systems(
            OnEnter(MenuState::Quit), 
            exit_game
        )
        .add_systems(
            Update, 
            (menu_action, button_system)
                .run_if(in_state(AppState::MainMenu))
        );
    }
}

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub struct StatsMenuScreen;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

fn menu_setup(
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    menu_state.set(MenuState::Main);
}

fn setup_main_menu(
    mut commands: Commands,
) {
    let button_node = Node {
            width: Val::Px(300.0),
            height: Val::Px(48.75),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()

        },
        MainMenuScreen,
        children![(
            Node{
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(YELLOW.into()),
            children![
                (
                    Text::new("SandFall mimimimim"),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    },

                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Play,
                    children![
                        (
                            Text::new("New Game"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![
                        (
                            Text::new("Quit"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
            ]
        )],
    ));
}

fn exit_game(
    mut app_exit_events: EventWriter<AppExit>,
) {
    app_exit_events.write(AppExit::Success);
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}


fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => {
                    app_state.set(AppState::InGame);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Quit => menu_state.set(MenuState::Quit),
            }
        }
    }
}


// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}