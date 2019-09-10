#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;

use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use rocket_contrib::json::JsonValue;
use std::fs;

#[derive(Deserialize)]
struct TZ {
    timezone: String,
    raw_offset: i32,
    dst_offset: i32,
    dst_from: String,
    dst_until: String,
    abbreviation: String,
    dst_abbreviation: String,
}

#[derive(Serialize)]
struct RespTz {
    timezone: String,
    raw_offset: i32,
    dst_offset: i32,
    utc_offset: String,
    utc_datetime: String,
    datetime: String,
    dst_from: String,
    dst_until: String,
    dst: bool,
    abbreviation: String,
}

#[get("/<region>/<city>")]
fn get_tzinfo(region: String, city: String) -> Option<JsonValue> {
    let mut s = String::new();
    s.push_str(&region);
    s.push_str("/");
    s.push_str(&city);
    s.push_str(".json");
    let z = match fs::read_to_string(s) {
        Ok(f) => f,
        Err(_) => return None
    };
    let tz: TZ = serde_json::from_str(&z).unwrap();
    let d = Utc::now();
    let dst_from = DateTime::parse_from_rfc3339(&tz.dst_from).unwrap();
    let dst_until = DateTime::parse_from_rfc3339(&tz.dst_until).unwrap();
    let dst = d.with_timezone(&FixedOffset::east(0)) > dst_from && d.with_timezone(&FixedOffset::east(0)) < dst_until;
    let raw_offset = tz.raw_offset;
    let dst_offset = tz.dst_offset;
    let utc_offset = if dst == true { FixedOffset::east(raw_offset + dst_offset) } else { FixedOffset::east(raw_offset) }; 
    let resp = RespTz {
        timezone: tz.timezone,
        raw_offset: tz.raw_offset,
        dst_offset: tz.dst_offset,
        utc_offset: format!("{}", utc_offset),
        utc_datetime: format!("{}", d),
        datetime: format!("{}", d.with_timezone(&utc_offset)),
        dst_from: tz.dst_from,
        dst_until: tz.dst_until,
        dst: dst,
        abbreviation: if dst {tz.dst_abbreviation} else {tz.abbreviation},
    };
    Some(json!(resp))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![get_tzinfo])
        .register(catchers![not_found])
        .launch();
}

#[catch(404)]
fn not_found(_req: &rocket::Request) -> String {
    format!("Timezone not implemented\n")
}
