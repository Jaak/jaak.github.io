use hyper_tls::HttpsConnector;
use hyper::{Body, Client, Request};
use hyper::body::HttpBody;
use hyper::client::{HttpConnector};
use std::time::{Duration};
use tower::{Service, ServiceBuilder};
use tower::timeout::Timeout;

type HyperClient = Client<HttpsConnector<HttpConnector>>;
fn make_vanilla_service() -> HyperClient {
    let https = HttpsConnector::new();
    Client::builder().build::<_, hyper::Body>(https)
}

type Svc = Timeout<HyperClient>;
fn make_service() -> Svc {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    ServiceBuilder::new()
        .timeout(Duration::from_secs(5))
        .service(client)
}

/*

error[E0277]: the size for values of type `dyn std::error::Error + Send + Sync` cannot be known at compilation time
  --> src/main.rs:44:43
   |
44 |     let mut resp = service.call(req).await?;
   |                                           ^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `dyn std::error::Error + Send + Sync`
   = note: required because of the requirements on the impl of `std::error::Error` for `Box<dyn std::error::Error + Send + Sync>`
   = note: required because of the requirements on the impl of `From<Box<dyn std::error::Error + Send + Sync>>` for `anyhow::Error`
   = note: required by `from`

*/

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let mut service = make_vanilla_service(); // XXX: works
    let mut service = make_service(); // XXX: does not compile
    let req = Request::builder().uri("https://httpbin.org/get").body(Body::empty())?;
    let mut resp = service.call(req).await?;
    let mut data = Vec::new();
    while let Some(next) = resp.body_mut().data().await {
        let chunk = next?;
        data.extend_from_slice(&chunk);
    }

    println!("Respose body: \"{}\"", String::from_utf8(data).unwrap_or_default());
    Ok(())
}
