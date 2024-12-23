mod credential;
mod robots;
mod utils;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn set_credential(
  service_name: String,
  author_name: String,
  password: String,
) -> napi::Result<()> {
  credential::set_password(service_name, author_name, password)
    .map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn get_credential(service_name: String, author_name: String) -> napi::Result<String> {
  credential::get_password(service_name, author_name)
    .map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn remove_template_engines_syntax(text: String) -> napi::Result<String> {
  let s = utils::remove_template_engines_syntax(&text);
  Ok(s)
}

#[napi]
pub fn format_path(path: String, content_type: String) -> napi::Result<String> {
  let s = utils::format_path(&path, &content_type);
  Ok(s)
}

#[napi]
pub fn generate_robots(noindex: Option<bool>, content_type: String) -> napi::Result<String> {
  let noindex = noindex.unwrap_or(false);
  let s = robots::generate_robots(noindex, &content_type);
  Ok(s.to_string())
}
