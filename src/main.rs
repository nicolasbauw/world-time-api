#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::{response::content::Json, http::Status};
use std::env;

// Return a String that's correctly formatted for Timezone lookup.
// Error if can't be parsed.
//
// We allow this to take types that can be converted into a String
// ToDo - we could convert accented characters to non-accents.
fn to_valid_format<S: Into<String>>(s: S) -> Result<String, String>{
    let s: String = s.into();
    // We'll first have to make sure we convert any URI encoding to regular text.
    // This means we URI decode the input, so "%20" becomes " ", etc.
    use rocket::http::uri::Uri;
    // Return the Decoded string as String, otherwise, return error
    Uri::percent_decode(s.as_bytes())
        .map(|s| s.to_string().replace(" ", "_"))
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
#[get("/<region>/<city>")]
fn get_tzinfo(region: String, city: String) -> Result<Option<Json<String>>, Status> {
    // If both geo-location values can be parsed...
    if let (Ok(region), Ok(city)) = (to_valid_format(region), to_valid_format(city)) {
        // TZfiles location can be customized through the TZFILES_DIR environment. Default location is /usr/share/zoneinfo.
        let mut tz_file = {
            let mut d = String::new();
            d.push_str(&env::var("TZFILES_DIR").unwrap_or(format!("/usr/share/zoneinfo/")));
            d
        };
    
        // Then we create a String using format to create a Timezone lookup key.
        let s = format!("{}/{}", region, city);
        tz_file.push_str(&s);

        // We run `.ok` to convert from Result to Option.
        // `and_then` lets us work directly with the values. 
        // If it encounters a problem, then it returns None.
        // No matter if we get a Some or None, we wrap it in Ok.
        // We do this because Rocket can handle None as a 404.
        let tz = tzparse::get_zoneinfo(&tz_file)
            .ok()
            .and_then(|tz_info| {
               tz_info
                   // This [returns a String](https://docs.rs/tzparse/1.0.3/src/tzparse/lib.rs.html#126)
                   .to_json()
                   // This returns the String as an Option
                   .ok()
                   // And we wrap the Option value in Json
                   .map(Json)
           });
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
    Json("\"Invalid Timezone\"")
}
// Again, nice to have Json.
#[catch(404)]
fn not_found<'a>(req: &'a rocket::Request) -> Json<String> {
    Json(format!("\"Unable to find Timezone {}\"", req.uri().path()))
}
    
fn main() {
    rocket::ignite()
        .mount("/", routes![get_tzinfo])
        .register(catchers![bad_request, not_found])
        .launch();
}
