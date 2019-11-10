use serenity::framework::standard::{
    macros::{
        group,
    },
};

pub mod ping;
use ping::*;

pub mod avatar;
use avatar::*;

pub mod status;
use status::*;

group!({
    name: "general",
    options: {},
    commands: [ping, avatar, status],
});
