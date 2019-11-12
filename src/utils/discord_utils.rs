use log::{debug, error, info, trace, warn};

use regex::Regex;

use serenity::model::{
    user::{
        User,
    },
    channel::{
        Message,
    },
    id::{
        GuildId,
        UserId,
    },
    guild::{
        Member
    },
};
use serenity::utils::Colour;
use serenity::cache::CacheRwLock;
use serenity::prelude::Context;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
    },
};
use serenity::Result as SerenityResult;


pub static DEFAULT_COLOUR : Colour = Colour::from_rgb(132, 75, 90);

pub fn find_member<C>(target: &String, guild_id: &GuildId, cache: &C) -> Option<Member>
    where C: AsRef<CacheRwLock> {
    let mentionRegex1 = Regex::new(r"<@\d+>").unwrap();
    let mentionRegex2 = Regex::new(r"<@!\d+>").unwrap();

    // Check for an user mention
    let mut offset = 0;
    if(mentionRegex1.is_match(target)) {
        offset = "<@".len();
    } else if(mentionRegex2.is_match(target)) {
        offset = "<@!".len();
    }

    let id : String = if(offset > 0) {
        (&target[offset..target.len()-1]).to_string()
    }else{
        // No mention provided, use target as user id
        target.clone()
    };

    let memberId = id.parse::<u64>();
    if let Err(e) = memberId {
        return None;
    }

    return cache.as_ref().read().member(guild_id, memberId.unwrap());
}

pub fn check_message(msg: SerenityResult<Message>) {
    if let Err(why) = msg {
        error!("An error occured while sending a message : {:?}", why);
    }
}
