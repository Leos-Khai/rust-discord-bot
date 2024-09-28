use crate::database::Database;
use async_trait::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex; // Use tokio::sync::Mutex instead of std::sync::Mutex

// Define the Command trait to be used by all commands
#[async_trait]
pub trait Command: Send + Sync {
    async fn run(&self, ctx: &Context, msg: &Message);
}

// Include individual command modules here
mod ping;
pub use ping::Ping;
mod setup;
pub use setup::Setup;

// Create a hashmap to store and manage command instances
pub fn get_commands(db: Arc<Mutex<Database>>) -> HashMap<String, Box<dyn Command + Send + Sync>> {
    let mut commands: HashMap<String, Box<dyn Command + Send + Sync>> = HashMap::new();
    commands.insert("ping".to_string(), Box::new(Ping));
    commands.insert("setup".to_string(), Box::new(Setup { db }));
    commands
}
