use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use oauth2::basic::{BasicClient, BasicErrorResponseType, BasicTokenType};
use oauth2::{StandardErrorResponse, StandardTokenResponse, EmptyExtraTokenFields, RequestTokenError};
//use oauth2::reqwest::async_http_client::async_http_client;
//use oauth2::AsyncHttpClient::async_http_client::async_http_client;
use reqwest::Client as ReqClient;

pub struct GoogleAuth {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUrl,
}

impl GoogleAuth {
    pub fn new(client_id: ClientId, client_secret: ClientSecret, redirect_uri: RedirectUrl) -> Self {
        GoogleAuth {
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    pub async fn login(&self) -> Result<String, warp::Rejection> {
        let auth_uri = AuthUrl::from_url(oauth2::url::Url::parse("https://accounts.google.com/o/oauth2/auth").unwrap());
        let token_uri = TokenUrl::from_url(oauth2::url::Url::parse("https://oauth2.googleapis.com/token").unwrap());
        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(ClientSecret::new("client_secret".to_string()))
            .set_auth_uri(auth_uri.clone())
            .set_token_uri(token_uri.clone())
            // Set the URL the user will be redirected to after the authorization process.
            .set_redirect_uri(self.redirect_uri.clone());
    
        // oauth2::Client::<StandardErrorResponse<BasicErrorResponseType>, 
        //                 StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, 
        //                 BasicTokenType>::new(self.client_id.clone())
        //     .set_client_secret(self.client_secret.clone())
        //     .set_auth_uri())
        //     .set_token_uri(TokenUrl::from_url(oauth2::url::Url::parse("https://oauth2.googleapis.com/token").unwrap()))
        //     .set_redirect_uri(self.redirect_uri.clone());

        let (authorize_url, _csrf_state) = client.authorize_url(oauth2::CsrfToken::new_random).url();

        Ok(authorize_url.to_string())
    }

    pub async fn callback(&self, code: AuthorizationCode) -> Result<oauth2::basic::BasicTokenResponse, warp::Rejection> {
        let token_response = self
            .exchange_code(code)
            .await
            .map_err(|_| warp::reject::not_found())?;

        Ok(token_response)
    }

    fn authorize_url(&self) -> oauth2::url::Url {
        oauth2::url::Url::parse("https://accounts.google.com/o/oauth2/auth").unwrap()
            .set_client_id(self.client_id.clone())
            .set_scope("https://www.googleapis.com/auth/userinfo.email".to_string())
    }

    async fn exchange_code(&self, code: AuthorizationCode) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, StandardErrorResponse<BasicErrorResponseType>> {
        let token_url = "https://oauth2.googleapis.com/token".to_string();
        let auth_url = "https://accounts.google.com/o/oauth2/auth".to_string();
        let client = oauth2::Client::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_auth_uri(AuthUrl::from_url(oauth2::url::Url::parse(&auth_url).unwrap()))
            .set_token_uri(TokenUrl::from_url(oauth2::url::Url::parse(&token_url).unwrap()));
    
        let http_client = ReqClient::new();

        client
            .exchange_code(code)
            .request_async(async_http_client(&http_client))
            .await
            .map_err(|e| match e {
                RequestTokenError::ServerResponse(err) => err,
                _ => StandardErrorResponse::new(BasicErrorResponseType::InvalidRequest, None, None),
            })
    }
    
}