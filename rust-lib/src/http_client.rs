use reqwest::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde_json::Value;

pub async fn http_get(base_url: &str, path: &str, token: Option<&str>) -> Result<String, String> {
  let client = create_client(token)?;
  let url = format!(
    "{}/{}",
    base_url.trim_end_matches('/'),
    path.trim_start_matches('/')
  );

  client
    .get(&url)
    .send()
    .await
    .map_err(|e| format!("HTTP GET request failed: {}", e))?
    .text()
    .await
    .map_err(|e| format!("Failed to read response: {}", e))
}

pub async fn http_post(
  base_url: &str,
  path: &str,
  data: &str,
  token: Option<&str>,
) -> Result<String, String> {
  let client = create_client(token)?;
  let url = format!(
    "{}/{}",
    base_url.trim_end_matches('/'),
    path.trim_start_matches('/')
  );

  let json_data: Value =
    serde_json::from_str(data).map_err(|e| format!("Invalid JSON data: {}", e))?;

  client
    .post(&url)
    .json(&json_data)
    .send()
    .await
    .map_err(|e| format!("HTTP POST request failed: {}", e))?
    .text()
    .await
    .map_err(|e| format!("Failed to read response: {}", e))
}

pub async fn http_delete(
  base_url: &str,
  path: &str,
  token: Option<&str>,
) -> Result<String, String> {
  let client = create_client(token)?;
  let url = format!(
    "{}/{}",
    base_url.trim_end_matches('/'),
    path.trim_start_matches('/')
  );

  client
    .delete(&url)
    .send()
    .await
    .map_err(|e| format!("HTTP DELETE request failed: {}", e))?
    .text()
    .await
    .map_err(|e| format!("Failed to read response: {}", e))
}

fn create_client(token: Option<&str>) -> Result<Client, String> {
  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

  if let Some(token) = token {
    let auth_value = format!("Bearer {}", token);
    headers.insert(
      AUTHORIZATION,
      HeaderValue::from_str(&auth_value)
        .map_err(|e| format!("Invalid authorization header: {}", e))?,
    );
  }

  Client::builder()
    .default_headers(headers)
    .build()
    .map_err(|e| format!("Failed to create HTTP client: {}", e))
}
