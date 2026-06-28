use serde::de::DeserializeOwned;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Conversion of request to json failed: {0}")]
    ReqConv(#[from] serde_json::Error),
    #[error("Response failed {0}")]
    RespError(#[from] reqwasm::Error),
}

pub const BACKEND_PREFIX: &str = std::env!("BACKEND_PREFIX");

pub async fn send_json<R: DeserializeOwned>(
    suffix: &str,
    body: impl serde::Serialize,
) -> Result<R, ApiError> {
    let body = serde_json::to_string(&body)?;
    log::info!("{BACKEND_PREFIX}/{suffix}");
    let response = reqwasm::http::Request::post(&format!("{BACKEND_PREFIX}/{}", suffix))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await?
        .json::<R>()
        .await?;
    Ok(response)
}
