use std::collections::HashSet;
use log::{debug, error, info, trace, warn};

use serenity::model::channel::Message;
use serenity::model::prelude::UserId;
use serenity::prelude::Context;
use serenity::framework::standard::{
    Args,
    HelpOptions,
    help_commands,
    CommandResult,
    CommandGroup,
    macros::{
        command,
        help,
    },
};

#[help]
pub fn my_help(ctx: &mut Context, msg: &Message, args: Args, help_options: &'static HelpOptions, groups: &[&'static CommandGroup], owners: HashSet<UserId>) -> CommandResult {
    help_commands::with_embeds(ctx, msg, args, &help_options, groups, owners)
}
