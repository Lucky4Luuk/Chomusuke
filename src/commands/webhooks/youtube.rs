use log::{debug, error, info, trace, warn};

use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
    },
};

//TODO: Implement this as a subgroup, with commands for setting up the channel it will post
//to and connect it to the right youtube account
#[command]
pub fn youtube(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "Pong!");
    Ok(())
}
