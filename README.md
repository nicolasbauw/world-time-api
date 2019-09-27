# World Time API
An API to retrieve date / time / daylight saving time informations about a timezone.
There are similar APIs already available on internet, my goals were:
- to write it myself
- access these informations within my organization without sending external requests.

Example:
```
curl http://localhost:8000/Europe/Paris
{"abbreviation":"CEST","datetime":"2019-09-27T18:58:03.604706700+02:00",
"dst_from":"2019-03-31T01:00:00Z","dst_offset":7200,"dst_until":"2019-10-27T01:00:00Z",
"raw_offset":3600,"timezone":"Europe/Paris","utc_datetime":"2019-09-27T16:58:03.604706700Z","utc_offset":"+02:00"}
```

It uses system TZfiles (default location on Linux and Macos /usr/share/zoneinfo).
You can override the TZfiles default location with the DATA_ROOT environment variable (ending by a /).
That also works on Windows, for example:

$env:DATA_ROOT="C:\Users\nbauw\Dev\rs-tzfile\zoneinfo\"; cargo run

Some explanations about the offset fields:
- raw_offset : the "normal" offset to utc, in seconds
- dst_offset : the offset to utc during daylight saving time, in seconds
- utc_offset : the current offset to utc, taking into account daylight saving time or not (according to dst_from and dst_until), in +/- HH:MM

Provided (untested since last major updates, are TZfiles in alpine image ?) dockerfile builds the app with statically compiled musl libc and then uses an alpine image to keep a low size.