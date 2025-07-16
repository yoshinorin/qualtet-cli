use serde_json::{Map, Value, json};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Check if the file extension is supported by the exif crate
/// Supported formats: TIFF, JPEG, HEIF/HEIC/AVIF, PNG, WebP, and some RAW formats
fn is_exif_supported_format(source: &str) -> bool {
  let exif_supported_extensions = [
    ".jpg", ".jpeg", ".jpe", ".jfif", // JPEG
    ".tiff", ".tif", // TIFF
    ".heif", ".heic", ".avif", // HEIF variants
    ".png",  // PNG
    ".webp", // WebP
    // RAW formats (common ones)
    ".cr2", ".cr3", ".nef", ".arw", ".dng", ".orf", ".rw2", ".pef", ".srw", ".raf",
  ];

  exif_supported_extensions
    .iter()
    .any(|&ext| source.to_lowercase().ends_with(ext))
}

pub fn is_valid(source: &str) -> Result<bool, String> {
  let skip_extensions = [
    ".md", ".mermaid", ".mp3", ".mp4", ".webm", ".pptx", ".svg", ".txt",
  ];

  if skip_extensions
    .iter()
    .any(|&ext| source.to_lowercase().ends_with(ext))
  {
    crate::log_warn(format!("asset validation skipped - : {}", source))
      .map_err(|e| e.to_string())?;
    return Ok(true);
  }

  // Check if the file format is supported by exif crate
  if !is_exif_supported_format(source) {
    crate::log_warn(format!(
      "{}: unsupported format for EXIF reading, skipping validation",
      source
    ))
    .map_err(|e| e.to_string())?;
    return Ok(true);
  }

  // Check if file exists
  let path = Path::new(source);
  if !path.exists() {
    return Err(format!("File not found: {}", source));
  }

  // Open file for EXIF reading
  let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
  let mut reader = BufReader::new(file);

  // Read EXIF data
  match exif::Reader::new().read_from_container(&mut reader) {
    Ok(exif_data) => {
      let gps_tags = [
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

      // Check for GPS information
      let has_gps_info = gps_tags
        .iter()
        .any(|&tag| exif_data.get_field(tag, exif::In::PRIMARY).is_some());

      if has_gps_info {
        crate::log_error(format!("{}: has GPS info", source)).map_err(|e| e.to_string())?;

        // Collect GPS data for JSON output
        let mut gps_data = Map::new();
        for &tag in &gps_tags {
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

        // Pretty print JSON with indentation
        match serde_json::to_string_pretty(&json_output) {
          Ok(formatted_json) => println!("{}", formatted_json),
          Err(json_err) => {
            crate::log_error(format!(
              "{}: Failed to format GPS data as JSON - {}",
              source, json_err
            ))
            .map_err(|e| e.to_string())?;
          }
        }
        return Ok(false);
      } else {
        crate::log_warn(format!("{}: has EXIF", source)).map_err(|e| e.to_string())?;
        return Ok(true);
      }
    }
    Err(exif_err) => {
      match exif_err {
        exif::Error::InvalidFormat(msg) => {
          crate::log_error(format!("{}: Invalid file format - {}", source, msg))
            .map_err(|e| e.to_string())?;
        }
        exif::Error::NotFound(_msg) => {
          // TODO: log level should be debug.
          // crate::log_info(format!("{}: No EXIF data found - {}", source, msg))
          //    .map_err(|e| e.to_string())?;
        }
        exif::Error::BlankValue(msg) => {
          crate::log_warn(format!("{}: EXIF contains blank values - {}", source, msg))
            .map_err(|e| e.to_string())?;
        }
        exif::Error::TooBig(msg) => {
          crate::log_error(format!(
            "{}: File is too large to process - {}",
            source, msg
          ))
          .map_err(|e| e.to_string())?;
        }
        _ => {
          crate::log_error(format!("{}: EXIF reading failed - {}", source, exif_err))
            .map_err(|e| e.to_string())?;
        }
      }

      return Ok(true);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
  fn test_exif_supported_formats() {
    assert!(is_exif_supported_format("image.jpg"));
    assert!(is_exif_supported_format("image.JPEG"));
    assert!(is_exif_supported_format("image.tiff"));
    assert!(is_exif_supported_format("image.png"));
    assert!(is_exif_supported_format("image.webp"));
    assert!(is_exif_supported_format("image.heic"));
    assert!(is_exif_supported_format("image.cr2"));

    assert!(!is_exif_supported_format("image.gif"));
    assert!(!is_exif_supported_format("image.bmp"));
    assert!(!is_exif_supported_format("video.mp4"));
    assert!(!is_exif_supported_format("document.pdf"));
  }

  #[test]
  fn test_unsupported_image_formats() {
    // These should return Ok(true) with a warning, not an error
    assert!(is_valid("test.gif").unwrap());
    assert!(is_valid("test.bmp").unwrap());
    assert!(is_valid("test.ico").unwrap());
    // Test case insensitive
    assert!(is_valid("test.GIF").unwrap());
    assert!(is_valid("test.BMP").unwrap());
    assert!(is_valid("test.ICO").unwrap());
  }
}
