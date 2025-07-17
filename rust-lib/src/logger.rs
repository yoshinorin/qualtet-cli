use colored::*;
use env_logger::Builder;
use log::info;
use std::{io::Write, sync::Once};

static INIT: Once = Once::new();

pub fn init() {
  INIT.call_once(|| {
    Builder::new()
      .format(|buf, record| {
        let level = match record.level() {
          log::Level::Info => "INFO".green(),
          log::Level::Warn => "WARN".yellow(),
          log::Level::Error => "ERROR".red(),
          _ => "DEBUG".normal(),
        };
        writeln!(buf, "{}: {}", level, record.args())
      })
      .target(env_logger::Target::Stdout)
      .filter_level(log::LevelFilter::Info)
      .init();

    info!("Logger initialized.");
  });
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
}
