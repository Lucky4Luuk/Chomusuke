use log::{debug, error, info, trace, warn, Level, LevelFilter};

use std::{env, thread, time::Duration, sync::Arc, collections::HashSet};

use serenity::{
    model::{
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
        macros::group,
    },
};

pub mod commands;
use commands::{
    general::*,
    memes::*,
};

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Succesfully connected as {}!", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed!");
    }
}

fn main() {
    let mut builder = pretty_env_logger::formatted_builder();
    //builder.filter(Some("Chomusuke"), LevelFilter::Info);
    builder.filter(Some("Chomusuke"), LevelFilter::max());
    builder.init();

    info!("Attempting login...");

    let token = include_str!("sensitive_data/token.txt");

    info!("Chomusuke is starting!");

    let mut client = match Client::new(token, Handler) {
        Ok(info) => info,
        Err(why) => panic!("Error creating client!"),
    };

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("cs!")) //.owners(owners)
        .group(&GENERAL_GROUP)
        //.group(&MEMES_GROUP)
    );

    if let Err(why) = client.start_autosharded() {
        error!("Client error: {:?}", why);
        panic!("Zoinks");
    }
}
