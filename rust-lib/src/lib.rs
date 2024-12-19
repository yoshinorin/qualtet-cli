mod credential;
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
