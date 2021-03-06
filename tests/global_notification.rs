extern crate bitmex;
extern crate dotenv;
extern crate tokio;

use std::env::var;

use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn get_global_notification() -> Result<()> {
    // This will fail for error access denied
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_global_notification()?;

    assert!(rt.block_on(fut).is_err());
    Ok(())
}
