use eyre::{Context, Result};
use serde::de::DeserializeOwned;

/// Initialises the application's [`reqwest::Client`].
pub fn init() -> Result<reqwest::Client> {
    let agent = format!("katabasis {}", env!("CARGO_PKG_VERSION"));

    let http = reqwest::Client::builder()
        .user_agent(&agent)
        .build()
        .context("Failed to initialise application http client")?;

    Ok(http)
}

const DEFAULT_RETRY_ATTEMPTS: u8 = 5;

/// Sends a request of the provided type to the provided URL. Uses
/// the passed in [`reqwest::Client`], with a default number of retries.
pub async fn fetch_url(
    method: reqwest::Method,
    url: &str,
    client: &reqwest::Client,
) -> Result<reqwest::Response> {
    for i in 0..DEFAULT_RETRY_ATTEMPTS {
        let request = client.request(method.clone(), url);
        let result = request.send().await;

        match result {
            Ok(response) => return Ok(response),
            Err(err) if i <= DEFAULT_RETRY_ATTEMPTS => {
                tracing::warn!(
                    "Failed to execute request on attempt {}/{}: {}",
                    i + 1,
                    DEFAULT_RETRY_ATTEMPTS,
                    err
                );
            }
            Err(error) => return Err(error.into()),
        }
    }

    unreachable!()
}

/// Fetches and parses a JSON response from the provided URL.
pub async fn fetch_json<T>(url: &str, client: &reqwest::Client) -> Result<T>
where
    T: DeserializeOwned,
{
    let response = fetch_url(reqwest::Method::GET, url, client).await?;

    Ok(response.json().await?)
}

/// Fetches a stream of bytes from the provided URL.
pub async fn fetch_stream(
    url: &str,
    client: &reqwest::Client,
) -> Result<impl futures::Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin> {
    let response = fetch_url(reqwest::Method::GET, url, client).await?;

    Ok(response.bytes_stream())
}

#[cfg(test)]
mod tests {
    use futures::TryStreamExt;
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, PartialEq, Deserialize)]
    struct JsonApiTodo {
        #[serde(rename = "userId")]
        user_id: i64,
        id: i64,
        title: String,
        completed: bool,
    }

    #[tokio::test]
    async fn test_fetch_json() {
        let client = init().unwrap();

        let todo: JsonApiTodo = fetch_json("https://jsonplaceholder.typicode.com/todos/1", &client)
            .await
            .unwrap();

        assert_eq!(todo.user_id, 1);
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "delectus aut autem");
        assert_eq!(todo.completed, false);
    }

    #[tokio::test]
    async fn test_fetch_stream() {
        let client = init().unwrap();

        let stream = fetch_stream("https://jsonplaceholder.typicode.com/todos/1", &client)
            .await
            .unwrap();

        let bytes = stream.try_collect::<Vec<_>>().await.unwrap();
        let as_slice = bytes
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>()
            .concat();

        let todo = serde_json::from_slice::<JsonApiTodo>(&as_slice).unwrap();

        assert_eq!(todo.user_id, 1);
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "delectus aut autem");
        assert_eq!(todo.completed, false);
    }
}
