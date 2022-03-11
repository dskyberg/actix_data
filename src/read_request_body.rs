use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web::{BytesMut},
    Error, HttpMessage,
};
use actix_http::h1::Payload;
use futures_util::{future::LocalBoxFuture, stream::StreamExt};

use crate::my_obj::MyObj;

pub struct Logging;

impl<S: 'static, B> Transform<S, ServiceRequest> for Logging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct LoggingMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
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
            log::info!("{:?}",&obj);
            
            let (_, mut payload) = Payload::create(true);
            payload.unread_data(body.into());
            req.set_payload(payload.into());

            let res = svc.call(req).await?;

            Ok(res)
        })
    }
}