mod openai {
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use std::error::Error;

    #[derive(Serialize)]
    struct OpenAIRequest {
        prompt: String,
        max_tokens: u32,
    }

    #[derive(Deserialize)]
    struct OpenAIResponse {
        choices: Vec<Choice>,
    }

    #[derive(Deserialize)]
    struct Choice {
        text: String,
    }

    pub struct Chatbot {
        client: Client,
        api_key: String,
    }

    impl Chatbot {
        pub fn new(api_key: String) -> Self {
            let client = Client::new();
            Chatbot { client, api_key }
        }

        pub async fn send_message(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
            let request = OpenAIRequest {
                prompt: prompt.to_string(),
                max_tokens: 100,
            };

            let response: OpenAIResponse = self.client
                .post("https://api.openai.com/v1/engines/davinci-codex/completions")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&request)
                .send()
                .await?
                .json()
                .await?;

            Ok(response.choices.first().map_or(String::new(), |choice| choice.text.clone()))
        }
    }
}