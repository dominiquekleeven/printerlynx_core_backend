use std::env;

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use tracing::error;

/// Generates a JWT token based on the claims
pub fn generate_token(claims: Claims) -> String {
    let signing_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let encoding_key = EncodingKey::from_secret(signing_key.as_bytes());
    encode(&Header::default(), &claims, &encoding_key).unwrap()
}

/// Decodes a JWT token and returns the claims
pub fn decode_token(token: &str) -> Result<TokenData<Claims>, ErrorKind> {
    let signing_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(signing_key.as_bytes());

    match decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256)) {
        Ok(token_data) => Ok(token_data),
        Err(err) => {
            error!("Error decoding token: {:?}", err);
            Err(ErrorKind::InvalidToken)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) exp: usize,
    pub(crate) iss: String,
    pub(crate) sub: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_generate_token() {
        env::set_var("JWT_SECRET", "4L9wP7nRyQs2F6vZ8KcGtA1DxH5eE3jY");

        let claims = Claims {
            exp: 0,
            iss: "Printerlynx".to_string(),
            sub: "test".to_string(),
        };

        let token = generate_token(claims);
        assert!(!token.is_empty());
    }

    #[test]
    fn test_decode_token() {
        env::set_var("JWT_SECRET", "4L9wP7nRyQs2F6vZ8KcGtA1DxH5eE3jY");

        let claims = Claims {
            exp: Utc::now().timestamp() as usize + 31536000,
            iss: "Printerlynx".to_string(),
            sub: "test".to_string(),
        };

        let token = generate_token(claims);
        let decoded_token = decode_token(&token);
        assert!(decoded_token.is_ok());
    }
}
