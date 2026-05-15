use crate::portfolio_v2_query_types::ResponseData;
use crate::{Key, PortfolioV2Query, PortfolioV2Request, PortfolioV2RequestVariablesError, RateLimits};
use derive_more::{From, Into};
use errgonomic::{handle, handle_opt, handle_opt_take};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client as HttpClient;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use secrecy::ExposeSecret;
use std::cell::LazyCell;
use std::time::Duration;
use thiserror::Error;
use url::Url;
use url_macro::url;

#[derive(From, Into, Clone, Debug)]
pub struct Client {
    pub inner: HttpClient,
    pub base: Url,
    pub limits: RateLimits,
}

impl Client {
    pub fn new(key: impl Into<Key>) -> Result<Self, ClientNewError> {
        use ClientNewError::*;
        let client = handle!(Self::try_from(key.into()), TryFromFailed);
        Ok(client)
    }

    pub fn default_base_url() -> Url {
        url!("https://public.zapper.xyz/graphql")
    }

    pub fn default_timeout() -> Duration {
        Duration::from_secs(30)
    }

    pub async fn portfolio_v2(&self, request: PortfolioV2Request) -> Result<ResponseData, ClientPortfolioV2Error> {
        use ClientPortfolioV2Error::*;
        LazyCell::force(&self.limits.portfolio_balances)
            .until_ready()
            .await;
        let base = self.base.clone();
        let variables = handle!(request.variables(), VariablesFailed, request, base);
        let body = PortfolioV2Query::build_query(variables);
        let request_builder = self.inner.post(base.clone()).json(&body);
        let response = handle!(request_builder.send().await, SendRequestFailed, request, base);
        let response = handle!(response.error_for_status(), ErrorForStatusFailed, request, base);
        let mut response = handle!(response.json::<Response<ResponseData>>().await, DeserializeResponseFailed, request, base);
        handle_opt_take!(response.errors, ResponseContainsErrors, errors, request, base);
        let data = handle_opt!(response.data, ResponseDataNotFound, request, base);
        Ok(data)
    }
}

#[derive(Error, Debug)]
pub enum ClientNewError {
    #[error("failed to convert key into client")]
    TryFromFailed { source: ConvertKeyToClientError },
}

impl TryFrom<Key> for Client {
    type Error = ConvertKeyToClientError;

    fn try_from(key: Key) -> Result<Self, Self::Error> {
        use ConvertKeyToClientError::*;
        let base = Self::default_base_url();
        let client = handle!(Self::try_from((key, base)), TryFromFailed);
        Ok(client)
    }
}

#[derive(Error, Debug)]
pub enum ConvertKeyToClientError {
    #[error("failed to convert key and base URL into client")]
    TryFromFailed { source: ConvertKeyAndUrlToClientError },
}

impl TryFrom<(Key, Url)> for Client {
    type Error = ConvertKeyAndUrlToClientError;

    fn try_from((key, base): (Key, Url)) -> Result<Self, Self::Error> {
        use ConvertKeyAndUrlToClientError::*;
        let header_name = HeaderName::from_static("x-zapper-api-key");
        let mut header_value = handle!(HeaderValue::from_str(key.expose_secret()), HeaderValueFromStrFailed, key);
        header_value.set_sensitive(true);
        let mut headers = HeaderMap::new();
        headers.insert(header_name, header_value);
        let inner = handle!(
            HttpClient::builder()
                .default_headers(headers)
                .timeout(Self::default_timeout())
                .build(),
            BuildClientFailed
        );
        Ok(Self::from((inner, base)))
    }
}

#[derive(Error, Debug)]
pub enum ConvertKeyAndUrlToClientError {
    #[error("failed to construct the zapper API key header value")]
    HeaderValueFromStrFailed { source: reqwest::header::InvalidHeaderValue, key: Key },
    #[error("failed to construct HTTP client")]
    BuildClientFailed { source: reqwest::Error },
}

impl From<(HttpClient, Url)> for Client {
    fn from((inner, base): (HttpClient, Url)) -> Self {
        Self {
            inner,
            base,
            limits: RateLimits::default(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ClientPortfolioV2Error {
    #[error("failed to convert portfolioV2 request to GraphQL variables")]
    VariablesFailed { source: PortfolioV2RequestVariablesError, request: PortfolioV2Request, base: Url },
    #[error("failed to send portfolioV2 request")]
    SendRequestFailed { source: reqwest::Error, request: PortfolioV2Request, base: Url },
    #[error("portfolioV2 response status is not successful")]
    ErrorForStatusFailed { source: reqwest::Error, request: PortfolioV2Request, base: Url },
    #[error("failed to deserialize portfolioV2 response")]
    DeserializeResponseFailed { source: reqwest::Error, request: PortfolioV2Request, base: Url },
    #[error("portfolioV2 response contains {len} GraphQL errors", len = errors.len())]
    ResponseContainsErrors { errors: Vec<graphql_client::Error>, request: PortfolioV2Request, base: Url },
    #[error("portfolioV2 response did not contain data")]
    ResponseDataNotFound { request: PortfolioV2Request, base: Url },
}
