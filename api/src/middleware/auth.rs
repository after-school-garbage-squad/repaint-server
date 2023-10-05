use axum::http::{header, Request, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::Json;
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet, RSAKeyParameters};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use repaint_server_util::envvar_str;
use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    status: &'static str,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn auth<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = parse_bearer_token(&req)?;
    let authority = envvar_str("AUTHORITY", None);
    let subject = get_auth0_id_from_bearer(token, authority).await?;
    req.extensions_mut().insert(subject);

    Ok(next.run(req).await)
}

fn parse_bearer_token<B>(req: &Request<B>) -> Result<&str, (StatusCode, Json<ErrorResponse>)> {
    let auth_value = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                status: "fail",
                message: "Authorized header is missed".to_string(),
            }),
        ))?;
    let token = match auth_value.starts_with("Bearer ") {
        true => auth_value.trim_start_matches("Bearer "),
        false => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    status: "fail",
                    message: "Invalid authorization header".to_string(),
                }),
            ))
        }
    };

    Ok(token)
}

async fn get_auth0_id_from_bearer(
    bearer_token: &str,
    base_url: String,
) -> Result<String, (StatusCode, Json<ErrorResponse>)> {
    let id = validate_jwt_token(bearer_token, base_url).await?.sub;

    Ok(id)
}

async fn validate_jwt_token(
    token: &str,
    base_url: String,
) -> Result<Claims, (StatusCode, Json<ErrorResponse>)> {
    let jwks = fetch_jwks(base_url).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                status: "fail",
                message: format!("Failed to fetch jwks: {}", e),
            }),
        )
    })?;
    let kid = decode_header(token)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    status: "fail",
                    message: format!("Failed to decode kid from token: {}", e),
                }),
            )
        })?
        .kid
        .ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                status: "fail",
                message: "Failed to get kid".to_string(),
            }),
        ))?;
    let jwk = match jwks.find(&kid) {
        Some(jwk) => jwk,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    status: "fail",
                    message: format!("No JWK found for kid: {}", kid),
                }),
            ))
        }
    };

    match &jwk.algorithm {
        AlgorithmParameters::RSA(rsa) => dec_jwt(rsa, token),
        _ => unreachable!("Unsupported algorithm"),
    }
}

async fn fetch_jwks(base_url: String) -> Result<JwkSet, ReqwestError> {
    let url = format!("{}/.well-known/jwks.json", base_url);
    let client = Client::new();
    let res = client.get(&url).send().await?;
    let jwks = res.json::<JwkSet>().await?;

    Ok(jwks)
}

fn dec_jwt(rsa: &RSAKeyParameters, jwt: &str) -> Result<Claims, (StatusCode, Json<ErrorResponse>)> {
    match decode::<Claims>(
        jwt,
        &DecodingKey::from_rsa_components(&rsa.n, &rsa.e).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    status: "fail",
                    message: format!("Failed to decode key: {}", e),
                }),
            )
        })?,
        &Validation::new(Algorithm::RS256),
    ) {
        Ok(c) => Ok(c.claims),
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                status: "fail",
                message: format!(
                    "Failed to decoding JWT or token is already expired: {:?}",
                    e
                ),
            }),
        )),
    }
}
