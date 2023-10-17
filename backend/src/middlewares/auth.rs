use axum::{
    http::{header::COOKIE, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use cookie::{Cookie, CookieJar};

pub async fn auth_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, impl IntoResponse> {
    let headers = request.headers().clone();

    let cookie_header = headers.get(COOKIE).unwrap();
    let cookie_string = cookie_header.to_str().unwrap();
    let cookie = Cookie::parse(cookie_string).unwrap();

    let mut jar = CookieJar::new();
    jar.add_original(cookie);

    // if let Some(cookie) = jar.get("token") {
    //     println!("Token: {}", cookie.value());
    // } else {
    //     // No token found in cookies. Return an error.
    //     return Err(StatusCode::UNAUTHORIZED);
    // }

    // // Extract cookies header from the request
    // if let Some(cookie_header) = headers.get(COOKIE) {
    //     if let Ok(cookie_str) = cookie_header.to_str() {
    //         // Parse cookies using CookieJar
    //         let cookie = Cookie::parse(cookie_str).unwrap();

    //         let mut jar = CookieJar::new();
    //         jar.add_original(cookie);

    //         // Get your token cookie
    //         if let Some(cookie) = jar.get("token") {
    //             println!("Token: {}", cookie.value());
    //         } else {
    //             // No token found in cookies. Return an error.
    //             return Err(StatusCode::UNAUTHORIZED);
    //         }
    //     }
    // } else {
    //     // No cookies found in the request. Return an error.
    //     return Err(StatusCode::UNAUTHORIZED);
    // }

    let response = next.run(request).await;

    Ok(response)
}
