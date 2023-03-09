// use axum::{
//     async_trait,
//     extract::FromRequestParts,
//     http::{header::AUTHORIZATION, request::Parts, StatusCode},
// };

// struct ExtractBearer(String);

// #[async_trait]
// impl<S> FromRequestParts<S> for ExtractBearer
// where
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, &'static str);

//     async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//         if let Some(bearer) = parts.headers.get(AUTHORIZATION) {
//             if let Ok(bearer) = bearer.to_str() {
//                 if bearer.starts_with("Bearer ") {
//                     let token = &bearer[8..];
//                     return Ok(ExtractBearer(token.to_owned()));
//                 }
//             }
//         }

//         Err((
//             StatusCode::BAD_REQUEST,
//             "`Authorization` header is missing or invalid",
//         ))
//     }
// }
