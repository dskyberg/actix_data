use actix_http::h1::Payload;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web::BytesMut,
    Error, HttpMessage,
};
use futures_util::{future::LocalBoxFuture, stream::StreamExt};
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::my_obj::MyObj;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct PayloadAccessor;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S: 'static, B> Transform<S, ServiceRequest> for PayloadAccessor
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PayloadAccessorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PayloadAccessorMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct PayloadAccessorMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for PayloadAccessorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();

            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }

            let obj = serde_json::from_slice::<MyObj>(&body)?;
            log::info!("{:?}", &obj);

            let (_, mut payload) = Payload::create(true);
            payload.unread_data(body.into());
            req.set_payload(payload.into());

            let res = svc.call(req).await?;

            Ok(res)
        })
    }
}
