use actix_web::{dev::{ServiceRequest, ServiceResponse, Transform, forward_ready}, Error, HttpMessage};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::task::{Context, Poll};
use std::pin::Pin;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::utils::auth::Claims;
use std::rc::Rc;
use crate::models::User;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: actix_service::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service: Rc::new(service) })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> actix_service::Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: actix_service::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let headers = req.headers().clone();
        let token_opt = headers.get("Authorization").and_then(|h| h.to_str().ok());

        if let Some(token) = token_opt {
            if let Some(token) = token.strip_prefix("Bearer ") {
                let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                let decoded = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(jwt_secret.as_bytes()),
                    &Validation::default(),
                );

                if decoded.is_ok() {
                    return Box::pin(service.call(req));
                }
            }
        }

        Box::pin(async {
            use actix_web::HttpResponse;
            Ok(req.into_response(HttpResponse::Unauthorized().body("Invalid or missing token").map_into_right_body(),))
        })
    }
}