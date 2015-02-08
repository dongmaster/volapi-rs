extern crate url;
extern crate websocket;
extern crate "rustc-serialize" as rustc_serialize;

use websocket::{WebSocket, WSMessage};
use url::Url;
/*
let base_url = "https://volafile.io";
let BASE_ROOM_URL: &str = format!("{}/r/", BASE_URL);
let BASE_REST_URL: &str = format!("{}/rest/", BASE_URL);
* */

fn main() {
    let url = Url::parse("ws://echo.websocket.org").unwrap();
    let mut ws = WebSocket::new(url);
    ws.connect().unwrap();
    
    let msg = WSMessage::text("Hello, World!");
    
    ws.send_message(&msg).unwrap();
}
