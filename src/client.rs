use crate::portfolio_v2_token_balances_by_token_types::{PortfolioV2TokenBalancesByTokenPortfolioV2TokenBalancesByToken, ResponseData, Variables};
use crate::{Key, PortfolioV2TokenBalancesByToken, PortfolioV2TokenBalancesByTokenRequest, RateLimits, TurnCursorPageError, turn_cursor_page};
use errgonomic::{handle, handle_opt, handle_opt_take};
use graphql_client::{GraphQLQuery, QueryBody, Response};
use page_turner::{PageTurner, TurnedPageResult};
use reqwest::Client as HttpClient;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use secrecy::ExposeSecret;
use std::sync::LazyLock;
use std::time::Duration;
use thiserror::Error;
use url::Url;
use url_macro::url;

#[derive(Debug)]
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

    pub async fn portfolio_v2_token_balances_by_token(&self, request: PortfolioV2TokenBalancesByTokenRequest) -> Result<ResponseData, ClientPortfolioV2TokenBalancesByTokenError> {
        use ClientPortfolioV2TokenBalancesByTokenError::*;
        LazyLock::force(&self.limits.portfolio_balances)
            .until_ready()
            .await;
        let body = PortfolioV2TokenBalancesByToken::build_query(request.into());
        let request_builder = self.inner.post(self.base.clone()).json(&body);
        let response = handle!(request_builder.send().await, SendRequestFailed, body);
        let response = handle!(response.error_for_status(), ErrorForStatusFailed, body);
        let mut response = handle!(response.json::<Response<ResponseData>>().await, DeserializeResponseFailed, body);
        handle_opt_take!(response.errors, ResponseContainsErrors, errors, body);
        let data = handle_opt!(response.data, ResponseDataNotFound, body);
        Ok(data)
    }
}

impl PageTurner<PortfolioV2TokenBalancesByTokenRequest> for Client {
    type PageItems = PortfolioV2TokenBalancesByTokenPortfolioV2TokenBalancesByToken;
    type PageError = PageTurnerPortfolioV2TokenBalancesByTokenRequestClientError;

    async fn turn_page(&self, request: PortfolioV2TokenBalancesByTokenRequest) -> TurnedPageResult<Self, PortfolioV2TokenBalancesByTokenRequest> {
        use PageTurnerPortfolioV2TokenBalancesByTokenRequestClientError::*;
        let request_for_query = request.clone();
        let data = handle!(
            self.portfolio_v2_token_balances_by_token(request_for_query)
                .await,
            PortfolioV2TokenBalancesByTokenFailed,
            request
        );
        let page = data.portfolio_v2.token_balances.by_token;
        let turned_page = handle!(turn_cursor_page(request, page), TurnCursorPageFailed);
        Ok(turned_page)
    }
}

#[derive(Error, Debug)]
pub enum PageTurnerPortfolioV2TokenBalancesByTokenRequestClientError {
    #[error("failed to query portfolioV2 token balances by token page")]
    PortfolioV2TokenBalancesByTokenFailed { source: ClientPortfolioV2TokenBalancesByTokenError, request: PortfolioV2TokenBalancesByTokenRequest },
    #[error("failed to turn portfolioV2 token balances by token cursor page")]
    TurnCursorPageFailed { source: TurnCursorPageError<PortfolioV2TokenBalancesByTokenRequest> },
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
        Self::from((inner, base, RateLimits::default()))
    }
}

impl From<(HttpClient, Url, RateLimits)> for Client {
    fn from((inner, base, limits): (HttpClient, Url, RateLimits)) -> Self {
        Self {
            inner,
            base,
            limits,
        }
    }
}

#[derive(Error, Debug)]
pub enum ClientPortfolioV2TokenBalancesByTokenError {
    #[error("failed to send portfolioV2 request")]
    SendRequestFailed { source: reqwest::Error, body: QueryBody<Variables> },
    #[error("portfolioV2 response status is not successful")]
    ErrorForStatusFailed { source: reqwest::Error, body: QueryBody<Variables> },
    #[error("failed to deserialize portfolioV2 response")]
    DeserializeResponseFailed { source: reqwest::Error, body: QueryBody<Variables> },
    #[error("portfolioV2 response contains {len} GraphQL errors", len = errors.len())]
    ResponseContainsErrors { errors: Vec<graphql_client::Error>, body: QueryBody<Variables> },
    #[error("portfolioV2 response did not contain data")]
    ResponseDataNotFound { body: QueryBody<Variables> },
}
