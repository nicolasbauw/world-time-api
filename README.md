# World Time API
An API to retrieve date / time / daylight saving time informations about a timezone.
There are similar APIs already available on internet, my goals were:
- to write it myself
- access these informations within my organization without sending external requests.

Example:
```
curl http://localhost:8000/Europe/Paris
{"abbreviation":"CEST","datetime":"2019-09-09 16:28:55.771668 +02:00","dst":true,"dst_from":"2019-03-31T01:00:00+00:00","dst_offset":3600,"dst_until":"2019-10-27T01:00:00+00:00","raw_offset":3600,"timezone":"Europe/Paris","utc_datetime":"2019-09-09 14:28:55.771668 UTC","utc_offset":"+02:00"}
```