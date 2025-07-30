use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::command;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiRequest {
    pub name: String,
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub body: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Collection {
    pub workspace: String,
    pub requests: Vec<ApiRequest>,
}

#[command]
pub fn load_collection() -> Result<Collection, String> {
    let path = PathBuf::from("src-tauri/collection.yaml");
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_yaml::from_str(&content).map_err(|e| e.to_string())
}

#[command]
#[tokio::main]
pub async fn send_request(req: ApiRequest) -> Result<String, String> {
    let client = reqwest::Client::new();
    let method = req.method.to_uppercase();

    let builder = match method.as_str() {
        "GET" => client.get(&req.url),
        "POST" => client.post(&req.url),
        "PUT" => client.put(&req.url),
        "DELETE" => client.delete(&req.url),
        _ => return Err("Unsupported HTTP method".to_string()),
    };

    let resp = if let Some(body) = req.body {
        builder.body(body).send().await
    } else {
        builder.send().await
    };

    match resp {
        Ok(r) => r.text().await.map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
