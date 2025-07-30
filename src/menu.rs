use crate::{player, twitch, util, GameState};
use bevy::color::palettes::basic;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChannelName::default())
            .add_systems(OnEnter(GameState::Start), setup_main_menu)
            .add_systems(
                Update,
                (
                    button_system,
                    (update_name, update_name_text).run_if(in_state(GameState::Start)),
                ),
            )
            .add_systems(OnEnter(GameState::Connected), update_menu)
            .add_systems(OnEnter(GameState::Spectating), despawn_main_menu)
            .add_systems(OnEnter(GameState::End), setup_end_menu);
    }
}

#[derive(Resource, Default)]
pub struct ChannelName(pub String);

#[derive(Component)]
struct NameText;

#[derive(Component, PartialEq, Eq)]
#[require(Button)]
enum ButtonAction {
    Connect,
    Start,
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
}

fn update_menu(
    mut commands: Commands,
    mut buttons_query: Query<(&Children, &mut Node, &mut ButtonAction)>,
) {
    let (children, mut node, mut button_action) = buttons_query.single_mut().unwrap();
    *button_action = ButtonAction::Start;

    commands
        .entity(children[0])
        .remove::<Text>()
        .insert(Text::new("START GAME"));

    node.width = Val::Px(200.);
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
