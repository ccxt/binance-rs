use error_chain::bail;
use crate::errors::{BinanceContentError, ErrorKind, Result};
use crate::util::sign_hmac;
use reqwest::StatusCode;
use reqwest::blocking::Response;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use crate::api::API;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    host: String,
    inner_client: reqwest::blocking::Client,
    verbose: bool,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String) -> Self {
        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            host,
            inner_client: reqwest::blocking::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
            verbose: false,
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn set_host(&mut self, host: String) {
        self.host = host;
    }

    pub fn get_signed<T: DeserializeOwned>(
        &self, endpoint: API, request: Option<String>,
    ) -> Result<T> {
        let url = self.sign_request(endpoint, request);
        let headers = self.build_headers(true)?;
        if self.verbose {
            println!("Request URL: {}", url);
            println!("Request Headers: {:?}", headers);
        }
        let client = &self.inner_client;
        let response = client.get(url.as_str()).headers(headers).send()?;

        self.handler(response)
    }

    pub fn post_signed<T: DeserializeOwned>(&self, endpoint: API, request: String) -> Result<T> {
        let url = self.sign_request(endpoint, Some(request));
        let client = &self.inner_client;

        let headers = self.build_headers(true)?;
        if self.verbose {
            println!("Request URL: {}", url);
            println!("Request Headers: {:?}", headers);
        }
        let response = client.post(url.as_str()).headers(headers).send()?;

        self.handler(response)
    }

    pub fn put_signed<T: DeserializeOwned>(&self, endpoint: API, request: String) -> Result<T> {
        let url = self.sign_request(endpoint, Some(request));
        let client = &self.inner_client;

        let headers = self.build_headers(true)?;
        if self.verbose {
            println!("Request URL: {}", url);
            println!("Request Headers: {:?}", headers);
        }
        let response = client.put(url.as_str()).headers(headers).send()?;

        self.handler(response)
    }

    pub fn delete_signed<T: DeserializeOwned>(
        &self, endpoint: API, request: Option<String>,
    ) -> Result<T> {
        let url = self.sign_request(endpoint, request);
        let headers = self.build_headers(true)?;
        if self.verbose {
            println!("Request URL: {}", url);
            println!("Request Headers: {:?}", headers);
        }
        let client = &self.inner_client;
        let response = client.delete(url.as_str()).headers(headers).send()?;

        self.handler(response)
    }

    pub fn get<T: DeserializeOwned>(&self, endpoint: API, request: Option<String>) -> Result<T> {
        let mut url: String = format!("{}{}", self.host, String::from(endpoint));
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }

        let client = &self.inner_client;
        if self.verbose {
            println!("Request URL: {}", url);
        }
        let response = client.get(url.as_str()).send()?;

        self.handler(response)
    }

    /// Unsigned GET that still sends the `X-MBX-APIKEY` header.
    ///
    /// Used for `MARKET_DATA` endpoints such as `historicalTrades`, which require an
    /// API key but no signature.
    pub fn get_with_api_key<T: DeserializeOwned>(
        &self, endpoint: API, request: Option<String>,
    ) -> Result<T> {
        let mut url: String = format!("{}{}", self.host, String::from(endpoint));
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }

        let headers = self.build_headers(false)?;
        if self.verbose {
            println!("Request URL: {}", url);
            println!("Request Headers: {:?}", headers);
        }
        let client = &self.inner_client;
        let response = client.get(url.as_str()).headers(headers).send()?;

        self.handler(response)
    }

    pub fn post<T: DeserializeOwned>(&self, endpoint: API) -> Result<T> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));

        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(false)?)
            .send()?;

        self.handler(response)
    }

    pub fn put<T: DeserializeOwned>(&self, endpoint: API, listen_key: &str) -> Result<T> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));
        let data: String = format!("listenKey={}", listen_key);

        let client = &self.inner_client;

        let headers = self.build_headers(true)?;
        if self.verbose {
            println!("Request URL: {}", url);
            println!("Request Headers: {:?}", headers);
            println!("Request Body: {}", data);
        }
        let response = client
            .put(url.as_str())
            .headers(headers)
            .body(data)
            .send()?;

        self.handler(response)
    }

    pub fn delete<T: DeserializeOwned>(&self, endpoint: API, listen_key: &str) -> Result<T> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));
        let data: String = format!("listenKey={}", listen_key);

        let client = &self.inner_client;
        let response = client
            .delete(url.as_str())
            .headers(self.build_headers(false)?)
            .body(data)
            .send()?;

        self.handler(response)
    }

    // Request must be signed
    fn sign_request(&self, endpoint: API, request: Option<String>) -> String {
        let request = request.unwrap_or_default();
        let signature = sign_hmac(&self.secret_key, &request);
        let request_body: String = if request.is_empty() {
            format!("&signature={}", signature)
        } else {
            format!("{}&signature={}", request, signature)
        };
        format!("{}{}?{}", self.host, String::from(endpoint), request_body)
    }

    fn build_headers(&self, content_type: bool) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-rs"));
        if content_type {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );

        Ok(custom_headers)
    }

    fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        match response.status() {
            StatusCode::OK => {
                let headers = response.headers().clone();
                let response_bytes = response.bytes()?;

                if self.verbose {
                    println!("Response Headers: {:?}", headers);
                    let pretty =
                        serde_json::from_slice::<serde_json::Value>(&response_bytes).unwrap();
                    println!("Response: {}", pretty);
                }
                let json: T = serde_json::from_slice(&response_bytes)?;
                Ok(json)
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            StatusCode::BAD_REQUEST => {
                let error: BinanceContentError = response.json()?;

                Err(ErrorKind::BinanceError(error).into())
            }
            s => {
                bail!(format!("Received response: {:?}", s));
            }
        }
    }
}
