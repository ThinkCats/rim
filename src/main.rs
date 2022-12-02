use std::io::Error;

use rim::{route::launch_web, ws::launch_ws};

extern crate rocket;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let _r = tokio::join!(launch_web(), launch_ws());

    Ok(())
}
