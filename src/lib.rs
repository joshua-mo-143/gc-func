use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Method, Request, Response, StatusCode,
};
use serde::{Deserialize, Serialize};

const PHRASE: &str = "Hello from RUST by GreenCloud!";

#[derive(Deserialize, Serialize, Debug)]
struct Thing {
    message: String,
}

pub async fn handle(req: Request<Body>) -> Result<Response<Body>, ApiError> {
    let (parts, body) = req.into_parts();

    match parts.method {
        Method::POST => {}
        _ => return Ok(Resp::NotPostMethod.into()),
    }

    let body = hyper::body::to_bytes(body).await?;

    let thing: Thing = match serde_json::from_slice(&body) {
        Ok(res) => res,
        Err(e) => return Ok(Resp::SerdeJsonError(e).into()),
    };

    Ok(Resp::OkThing(thing).into())
}

#[derive(Debug)]
pub enum ApiError {
    Hyper(hyper::Error),
}

impl std::error::Error for ApiError {}

impl From<hyper::Error> for ApiError {
    fn from(e: hyper::Error) -> Self {
        Self::Hyper(e)
    }
}

use std::fmt;

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hyper(err) => write!(f, "hyper error: {err}"),
        }
    }
}

#[derive(Debug)]
enum Resp {
    OkThing(Thing),
    NotPostMethod,
    SerdeJsonError(serde_json::Error),
}

impl From<serde_json::Error> for Resp {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJsonError(e)
    }
}

impl From<Resp> for Response<Body> {
    fn from(resp: Resp) -> Response<Body> {
        let (response_text, content_type, status_code) = match resp {
            Resp::OkThing(thing) => (
                serde_json::to_string_pretty(&thing).unwrap().into_bytes(),
                HeaderValue::from_static("application/json"),
                StatusCode::OK,
            ),
            Resp::NotPostMethod => (
                b"This endpoint only accepts POST methods!".to_vec(),
                HeaderValue::from_static("text/plain"),
                StatusCode::METHOD_NOT_ALLOWED,
            ),

            Resp::SerdeJsonError(err) => (
                format!("serde_json error: {err}").into_bytes(),
                HeaderValue::from_static("text/plain"),
                StatusCode::METHOD_NOT_ALLOWED,
            ),
        };

        let mut response = Response::new(Body::from(response_text));

        response.headers_mut().insert(CONTENT_TYPE, content_type);

        *response.status_mut() = status_code;

        response
    }
}
