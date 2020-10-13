use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready, guild::Emoji},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            let channel = match msg.channel_id.to_channel(&context) {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);

                    return;
                },
            };

            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" used the 'ping' command in the ")
                .mention(&channel)
                .push(" channel")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response) {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "!info" {
            let channel = match msg.channel_id.to_channel(&context) {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);

                    return;
                },
            };

            let response = MessageBuilder::new()
                .push_bold_safe("What is Tooly ?\n")
                .push("Tooly is a Discord Bot written in Rust that is going to provide useful tools for server members.")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response) {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content == "!lxdb" {
            let channel = match msg.channel_id.to_channel(&context) {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);

                    return;
                },
            };

            let response = MessageBuilder::new()
                .push("Coming soon")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
