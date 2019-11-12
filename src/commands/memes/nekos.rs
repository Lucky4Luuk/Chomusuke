use log::{debug, error, info, trace, warn};

use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
    },
};

use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResult {
    url: String
}

fn api_call(endpoint: String) -> Result<String, &'static str> {
    let mut request = match reqwest::get(&(format!("https://nekos.life/api/v2/img/{}", endpoint))) {
        Ok(req) => req,
        Err(why) => {
            error!("Failed to request API: {}", why);
            return Err("Rip");
        }
    };
    let data: ApiResult = request.json().unwrap(); //No need to check for errors here, as this should either always work or never work
    Ok(data.url)
}

#[command]
pub fn slap(ctx: &mut Context, msg: &Message) -> CommandResult {
    let url = api_call("slap".to_string())?;
    msg.channel_id.say(ctx, &url)?;
    Ok(())
}
