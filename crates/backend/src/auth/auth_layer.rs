use axum::{http::{Request, StatusCode}, middleware::Next, response::Response, headers::{HeaderMapExt, Authorization, authorization::Bearer}};
use prisma::utenti::WhereParam;

use crate::{entities::UtentiEntity, get_prisma_client, utils::api_error::APIError};

use super::jwt::decode_jwt;






pub async fn guard<T>(mut req: Request<T>, next: Next<T>) -> Result<Response,APIError> {
    let token = req.headers().typed_get::<Authorization<Bearer>>()
    .ok_or(APIError { message: "No Auth token found".to_owned(), status_code: StatusCode::BAD_REQUEST, error_code: Some(401)  })?.token().to_owned();

    let claim = decode_jwt(token)
    .map_err(|err| APIError { message: "Unauthorized".to_owned(), status_code: StatusCode::UNAUTHORIZED, error_code: Some(403)  })?.claims;
    let username = claim.email;
    let prisma = get_prisma_client().await.expect("Error in instatiate prisma client");

    let user = prisma
        .utenti()
        .find_first(vec![
            WhereParam::Username(prisma::read_filters::StringFilter::Equals(username))])
        .exec()
        .await
        .map_err(|err| APIError { message: err.to_string(), status_code: StatusCode::NOT_FOUND, error_code: Some(404) })?;

    let user: UtentiEntity = user.unwrap();
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}