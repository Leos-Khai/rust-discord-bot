use super::Command;
use crate::database::Database;
use async_trait::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Setup {
    pub db: Arc<Mutex<Database>>,
}

#[async_trait]
impl Command for Setup {
    async fn run(&self, ctx: &Context, msg: &Message) {
        let args: Vec<&str> = msg.content.split_whitespace().collect();
        if args.len() != 3 {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    "Usage: !setup <voice-channel-id> <text-channel-id>",
                )
                .await;
            return;
        }

        if let (Ok(voice_id), Ok(text_id)) = (args[1].parse::<i64>(), args[2].parse::<i64>()) {
            let db = self.db.lock().await; // Use `tokio::sync::Mutex`'s async `lock` method
            match db.link_channels(voice_id, text_id) {
                Ok(_) => {
                    let _ = msg
                        .channel_id
                        .say(&ctx.http, "Channels linked successfully!")
                        .await;
                }
                Err(_) => {
                    let _ = msg
                        .channel_id
                        .say(&ctx.http, "Failed to link channels.")
                        .await;
                }
            }
        } else {
            let _ = msg.channel_id.say(&ctx.http, "Invalid channel IDs.").await;
        }
    }
}
