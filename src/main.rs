#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;
extern crate tzparse;

use serde_derive::{Serialize};
use rocket_contrib::json::JsonValue;
use chrono::prelude::*;

#[derive(Serialize)]
struct RespTz {
    timezone: String,
    raw_offset: i32,
    dst_offset: i32,
    utc_offset: String,
    utc_datetime: String,
    datetime: String,
    dst_from: Option<String>,
    dst_until: Option<String>,
    dst_period: bool,
    abbreviation: String,
}

#[get("/<region>/<city>")]
fn get_tzinfo(region: String, city: String) -> Option<JsonValue> {
    let mut s = String::new();
    s.push_str(&region);
    s.push_str("/");
    s.push_str(&city);
    
    let d = Utc::now();
    let y = d.format("%Y").to_string().parse().unwrap();
    let t = match tzparse::get(&s, Some(y)) {
        Some(tz) => tzparse::worldtime(&tz).unwrap(),
        None => return None
    };

    let dst_from = match t.dst_from {
        Some(d) => Some(format!("{:?}", d)),
        None => None
    };

    let dst_until = match t.dst_until {
        Some(d) => Some(format!("{:?}", d)),
        None => None
    };

    let resp = RespTz {
        timezone: s,
        raw_offset: t.raw_offset as i32,
        dst_offset: t.dst_offset as i32,
        utc_offset: format!("{:?}", t.utc_offset),
        utc_datetime: format!("{:?}", t.utc_datetime),
        datetime: format!("{:?}", t.datetime),
        dst_from: dst_from,
        dst_until: dst_until,
        dst_period: t.dst_period,
        abbreviation: t.abbreviation
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
