use log::{debug, error, info, trace, warn};

use serenity::model::misc::Mentionable;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::{
        command,
    },
};
use serenity::utils::Colour;

use serde::Deserialize;

use crate::utils::discord_utils;

#[derive(Deserialize)]
struct ApiResult {
    url: String
}

fn api_call(endpoint: String) -> Result<String, &'static str> {
    let mut request = match reqwest::get(&(format!("https://nekos.life/api/v2/img/{}", endpoint))) {
        Ok(req) => req,
        Err(why) => {
            error!("Failed to request API: {}", why);
            return Err("Rip");
        }
    };
    let data: ApiResult = request.json().unwrap(); //No need to check for errors here, as this should either always work or never work
    Ok(data.url)
}

#[command]
pub fn pat(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = args.single::<String>();
    let name: String;

    match user {
        Ok(target) => {
            let mut guild_channel = ctx.cache.read().guild_channel(msg.channel_id);
            if guild_channel.is_none() {
                discord_utils::check_message(msg.channel_id.say(ctx, "Sorry, this command only works in servers!"));
                return Ok(());
            }

            let mut target = discord_utils::find_member(&target, &guild_channel.unwrap().read().guild_id, &ctx.cache);
            if target.is_none() {
                discord_utils::check_message(msg.channel_id.say(ctx,
                    "Sorry, I couldn't find that user!\nPlease make sure you are provding the User ID or a mention!"
                ));
                return Ok(());
            }

            let member = target.unwrap();
            let user = member.user.read();
            name = user.mention();
        },
        Err(e) => {
            //No arguments were provided, use the author
            //name = msg.author.mention();
            name = "themselves :O".to_string();
        }
    }

    let url = api_call("pat".to_string())?;
    discord_utils::check_message(msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.description(format!("{} patted {}!", msg.author.mention(), name));
            e.image(url);
            e.colour(discord_utils::NEKOS_COLOUR);
            e
        })
    }));
    Ok(())
}

#[command]
pub fn hug(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = args.single::<String>();
    let name: String;

    match user {
        Ok(target) => {
            let mut guild_channel = ctx.cache.read().guild_channel(msg.channel_id);
            if guild_channel.is_none() {
                discord_utils::check_message(msg.channel_id.say(ctx, "Sorry, this command only works in servers!"));
                return Ok(());
            }

            let mut target = discord_utils::find_member(&target, &guild_channel.unwrap().read().guild_id, &ctx.cache);
            if target.is_none() {
                discord_utils::check_message(msg.channel_id.say(ctx,
                    "Sorry, I couldn't find that user!\nPlease make sure you are provding the User ID or a mention!"
                ));
                return Ok(());
            }

            let member = target.unwrap();
            let user = member.user.read();
            name = user.mention();
        },
        Err(e) => {
            //No arguments were provided, use the author
            //name = msg.author.mention();
            name = "themselves :O".to_string();
        }
    }

    let url = api_call("hug".to_string())?;
    discord_utils::check_message(msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.description(format!("{} hugged {}!", msg.author.mention(), name));
            e.image(url);
            e.colour(discord_utils::NEKOS_COLOUR);
            e
        })
    }));
    Ok(())
}

#[command]
pub fn slap(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = args.single::<String>();
    let name: String;

    match user {
        Ok(target) => {
            let mut guild_channel = ctx.cache.read().guild_channel(msg.channel_id);
            if guild_channel.is_none() {
                discord_utils::check_message(msg.channel_id.say(ctx, "Sorry, this command only works in servers!"));
                return Ok(());
            }

            let mut target = discord_utils::find_member(&target, &guild_channel.unwrap().read().guild_id, &ctx.cache);
            if target.is_none() {
                discord_utils::check_message(msg.channel_id.say(ctx,
                    "Sorry, I couldn't find that user!\nPlease make sure you are provding the User ID or a mention!"
                ));
                return Ok(());
            }

            let member = target.unwrap();
            let user = member.user.read();
            name = user.mention();
        },
        Err(e) => {
            //No arguments were provided, use the author
            //name = msg.author.mention();
            name = "themselves :O".to_string();
        }
    }

    let url = api_call("slap".to_string())?;
    discord_utils::check_message(msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.description(format!("{} slapped {}!", msg.author.mention(), name));
            e.image(url);
            e.colour(discord_utils::NEKOS_COLOUR);
            e
        })
    }));
    Ok(())
}

