use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, RoleId};
use serenity::model::voice::VoiceState;
use serenity::prelude::*;
//use std::env;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
struct Config {
    channel_id: u64,
    role_id: u64,
}

struct Handler {
    config: Config,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        if new.member.as_ref().unwrap().user.bot {
            return;
        }
        let text_channel = ChannelId(self.config.channel_id);
        let mention = RoleId(self.config.role_id).mention();

        if let Some(new_channel_id) = new.channel_id {
            let user_name = new.member.as_ref().unwrap().display_name();
            let actual_name = &new.member.as_ref().unwrap().user.name;
            let channel_name = match new_channel_id.name(&ctx.cache).await {
                Some(name) => name,
                None => "unknown".to_string(),
            };

            // Check if user moved from another channel or not
            if let Some(old_channel_id) = old.and_then(|v| v.channel_id) {
                // User moved from old channel to new channel
                if new_channel_id != old_channel_id {
                    let old_channel_name = match old_channel_id.name(&ctx.cache).await {
                        Some(name) => name,
                        None => "Unkown".to_string(),
                    };
                    let msg = format!(
                        "{}, {} moved from {} to {}",
                        mention, user_name, old_channel_name, channel_name
                    );
                    // Send message to text channel
                    if let Err(why) = text_channel.say(&ctx.http, msg).await {
                        println!("{:?}", why);
                    };
                }
            } else {
                // User joined new channel
                let msg = format!(
                    "{}, {}({}) joined {}",
                    mention, user_name, actual_name, channel_name
                );
                // Send message to text channel
                if let Err(why) = text_channel.say(&ctx.http, msg).await {
                    println!("{:?}", why);
                };
            }
        } else if let Some(old_channel_id) = old.as_ref().and_then(|v| v.channel_id) {
            // User left a voice channel
            let user_name = old
                .as_ref()
                .unwrap()
                .member
                .as_ref()
                .unwrap()
                .display_name();
            let channel_name = match old_channel_id.name(&ctx.cache).await {
                Some(name) => name,
                None => "Unknown".to_string(),
            };
            let msg = format!("{}, {} left {}", mention, user_name, channel_name);
            // Send message to text channel
            if let Err(why) = text_channel.say(&ctx.http, msg).await {
                println!("{:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let config = load_config();
    let mut file = File::open("config.txt").expect("Fail to open");
    let mut token = String::new();
    file.read_to_string(&mut token).expect("Fail to read");
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { config })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

fn load_config() -> Config {
    let config_str = fs::read_to_string("config.json").expect("Failed to read config file");
    serde_json::from_str(&config_str).expect("Failed to parse config file")
}
