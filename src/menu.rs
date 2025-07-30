use crate::{player, twitch, GameState};
use bevy::color::palettes::basic;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), setup_main_menu)
            .add_systems(Update, connect_button_system)
            .add_systems(OnEnter(GameState::Spectating), despawn_main_menu)
            .add_systems(OnEnter(GameState::End), setup_end_menu);
    }
}

#[derive(Component)]
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
                Text::new("channel name: imnethen (hardcoded currently,)"),
                TextFont::default().with_font_size(30.),
            ),
            button(ButtonAction::Connect, "CONNECT", Val::Px(150.)),
            button(ButtonAction::Start, "START GAME", Val::Px(200.)),
            Node {
                height: Val::Percent(0.),
                ..Default::default()
            },
        ],
    ));
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

fn connect_button_system(
    mut connect_events: EventWriter<twitch::ConnectEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
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
                        connect_events.write(twitch::ConnectEvent("imnethen".into()));
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
