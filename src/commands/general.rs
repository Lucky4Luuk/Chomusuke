use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group,
    },
};

group!({
    name: "general",
    options: {},
    commands: [ping],
});

#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!");

    Ok(())
}
