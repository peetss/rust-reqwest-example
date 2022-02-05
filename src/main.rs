use crate::reqwest::header::USER_AGENT;
use reqwest;

use futures::executor::block_on;

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

async fn send_request(url: &str) -> Result<()> {
    println!("Request: {}", url);
    let resp = reqwest::Client::new()
        .post(url)
        .basic_auth("username", Some("password"))
        .form(&[("key1", "value1"), ("key2", "value2")])
        .header(USER_AGENT, "rust reqwest example")
        .send().await?;

    println!("Status: {}", resp.status());
    println!("Headers: {:#?}", resp.headers());
    let body = resp.text().await?;
    println!("Body: {}", body);
    Ok(())
}

#[tokio::main]
async fn main() {
    let future = send_request("https://some-url");
    let result = block_on(future);
    println!("{:#?}", result);
}
