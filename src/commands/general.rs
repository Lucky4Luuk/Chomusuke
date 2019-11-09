use log::{debug, error, info, trace, warn};

use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
    },
};

#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    //let _ = msg.channel_id.say(&ctx.http, "Pong!");
    msg.reply(ctx, "Pong!")?;

    Ok(())
}
