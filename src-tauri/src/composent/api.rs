use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use std::time::Duration;
use crate::core::error::ApiError;
use crate::models::{ApiData};
use super::config::ApiConfig;



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
) -> Result<Option<ApiData>, ApiError> {
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
            ("language", "en".to_string()),
        ])
        .send()?;

    // Gestion spécifique du statut 401
    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(ApiError::AuthError("Invalid API token".to_string()));
    }


    // Vérification des autres erreurs HTTP
    let response = response.error_for_status()?;

    // Désérialisation de la réponse
    let locations: Vec<ApiData> = response.json()?;
    println!("Locations: {:?}", locations);
    Ok(locations.into_iter().next())
}
