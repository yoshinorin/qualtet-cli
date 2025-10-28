# Qualtet-CLI

[![CI(Windows Only)](https://github.com/yoshinorin/qualtet-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/yoshinorin/qualtet-cli/actions/workflows/ci.yml) <sub> [Unit Test Report(Nodejs code Only)](https://yoshinorin.github.io/qualtet-cli/coverage/) </sub>

The cli for [Qualtet](https://github.com/yoshinorin/qualtet). A set of wrappers that call the APIs of [Hexo](https://github.com/hexojs/hexo) and [Qualtet](https://github.com/yoshinorin/qualtet).

## Requirements

* Node.js 24.x
* rustup 1.28.x
* rustc 1.90.x

## Setup

Clone this repository at the same hierarchy as the Hexo folder.

## Build

Some parts of the code are written in Rust. Therefore, a build using [napi-rs](https://github.com/napi-rs/napi-rs) is required.

```sh
$ cd rust-lib
$ npm run build
```

## Commands

| Command | Description | Usage |
|---------|-------------|-------|
| `assertImages` | Validate images for EXIF/GPS info | `node ./cmd/assertImages.js --days-ago=<daysAgo>` |
| `delete` | Delete content by ID | `node ./cmd/delete.js --api-url=<apiUrl> --service=<serviceName> --author=<authorName> --content-id=<contentId>` |
| `deleteTag` | Delete tag by ID | `node ./cmd/deleteTag.js --api-url=<apiUrl> --service=<serviceName> --author=<authorName> --tag-id=<tagId>` |
| `invalidateCaches` | Invalidate cache | `node ./cmd/invalidateCaches.js --api-url=<apiUrl> --service=<serviceName> --author=<authorName>` |
| `postSeriesFromFile` | Create/update series from JSON | `node ./cmd/postSeriesFromFile.js --api-url=<apiUrl> --service=<serviceName> --author=<authorName> --file-path=<filePath>` |
| `publish` | Publish updated articles | `node ./cmd/publish.js --api-url=<apiUrl> --service=<serviceName> --author=<authorName> --days-ago=<daysAgo> --deploy-assets-dir=<deployAssetsDir>` |
| `setCredential` | Set credential to keytar | `node ./cmd/setCredential.js` |
| `watch` | Watch file changes and publish | `node ./cmd/watch.js --api-url=<apiUrl> --service=<serviceName> --author=<authorName> --deploy-assets-dir=<deployAssetsDir>` |

### `assertImages`

Validate an image has EXIF(includes GPS) info or not.

```sh
$ node ./cmd/assertImages.js --days-ago=<daysAgo (default: 10000)>

INFO  check updated in 10000 days ago articles assets.
INFO  Validating config
INFO  Start processing
ERROR C:\Users\<userName>\source\_posts\example\gps.jpg: has GPS info
{
  file: 'C:\Users\<userName>\source\_posts\example\gps.jpg',
  gps: [
    undefined,
    undefined,
    undefined,
    { id: 2, value: [Array], description: 39.9987 },
    { id: 1, value: [Array], description: 'North latitude' },
    { id: 4, value: [Array], description: 138.6517 },
    { id: 3, value: [Array], description: 'East longitude' },
    { id: 18, value: [Array], description: 'WGS-84' }
  ]
}
WARN  C:\Users\<userName>\source\_posts\example2\hoge.jpg: has EXIF
```

### `delete`

Delete content (post or page) by its id.

```sh
$ node ./cmd/delete.js --api-url=<apiUrl> --service=<serviceName> --author=<authorName> --content-id=<contentId>
```

### `deleteTag`

Delete tag by its id.

```sh
$ node ./cmd/deleteTag.js --api-url=<apiUrl> --service=<serviceName> --author=<authorName> --tag-id=<tagId>
```

### `invalidateCaches`

Invalidate Cache.

```
$ node ./cmd/invalidateCaches.js --api-url=<apiUrl> --service=<serviceName> --author=<authorName>
INFO  caches: invalidated
```

### `postSeriesFromFile`

Create or Update series from JSON file.

```
$ node ./cmd/postSeriesFromFile.js --api-url=<apiUrl> --service=<serviceName> --author=<authorName> --file-path=<JSON filePath>

// example JSON
{
  "name": "example",
  "title": "example title",
  "description": "example description"
}
```

### `publish`

Publish articles (posts or pages) that have been updated up to n days ago.

The `--deploy-assets-dir` option specifies the directory where assets will be stored for deployment (e.g., via rsync). The actual deployment process is not handled by this CLI and should be implemented separately using shell scripts or other tools.

```sh
$ node ./cmd/publish.js --api-url=<apiUrl> --service=<serviceName> --author=<authorName> --days-ago=<daysAgo> --deploy-assets-dir=<deployAssetsDir>
INFO  caches: invalidated
INFO  Validating config
INFO  Start processing
INFO  created - 1: 01gz702w32kxdhe8417fxcybcm - /example
INFO  created - 2: 01gs876d3rmembfhnt9qskwz9a - /articles/example2/
INFO  created - 3: 01gs876n2adwaznhv6m0yz8drw - /articles/example3/
...
```

### `setCredential`

Set credential to your keytar.

```sh
$ node ./cmd/setCredential.js
Please input serviceName, authorName, and author's password: <serviceName> <authorName> <password>
```

### `watch`

Watch for file changes and automatically publish modified content to the API server.

The `--deploy-assets-dir` option specifies the directory where assets will be stored for deployment (e.g., via rsync). The actual deployment process is not handled by this CLI and should be implemented separately using shell scripts or other tools.

```sh
$ node ./cmd/watch.js --api-url=<apiUrl> --service=<serviceName> --author=<authorName> --deploy-assets-dir=<deployAssetsDir>
INFO  API server is ready at http://localhost:9000
INFO  caches: invalidated
INFO  hexo initialized. Watching for file changes...
INFO  watch mode started. files will be sent to API on change.
INFO  file changed: source/_posts/example.md
INFO  created - 1: 01gz702w32kxdhe8417fxcybcm - /example
```

## LICENSE

MIT