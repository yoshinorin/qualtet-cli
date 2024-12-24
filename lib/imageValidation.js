const ExifReader = require("exifreader");
const log = require("hexo-log").default({
  debug: false,
  silent: false,
});

async function isValid(source) {
  try {
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
      log.error(`${source}: has GPS info`);
      console.log({
        file: source,
        gps: gpsTags.map((tag) => exif[tag]),
      });
      return false;
    } else {
      log.warn(`${source}: has EXIF`);
      return true;
    }
  } catch (e) {
    console.warn(`${source}: exception`);
    return false;
  }
}

module.exports = {
  isValid,
};
