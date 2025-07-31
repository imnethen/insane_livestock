use bevy::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub text: String,
}

#[derive(Event)]
pub struct UserJoined(pub Message);

#[derive(Event)]
pub struct ConnectEvent(pub String);

pub struct TwitchPlugin;

impl Plugin for TwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
            .add_systems(Update, connect)
            .add_event::<UserJoined>()
            .add_event::<ConnectEvent>();
    }
}

fn connect(
    mut connect_events: EventReader<ConnectEvent>,
    tokio_runtime: ResMut<bevy_tokio_tasks::TokioTasksRuntime>,
) {
    let events = connect_events.read().collect::<Vec<_>>();
    if !events.is_empty() {
        let name = events[0].0.clone();
        tokio_runtime.spawn_background_task(|ctx| start_listening(ctx, name));
    }
}

async fn start_listening(mut ctx: bevy_tokio_tasks::TaskContext, name: String) {
    let config = twitch_irc::ClientConfig::default();
    let (incoming_messages, client) = twitch_irc::TwitchIRCClient::<
        twitch_irc::SecureTCPTransport,
        twitch_irc::login::StaticLoginCredentials,
    >::new(config);

    if let Err(_) = client.join(name) {
        ctx.run_on_main_thread(move |ctx| {
            ctx.world.commands().spawn((
                Text::new("COULDNT CONNECT RESTART THE GAME"),
                TextFont {
                    font_size: 100.,
                    ..Default::default()
                },
                TextColor(bevy::color::palettes::basic::RED.into()),
            ));
        })
        .await;
        return;
    }

    let big_receiver = Arc::new(Mutex::new(incoming_messages));

    loop {
        let receiver = Arc::clone(&big_receiver);
        ctx.run_on_main_thread(move |ctx| {
            while let Ok(message) = receiver.lock().unwrap().try_recv() {
                if let twitch_irc::message::ServerMessage::Privmsg(msg) = message {
                    ctx.world.send_event(UserJoined(Message {
                        sender: msg.sender.name,
                        text: msg.message_text,
                    }));
                }
            }
        })
        .await;
        tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
    }
}
