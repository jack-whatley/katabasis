use bytes::Bytes;
use log::{error, warn};
use reqwest::{Method, Response};
use serde::de::DeserializeOwned;
use thiserror::Error;
use crate::utils::NetSemaphore;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Acquiring Semaphore Error: {0}")]
    AcquireError(#[from] tokio::sync::AcquireError),

    #[error("Unsuccessful Status Code Error: {0}")]
    StatusCodeError(String),

    #[error("Json Parse Error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

#[tracing::instrument]
async fn fetch_url(
    method: Method,
    url: &str,
    semaphore: &NetSemaphore,
    client: &reqwest::Client,
    retry_attempts: i64,
) -> Result<Response, HttpError> {
    let _permit = semaphore.0.acquire().await?;

    for i in 0..=retry_attempts {
        let request = client.request(method.clone(), url);
        let result = request.send().await;

        match result {
            Ok(res) => {
                if !res.status().is_success() {
                    if i <= retry_attempts {
                        warn!("Did not receive a successful HTTP code, retrying: {:#?}", res.status());
                    }
                    else {
                        error!("Did not receive a successful HTTP code after using all retries: {:#?}", res.status());
                        return Err(
                            HttpError::StatusCodeError(
                                format!("Did not receive a successful HTTP code after using all retries: {:#?}", res.status())
                            )
                        )
                    }
                }

                return Ok(res)
            }
            Err(err) if i <= retry_attempts => {
                warn!("Failed to fetch URL on attempt {}:\n{:#?}", i, err);
                continue;
            }
            Err(err) => {
                error!("Failed to fetch URL after using all attempts:\n{:#?}", err);
                return Err(err.into());
            }
        }
    }

    unreachable!()
}

pub async fn fetch_stream(
    url: &str,
    semaphore: &NetSemaphore,
    client: &reqwest::Client,
    retry_attempts: i64,
) -> Result<impl futures::Stream<Item=Result<Bytes, reqwest::Error>> + Unpin, HttpError> {
    let response = fetch_url(
        Method::GET,
        url,
        semaphore,
        client,
        retry_attempts,
    ).await?;

    Ok(response.bytes_stream())
}

pub async fn fetch_json<T>(
    url: &str,
    semaphore: &NetSemaphore,
    client: &reqwest::Client,
    retry_attempts: i64,
) -> Result<T, HttpError> where T: DeserializeOwned {
    let response = fetch_url(
        Method::GET,
        url,
        semaphore,
        client,
        retry_attempts,
    ).await?;

    Ok(response.json().await?)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use tokio::sync::Semaphore;
    use crate::state::build_reqwest_client;
    use crate::utils::NetSemaphore;
    use super::fetch_json;

    #[derive(Debug, PartialEq, Deserialize)]
    struct JsonApiTodo {
        #[serde(rename(deserialize = "userId"))]
        user_id: i64,
        id: i64,
        title: String,
        completed: bool,
    }

    impl Default for JsonApiTodo {
        fn default() -> Self {
            Self {
                user_id: 1,
                id: 1,
                title: "delectus aut autem".to_owned(),
                completed: false,
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_json() {
        let net_semaphore = NetSemaphore(Semaphore::new(10));
        let client = match build_reqwest_client() {
            Ok(client) => client,
            Err(e) => {
                println!("Failed to create client: {:#?}", e);
                assert!(false);

                return;
            }
        };

        let fetched = fetch_json::<JsonApiTodo>(
            "https://jsonplaceholder.typicode.com/todos/1",
            &net_semaphore,
            &client,
            5
        ).await.unwrap();

        assert_eq!(fetched, JsonApiTodo::default());
    }
}
