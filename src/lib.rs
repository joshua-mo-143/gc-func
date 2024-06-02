use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    rt::{Future, Stream},
};
use hyper::{Body, Request, Response};
use serde::{
    de::{Deserializer, Error, Visitor},
    Deserialize,
};
use std::fmt;

const PHRASE: &str = "Hello from RUST by GreenCloud!";

#[derive(Deserialize)]
struct Thing {
    message: String,
}

//struct StringVis;
//
//impl<'de> Visitor<'de> for StringVis {
//    type Value = String;
//
//    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//        formatter.write_str("a UTF-8 string")
//    }
//
//    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
//    where
//        E: Error,
//    {
//        Ok(value)
//    }
//}
//
//impl<'de> Deserialize<'de> for Thing {
//    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//    where
//        D: Deserializer<'de>,
//    {
//        let message = deserializer.deserialize_string(StringVis)?;
//
//        Ok(Self { message })
//    }
//}

pub fn handle(req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    println!("Hello world!");
    let body = req
        .into_body()
        .concat2()
        .wait()
        .unwrap()
        .into_bytes()
        .to_vec();

    let _meme: Thing = serde_json::from_slice(&body).unwrap();

    let mut response = Response::new(Body::from(PHRASE));

    let content_type_header = HeaderValue::from_static("text/plain");
    response
        .headers_mut()
        .insert(CONTENT_TYPE, content_type_header);

    Ok(response)
}
