#[macro_use]
extern crate rocket;

use libtzfile::Tz;
use rocket::{http::Status, http::RawStr, serde::json::Json};
use std::env;
use case_convert::CaseConvert;

// Return a String that's correctly formatted for Timezone lookup.
// Error if can't be parsed.
//
// We allow this to take types that can be converted into a String
// ToDo - we could convert accented characters to non-accents.
fn to_valid_format<S: Into<String>>(s: S) -> Result<String, String> {
    let s: String = s.into();
    let raw_str = RawStr::new(&s);
    // We'll first have to make sure we convert any URI encoding to regular text.
    // This means we URI decode the input, so "%20" becomes " ", etc.
    // We return the Decoded string as String, otherwise, return error
    RawStr::percent_decode(raw_str)
        .map(|s| s.to_string().uppercase_first().replace(" ", "_"))
        .map_err(|_| String::from("Decoding error"))
}

// Return info on a Timezone.
//
// In the best case scenario, this returns the information for a requested Timezone.
//
// If neither the region or city match a known Timezone, we return a 404, which
// is done via Ok(None).
//
// If we do find a valid region and city, then we return that data as a Josn
// object via Ok(Some) with a 200.
//
// If the requested Timezone has bad formatting, then we want to return an
// error specifying that this is a 400, which we do via an Err.
#[get("/zoneinfo/<region>/<city>")]
fn get_tzinfo(region: &str, city: &str) -> Result<Option<Json<libtzfile::Tzinfo>>, Status> {
    // If both geo-location values can be parsed...
    if let (Ok(region), Ok(city)) = (to_valid_format(region), to_valid_format(city)) {
        // TZfiles location can be customized through the TZFILES_DIR environment. Default location is /usr/share/zoneinfo.
        let mut tz_file = String::new();
        tz_file.push_str(&env::var("TZFILES_DIR").unwrap_or(format!("/usr/share/zoneinfo/")));
        // Then we create a String using format to create a Timezone lookup key.
        let s = if cfg!(windows) {
            format!("{}\\{}", region, city)
        } else {
            format!("{}/{}", region, city)
        };
        tz_file.push_str(&s);

        // We run `.ok` to convert from Result to Option.
        // `and_then` lets us work directly with the values.
        // If it encounters a problem, then it returns None.
        // No matter if we get a Some or None, we wrap it in Ok.
        // We do this because Rocket can handle None as a 404.
        // The Tzinfo struct implements the Serialize trait.
        let tz = Tz::new(&tz_file)
            .and_then(|z| z.zoneinfo())
            .ok()
            .map(Json);
        Ok(tz)
    } else {
        // If both geo-location values can't be parsed, return a BadRequest
        Err(Status::BadRequest)
    }
}

#[get("/zoneinfo/<timezone>")]
fn get_standardtime(timezone: &str) -> Result<Option<Json<libtzfile::Tzinfo>>, Status> {
    // If timezone can be parsed...
    if let Ok(timezone) = to_valid_format(timezone) {
        // TZfiles location can be customized through the TZFILES_DIR environment. Default location is /usr/share/zoneinfo.
        let mut tz_file = String::new();
        tz_file.push_str(&env::var("TZFILES_DIR").unwrap_or(format!("/usr/share/zoneinfo/")));
        tz_file.push_str(&timezone);

        let tz = Tz::new(&tz_file)
            .and_then(|z| z.zoneinfo())
            .ok()
            .map(Json);
        Ok(tz)
    } else {
        // If both geo-location values can't be parsed, return a BadRequest
        Err(Status::BadRequest)
    }
}

// Error Catchers

// It's nice to have this come back as Json.
// Since we're working with a known str value, we can specify it as static.
// Removed rocket::Request variable since we aren't using it.
#[catch(400)]
fn bad_request() -> Json<&'static str> {
    Json("Invalid Timezone")
}
// Again, nice to have Json.
#[catch(404)]
fn not_found<'a>(req: &'a rocket::Request) -> Json<String> {
    Json(format!("Unable to find Timezone {}", req.uri().path()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_tzinfo, get_standardtime])
        .register("/", catchers![bad_request, not_found])
}
