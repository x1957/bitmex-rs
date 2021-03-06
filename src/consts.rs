use std::env::var;

use log::warn;

// dotenv is a must run in every test otherwise the url will be mis-loaded
lazy_static! {
    pub static ref WS_URL: &'static str = {
        if var("BITMEX_TESTNET").unwrap_or("0".to_string()) == "0" {
            "wss://www.bitmex.com/realtime"
        } else {
            warn!("Your are using BitMEX testnet Restful API");
            "wss://testnet.bitmex.com/realtime"
        }
    };
    pub static ref REST_URL: &'static str = {
        if var("BITMEX_TESTNET").unwrap_or("0".to_string()) == "0" {
            "https://www.bitmex.com/api/v1"
        } else {
            warn!("Your are using BitMEX testnet Websocket");
            "https://testnet.bitmex.com/api/v1"
        }
    };
}
