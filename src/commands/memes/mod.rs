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
    commands: [uwuize, pat, hug, slap, tickle, kiss, poke],
});
