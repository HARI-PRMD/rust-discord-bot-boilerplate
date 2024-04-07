use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::RoleId},
    prelude::*,
};
extern crate dotenv;

use dotenv::dotenv;
use std::env;

struct Handler;

// starter code adapted from https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content.starts_with("!verify") {
            let guild_id = match msg.guild_id {
                Some(id) => id,
                None => return,
            };

            let user_mention = msg.mentions.iter().next();

            if let Some(user) = user_mention {
                let member = match guild_id.member(&ctx.http, user).await {
                    Ok(member) => member,
                    Err(why) => {
                        println!("Error getting member: {:?}", why);
                        return;
                    }
                };

                let verified_role_id = RoleId::new(1226345788406239343);

                if let Err(why) = member.add_role(&ctx.http, verified_role_id).await {
                    println!("Error adding role: {:?}", why);
                    return;
                }

                match msg
                    .channel_id
                    .say(&ctx.http, format!("Verified {}", user))
                    .await
                {
                    Err(why) => println!("Error sending message: {:?}", why),
                    Ok(_) => println!("Verified @{}", user.tag()),
                }
            } else {
                if let Err(why) = msg.channel_id.say(&ctx.http, "No user mentioned").await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .intents(intents)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
