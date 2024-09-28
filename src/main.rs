mod commands;
mod database;

use commands::get_commands;
use database::Database;
use serde::Deserialize;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, RoleId};
use serenity::model::voice::VoiceState;
use serenity::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
struct Config {
    role_id: u64,
}

struct Handler {
    role_id: u64,
    db: Arc<Mutex<Database>>, // Updated to use Arc<tokio::sync::Mutex> for thread safety
    commands: HashMap<String, Box<dyn commands::Command + Send + Sync>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let content = msg.content.trim();
        println!("Received message: {}", content);

        if content.starts_with('!') {
            let command = content[1..].split_whitespace().next().unwrap_or("");
            println!("Parsed command: {}", command);

            if let Some(cmd) = self.commands.get(command) {
                cmd.run(&ctx, &msg).await;
            } else {
                println!("Unknown command: {}", command);
            }
        }
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        if new.member.as_ref().unwrap().user.bot {
            return;
        }

        let db = Arc::clone(&self.db);
        let mention = RoleId(self.role_id).mention();

        if let Some(new_channel_id) = new.channel_id {
            let linked_text_channel = db
                .lock()
                .await
                .get_linked_text_channel(new_channel_id.0 as i64)
                .expect("Failed to retrieve linked channel");

            if let Some(text_channel_id) = linked_text_channel {
                let text_channel = ChannelId(text_channel_id as u64);
                let user_name = new.member.as_ref().unwrap().display_name();
                let actual_name = &new.member.as_ref().unwrap().user.name;
                let channel_name = match new_channel_id.name(&ctx.cache).await {
                    Some(name) => name,
                    None => "unknown".to_string(),
                };

                if let Some(old_channel_id) = old.and_then(|v| v.channel_id) {
                    if new_channel_id != old_channel_id {
                        let old_channel_name = match old_channel_id.name(&ctx.cache).await {
                            Some(name) => name,
                            None => "Unknown".to_string(),
                        };
                        let msg = format!(
                            "{}, {} moved from {} to {}",
                            mention, user_name, old_channel_name, channel_name
                        );
                        if let Err(why) = text_channel.say(&ctx.http, msg).await {
                            println!("{:?}", why);
                        }
                    }
                } else {
                    let msg = format!(
                        "{}, {}({}) joined {}",
                        mention, user_name, actual_name, channel_name
                    );
                    if let Err(why) = text_channel.say(&ctx.http, msg).await {
                        println!("{:?}", why);
                    }
                }
            }
        } else if let Some(old_channel_id) = old.as_ref().and_then(|v| v.channel_id) {
            let linked_text_channel = db
                .lock()
                .await
                .get_linked_text_channel(old_channel_id.0 as i64)
                .expect("Failed to retrieve linked channel");

            if let Some(text_channel_id) = linked_text_channel {
                let text_channel = ChannelId(text_channel_id as u64);
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
                if let Err(why) = text_channel.say(&ctx.http, msg).await {
                    println!("{:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let db = self.db.lock().await;
        db.print_existing_tables().unwrap();
    }
}

#[tokio::main]
async fn main() {
    let config = load_config();
    let db = Arc::new(Mutex::new(Database::new("data.db").unwrap()));
    let mut file = fs::File::open("config.txt").expect("Fail to open");
    let mut token = String::new();
    file.read_to_string(&mut token).expect("Fail to read");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            role_id: config.role_id,
            db: Arc::clone(&db),
            commands: get_commands(Arc::clone(&db)),
        })
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
