use crate::error::*;
use crate::response::*;
use crate::token::*;
use crate::writer::*;
use reqwest::header::HeaderMap;

const DEFAULT_URL: &str = "http://localhost:8080";

#[derive(Debug, Clone)]
pub struct ClientBuilder {
    url: String,
    read_token: Option<String>,
    write_token: Option<String>,
    http: Option<reqwest::Client>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
            read_token: None,
            write_token: None,
            http: None,
        }
    }
}

impl ClientBuilder {
    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn read_token(mut self, token: &str) -> Self {
        self.read_token = Some(token.to_string());
        self
    }

    pub fn write_token(mut self, token: &str) -> Self {
        self.write_token = Some(token.to_string());
        self
    }

    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http = Some(client);
        self
    }

    pub fn build(self) -> Result<Client> {
        let base_url = url::Url::parse(&self.url)?;
        let http = self.http.unwrap_or_default();

        Ok(Client {
            base_url,
            http,
            read_token: self.read_token.map(Token::new),
            write_token: self.write_token.map(Token::new),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: url::Url,
    http: reqwest::Client,
    read_token: Option<Token>,
    write_token: Option<Token>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn get_writer(&self) -> Writer {
        let update_url = self
            .base_url
            .join("/api/v0/update")
            .expect("valid update URL");
        Writer::new(
            self.http.clone(),
            update_url,
            self.write_token
                .clone()
                .expect("write_token is required for get_writer()"),
        )
    }

    pub async fn exec(&self, script: &str) -> Result<ExecResponse> {
        let exec_url = self.base_url.join("/api/v0/exec").expect("valid exec URL");

        let response = self
            .http
            .post(exec_url)
            .header("content-type", "text/plain; charset=utf-8")
            .body(script.to_string())
            .send()
            .await?;

        let status = response.status();
        let headers = response.headers().clone();

        if status.is_success() {
            let meta = ExecMeta {
                elapsed: header_str(&headers, "x-warp10-elapsed"),
                ops: header_str(&headers, "x-warp10-ops"),
                fetched: header_str(&headers, "x-warp10-fetched"),
            };
            let body: serde_json::Value = response.json().await?;
            Ok(ExecResponse { body, meta })
        } else {
            let message = header_str(&headers, "x-warp10-error-message")
                .unwrap_or_else(|| status.to_string());
            let line = header_str(&headers, "x-warp10-error-line");
            let position = header_str(&headers, "x-warp10-error-position");
            Err(Error::Exec(ExecErrorDetail {
                message,
                line,
                position,
            }))
        }
    }

    pub async fn find(&self, selector: &str) -> Result<FindResponse> {
        let mut find_url = self.base_url.join("/api/v0/find").expect("valid find URL");
        find_url.query_pairs_mut().append_pair("selector", selector);

        let token = self
            .read_token
            .as_ref()
            .expect("read_token is required for find()");

        let mut headers = HeaderMap::new();
        token.set_header(&mut headers);

        let response = self.http.get(find_url).headers(headers).send().await?;

        let status = response.status();

        if status.is_success() {
            let text = response.text().await?;
            let lines: Vec<String> = text
                .lines()
                .filter(|l| !l.is_empty())
                .map(String::from)
                .collect();
            Ok(FindResponse(lines))
        } else {
            let message = header_str(response.headers(), "x-warp10-error-message");
            let body = response.text().await?;
            Err(Error::Api {
                status,
                body,
                message,
            })
        }
    }
}

fn header_str(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}
