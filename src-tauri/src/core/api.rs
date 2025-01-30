use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

use super::config::{ApiConfig, BotConfig};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid header: {0}")]
    InvalidHeader(String),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize)]
pub struct LocationData {
    pub pos_x: i32,
    pub pos_y: i32,
    pub distance: i32,
}

fn get_headers(config: ApiConfig) -> Result<HeaderMap, ApiError> {
    let mut headers = HeaderMap::new();

    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", config.token))
            .map_err(|e| ApiError::InvalidHeader(format!("Invalid token: {}", e)))?,
    );

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    Ok(headers)
}

pub fn find_next_location(
    config: ApiConfig,
    x: i32,
    y: i32,
    direction: &str,
    hint: &str,
) -> Result<Option<LocationData>, ApiError> {
    let url = format!("{}/api/treasure-hunt", config.url);
    let headers = get_headers(config)?;

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let response = client
        .get(&url)
        .headers(headers)
        .query(&[
            ("x", x.to_string()),
            ("y", y.to_string()),
            ("direction", direction.to_string()),
            ("hint", hint.to_string()),
        ])
        .send()?;

    // Gestion spécifique du statut 401
    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(ApiError::AuthError("Invalid API token".to_string()));
    }


    // Vérification des autres erreurs HTTP
    let response = response.error_for_status()?;

    // Désérialisation de la réponse
    let locations: Vec<LocationData> = response.json()?;
    println!("Locations: {:?}", locations);
    Ok(locations.into_iter().next())
}
