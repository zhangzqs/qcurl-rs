use std::str::FromStr;

use clap::Parser;
use reqwest;
use anyhow::Result;

#[derive(Parser, Debug)]
struct CURL {
    #[arg()]
    url: String,

    #[arg(default_value="GET", short='X', long="request")]
    request: String,

    #[arg(short='H', long="header")]
    header: Vec<String>,

    #[arg(short='d', long="data")]
    data: Option<String>,

    #[arg(short='v', long="verbose")]
    verbose: bool,

    #[arg(short='V', long="version")]
    version: bool,

    #[arg(short='F', long="form")]
    form: Vec<String>,

    #[arg(short='o', long="output")]
    output: Option<String>,

    #[arg(short='A', long="user-agent")]
    user_agent: Option<String>,

    #[arg(long="access-key")]
    access_key: Option<String>,

    #[arg(long="secret-key")]
    secret_key: Option<String>,

    #[arg(long="auth")]
    auth: Option<String>,

    #[arg(long="auth-url")]
    auth_url: bool,

    #[arg(long="auth-up-policy")]
    auth_up_policy: Option<String>,

    #[arg(long="auth-up-form")]
	auth_up_form:   bool,

    #[arg(long="json")]
    json_content_type: bool,

    #[arg(long="binary")]
    binary_content_type: bool,

    #[arg(long="form-urlencoded")]
    form_urlencoded_content_type: bool,

    #[arg(long="pretty")]
    pretty: bool,

    #[arg(long="su-uid")]
    su_uid: Option<String>,

    #[arg(long="content-md5")]
    content_md5: Option<String>,

    #[arg(long="region")]
    region: Option<String>,
}

use reqwest::{Client, Request};

impl CURL {
    fn build_unsigned_request(&self) -> Result<(Client, Request)> {
        let url = reqwest::Url::from_str(&self.url)?;
        let method = reqwest::Method::from_str(&self.request)?;
        let client = reqwest::Client::new();
        let mut builder = client.request(method, url);
        
        if let Some(ref data) = self.data {
            if data.starts_with("@") {
                // TODO: read from file
            } else {
                builder = builder.body(data.clone());
            }
        } else if !self.form.is_empty() {
            // TODO: read from form
        }

        if let Some(ref user_agent) = self.user_agent {
            builder = builder.header("User-Agent", user_agent.clone());
        }

        if self.json_content_type {
            builder = builder.header("Content-Type", "application/json");
        } else if self.binary_content_type {
            builder = builder.header("Content-Type", "application/octet-stream");
        } else if self.form_urlencoded_content_type {
            builder = builder.header("Content-Type", "application/x-www-form-urlencoded");
        }

        for header in &self.header {
            let mut parts = header.splitn(2, ':');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            builder = builder.header(key, value);
        }

        Ok((client, builder.build()?))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let curl = CURL::parse();
    let (client, request) = curl.build_unsigned_request()?;
    let res = client.execute(request).await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}
