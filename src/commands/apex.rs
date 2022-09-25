use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Apex情報取得"]
async fn apex(ctx: &Context, msg: &Message) -> CommandResult {
    use reqwest::Client;
    use std::env;

    let key = match env::var("APEX_API_KEY") {
        Ok(val) => val,
        Err(_) => "local".to_string(),
    };

    // TODO コマンド引数を取れるようにする
    let url = "https://api.mozambiquehe.re/predator?auth=";

    let client = Client::new();

    let resp = client
        .get(String::from(url) + &key)
        .header(reqwest::header::CONTENT_TYPE, "application_json")
        .send()
        .await
        .expect("Err creating client");

    let body = resp.text().await?;

    let json: serde_json::Value = serde_json::from_str(&body)?;

    let obj = json.as_object().unwrap();

    msg.channel_id
        .say(
            &ctx.http,
            format!("PCのプレデターボーダーは{}RPです。", obj["RP"]["PC"]["val"]),
        )
        .await?;

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "PS4のプレデターボーダーは{}RPです。",
                obj["RP"]["PS4"]["val"]
            ),
        )
        .await?;

    Ok(())
}
