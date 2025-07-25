use bevy::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Event)]
pub struct UserJoined(pub String);

pub struct TwitchPlugin;

impl Plugin for TwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
            .add_systems(Startup, setup)
            .add_event::<UserJoined>();
    }
}

fn setup(runtime: ResMut<bevy_tokio_tasks::TokioTasksRuntime>) {
    runtime.spawn_background_task(start_listening);
}

async fn start_listening(mut ctx: bevy_tokio_tasks::TaskContext) {
    let config = twitch_irc::ClientConfig::default();
    let (incoming_messages, client) = twitch_irc::TwitchIRCClient::<
        twitch_irc::SecureTCPTransport,
        twitch_irc::login::StaticLoginCredentials,
    >::new(config);

    client.join("imnethen".to_owned()).unwrap();

    let big_receiver = Arc::new(Mutex::new(incoming_messages));

    loop {
        let receiver = Arc::clone(&big_receiver);
        ctx.run_on_main_thread(move |ctx| {
            while let Ok(message) = receiver.lock().unwrap().try_recv() {
                if let twitch_irc::message::ServerMessage::Privmsg(msg) = message {
                    ctx.world.send_event(UserJoined(msg.sender.name));
                }
            }
        })
        .await;
        tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
    }
}
