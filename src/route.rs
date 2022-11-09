use rocket::{routes, get};

use crate::store::query_group;



pub async fn launch_web() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, hello])
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[get("/hello")]
fn hello() -> &'static str {
    query_group(1_u32);
    "query ok"
}
