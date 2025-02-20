use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    Error, HttpResponse,
};
use futures::future::{ok, Ready};
use futures::Future;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::auth::dtos::Claims;

pub struct AuthMiddleware {
    jwt_secret: String,
}

impl AuthMiddleware {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service,
            jwt_secret: self.jwt_secret.clone(),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    jwt_secret: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(_stripped) = auth_str.strip_prefix("Bearer ") {
                    let token = auth_str[7..].to_string();
                    if decode::<Claims>(
                        &token,
                        &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
                        &Validation::default(),
                    )
                    .is_ok()
                    {
                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?;
                            Ok(res.map_into_left_body())
                        });
                    }
                }
            }
        }

        let (http_req, _) = req.into_parts();
        let res = HttpResponse::build(StatusCode::UNAUTHORIZED).body("Unauthorized");

        Box::pin(async move { Ok(ServiceResponse::new(http_req, res).map_into_right_body()) })
    }
}
