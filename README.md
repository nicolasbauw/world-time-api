# World Time API

[![Current Crates.io Version](https://img.shields.io/crates/v/world-time-api.svg)](https://crates.io/crates/world-time-api)
[![Downloads badge](https://img.shields.io/crates/d/world-time-api.svg)](https://crates.io/crates/world-time-api)

An API to retrieve date / time / daylight saving time informations about a timezone.

Example:
```
curl http://localhost:8000/Europe/Paris
{"abbreviation":"CEST","datetime":"2019-10-08T20:16:13.612030900+02:00","dst_from":"2019-03-31T01:00:00Z","dst_offset":7200,"dst_period":true,"dst_until":"2019-10-27T01:00:00Z","raw_offset":3600,"timezone":"Europe/Paris","utc_datetime":"2019-10-08T18:16:13.612030900Z","utc_offset":"+02:00","week":40}
```

Since 1.4, uppercase is not mandatory when requesting regions and cities starting with (non containing) an uppercase in their name:  
```
curl http://localhost:8000/europe/paris
```

It uses system TZfiles (default location on Linux and Macos /usr/share/zoneinfo).
You can override the TZfiles default location with the TZFILES_DIR environment variable (ending by a /).
That also works on Windows, for example:

```
$env:TZFILES_DIR="C:\Users\nbauw\Dev\rs-tzfile\zoneinfo\"; cargo run
```

Some explanations about the offset fields:
- raw_offset : the "normal" offset to utc, in seconds
- dst_offset : the offset to utc during daylight saving time, in seconds
- utc_offset : the current offset to utc, taking into account daylight saving time or not (according to dst_from and dst_until), in +/- HH:MM

Provided dockerfile (which is no longer maintained) builds the app with statically compiled musl libc and then uses an alpine image to keep a low size.
