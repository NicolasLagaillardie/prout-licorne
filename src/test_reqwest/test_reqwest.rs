use reqwest::{self, Error};
use trpl;

async fn get() -> Result<String, Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    return Ok(body);
}

async fn post() -> Result<reqwest::Response, Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await?;

    return Ok(res);
}

pub fn main_test_get() {
    trpl::block_on(async {
        let body = get().await;
        println!("body = {body:?}")
    });
}

pub fn main_test_post() {
    trpl::block_on(async {
        let res = post().await;
        println!("res = {res:?}")
    });
}
