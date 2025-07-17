use serde_json::{Map, Value, json};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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
fn has_gps_information(exif_data: &exif::Exif) -> bool {
  GPS_TAGS
    .iter()
    .any(|&tag| exif_data.get_field(tag, exif::In::PRIMARY).is_some())
}

/// Collect GPS data from EXIF and format as JSON
fn collect_gps_data(exif_data: &exif::Exif, source: &str) -> Result<(), String> {
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

  match serde_json::to_string_pretty(&json_output) {
    Ok(formatted_json) => {
      println!("{}", formatted_json);
      Ok(())
    }
    Err(json_err) => {
      crate::log_error(format!(
        "{}: Failed to format GPS data as JSON - {}",
        source, json_err
      ))
      .map_err(|e| e.to_string())?;
      Ok(())
    }
  }
}

/// Handle EXIF reading errors with appropriate logging
fn handle_exif_error(source: &str, exif_err: exif::Error) -> Result<bool, String> {
  match exif_err {
    exif::Error::InvalidFormat(msg) => {
      crate::log_error(format!("{}: Invalid file format - {}", source, msg))
        .map_err(|e| e.to_string())?;
      return Ok(false);
    }
    exif::Error::NotFound(_msg) => {
      // TODO: log level should be debug.
      // crate::log_info(format!("{}: No EXIF data found - {}", source, msg))
      //    .map_err(|e| e.to_string())?;
      return Ok(true);
    }
    exif::Error::BlankValue(msg) => {
      crate::log_warn(format!("{}: EXIF contains blank values - {}", source, msg))
        .map_err(|e| e.to_string())?;
      return Ok(true);
    }
    exif::Error::TooBig(msg) => {
      crate::log_error(format!(
        "{}: File is too large to process - {}",
        source, msg
      ))
      .map_err(|e| e.to_string())?;
      return Ok(false);
    }
    _ => {
      crate::log_error(format!("{}: EXIF reading failed - {}", source, exif_err))
        .map_err(|e| e.to_string())?;
      return Ok(false);
    }
  }
}

pub fn is_valid(source: &str) -> Result<bool, String> {
  if should_skip_validation(source) {
    crate::log_warn(format!("asset validation skipped - : {}", source))
      .map_err(|e| e.to_string())?;
    return Ok(true);
  }

  let path = Path::new(source);
  if !path.exists() {
    return Err(format!("File not found: {}", source));
  }

  let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
  let mut reader = BufReader::new(file);

  // Read EXIF data
  match exif::Reader::new().read_from_container(&mut reader) {
    Ok(exif_data) => {
      if has_gps_information(&exif_data) {
        crate::log_error(format!("{}: has GPS info", source)).map_err(|e| e.to_string())?;

        collect_gps_data(&exif_data, source)?;
        return Ok(false);
      } else {
        crate::log_warn(format!("{}: has EXIF", source)).map_err(|e| e.to_string())?;
        return Ok(true);
      }
    }
    Err(exif_err) => {
      return handle_exif_error(source, exif_err);
    }
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
    let test_file = "test.jpg";

    // Test InvalidFormat error - should return Ok(false)
    let invalid_format_err = exif::Error::InvalidFormat("Invalid JPEG");
    let result = handle_exif_error(test_file, invalid_format_err);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);

    // Test NotFound error - should return Ok(true)
    let not_found_err = exif::Error::NotFound("No EXIF");
    let result = handle_exif_error(test_file, not_found_err);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);

    // Test BlankValue error - should return Ok(true)
    let blank_value_err = exif::Error::BlankValue("Empty value");
    let result = handle_exif_error(test_file, blank_value_err);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);

    // Test TooBig error - should return Ok(false)
    let too_big_err = exif::Error::TooBig("File too large");
    let result = handle_exif_error(test_file, too_big_err);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
  }

  #[test]
  fn test_skip_extensions() {
    assert!(is_valid("test.md").unwrap());
    assert!(is_valid("test.svg").unwrap());
    assert!(is_valid("test.txt").unwrap());
    // Test case insensitive extensions
    assert!(is_valid("test.MD").unwrap());
    assert!(is_valid("test.SVG").unwrap());
    assert!(is_valid("test.TXT").unwrap());
    assert!(is_valid("test.Mp4").unwrap());
  }

  #[test]
  fn test_unsupported_image_formats() {
    // These formats are not in SKIP_EXTENSIONS and files don't exist,
    // so they should return Err with "File not found"
    let result = is_valid("test.gif");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File not found"));

    let result = is_valid("test.bmp");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File not found"));

    // .ico is in SKIP_EXTENSIONS, so it should be skipped
    assert!(is_valid("test.ico").unwrap());

    // Test case insensitive
    let result = is_valid("test.GIF");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File not found"));

    let result = is_valid("test.BMP");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File not found"));

    // .ICO is in SKIP_EXTENSIONS, so it should be skipped (case insensitive)
    assert!(is_valid("test.ICO").unwrap());
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

    // Test filename with multiple dots - .md is in SKIP_EXTENSIONS
    assert!(should_skip_validation("file.backup.md"));

    // Test very long filenames - .jpg is not in SKIP_EXTENSIONS
    // Should fail with file not found
    let long_filename = format!("{}.jpg", "a".repeat(1000));
    let result = is_valid(&long_filename);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File not found"));
  }
}
