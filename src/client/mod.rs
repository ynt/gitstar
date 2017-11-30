use std::io;
use tokio_core::reactor::Core;

use futures::{Future, Stream};
use hyper::{Client, Method, Request, Error};
use hyper_tls::HttpsConnector;
use hyper::header::UserAgent;

use serde_json;
use serde_json::Value;


pub fn get(url: String) -> Result<Value, Error> {
    let mut core = Core::new()?;
    let handle = core.handle();

    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).expect("http tls error."))
        .build(&handle);

    let mut req: Request = Request::new(Method::Get, url.parse()?);
    req.headers_mut().set(UserAgent::new("User-Agent:Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 Safari/537.36"));

    let work = client.request(req).and_then(|res| res.body().concat2());

    let res = core.run(work)?;
    let v: Value = serde_json::from_slice(&res).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e)
    })?;

    Ok(v)
}
