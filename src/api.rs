use serde::de::DeserializeOwned;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Conversion of request to json failed: {0}")]
    ReqConv(#[from] serde_json::Error),
    #[error("Response failed {0}")]
    RespError(#[from] reqwasm::Error),
    #[error("Response didn't contain json: status: {0:?}, text: {1:?}, error: {2:?}")]
    NoJsonError(u16, String, Option<serde_json::Error>),
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum SharableError {
    #[error("Conversion of request to json failed: {0}")]
    ReqConv(String),
    #[error("Response failed {0}")]
    RespError(String),
    #[error("Response didn't contain json: status: {status:?}, text: {text:?}, error: {err:?}")]
    NoJsonError {
        status: u16,
        text: String,
        err: String,
    },
}

impl ApiError {
    pub fn into_sharable(&self) -> SharableError {
        match self {
            Self::ReqConv(err) => SharableError::ReqConv(err.to_string()),
            Self::RespError(err) => SharableError::RespError(err.to_string()),
            Self::NoJsonError(status, text, err) => SharableError::NoJsonError {
                status: *status,
                text: text.to_string(),
                err: err.as_ref().map(|s| s.to_string()).unwrap_or_default(),
            },
        }
    }
}

pub const BACKEND_PREFIX: &str = std::env!("BACKEND_PREFIX");

fn make_host() -> String {
    let host = BACKEND_PREFIX.trim_end_matches("/");
    let lowercase = host.to_lowercase();
    if lowercase.starts_with("http://")
        || lowercase.starts_with("https://")
        || std::option_env!("NO_SMART_BACKEND_PREFIX").unwrap_or("0") == "1"
    {
        host.to_string()
    } else {
        format!("http://{host}")
    }
}

pub async fn send_json_post_json<R: DeserializeOwned>(
    suffix: &str,
    body: impl serde::Serialize,
) -> Result<R, ApiError> {
    let body = serde_json::to_string(&body)?;
    let host = make_host();
    log::info!("{host}/{suffix}");

    let response = reqwasm::http::Request::post(&format!("{host}/{}", suffix))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await?;
    let text = response.text().await?;
    let json = serde_json::from_str::<R>(&text);

    match json {
        Ok(r) => Ok(r),
        Err(err) => Err(ApiError::NoJsonError(response.status(), text, Some(err))),
    }
}

pub async fn send_json_get_status(suffix: &str) -> Result<u16, reqwasm::Error> {
    let host = make_host();
    log::info!("{host}/{suffix}");
    let response = reqwasm::http::Request::get(&format!("{host}/{}", suffix))
        .header("content-type", "application/json")
        .send()
        .await?
        .status();
    Ok(response)
}
