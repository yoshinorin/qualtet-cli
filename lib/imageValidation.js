
const ExifReader = require('exifreader');
const log = require('hexo-log').default({
  debug: false,
  silent: false
});

function validate(source) {
  ExifReader.load(source, {expanded: true}).then(tags => {
    const exif = tags['exif']
    if (!exif) {
      return false;
    }
    if (exif['GPS'] || exif['GPSLatitude'] || exif['GPSAltitude'] || exif['GPSAltitudeRef'] || exif['GPSLatitudeRef'] || exif['GPSLongitudeRef'] || exif['GPSLongitude'] || exif['GPSMapDatum']) {
      log.error(`${source}: has GPS info`)
      console.log({
        "file": source,
        "gps": [
          exif['GPS'],
          exif['GPSAltitude'],
          exif['GPSAltitudeRef'],
          exif['GPSLatitude'],
          exif['GPSLatitudeRef'],
          exif['GPSLongitude'],
          exif['GPSLongitudeRef'],
          exif['GPSMapDatum']
        ]
      });
      return false;
    } else {
      log.warn(`${source}: has EXIF`);
    }
  }).catch(e => {
    console.warn(`${source}: exception`);
    // console.log(e);
    return false;
  });
};

module.exports = {
  validate
};