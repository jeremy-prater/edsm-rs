use anyhow::Result;
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;

pub mod systems;
pub mod system;

async fn do_api_call(url: &str, request: String) -> Result<Value> {
    let dump = std::env::var("DUMP_JSON").is_ok();
    if dump {
        println!("{} : {}", url, request);
    }
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(request)
        .send()
        .await?;

    let response_json = response.text().await?;

    if dump {
        println!("{}", response_json);
    }

    Ok(serde_json::from_str::<Value>(&response_json)?)
}