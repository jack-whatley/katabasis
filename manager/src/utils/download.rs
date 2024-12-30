use bytes::Bytes;
use lazy_static::lazy_static;
use reqwest::Method;
use thiserror::Error;
use crate::storage::NetSemaphore;

lazy_static! {
    pub static ref REQWEST_CLIENT: reqwest::Client = {
        let mut headers = reqwest::header::HeaderMap::new();

        let header = reqwest::header::HeaderValue::from_str(
            format!("katabasis {}", env!("CARGO_PKG_VERSION")).as_str()
        ).unwrap();

        headers.insert(reqwest::header::USER_AGENT, header);

        reqwest::Client::builder()
            .tcp_keepalive(Some(std::time::Duration::from_secs(10)))
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest client")
    };
}

#[derive(Error, Debug)]
pub enum HttpRequestError {
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Acquiring Semaphore Error: {0}")]
    AcquireError(#[from] tokio::sync::AcquireError),

    #[error("Server Error: {0}")]
    ServerError(String),

    #[error("Json Parse Error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

pub(crate) type HttpResult<T> = Result<T, HttpRequestError>;

// Should possibly make this configurable?
const FETCH_ATTEMPTS: usize = 3;

/// Perform a REST operation on a provided URL using reqwest and return the bytes from that operation.
pub async fn fetch_url(
    method: reqwest::Method,
    url: &str,
    semaphore: &NetSemaphore
) -> HttpResult<Bytes> {
    let _permit = semaphore.0.acquire().await?;

    for attempt in 0..FETCH_ATTEMPTS + 1 {
        let request = REQWEST_CLIENT.request(method.clone(), url);
        let result = request.send().await;

        match result {
            Ok(res) => {
                if res.status().is_server_error() {
                    if attempt <= FETCH_ATTEMPTS { continue; }
                    else {
                        return Err(
                            HttpRequestError::ServerError(
                                format!("Failed to fetch from URL '{}' due to server error", url)
                            )
                        )
                    }
                }

                let bytes = res.bytes().await;

                if let Ok(bytes) = bytes {
                    // Use for any future placement of extra steps (like validation)
                    return Ok(bytes);
                }
                else if attempt <= FETCH_ATTEMPTS { continue; }
                else if let Err(err) = bytes { return Err(err.into()) }
            }
            Err(_) if attempt <= FETCH_ATTEMPTS => continue,
            Err(err) => {
                return Err(err.into());
            }
        }
    }

    unreachable!()
}

pub async fn fetch_json(
    url: &str,
    semaphore: &NetSemaphore
) -> HttpResult<serde_json::Value> {
    let fetched_bytes = fetch_url(Method::GET, url, semaphore).await?;

    Ok(serde_json::from_slice(&fetched_bytes)?)
}
