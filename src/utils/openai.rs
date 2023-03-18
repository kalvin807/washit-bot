use log::info;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatGPTRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f64,
}

#[derive(Debug, Deserialize)]
struct ChatGPTResponse {
    id: String,
    object: String,
    created: u64,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    index: u32,
    message: Message,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIErrorResponse {
    error: OpenAIError,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIError {
    message: String,
    #[serde(default)]
    #[serde(rename = "type")]
    error_type: String,
    param: Option<String>,
    code: Option<String>,
}

const DEFAULT_PROMPT: &str = "Your are a helpful bot call 'washit'. You are chatting with a user. You always give advice and opinion in best effort. Always reply in Cantonese or English.";

const MODEL_ID: &str = "gpt-3.5-turbo";

fn build_request(user_prompt: String, assist_prompt: String) -> ChatGPTRequest {
    let mut messages = vec![Message {
        role: "system".to_string(),
        content: get_default_prompt(),
    }];

    if !assist_prompt.is_empty() {
        messages.push(Message {
            role: "assistance".to_string(),
            content: assist_prompt,
        });
    }

    messages.push(Message {
        role: "user".to_string(),
        content: user_prompt,
    });

    ChatGPTRequest {
        model: MODEL_ID.to_string(),
        messages,
        temperature: 1.5,
    }
}

async fn get_response(request: ChatGPTRequest, api_key: &str) -> Result<ChatGPTResponse, String> {
    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Error getting response from OpenAI: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Error when reading response from OpenAI: {}", e))?;

    println!("Response from OpenAI: {}", response_text);

    if let Ok(error_response) = serde_json::from_str::<OpenAIErrorResponse>(&response_text) {
        return Err(error_response.error.message);
    }

    let response_obj: ChatGPTResponse = serde_json::from_str(&response_text)
        .map_err(|e| format!("Error when parsing response from OpenAI: {}", e))?;

    info!("OpenAI response: {:#?}", response_obj);
    Ok(response_obj)
}

fn get_api_key() -> String {
    env::var("OPENAI_KEY").expect("OPENAI_KEY must be set")
}

fn get_default_prompt() -> String {
    env::var("SYSTEM_PROMPT").unwrap_or(DEFAULT_PROMPT.to_string())
}

pub async fn ask_chat_gpt(user_prompt: String, assist_prompt: String) -> String {
    let request = build_request(user_prompt, assist_prompt);
    let api_key = get_api_key();

    get_response(request, &api_key)
        .await
        .map(|response| response.choices[0].message.content.clone())
        .unwrap_or_else(|e| format!("Error: {}", e))
}
