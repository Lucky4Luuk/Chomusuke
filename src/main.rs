use log::{debug, error, info, trace, warn, Level, LevelFilter};

use std::{env, thread, time::Duration};

use serenity::{
    model::gateway::Ready,
    prelude::*,
    client::Client,
};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group,
    },
};

pub mod commands;
use commands::{
    general,
};

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            //Note: Array index 0 is 0-indexed, while index 1 is 1-indexed. This follows Discord's behaviour.
            info!(
                "{} is connected on shard {}/{}",
                ready.user.name,
                shard[0] + 1,
                shard[1],
            );
        }
    }
}

fn main() {
    let mut builder = pretty_env_logger::formatted_builder();
    builder.filter(Some("FukimageTokoyami"), LevelFilter::Info);
    builder.init();

    info!("Attempting login...");

    let token = include_str!("sensitive_data/token.txt");

    info!("Fukimage Tokoyami is starting!");

    let mut client = Client::new(&token, Handler).expect("Error creating the client!");

    //Here we clone a lock to the Shard Manager, and then move it into a new thread.
    //The thread will unlock the manager and print shards' status on a loop.
    let manager = client.shard_manager.clone();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(30));

            let lock = manager.lock();
            let shard_runners = lock.runners.lock();

            for (id, runner) in shard_runners.iter() {
                info!(
                    "Shard ID{} is {} with a latency of {:?}",
                    id,
                    runner.stage,
                    runner.latency,
                );
            }
        }
    });

    //Start `n` shards. Note that there is about a 5 second ratelimit period between starting threads
    if let Err(why) = client.start_shards(20) {
        error!("Client error: {:?}", why);
    }
}
