use log::{debug, error, info, trace, warn};

use chrono::Utc;

use regex::Regex;

use rand::distributions::{IndependentSample, Range};
use rand::Rng;

use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
    },
};

use crate::utils::discord_utils;

pub fn replace(string: &String, replacement: &Vec<&str>, regex: &Regex, chance: &f32) -> String {
    let range = Range::new(0f32, 1f32);
    let mut result : String = Default::default();

    let mut prev_end = 0;
    for mat in regex.find_iter(string) {
        result.push_str(&string[prev_end .. mat.start()]);

        if range.ind_sample(&mut rand::thread_rng()) < *chance {
            let insert = &replacement[rand::thread_rng().gen::<usize>() % replacement.len()];
            let value : String = (*insert).into();
            result.push_str(&value);
        } else {
            result.push_str(mat.as_str());
        }
        prev_end = mat.end();
    }
    result.push_str(&string[prev_end .. string.len()]);

    return result;
}

pub fn caps_spree(string: &String, chance: &f32, min_size: &usize, max_size: &usize) -> String {
    let mut result : String = Default::default();
    let range = Range::new(0f32, 1f32);
    let mut curr_caps_spree : usize = 0;

    for c in string.chars() {
        if curr_caps_spree > 0 {
            curr_caps_spree -= 1;
            for upper in c.to_uppercase() {
                result.push(upper);
            }
        }else{
            if range.ind_sample(&mut rand::thread_rng()) < *chance {
                curr_caps_spree = rand::thread_rng().gen::<usize>() % (max_size - min_size) + min_size;
            }
            result.push(c);
        }
    }

    return result;
}

const emoticons : &[&str] = &[
    " OwO ",
    " UwU ",
    " 𝕆𝕨𝕆 ",
    "  (。O⁄ ⁄ω⁄ ⁄ O。) ",
    " >//< ",
    " _OwO_ ",
    " :3 ",
    " ( o͡ ꒳ o͡ ) ",
    " * 𝓌𝒶𝓉𝓈 𝒹𝒾𝓈? * ",
    " * 𝓃𝓊𝓏𝓏𝓁𝑒𝓈 𝓎𝑜𝓊 * ",
    " ***rawr*** ",
    " ~~rawrr~~ ",
    " ✼ ҉ (O꒳O) ҉ ✼ ",
    " ✧･ﾟ: *✧･ﾟ:*(OwO) ",
];

#[command]
#[aliases("owoize", "uwu", "owo")]
pub fn uwuize(ctx: &mut Context, msg: &Message) -> CommandResult {
    let has_messages = msg.channel_id.messages(&ctx.http, |r| {r.before(msg.id).limit(1)});

    // Check if there was a message before
    if let Err(e) = has_messages {
        discord_utils::check_message(msg.channel_id.say(ctx, "Sorry, I can\'t find a message to UwUize"));
        return Ok(());
    }
    let messages = has_messages.unwrap();
    if messages.len() == 0 {
        discord_utils::check_message(msg.channel_id.say(ctx, "Sorry, I can\'t find a message to UwUize"));
        return Ok(());
    }

    let prev_message = &messages[0];
    let mut uwuized = prev_message.content.clone();
    uwuized = replace(&uwuized, &vec!["ww"], &Regex::new(r"w").unwrap(), &0.1f32);
    uwuized = replace(&uwuized, &vec!["w"], &Regex::new(r"[lr]").unwrap(), &1f32);
    uwuized = replace(&uwuized, &vec!["W"], &Regex::new(r"[LR]").unwrap(), &1f32);
    uwuized = replace(&uwuized, &vec!["ss"], &Regex::new(r"s").unwrap(), &0.1f32);
    uwuized = replace(&uwuized, &vec!["yy"], &Regex::new(r"y").unwrap(), &0.1f32);
    uwuized = replace(&uwuized, &vec!["yy "], &Regex::new(r"y\s").unwrap(), &0.25f32);
    uwuized = replace(&uwuized, &vec!["yy "], &Regex::new(r"y\s").unwrap(), &0.25f32);
    uwuized = replace(&uwuized, &vec!["y "], &Regex::new(r"\s").unwrap(), &0.125f32);
    uwuized = replace(&uwuized, &vec!["nya"], &Regex::new(r"na").unwrap(), &1f32);
    uwuized = replace(&uwuized, &vec!["nyo"], &Regex::new(r"no").unwrap(), &1f32);
    uwuized = replace(&uwuized, &vec!["nyu"], &Regex::new(r"nu").unwrap(), &1f32);
    uwuized = replace(&uwuized, &vec!["nyi"], &Regex::new(r"ni").unwrap(), &1f32);
    uwuized = replace(&uwuized, &vec!["nye"], &Regex::new(r"ne").unwrap(), &1f32);
    uwuized = replace(&uwuized, &vec![",, ", ",,, "], &Regex::new(r"\s").unwrap(), &0.1f32);
    uwuized = replace(&uwuized, &vec![" "], &Regex::new(r".").unwrap(), &0.05f32);
    uwuized = replace(&uwuized, &vec![" "], &Regex::new(r"\s").unwrap(), &0.5f32);
    uwuized = replace(&uwuized, &emoticons.to_vec(), &Regex::new(r"\.\s").unwrap(), &0.8f32);
    uwuized = replace(&uwuized, &emoticons.to_vec(), &Regex::new(r",\s").unwrap(), &0.1f32);
    uwuized = replace(&uwuized, &emoticons.to_vec(), &Regex::new(r"!\s").unwrap(), &0.8f32);
    uwuized = replace(&uwuized, &emoticons.to_vec(), &Regex::new(r"\?\s").unwrap(), &0.8f32);
    uwuized = replace(&uwuized, &emoticons.to_vec(), &Regex::new(r"$").unwrap(), &1f32);

    uwuized = caps_spree(&uwuized, &0.025, &5, &20);

    let author = match ctx.cache.read().guild_channel(prev_message.channel_id) {
        Some(channel) => ctx.cache.read().member(channel.read().guild_id, prev_message.author.id),
        None => None
    };

    discord_utils::check_message(msg.channel_id.say(ctx, uwuized));

    Ok(())
}
