#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::content;

#[get("/<region>/<city>")]
fn get_tzinfo(region: String, city: String) -> Option<content::Json<String>> {
    let mut s = String::new();
    s.push_str(&region);
    s.push_str("/");
    s.push_str(&city);
    
    let t = match tzparse::get_zoneinfo(&s){
        Some(t) => t,
        None => return None
    };

    let resp = serde_json::to_string(&t).unwrap();
    Some(content::Json(resp))
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