#[command]
pub fn tickle(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = args.single::<String>();
    let name: String;

    match user {
        Ok(target) => {
            let mut guild_channel = ctx.cache.read().guild_channel(msg.channel_id);
            if guild_channel.is_none() {
                discord_utils::check_message(msg.channel_id.say(ctx, "Sorry, this command only works in servers!"));
                return Ok(());
            }

            let mut target = discord_utils::find_member(&target, &guild_channel.unwrap().read().guild_id, &ctx.cache);
            if target.is_none() {
                discord_utils::check_message(msg.channel_id.say(ctx,
                    "Sorry, I couldn't find that user!\nPlease make sure you are provding the User ID or a mention!"
                ));
                return Ok(());
            }

            let member = target.unwrap();
            let user = member.user.read();
            name = user.mention();
        },
        Err(e) => {
            //No arguments were provided, use the author
            //name = msg.author.mention();
            name = "themselves :O".to_string();
        }
    }

    let url = api_call("tickle".to_string())?;
    discord_utils::check_message(msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.description(format!("{} tickled {}!", msg.author.mention(), name));
            e.image(url);
            e.colour(discord_utils::NEKOS_COLOUR);
            e
        })
    }));
    Ok(())
}

#[command]
pub fn kiss(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = args.single::<String>();
    let name: String;

    match user {
        Ok(target) => {
            let mut guild_channel = ctx.cache.read().guild_channel(msg.channel_id);
            if guild_channel.is_none() {
                discord_utils::check_message(msg.channel_id.say(ctx, "Sorry, this command only works in servers!"));
                return Ok(());
            }

            let mut target = discord_utils::find_member(&target, &guild_channel.unwrap().read().guild_id, &ctx.cache);
            if target.is_none() {
                discord_utils::check_message(msg.channel_id.say(ctx,
                    "Sorry, I couldn't find that user!\nPlease make sure you are provding the User ID or a mention!"
                ));
                return Ok(());
            }

            let member = target.unwrap();
            let user = member.user.read();
            name = user.mention();
        },
        Err(e) => {
            //No arguments were provided, use the author
            //name = msg.author.mention();
            name = "themselves :O".to_string();
        }
    }

    let url = api_call("kiss".to_string())?;
    discord_utils::check_message(msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.description(format!("{} kissed {}!", msg.author.mention(), name));
            e.image(url);
            e.colour(discord_utils::NEKOS_COLOUR);
            e
        })
    }));
    Ok(())
}

#[command]
pub fn poke(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = args.single::<String>();
    let name: String;

    match user {
        Ok(target) => {
            let mut guild_channel = ctx.cache.read().guild_channel(msg.channel_id);
            if guild_channel.is_none() {
                discord_utils::check_message(msg.channel_id.say(ctx, "Sorry, this command only works in servers!"));
                return Ok(());
            }

            let mut target = discord_utils::find_member(&target, &guild_channel.unwrap().read().guild_id, &ctx.cache);
            if target.is_none() {
                discord_utils::check_message(msg.channel_id.say(ctx,
                    "Sorry, I couldn't find that user!\nPlease make sure you are provding the User ID or a mention!"
                ));
                return Ok(());
            }

            let member = target.unwrap();
            let user = member.user.read();
            name = user.mention();
        },
        Err(e) => {
            //No arguments were provided, use the author
            //name = msg.author.mention();
            name = "themselves :O".to_string();
        }
    }

    let url = api_call("poke".to_string())?;
    discord_utils::check_message(msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.description(format!("{} poked {}!", msg.author.mention(), name));
            e.image(url);
            e.colour(discord_utils::NEKOS_COLOUR);
            e
        })
    }));
    Ok(())
}
