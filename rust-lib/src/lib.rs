mod credential;
mod external_link;
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
pub fn external_link(data: String, base_url: String) -> napi::Result<String> {
  let result = external_link::replace_external_link(&data, &base_url);
  Ok(result)
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
pub fn log_debug(message: String) -> napi::Result<()> {
  init_logger();
  log::debug!("{}", message);
  Ok(())
}

#[napi]
pub fn set_log_level(level: String) -> napi::Result<()> {
  logger::set_log_level(&level).map_err(|e| napi::Error::from_reason(e))
}

#[napi]
pub fn get_log_level() -> napi::Result<String> {
  Ok(logger::get_log_level())
}

#[napi]
pub fn is_valid_image(source: String) -> napi::Result<bool> {
  // TODO: move somewhere
  match image_validator::is_valid(&source) {
    Ok(result) => match result {
      image_validator::ValidationResult::Valid { reason } => {
        match reason {
          image_validator::ValidReason::HasExifNoGps => {
            log_warn(format!("{}: has EXIF", source))?;
          }
          image_validator::ValidReason::NoExifData => {
            log_debug(format!("{}: no EXIF", source))?;
          }
          image_validator::ValidReason::BlankExifValues => {
            log_warn(format!("{}: EXIF contains blank values", source))?;
          }
        }
        Ok(true)
      }
      image_validator::ValidationResult::Invalid { reason, gps_data } => {
        match reason {
          image_validator::InvalidReason::GpsInfoFound => {
            log_error(format!("{}: has GPS info", source))?;
            if let Some(json_data) = gps_data {
              println!("{}", json_data);
            }
          }
          image_validator::InvalidReason::InvalidFormat(msg) => {
            log_error(format!("{}: Invalid file format - {}", source, msg))?;
          }
          image_validator::InvalidReason::FileTooLarge(msg) => {
            log_error(format!(
              "{}: File is too large to process - {}",
              source, msg
            ))?;
          }
          image_validator::InvalidReason::ExifError(msg) => {
            log_error(format!("{}: EXIF reading failed - {}", source, msg))?;
          }
        }
        Ok(false)
      }
      image_validator::ValidationResult::Skipped { reason } => {
        match reason {
          image_validator::SkipReason::SkippedExtension => {
            log_warn(format!("asset validation skipped - : {}", source))?;
          }
        }
        Ok(true)
      }
    },
    Err(e) => Err(napi::Error::from_reason(e)),
  }
}
