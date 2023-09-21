#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]

use anyhow::{anyhow, Error};
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet, RSAKeyParameters};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Clone)]
pub struct Auth0 {
    client: Client,
    base_url: String,
}

impl Auth0 {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_auth0_id_from_bearer(&self, bearer_token: &str) -> Result<String, Error> {
        let id = self.validate_jwt_token(bearer_token).await?.sub;

        Ok(id)
    }

    async fn validate_jwt_token(&self, token: &str) -> Result<Claims, Error> {
        let jwks = self.fetch_jwks().await?;
        let kid = decode_header(token)?
            .kid
            .ok_or(anyhow!("Error getting kid from token"))?;
        let jwk = match jwks.find(&kid) {
            Some(jwk) => jwk,
            None => return Err(anyhow!("No JWK found for kid: {}", kid)),
        };

        match &jwk.algorithm {
            AlgorithmParameters::RSA(rsa) => Auth0::dec_jwt(rsa, token),
            _ => unreachable!("Unsupported algorithm"),
        }
    }

    async fn fetch_jwks(&self) -> Result<JwkSet, ReqwestError> {
        let url = format!("{}/.well-known/jwks.json", self.base_url);
        let res = self.client.get(&url).send().await?;
        let jwks = res.json::<JwkSet>().await?;

        Ok(jwks)
    }

    fn dec_jwt(rsa: &RSAKeyParameters, jwt: &str) -> Result<Claims, Error> {
        match decode::<Claims>(
            jwt,
            &DecodingKey::from_rsa_components(&rsa.n, &rsa.e)?,
            &Validation::new(Algorithm::RS256),
        ) {
            Ok(c) => Ok(c.claims),
            Err(e) => Err(anyhow!("Error decoding JWT: {:?}", e)),
        }
    }
}
