use serenity::framework::standard::{
    macros::{
        group,
    },
};

pub mod twitter;
use twitter::*;

pub mod youtube;
use youtube::*;

group!({
    name: "webhooks",
    options: {},
    commands: [twitter, youtube],
});
