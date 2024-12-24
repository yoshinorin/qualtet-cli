const ExifReader = require("exifreader");
const { logWarn, logError } = require("../rust-lib/index.js");

async function isValid(source) {
  try {
    const allowedExtensions = [
      ".md",
      ".mermaid",
      ".mp3",
      ".mp4",
      ".pptx",
      ".svg",
      ".txt",
    ];
    if (source && allowedExtensions.some((ext) => source.endsWith(ext))) {
      logWarn(`asset validation skipped - : ${source}`);
      return true;
    }
    const tags = await ExifReader.load(source, { expanded: true });
    const exif = tags["exif"];
    if (!exif) {
      return true;
    }
    const gpsTags = [
      "GPS",
      "GPSLatitude",
      "GPSAltitude",
      "GPSAltitudeRef",
      "GPSLatitudeRef",
      "GPSLongitudeRef",
      "GPSLongitude",
      "GPSMapDatum",
    ];
    const hasGpsInfo = gpsTags.some((tag) => exif[tag]);
    if (hasGpsInfo) {
      logError(`${source}: has GPS info`);
      console.log({
        file: source,
        gps: gpsTags.map((tag) => exif[tag]),
      });
      return false;
    } else {
      logWarn(`${source}: has EXIF`);
      return true;
    }
  } catch (e) {
    logWarn(`${source}: exception`);
    return false;
  }
}

module.exports = {
  isValid,
};
