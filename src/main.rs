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
};

group!({
    name: "general",
    options: {},
    commands: [ping],
});

// struct ShardManagerContainer;

// impl TypeMapKey for ShardManagerContainer {
//     type Value = Arc<Mutex<ShardManager>>;
// }

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

    // {
    //     let mut data = client.data.write();
    //     data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    // }
    //
    // let owners = match client.cache_and_http.http.get_current_application_info() {
    //     Ok(info) => {
    //         let mut set = HashSet::new();
    //         set.insert(info.owner.id);
    //
    //         set
    //     },
    //     Err(why) => panic!("Couldn't get application info: {:?}", why),
    // };

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("cs")) //.owners(owners)
        .group(&GENERAL_GROUP));

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
        panic!("Zoinks");
    }
}
