use log::{debug, error, info, trace, warn};

use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::{
        command,
    },
};

#[command]
pub fn set_welcome_channel(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.channel_id.say(ctx, "Pong!")?;
    Ok(())
}
