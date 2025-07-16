mod credential;
mod image_validator;
mod logger;
mod markdown;
mod robots;
mod utils;

fn init_logger() {
  logger::init();
}

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
pub fn should_skip_paths(path: String, skip_paths: Vec<String>) -> napi::Result<bool> {
  let skip_paths: Vec<&str> = skip_paths.iter().map(|s| s.as_str()).collect();
  let b = utils::should_skip_paths(&path, &skip_paths);
  Ok(b)
}

#[napi]
pub fn generate_robots(noindex: Option<bool>, content_type: String) -> napi::Result<String> {
  let noindex = noindex.unwrap_or(false);
  let s = robots::generate_robots(noindex, &content_type);
  Ok(s.to_string())
}

#[napi]
pub fn render_markdown(input: String) -> napi::Result<String> {
  let s = markdown::render(&input);
  Ok(s)
}

#[napi]
pub fn log_info(message: String) -> napi::Result<()> {
  init_logger();
  log::info!("{}", message);
  Ok(())
}

#[napi]
pub fn log_warn(message: String) -> napi::Result<()> {
  init_logger();
  log::warn!("{}", message);
  Ok(())
}

#[napi]
pub fn log_error(message: String) -> napi::Result<()> {
  init_logger();
  log::error!("{}", message);
  Ok(())
}

#[napi]
pub fn is_valid_image(source: String) -> napi::Result<bool> {
  image_validator::is_valid(&source).map_err(|e| napi::Error::from_reason(e))
}
