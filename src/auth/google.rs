mod oauth2;
use oauth2::{AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, TokenResponse, TokenUrl};
use std::sync::Arc;
use warp::Filter;

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
        let (authorize_url, _csrf_state) = self
            .authorize_url()
            .set_redirect_uri(self.redirect_uri.clone())
            .url();

        Ok(authorize_url.to_string())
    }

    pub async fn callback(&self, code: AuthorizationCode) -> Result<TokenResponse, warp::Rejection> {
        let token_response = self
            .exchange_code(code)
            .await
            .map_err(|_| warp::reject::not_found())?;

        Ok(token_response)
    }

    fn authorize_url(&self) -> oauth2::AuthorizationUrl {
        oauth2::AuthorizationUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())
            .set_client_id(self.client_id.clone())
            .set_scope("https://www.googleapis.com/auth/userinfo.email".to_string())
    }

    async fn exchange_code(&self, code: AuthorizationCode) -> Result<TokenResponse, oauth2::Error> {
        let token_url = "https://oauth2.googleapis.com/token".to_string();
        let client = oauth2::Client::new(self.client_id.clone(), self.client_secret.clone());

        client
            .exchange_code(code)
            .set_token_url(token_url)
            .request()
            .await
    }
}