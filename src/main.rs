use log::{debug, error, info, trace, warn, Level, LevelFilter};

use std::{env, thread, time::Duration, sync::Arc, collections::HashSet};

use serenity::{
    model::{
        user::CurrentUser,
        gateway::Ready,
        event::{
            ResumedEvent,
        },
    },
    prelude::*,
    client::{
        Client,
        bridge::gateway::ShardManager,
    },
    framework::standard::{
        StandardFramework,
        Configuration,
        DispatchError,
        macros::group,
    },
};

pub mod commands;
use commands::{
    general::GENERAL_GROUP,
    meta::help::MY_HELP,
};

pub mod utils;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Succesfully connected as {}!", ready.user.tag());
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed!");
    }
}

fn main() {
    let mut builder = pretty_env_logger::formatted_builder();
    builder.filter(Some("Chomusuke"), LevelFilter::max());
    builder.init();

    info!("Attempting login...");

    let token = include_str!("sensitive_data/token.txt");

    info!("Chomusuke is starting!");

    let mut client = match Client::new(&token, Handler) {
        Ok(info) => info,
        Err(why) => panic!("Error creating client!"),
    };

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("cs!")
                        .ignore_bots(true)
                        .delimiters(vec![",", ", ", " "])
                        //.allow_whitespace(true)
                        //.on_mention(true)
                        .case_insensitivity(true))
        //Yikes
        .on_dispatch_error(|ctx, msg, error| {
            if let DispatchError::Ratelimited(seconds) = error {
                msg.channel_id.say(&ctx, &format!("Try again in {} seconds!", seconds));
            }
            if let DispatchError::LackingPermissions(permissions) = error {
                msg.channel_id.say(&ctx, &format!("You lack the following permissions: {:?}", permissions));
            }
            if let DispatchError::IgnoredBot = error {
                msg.channel_id.say(&ctx, "Sorry, commands cannot be triggered by bot users!");
            }
        })
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
    );

    //Start bot autosharded. I really hope this holds up in the future, but it's kinda hard
    //to test sharding without having access to more than 2500 guilds lmao
    if let Err(why) = client.start_autosharded() {
        error!("Client error: {:?}", why);
    }
}
