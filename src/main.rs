use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use rim::{route::launch_web, ws::launch_ws};
use std::io::Error;
use std::io::Write;

extern crate rocket;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let _r = tokio::join!(launch_web(), launch_ws());

    Ok(())
}
