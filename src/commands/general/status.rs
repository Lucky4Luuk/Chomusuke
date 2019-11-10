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
pub fn status(ctx: &mut Context, msg: &Message) -> CommandResult {
    Ok(())
}
