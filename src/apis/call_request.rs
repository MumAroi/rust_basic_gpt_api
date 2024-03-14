use crate::models::general::llm::{Message, ChatCompletion};
use dotenv::dotenv;
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use std::env;

pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in env");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in env");

    let url: &str = "https://api.openai.com/v1/chat/completions";

    let mut headers: HeaderMap = HeaderMap::new();

    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap()
    );

    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap()
    );


    let client: Client = Client::builder().default_headers(headers).build().unwrap();

    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1
    };


    let res_raw  = client.post(url).json(&chat_completion).send().await.unwrap();

    dbg!(res_raw.text().await.unwrap());
}


#[cfg(test)]
mod test{
    use super::*;

    #[tokio::test]
    async fn test_call_gpt() {
        let messages: Vec<Message> = vec![Message {
            role: "user".to_string(),
            content: "Hello, how are you?. Give me a short response.".to_string()
        }];
        call_gpt(messages).await;
    }
}