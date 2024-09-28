use super::Command;
use async_trait::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

pub struct Ping;

#[async_trait]
impl Command for Ping {
    async fn run(&self, ctx: &Context, msg: &Message) {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
            println!("Error sending message: {:?}", why);
        }
    }
}
