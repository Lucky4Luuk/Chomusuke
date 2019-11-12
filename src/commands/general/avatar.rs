use log::{debug, error, info, trace, warn};

use std::sync::Arc;

use regex::Regex;

use serenity::model::{
    user::{
        User,
    },
    channel::{
        Message,
    },
};
use serenity::utils::Colour;
use serenity::prelude::Context;
use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::{
        command,
    },
};

use crate::utils::discord_utils;

#[command]
#[aliases("avi", "picture", "pic", "profilepicture", "profile_picture")]
pub fn avatar(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = args.single::<String>();
    let profile_picture : String;
    let name : String;

    match user {
        Ok(target) => {
            // An argument was provided, search for that user
            let mut guild_channel = ctx.cache.read().guild_channel(msg.channel_id);
            if(guild_channel.is_none()) {
                discord_utils::check_message(msg.channel_id.say(ctx, "Sorry, this command only works in servers!"));
                return Ok(());
            }

            let mut target = discord_utils::find_member(&target, &guild_channel.unwrap().read().guild_id, &ctx.cache);

            if(target.is_none()) {
                discord_utils::check_message(msg.channel_id.say(ctx,
                    "Sorry, I couldn't find that user!\nPlease make sure you are provding the User ID or a mention!"
                ));
                return Ok(());
            }

            let member = target.unwrap();
            let user = member.user.read();
            name = format!("{}#{}", user.name, user.discriminator);

            match user.avatar_url() {
                Some(url) => profile_picture = url,
                None => {
                    discord_utils::check_message(msg.channel_id.say(ctx, "Sorry, that user doesn't have a profile picture!"));
                    return Ok(());
                }
            }
        },
        Err(e) => {
            // No arguments were provided, use the author
            name = format!("{}#{}", msg.author.name, msg.author.discriminator);

            match msg.author.avatar_url() {
                Some(url) => profile_picture = url,
                None => {
                    discord_utils::check_message(msg.channel_id.say(ctx, "Sorry, you don't have a profile picture!"));
                    return Ok(());
                }
            }
        }
    }

    // Send the profile picture in the channel
    discord_utils::check_message(msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.title(format!("Here\'s {}\'s Profile Picture!", name));
            e.image(profile_picture);
            e.colour(discord_utils::DEFAULT_COLOUR);
            e
        });
        m
    }));
    return Ok(());
}
