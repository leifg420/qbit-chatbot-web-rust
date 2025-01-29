mod oauth;
use oauth::{TwitterOAuth, TwitterUser};

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