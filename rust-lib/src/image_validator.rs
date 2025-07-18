use serde_json::{Map, Value, json};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum GpsDetectionResult {
  Found,
  NotFound,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
  Valid {
    reason: ValidReason,
  },
  Invalid {
    reason: InvalidReason,
    gps_data: Option<String>,
  },
  Skipped {
    reason: SkipReason,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidReason {
  HasExifNoGps,
  NoExifData,
  BlankExifValues,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidReason {
  GpsInfoFound,
  InvalidFormat(String),
  FileTooLarge(String),
  ExifError(String),
}

/// Reasons why file validation was skipped
#[derive(Debug, Clone, PartialEq)]
pub enum SkipReason {
  /// File extension is in skip list
  SkippedExtension,
}

const SKIP_EXTENSIONS: &[&str] = &[
  ".md", ".mermaid", ".mp3", ".mp4", ".webm", ".pptx", ".svg", ".txt", ".ico",
];

const GPS_TAGS: &[exif::Tag] = &[
  exif::Tag::GPSVersionID,
  exif::Tag::GPSLatitudeRef,
  exif::Tag::GPSLatitude,
  exif::Tag::GPSLongitudeRef,
  exif::Tag::GPSLongitude,
  exif::Tag::GPSAltitudeRef,
  exif::Tag::GPSAltitude,
  exif::Tag::GPSTimeStamp,
  exif::Tag::GPSSatellites,
  exif::Tag::GPSStatus,
  exif::Tag::GPSMeasureMode,
  exif::Tag::GPSDOP,
  exif::Tag::GPSSpeedRef,
  exif::Tag::GPSSpeed,
  exif::Tag::GPSTrackRef,
  exif::Tag::GPSTrack,
  exif::Tag::GPSImgDirectionRef,
  exif::Tag::GPSImgDirection,
  exif::Tag::GPSMapDatum,
  exif::Tag::GPSDestLatitudeRef,
  exif::Tag::GPSDestLatitude,
  exif::Tag::GPSDestLongitudeRef,
  exif::Tag::GPSDestLongitude,
  exif::Tag::GPSDestBearingRef,
  exif::Tag::GPSDestBearing,
  exif::Tag::GPSDestDistanceRef,
  exif::Tag::GPSDestDistance,
  exif::Tag::GPSProcessingMethod,
  exif::Tag::GPSAreaInformation,
  exif::Tag::GPSDateStamp,
  exif::Tag::GPSDifferential,
  exif::Tag::GPSHPositioningError,
];

/// Check if the file should skip validation based on its extension
fn should_skip_validation(source: &str) -> bool {
  SKIP_EXTENSIONS
    .iter()
    .any(|&ext| source.to_lowercase().ends_with(ext))
}

/// Check if EXIF data contains any GPS information
fn has_gps_information(exif_data: &exif::Exif) -> GpsDetectionResult {
  let has_gps = GPS_TAGS
    .iter()
    .any(|&tag| exif_data.get_field(tag, exif::In::PRIMARY).is_some());

  if has_gps {
    GpsDetectionResult::Found
  } else {
    GpsDetectionResult::NotFound
  }
}

/// Collect GPS data from EXIF and format as JSON string
fn collect_gps_data(exif_data: &exif::Exif, source: &str) -> Result<String, String> {
  let mut gps_data = Map::new();
  for &tag in GPS_TAGS {
    if let Some(field) = exif_data.get_field(tag, exif::In::PRIMARY) {
      gps_data.insert(
        format!("{:?}", tag),
        Value::String(field.display_value().to_string()),
      );
    }
  }

  let json_output = json!({
      "file": source,
      "gps": gps_data
  });

  serde_json::to_string_pretty(&json_output)
    .map_err(|e| format!("Failed to format GPS data as JSON: {}", e))
}

fn handle_exif_error(exif_err: exif::Error) -> ValidationResult {
  match exif_err {
    exif::Error::InvalidFormat(msg) => ValidationResult::Invalid {
      reason: InvalidReason::InvalidFormat(msg.to_string()),
      gps_data: None,
    },
    exif::Error::NotFound(_msg) => ValidationResult::Valid {
      reason: ValidReason::NoExifData,
    },
    exif::Error::BlankValue(_msg) => ValidationResult::Valid {
      reason: ValidReason::BlankExifValues,
    },
    exif::Error::TooBig(msg) => ValidationResult::Invalid {
      reason: InvalidReason::FileTooLarge(msg.to_string()),
      gps_data: None,
    },
    _ => ValidationResult::Invalid {
      reason: InvalidReason::ExifError(exif_err.to_string()),
      gps_data: None,
    },
  }
}

pub fn is_valid(source: &str) -> Result<ValidationResult, String> {
  if should_skip_validation(source) {
    return Ok(ValidationResult::Skipped {
      reason: SkipReason::SkippedExtension,
    });
  }

  let path = Path::new(source);
  if !path.exists() {
    return Err(format!("File not found: {}", source));
  }

  let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
  let mut reader = BufReader::new(file);

  // Read EXIF data
  match exif::Reader::new().read_from_container(&mut reader) {
    Ok(exif_data) => match has_gps_information(&exif_data) {
      GpsDetectionResult::Found => match collect_gps_data(&exif_data, source) {
        Ok(gps_json) => Ok(ValidationResult::Invalid {
          reason: InvalidReason::GpsInfoFound,
          gps_data: Some(gps_json),
        }),
        Err(_json_error) => Ok(ValidationResult::Invalid {
          reason: InvalidReason::GpsInfoFound,
          gps_data: None,
        }),
      },
      GpsDetectionResult::NotFound => Ok(ValidationResult::Valid {
        reason: ValidReason::HasExifNoGps,
      }),
    },
    Err(exif_err) => Ok(handle_exif_error(exif_err)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_should_skip_validation() {
    // Test skip extensions
    assert!(should_skip_validation("document.md"));
    assert!(should_skip_validation("diagram.mermaid"));
    assert!(should_skip_validation("audio.mp3"));
    assert!(should_skip_validation("video.mp4"));
    assert!(should_skip_validation("presentation.pptx"));
    assert!(should_skip_validation("icon.svg"));
    assert!(should_skip_validation("readme.txt"));

    // Test case insensitive
    assert!(should_skip_validation("document.MD"));
    assert!(should_skip_validation("video.MP4"));
    assert!(should_skip_validation("icon.SVG"));

    // Test non-skip extensions
    assert!(!should_skip_validation("image.jpg"));
    assert!(!should_skip_validation("photo.png"));
    assert!(!should_skip_validation("data.json"));
    assert!(!should_skip_validation("config.toml"));
  }

  #[test]
  fn test_handle_exif_error_types() {
    // Test different error types

    // Test InvalidFormat error - should return Invalid
    let invalid_format_err = exif::Error::InvalidFormat("Invalid JPEG");
    let result = handle_exif_error(invalid_format_err);
    match result {
      ValidationResult::Invalid {
        reason: InvalidReason::InvalidFormat(_),
        ..
      } => {}
      _ => panic!("Expected Invalid with InvalidFormat reason"),
    }

    // Test NotFound error - should return Valid with NoExifData
    let not_found_err = exif::Error::NotFound("No EXIF");
    let result = handle_exif_error(not_found_err);
    match result {
      ValidationResult::Valid {
        reason: ValidReason::NoExifData,
      } => {}
      _ => panic!("Expected Valid with NoExifData reason"),
    }

    // Test BlankValue error - should return Valid with BlankExifValues
    let blank_value_err = exif::Error::BlankValue("Empty value");
    let result = handle_exif_error(blank_value_err);
    match result {
      ValidationResult::Valid {
        reason: ValidReason::BlankExifValues,
      } => {}
      _ => panic!("Expected Valid with BlankExifValues reason"),
    }

    // Test TooBig error - should return Invalid with FileTooLarge
    let too_big_err = exif::Error::TooBig("File too large");
    let result = handle_exif_error(too_big_err);
    match result {
      ValidationResult::Invalid {
        reason: InvalidReason::FileTooLarge(_),
        ..
      } => {}
      _ => panic!("Expected Invalid with FileTooLarge reason"),
    }
  }

  #[test]
  fn test_skip_extensions() {
    // Test that skip extensions return Skipped result
    match is_valid("test.md").unwrap() {
      ValidationResult::Skipped {
        reason: SkipReason::SkippedExtension,
      } => {}
      _ => panic!("Expected Skipped result for .md file"),
    }

    match is_valid("test.svg").unwrap() {
      ValidationResult::Skipped {
        reason: SkipReason::SkippedExtension,
      } => {}
      _ => panic!("Expected Skipped result for .svg file"),
    }

    match is_valid("test.txt").unwrap() {
      ValidationResult::Skipped {
        reason: SkipReason::SkippedExtension,
      } => {}
      _ => panic!("Expected Skipped result for .txt file"),
    }

    // Test case insensitive extensions
    match is_valid("test.MD").unwrap() {
      ValidationResult::Skipped {
        reason: SkipReason::SkippedExtension,
      } => {}
      _ => panic!("Expected Skipped result for .MD file"),
    }

    match is_valid("test.SVG").unwrap() {
      ValidationResult::Skipped {
        reason: SkipReason::SkippedExtension,
      } => {}
      _ => panic!("Expected Skipped result for .SVG file"),
    }

    match is_valid("test.TXT").unwrap() {
      ValidationResult::Skipped {
        reason: SkipReason::SkippedExtension,
      } => {}
      _ => panic!("Expected Skipped result for .TXT file"),
    }

    match is_valid("test.Mp4").unwrap() {
      ValidationResult::Skipped {
        reason: SkipReason::SkippedExtension,
      } => {}
      _ => panic!("Expected Skipped result for .Mp4 file"),
    }
  }

  #[test]
  fn test_unsupported_image_formats() {
    // These should return ValidationResult indicating valid (no GPS found) or error
    // since these files don't exist, they should return an error
    assert!(is_valid("test.gif").is_err());
    assert!(is_valid("test.bmp").is_err());
    // .ico is in SKIP_EXTENSIONS, so it should be skipped
    match is_valid("test.ico").unwrap() {
      ValidationResult::Skipped {
        reason: SkipReason::SkippedExtension,
      } => {}
      _ => panic!("Expected Skipped result for .ico file"),
    }

    // Test case insensitive
    assert!(is_valid("test.GIF").is_err());
    assert!(is_valid("test.BMP").is_err());
    match is_valid("test.ICO").unwrap() {
      ValidationResult::Skipped {
        reason: SkipReason::SkippedExtension,
      } => {}
      _ => panic!("Expected Skipped result for .ICO file"),
    }
  }

  #[test]
  fn test_file_not_found() {
    // Test with non-existent file that has supported extension
    let result = is_valid("non_existent_image.jpg");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File not found"));
  }

  #[test]
  fn test_edge_cases() {
    // Test empty filename - no extension, so not in SKIP_EXTENSIONS
    // Should fail with file not found
    let result = is_valid("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File not found"));

    // Test filename without extension - not in SKIP_EXTENSIONS
    // Should fail with file not found
    let result = is_valid("filename_without_extension");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File not found"));

    // Test filename with multiple dots
    assert!(should_skip_validation("file.backup.md"));

    // Test very long filenames - .jpg is not in SKIP_EXTENSIONS
    // Should fail with file not found
    let long_filename = format!("{}.jpg", "a".repeat(1000));
    let result = is_valid(&long_filename);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File not found"));
  }
}
