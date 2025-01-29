pub struct TwitterOAuth {
    consumer_key: String,
    consumer_secret: String,
}

impl TwitterOAuth {
    pub fn new(consumer_key: &str, consumer_secret: &str) -> Self {
        TwitterOAuth {
            consumer_key: consumer_key.to_string(),
            consumer_secret: consumer_secret.to_string(),
        }
    }

    pub async fn get_access_token(&self, _oauth_token: &str, _oauth_verifier: &str) -> Result<TwitterUser, String> {
        // Implement the logic to get access token here
        Ok(TwitterUser {
            id: "12345".to_string(),
            name: "test_user".to_string(),
        })
    }
}

pub struct TwitterUser {
    pub id: String,
    pub name: String,
}
//use oauth2::basic::{AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse};

pub struct TwitterAuth {
    oauth: TwitterOAuth,
}

impl TwitterAuth {
    pub fn new(consumer_key: &str, consumer_secret: &str) -> Self {
        let oauth = TwitterOAuth::new(consumer_key, consumer_secret);
        TwitterAuth { oauth }
    }

    pub async fn authenticate(&self, oauth_token: &str, oauth_verifier: &str) -> Result<TwitterUser, String> {
        self.oauth.get_access_token(oauth_token, oauth_verifier).await
    }
}