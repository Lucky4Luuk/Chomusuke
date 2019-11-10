use serenity::framework::standard::{
    macros::{
        group,
    },
};

pub mod ping;
use ping::*;

pub mod avatar;
use avatar::*;

group!({
    name: "general",
    options: {},
    commands: [ping, avatar],
});
