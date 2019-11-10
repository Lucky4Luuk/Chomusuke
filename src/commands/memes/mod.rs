use serenity::framework::standard::{
    macros::{
        group,
    },
};

pub mod uwuize;
use uwuize::*;

group!({
    name: "memes",
    options: {},
    commands: [uwuize],
});
