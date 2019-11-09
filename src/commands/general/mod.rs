use serenity::framework::standard::{
    macros::{
        group,
    },
};

pub mod ping;
use ping::*;

group!({
    name: "general",
    options: {},
    commands: [ping],
});
