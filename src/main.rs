use reqwest::{Response, Error};
use serde_json::json;

#[tokio::main]
async fn main() {
	let client = reqwest::Client::new();

	let body = json!({
		"page": {
			"pdf": {
				"printBackground": true,
				"format": "A4",
			},
			"goto": {
				"url": "https://doppio.sh/",
			},
		}
	});

	let res: Result<Response, Error> = client.post("https://api.doppio.sh/v1/render/pdf/sync")
		.bearer_auth("<YOUR_API_KEY>")
		.json(&body)
		.send()
		.await;

	match res {
		Ok(res) => {
			println!("Status: {}", res.status());
			println!("Body: {:?}", res.text().await);
		}
		Err(e) => {
			println!("Error: {}", e);
		}
	}
}