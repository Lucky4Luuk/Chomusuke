use log::{debug, error, info, trace, warn};

use regex::Regex;

use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
    },
};

#[command]
pub fn avatar(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut target = &msg.author;

    // TODO: Put this in a seperate function in an util module
    let regex = Regex::new(r"\s+").unwrap(); //No error handling needed here, as a panic here would mean our code is completely fucking broken
    let formatted = regex.replace_all(msg.content["cs!".len()..].trim(), " ");
    let arguments = formatted.split(' ');

    println!("{:?}", arguments);

    // TODO: Put this in a util module as well
    {
        // Check if an user was mentioned
        let mentions = &msg.mentions;
        if !mentions.is_empty() {
            target = &mentions[0];
        }
    }

    let profile_picture = target.avatar_url();

    // Send a message if the target doesn't have a profile picture
    if profile_picture.is_none() {
        if (target == &msg.author) {
            if let Err(why) = msg.channel_id.say(ctx, "Sorry, you don't have a profile picture!") {
                error!("A channel became inaccessible during execution of a command!");
                error!("Err: {:?}", why);
            }
        } else {
            if let Err(why) = msg.channel_id.say(ctx, "Sorry, that user doesn't have a profile picture!") {
                error!("A channel became inaccessible during execution of a command!");
                error!("Err: {:?}", why);
            }
        }
        return Ok(())
    }

    // Send the profile picture in the channel
    if let Err(why) = msg.channel_id.say(ctx, profile_picture.unwrap()) {
        error!("A channel became inaccessible during execution of a command!");
        error!("Err: {:?}", why);
    }
    Ok(())
}
