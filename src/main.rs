#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    eprintln!("*** 開始 ***");
    let key = match env::var("TRN_API_KEY") {
        Ok(val) => val,
        Err(_) => "local".to_string(),
    };
    println!("{:#?}",key);
    let url = "https://public-api.tracker.gg/v2/apex/standard/profile/origin/NsY_KURI";
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

    for (key,value) in obj.iter() {
        println!("{}\t{}",key,value);
    }

    eprintln!("*** 終了 ***");
    Ok(())
}
