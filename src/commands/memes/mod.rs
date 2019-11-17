use serenity::framework::standard::{
    macros::{
        group,
    },
};

pub mod uwuize;
use uwuize::*;

pub mod nekos;
use nekos::*;

group!({
    name: "memes",
    options: {},
    commands: [uwuize, feed, cuddle, pat, hug, slap, tickle, kiss, poke, wallpaper, ngif, meow, gecg, kemonomimi, gasm, rand_avatar, lizard, waifu, neko, fox_girl, baka, smug, woof],
});
