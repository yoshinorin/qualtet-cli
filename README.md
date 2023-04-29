# Qualtet-CLI

The cli for [Qualtet](https://github.com/yoshinorin/qualtet).

## Setup

Clone this repository at the same hierarchy as the Hexo folder.

## Commands

- assertImages: `node ./cmd/assertImages.js <daysAgo>`
- delete: `node ./cmd/delete.js <apiUrl> <serviceName> <authorName> <contentId>`
- invalidateCaches: `node ./cmd/invalidateCaches.js <apiUrl> <serviceName> <authorName>`
- publish: `node ./cmd/publish.js <apiUrl> <serviceName> <authorName> <daysAgo>`
- setCredential: `node ./cmd/setCredential.js`

### `assertImages`

Validate an image has EXIF(includes GPS) info or not.

```sh
$ node ./cmd/assertImages.js <daysAgo (default: 10000)>

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
$ node ./cmd/delete.js <apiUrl> <serviceName> <authorName> <contentId>
```

### `invalidateCaches`

Invalidate Cache.

```
$ node ./cmd/invalidateCaches.js <apiUrl> <serviceName> <authorName>
INFO  caches: invalidated
```

### `publish`

Publish articles (posts or pages) that have been updated up to n days ago.

```sh
$ node ./cmd/publish.js <apiUrl> <serviceName> <authorName> <daysAgo>
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

## LICENSE

© yoshinorin

> [NO LICENSE (NO PERMISSION)](https://choosealicense.com/no-permission/)

> [GitHub Licensing a repository](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository)