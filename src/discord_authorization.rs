use std::future::{Ready, ready};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::body::MessageBody;
use log::info;

pub struct DiscordAuthorizationMiddleware<S> {
    service: S,
    public_key: ed25519_dalek::PublicKey,
}

impl<S: Service<ServiceRequest, Response=ServiceResponse<B>>, B: MessageBody> Service<ServiceRequest> for DiscordAuthorizationMiddleware<S> {
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        info!("Hit!");
        self.service.call(req)
    }
}

#[derive(Copy, Clone)]
pub struct DiscordAuthorization {
    pub public_key: ed25519_dalek::PublicKey,
}

impl<S: Service<ServiceRequest, Response=ServiceResponse<B>>, B: MessageBody> Transform<S, ServiceRequest> for DiscordAuthorization {
    type Response = S::Response;
    type Error = S::Error;
    type Transform = DiscordAuthorizationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(DiscordAuthorizationMiddleware {
            service,
            public_key: self.public_key,
        }))
    }
}

//     if let Some((signature, timestamp)) = headers.get("X-Signature-Ed25519")
//         .and_then(|x| x.to_str().ok())
//         .and_then(|x| hex::decode(x).ok())
//         .and_then(|x| ed25519_dalek::Signature::from_bytes(x.as_slice()).ok())
//         .zip(headers.get("X-Signature-Timestamp")) {
//         info!("Timestamp: {:#?}", timestamp.to_str());
//         info!("Signature: {:#?}", signature.to_string());
//         let body_bytes = {
//             let mut bytes = web::BytesMut::new();
//             bytes.extend_from_slice(timestamp.as_bytes());
//             while let Some(item) = payload.next().await {
//                 bytes.extend_from_slice(&item?);
//             }
//             bytes
//         };
//         info!("Bytes count: {}", body_bytes.len());
//         info!("Concatenated body: {:#?}", String::from_utf8(body_bytes.to_vec()));
//         let verification = public_key.verify(&body_bytes, &signature);
//         info!("Signature validated {:#?}", verification);
//     }