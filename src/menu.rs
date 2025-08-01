use crate::{player, twitch, util, GameState};
use bevy::color::palettes::basic;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChannelName::default())
            .insert_resource(Settings::default())
            .add_systems(OnEnter(GameState::Start), setup_main_menu)
            .add_systems(
                Update,
                (
                    button_system,
                    update_filter_text,
                    update_gpp_text,
                    (update_name, update_name_text).run_if(in_state(GameState::Start)),
                ),
            )
            .add_systems(OnEnter(GameState::Connected), update_menu)
            .add_systems(OnEnter(GameState::Spectating), despawn_main_menu)
            .add_systems(OnEnter(GameState::End), setup_end_menu);
    }
}

#[derive(Resource)]
pub struct Settings {
    pub filter_joins: bool,
    pub goats_per_player: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            filter_joins: true,
            goats_per_player: 1,
        }
    }
}

#[derive(Resource, Default)]
pub struct ChannelName(pub String);

#[derive(Component)]
struct NameText;

#[derive(Component)]
struct FilterText;

#[derive(Component)]
struct GPPText;

#[derive(Component, PartialEq, Eq)]
#[require(Button)]
enum ButtonAction {
    Connect,
    Start,
    ToggleFilter,
    ChangeGPP(bool),
}

#[derive(Component)]
#[require(Node)]
struct MenuRootNode;

fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        MenuRootNode,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![
            Node {
                height: Val::Percent(0.),
                ..Default::default()
            },
            (
                Text::new("INSANE_LIVESTOCK"),
                TextFont::default().with_font_size(100.),
            ),
            (
                Text::new("channel name: "),
                TextFont::default().with_font_size(30.),
                NameText,
            ),
            button(ButtonAction::Connect, "CONNECT", Val::Px(150.)),
            Node {
                height: Val::Percent(0.),
                ..Default::default()
            },
        ],
    ));

    commands.spawn((
        MenuRootNode,
        Node {
            height: Val::Percent(10.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },
        children![
            (
                ButtonAction::ToggleFilter,
                Node {
                    width: Val::Px(400.),
                    height: Val::Px(60.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(5.)),
                    ..Default::default()
                },
                BorderColor(basic::BLACK.into()),
                BorderRadius::all(Val::Px(8.)),
                BackgroundColor(basic::GRAY.into()),
                children![(
                    Text::new("Only count 'play!' messages: yes".to_owned()),
                    TextFont {
                        font_size: 20.,
                        ..Default::default()
                    },
                    FilterText,
                )]
            ),
            (
                Node {
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                children![
                    button(ButtonAction::ChangeGPP(true), "+", Val::Px(60.)),
                    button(ButtonAction::ChangeGPP(false), "-", Val::Px(60.)),
                    (
                        Text::new("Goats per player: 1"),
                        TextFont {
                            font_size: 20.,
                            ..Default::default()
                        },
                        GPPText,
                    )
                ]
            )
        ],
    ));
}

fn update_menu(
    mut commands: Commands,
    buttons_query: Query<(&Children, &mut Node, &mut ButtonAction)>,
) {
    for (children, mut node, mut button_action) in buttons_query {
        if *button_action != ButtonAction::Connect {
            continue;
        }
        *button_action = ButtonAction::Start;

        commands
            .entity(children[0])
            .remove::<Text>()
            .insert(Text::new("START GAME"));

        node.width = Val::Px(200.);
    }
}

fn button(action: ButtonAction, text: impl Into<String>, width: Val) -> impl Bundle {
    (
        action,
        Node {
            width,
            height: Val::Px(60.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(5.)),
            ..Default::default()
        },
        BorderColor(basic::BLACK.into()),
        BorderRadius::all(Val::Px(8.)),
        BackgroundColor(basic::GRAY.into()),
        children![(
            Text::new(text.into()),
            TextFont {
                font_size: 30.,
                ..Default::default()
            },
        )],
    )
}

fn button_system(
    mut connect_events: EventWriter<twitch::ConnectEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut settings: ResMut<Settings>,
    channel_name: Res<ChannelName>,
    mut button_query: Query<
        (&Interaction, &ButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action, mut color) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                match action {
                    ButtonAction::Connect => {
                        connect_events.write(twitch::ConnectEvent(channel_name.0.clone()));
                        next_game_state.set(GameState::Connected);
                    }
                    ButtonAction::Start => {
                        next_game_state.set(GameState::Spectating);
                    }
                    ButtonAction::ToggleFilter => {
                        settings.filter_joins = !settings.filter_joins;
                    }
                    ButtonAction::ChangeGPP(b) => {
                        if settings.goats_per_player == 1 && !*b {
                            continue;
                        }
                        let diff = if settings.goats_per_player < 10 {
                            1
                        } else {
                            10
                        };
                        if *b {
                            settings.goats_per_player += diff;
                        } else {
                            settings.goats_per_player -= diff;
                        }
                    }
                };
            }
            Interaction::Hovered => {
                *color = basic::BLACK.into();
            }
            Interaction::None => {
                *color = basic::GRAY.into();
            }
        }
    }
}

fn update_name(mut name: ResMut<ChannelName>, input: Res<ButtonInput<KeyCode>>) {
    for keycode in input.get_just_pressed() {
        match util::keycode_to_string(keycode) {
            Ok(s) => name.0 += s,
            Err(_) => {
                if *keycode == KeyCode::Backspace {
                    name.0.pop();
                }
            }
        };
    }
}

fn update_name_text(name: Res<ChannelName>, mut text_query: Single<&mut Text, With<NameText>>) {
    text_query.0 = "channel name: ".to_owned() + &name.0.clone();
}

fn update_filter_text(
    settings: Res<Settings>,
    mut text_query: Single<&mut Text, With<FilterText>>,
) {
    text_query.0 = "Only count '!play' messages: ".to_owned()
        + if settings.filter_joins { "yes" } else { "no" };
}

fn update_gpp_text(settings: Res<Settings>, mut text_query: Single<&mut Text, With<GPPText>>) {
    text_query.0 = "Goats per player: ".to_owned() + &settings.goats_per_player.to_string();
}

fn despawn_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuRootNode>>) {
    for menu in menu_query {
        commands.entity(menu).despawn();
    }
}

fn setup_end_menu(mut commands: Commands, alive_players: Res<player::Players>) {
    let name_opt = alive_players.0.iter().last();
    let name = match name_opt {
        Some(n) => n.clone(),
        None => "SOMEONE".to_owned(),
    };

    commands.spawn((
        MenuRootNode,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![
            Node {
                height: Val::Percent(0.),
                ..Default::default()
            },
            (
                Text::new(name + " WON"),
                TextFont::default().with_font_size(100.),
            ),
            Node {
                height: Val::Percent(0.),
                ..Default::default()
            },
        ],
    ));
}
