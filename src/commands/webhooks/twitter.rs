use log::{debug, error, info, trace, warn};

use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
    },
};

//Use egg-mode + tokio to scrape twitter
//TODO: Implement this as a subgroup, with commands for setting up the channel it will post
//to and connect it to the right twitter account
#[command]
pub fn twitter(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "Pong!");
    Ok(())
}
