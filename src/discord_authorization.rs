use actix_web::dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::PayloadError;
use actix_web::http::{header, StatusCode};
use actix_web::web::{Bytes, BytesMut};
use actix_web::{Error, HttpMessage, HttpResponse, ResponseError};
use ed25519_dalek::PublicKey;
use async_std::prelude::*;
use log::{info};
use signature::Verifier;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::future::{ready, Ready};
use std::pin::Pin;
use std::rc::Rc;
use actix_web::body::BoxBody;
use actix_web::http::header::HeaderValue;
use async_std::stream;
use futures_util::future::LocalBoxFuture;

pub struct DiscordAuthorizationMiddleware<S> {
    service: Rc<RefCell<S>>,
    public_key: PublicKey,
}

#[derive(Debug)]
pub enum ServiceError {
    Unauthorized,
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ServiceError::Unauthorized => f.write_str("Unauthorized"),
        }
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut res = HttpResponse::new(self.status_code());
        res.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
        res.set_body(BoxBody::new("{}"))
    }
}

impl<S, B> Service<ServiceRequest> for DiscordAuthorizationMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let headers = req.headers();
        if let Some((signature, timestamp)) = headers
            .get("X-Signature-Ed25519")
            .and_then(|x| x.to_str().ok())
            .and_then(|x| hex::decode(x).ok())
            .and_then(|x| ed25519_dalek::Signature::from_bytes(x.as_slice()).ok())
            .zip(headers.get("X-Signature-Timestamp").map(|x| x.to_owned()))
        {
            let public_key = self.public_key;
            let svc = self.service.clone();
            Box::pin(async move {
                let mut request_body = BytesMut::new();
                let timestamp_bytes = timestamp.as_bytes();
                request_body.extend_from_slice(timestamp_bytes);
                let timestamp_offset = timestamp_bytes.len();
                // READ PAYLOAD
                while let Some(chunk) = req.take_payload().next().await {
                    request_body.extend_from_slice(&chunk?);
                }
                let body = request_body.freeze();
                if public_key.verify(&body, &signature).is_ok() {
                    info!("request authorized");

                    // RESTORE PAYLOAD
                    let orig_payload = body.slice(timestamp_offset..);
                    let single_part: Result<Bytes, PayloadError> = Ok(orig_payload);
                    let in_memory_stream = stream::once(single_part);
                    let pinned_stream: Pin<Box<dyn Stream<Item=Result<Bytes, PayloadError>>>> = Box::pin(in_memory_stream);
                    let in_memory_payload: Payload = pinned_stream.into();
                    req.set_payload(in_memory_payload);

                    Ok(svc.call(req).await?)
                } else {
                    info!("Request unauthorized");
                    Err(ServiceError::Unauthorized.into())
                }
            })
        } else {
            Box::pin(ready(Err(ServiceError::Unauthorized.into())))
        }
    }
}

#[derive(Copy, Clone)]
pub struct DiscordAuthorization {
    public_key: PublicKey,
}

impl DiscordAuthorization {
    pub fn new(public_key: PublicKey) -> Self {
        DiscordAuthorization { public_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for DiscordAuthorization
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = DiscordAuthorizationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(DiscordAuthorizationMiddleware {
            service: Rc::new(RefCell::new(service)),
            public_key: self.public_key,
        }))
    }
}
