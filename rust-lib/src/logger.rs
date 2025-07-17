use colored::*;
use env_logger::Builder;
use log::{LevelFilter, info};
use std::{
  io::Write,
  sync::{Mutex, Once},
};

static INIT: Once = Once::new();
static LOG_LEVEL: Mutex<Option<LevelFilter>> = Mutex::new(Some(LevelFilter::Info));

pub fn init() {
  init_with_level(None);
}

pub fn init_with_level(level: Option<Option<LevelFilter>>) {
  INIT.call_once(|| {
    let filter_level = if let Some(level) = level {
      {
        let mut current_level = LOG_LEVEL.lock().unwrap();
        *current_level = level;
      }
      level.unwrap_or(LevelFilter::Off)
    } else {
      if let Some(current) = *LOG_LEVEL.lock().unwrap() {
        current
      } else {
        LevelFilter::Off
      }
    };

    Builder::new()
      .format(|buf, record| {
        let level = match record.level() {
          log::Level::Info => "INFO".green(),
          log::Level::Warn => "WARN".yellow(),
          log::Level::Error => "ERROR".red(),
          log::Level::Debug => "DEBUG".blue(),
          _ => "INFO".normal(),
        };
        writeln!(buf, "{}: {}", level, record.args())
      })
      .target(env_logger::Target::Stdout)
      .filter_level(filter_level)
      .init();

    info!("Logger initialized with level: {:?}", filter_level);
  });
}

pub fn set_log_level(level: &str) -> Result<(), String> {
  let log_level = match level.to_lowercase().as_str() {
    "error" => Some(LevelFilter::Error),
    "warn" => Some(LevelFilter::Warn),
    "info" => Some(LevelFilter::Info),
    "debug" => Some(LevelFilter::Debug),
    "off" => None,
    _ => {
      return Err(format!(
        "Invalid log level: {}. Valid levels are: error, warn, info, debug, off",
        level
      ));
    }
  };

  {
    let mut current_level = LOG_LEVEL.lock().unwrap();
    *current_level = log_level;
  }
  Ok(())
}

#[allow(dead_code)]
pub fn get_log_level_option() -> Option<LevelFilter> {
  *LOG_LEVEL.lock().unwrap()
}

pub fn get_log_level() -> String {
  let level = *LOG_LEVEL.lock().unwrap();
  match level {
    Some(LevelFilter::Error) => "error".to_string(),
    Some(LevelFilter::Warn) => "warn".to_string(),
    Some(LevelFilter::Info) => "info".to_string(),
    Some(LevelFilter::Debug) => "debug".to_string(),
    None => "off".to_string(),
    _ => "debug".to_string(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use log::{debug, error, info, warn};

  #[test]
  fn test_info_log() {
    init();
    info!("This is an info message");
  }

  #[test]
  fn test_warn_log() {
    init();
    warn!("This is a warning message");
  }

  #[test]
  fn test_error_log() {
    init();
    error!("This is an error message");
  }

  #[test]
  fn test_debug_log() {
    init();
    debug!("This is a debug message");
  }

  #[test]
  fn test_set_log_level() {
    // Test valid log levels
    assert!(set_log_level("error").is_ok());
    assert_eq!(get_log_level(), "error");

    assert!(set_log_level("warn").is_ok());
    assert_eq!(get_log_level(), "warn");

    assert!(set_log_level("info").is_ok());
    assert_eq!(get_log_level(), "info");

    assert!(set_log_level("debug").is_ok());
    assert_eq!(get_log_level(), "debug");

    assert!(set_log_level("off").is_ok());
    assert_eq!(get_log_level(), "off");

    // Test case insensitive
    assert!(set_log_level("ERROR").is_ok());
    assert_eq!(get_log_level(), "error");

    // Test invalid log level
    assert!(set_log_level("invalid").is_err());
  }

  #[test]
  fn test_option_api() {
    // Test Option-based API through string interface
    assert!(set_log_level("error").is_ok());
    assert_eq!(get_log_level_option(), Some(LevelFilter::Error));
    assert_eq!(get_log_level(), "error");

    assert!(set_log_level("off").is_ok());
    assert_eq!(get_log_level_option(), None);
    assert_eq!(get_log_level(), "off");

    // Test other levels
    assert!(set_log_level("warn").is_ok());
    assert_eq!(get_log_level_option(), Some(LevelFilter::Warn));

    assert!(set_log_level("debug").is_ok());
    assert_eq!(get_log_level_option(), Some(LevelFilter::Debug));
  }
}
