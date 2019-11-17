use log::{debug, error, info, trace, warn, Level, LevelFilter};

use std::{env, thread, time::Duration, sync::Arc, collections::HashSet};

use serenity::{
    model::{
        user::CurrentUser,
        gateway::Ready,
        event::{
            ResumedEvent,
        },
        id::GuildId,
        guild::{
            Member,
        },
        channel::*,
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
use serenity::model::event::GuildMemberAddEvent;

use postgres::{
    Connection,
    TlsMode,
    params::ConnectParams,
    types::*,
};

pub mod commands;
use commands::{
    meta::help::MY_HELP,
    general::GENERAL_GROUP,
    memes::MEMES_GROUP,
};

pub mod utils;

struct ConnectionKey;
impl TypeMapKey for ConnectionKey {
    type Value = Arc<Mutex<Connection>>;
}

use regex::Regex;

struct Handler;

struct Handler;
impl EventHandler for Handler {
    //Bot is fully connected
    fn ready(&self, _: Context, ready: Ready) {
        info!("Succesfully connected as {}!", ready.user.tag());
    }

    //Event for reconnecting
    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Reconnected!");
    }

    //Member joined
    fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, new_member: Member) {
        debug!("Member joined!");

        //We don't add users to the DB here, as that would needlessly clog up the DB with users who don't talk
    }

    fn message(&self, ctx: Context, message: Message) {
        //THIS DOESNT BLOCK COMMANDS FROM WORKING
        //THANKS SERENITY DEVS
        // debug!("MESSAGE! {}", message.content);
        let data = ctx.data.read();
        let conn = data.get::<ConnectionKey>().expect("Failed to read DB connection").lock();
        let query_rows = &conn.query(&format!("SELECT COUNT(1) FROM Users WHERE UserID = {};", message.author.id), &[]).expect("DB query failed");
        let member_data_count: i64 = query_rows.get(0).get("count");
        // debug!("Hmm? {:?}", member_data_count);
        if member_data_count == 0 {
            //Member doesn't exist yet, so add it to the DB
            if let Err(why) = conn.execute(&format!("INSERT INTO Users (UserID) VALUES ({UserID})", UserID = message.author.id), &[]) {
                error!("{}", why);
            } else {
                debug!("Added new user to the database!");
            }
        }
    }
}

//TODO: Move this to utilities
fn trim_trailing(s: String) -> String {
    let mut result = s;
    let len_withoutcrlf = result.trim_end().len();
    result.truncate(len_withoutcrlf);
    result
}

fn main() {
    let mut builder = pretty_env_logger::formatted_builder();
    builder.filter(Some("Chomusuke"), LevelFilter::max());
    builder.init();

    //TODO: Store all this data in a single JSON file
    let db_address = trim_trailing(include_str!("sensitive_data/db_address.txt").to_string());
    let db_passwd = trim_trailing(include_str!("sensitive_data/db_passwd.txt").to_string());

    info!("Connection to database `{}:5432/Chomusuke`!", db_address);

    let db_params = ConnectParams::builder()
        .user("postgres", Some(&db_passwd))
        .database("Chomusuke")
        .build(postgres::params::Host::Tcp(db_address));

    let conn = match Connection::connect(db_params, TlsMode::None) {
        Ok(conn) => conn,
        Err(why) => {
            error!("Connection to database failed! Reason: `{}`", why);
            panic!("Failed to connect to database!");
        }
    };

    info!("Succesfully connected to database! Welcome!");
    info!("Attempting login...");

    let token = include_str!("sensitive_data/token.txt");

    info!("Chomusuke is starting!");

    let mut client = match Client::new(&token, Handler) {
        Ok(info) => info,
        Err(why) => panic!("Error creating client!"),
    };

    // Insert data into the client. Retrieve with <context>.data.read().get::<ConnectionKey>().expect("Failed to read DB connection");
    {
        let mut data = client.data.write();

        data.insert::<ConnectionKey>(Arc::new(Mutex::new(conn)));
    }

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
        .group(&MEMES_GROUP)
    );

    //Start bot autosharded. I really hope this holds up in the future, but it's kinda hard
    //to test sharding without having access to more than 2500 guilds lmao
    if let Err(why) = client.start_autosharded() {
        error!("Client error: {:?}", why);
    }
}
