use serenity::framework::standard::{
    macros::{
        group,
    },
};

pub mod checks;
use checks::*;

pub mod config;
use config::*;

group!({
    name: "admin",
    options: {
        // Limit all commands to be guild-restricted.
        only_in: "guilds",
        // Adds checks that need to be passed.
        checks: [Admin],
    },
    commands: [set_welcome_channel],
});
