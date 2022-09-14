#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    eprintln!("*** req ***");
    let key = match env::var("TRN_API_KEY") {
        Ok(val) => val,
        Err(_) => "local".to_string(),
    };
    // TODO コマンド引数を取れるようにする
    let url = "https://public-api.tracker.gg/v2/apex/standard/profile/origin/CR_Ras_LOG";
    let client = reqwest::Client::new();
    let resp = client.get(url)
        .header(reqwest::header::CONTENT_TYPE,"application_json")
        .header("TRN-Api-Key", &key)
        .send()
        .await?;

    // テスト用
    // println!("{:#?}", resp);

    let body = resp.text().await? ;

    let json: serde_json::Value = serde_json::from_str(&body)?;

    let obj = json.as_object().unwrap();

    println!("{:#?}",obj["data"]["segments"][0]["stats"]["rankScore"]);
    // for (key,value) in obj.iter() {
    //     println!("{}\t{}",key,value);
    // }

    Ok(())
}
