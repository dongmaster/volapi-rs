extern crate url;
extern crate websocket;
extern crate "rustc-serialize" as rustc_serialize;

const BASE_URL = "https://volafile.io";
const BASE_ROOM_URL = format!("{}/r/", BASE_URL);
const BASE_REST_URL = format!("{}/rest/", BASE_URL)

fn main() {
    println!("Hello, world!");
}
