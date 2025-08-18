use anyhow::Result;
use reqwest::{Client, Method, Response};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone)]
pub struct GitHubRestApiClient {
    client: Client,
    base_url: String,
}

impl GitHubRestApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.github.com".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    #[allow(dead_code)]
    pub async fn get(
        &self,
        path: &str,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.request(Method::GET, path, None::<&()>, headers).await
    }

    #[allow(dead_code)]
    pub async fn post<T: Serialize>(
        &self,
        path: &str,
        body: Option<&T>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.request(Method::POST, path, body, headers).await
    }

    #[allow(dead_code)]
    pub async fn put<T: Serialize>(
        &self,
        path: &str,
        body: Option<&T>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.request(Method::PUT, path, body, headers).await
    }

    #[allow(dead_code)]
    pub async fn patch<T: Serialize>(
        &self,
        path: &str,
        body: Option<&T>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.request(Method::PATCH, path, body, headers).await
    }

    #[allow(dead_code)]
    pub async fn delete(
        &self,
        path: &str,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        self.request(Method::DELETE, path, None::<&()>, headers)
            .await
    }

    async fn request<T: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        let url = if path.starts_with("http") {
            path.to_string()
        } else {
            format!("{}{}", self.base_url, path)
        };
        log::info!("Requesting {url}");

        let mut request = self.client.request(method, &url);

        // Set default headers
        request = request
            .header("User-Agent", "reportify")
            .header("Accept", "application/vnd.github.v3+json");

        // Add custom headers
        if let Some(custom_headers) = headers {
            for (key, value) in custom_headers {
                request = request.header(key, value);
            }
        }

        // Add body if provided
        if let Some(body_data) = body {
            request = request.json(body_data);
        }

        let response = request.send().await?;
        log::info!("Response status: {:?}", response.status());
        log::info!("Response headers: {:?}", response.headers());
        Ok(response)
    }
}

impl Default for GitHubRestApiClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_client() {
        let client = GitHubRestApiClient::new();
        assert_eq!(client.base_url, "https://api.github.com");
    }

    #[test]
    fn test_with_base_url() {
        let custom_url = "https://api.github.enterprise.com".to_string();
        let client = GitHubRestApiClient::with_base_url(custom_url.clone());
        assert_eq!(client.base_url, custom_url);
    }
}
