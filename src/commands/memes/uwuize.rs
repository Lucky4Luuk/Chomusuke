use log::{debug, error, info, trace, warn};

use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
    },
};

//TODO: Dennis please implement this
#[command]
pub fn uwuize(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "Pong!");
    Ok(())
}
