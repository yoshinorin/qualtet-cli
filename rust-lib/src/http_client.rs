use std::error::Error;

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

  let response = client
    .get(&url)
    .send()
    .await
    .map_err(|e| format!("HTTP GET request failed: {} (source: {:?})", e, e.source()))?;

  handle_response(response, "GET").await
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

  let response = client
    .post(&url)
    .json(&json_data)
    .send()
    .await
    .map_err(|e| format!("HTTP POST request failed: {} (source: {:?})", e, e.source()))?;

  handle_response(response, "POST").await
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

  let response = client.delete(&url).send().await.map_err(|e| {
    format!(
      "HTTP DELETE request failed: {} (source: {:?})",
      e,
      e.source()
    )
  })?;

  handle_response(response, "DELETE").await
}

async fn handle_response(response: reqwest::Response, method: &str) -> Result<String, String> {
  let status = response.status();
  let body = response
    .text()
    .await
    .map_err(|e| format!("Failed to read response: {}", e))?;

  if status.is_client_error() || status.is_server_error() {
    return Err(format!(
      "HTTP {} failed with status {}: {}",
      method,
      status,
      format_error_body(&body)
    ));
  }

  Ok(body)
}

fn format_error_body(body: &str) -> String {
  match serde_json::from_str::<Value>(body) {
    Ok(json) => serde_json::to_string_pretty(&json).unwrap_or_else(|_| body.to_string()),
    Err(_) => body.to_string(),
  }
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
