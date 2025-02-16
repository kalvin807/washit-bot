use serde::{Deserialize, Serialize};
use serenity::model::prelude::Message as DiscordMessage;
use std::env;
use tracing::{debug, warn};

use crate::handlers::chat::BOT_ID;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
    #[serde(default)]
    refusal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatGPTRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ChatGPTResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
    #[serde(default)]
    system_fingerprint: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Choice {
    index: u32,
    message: Message,
    #[serde(default)]
    logprobs: Option<serde_json::Value>,
    finish_reason: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
    #[serde(default)]
    prompt_tokens_details: Option<PromptTokensDetails>,
    #[serde(default)]
    completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct PromptTokensDetails {
    #[serde(default)]
    cached_tokens: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct CompletionTokensDetails {
    #[serde(default)]
    reasoning_tokens: u32,
    #[serde(default)]
    accepted_prediction_tokens: u32,
    #[serde(default)]
    rejected_prediction_tokens: u32,
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

#[derive(Debug, Serialize, Deserialize)]
struct ImageGenerationPayload {
    model: String,
    prompt: String,
    #[serde(default)]
    n: Option<u32>,
    #[serde(default)]
    quality: Option<String>,
    #[serde(default)]
    response_format: Option<String>,
    #[serde(default)]
    size: Option<String>,
    #[serde(default)]
    style: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ImageResponse {
    #[serde(default)]
    b64_json: Option<String>,
    revised_prompt: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ImageGenerationResponse {
    created: i64,
    data: Vec<ImageResponse>,
}

const DEFAULT_PROMPT: &str = "Your are a helpful bot call 'washit'. You always give advice and opinion in best effort. Reply in full Cantonese for casual question; Full English if it is a serious question. Reply in japanese if the user is using japanese";

fn build_request(user_prompt: String, history_messages: Vec<Message>) -> ChatGPTRequest {
    let mut messages = vec![Message {
        role: "system".to_string(),
        content: get_default_prompt(),
        refusal: None,
    }];

    messages.extend(history_messages);

    messages.push(Message {
        role: "user".to_string(),
        content: user_prompt,
        refusal: None,
    });

    ChatGPTRequest {
        model: get_model_id(),
        messages,
        temperature: 1.0,
    }
}

async fn get_response(request: ChatGPTRequest, api_key: &str) -> Result<ChatGPTResponse, String> {
    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    debug!(target:"open_ai", request = ?request, "prompt");
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            warn!(?e, warning = "Error getting response from OpenAI",);
            format!("Error getting response from OpenAI: {:?}", e)
        })?;

    let response_text = response.text().await.map_err(|e| {
        warn!(?e, warning = "Error reading response from OpenAI",);
        format!("Error when reading response from OpenAI: {}", e)
    })?;

    if let Ok(error_response) = serde_json::from_str::<OpenAIErrorResponse>(&response_text) {
        return Err(error_response.error.message);
    }

    let response_obj: ChatGPTResponse = serde_json::from_str(&response_text).map_err(|e| {
        warn!(?e, warning = "Error parsing response from OpenAI",);
        format!("Error when parsing response from OpenAI: {}", e)
    })?;
    debug!(target:"open_ai", response = ?response_obj, "response");

    Ok(response_obj)
}

fn get_api_key() -> String {
    env::var("OPENAI_KEY").expect("OPENAI_KEY must be set")
}

fn get_default_prompt() -> String {
    env::var("SYSTEM_PROMPT").unwrap_or_else(|_| DEFAULT_PROMPT.to_string())
}

fn get_model_id() -> String {
    env::var("MODEL_ID").unwrap_or_else(|_| "gpt-3.5-turbo".to_string())
}

fn build_history_messages(history: Vec<DiscordMessage>) -> Vec<Message> {
    history
        .into_iter()
        .map(|message| Message {
            role: {
                if message.id == BOT_ID {
                    "assistant".to_string()
                } else {
                    "user".to_string()
                }
            },
            content: message.content,
            refusal: None,
        })
        .collect()
}

pub async fn ask_chat_gpt(user_prompt: String, history: Vec<DiscordMessage>) -> String {
    let history_message = build_history_messages(history);
    let request = build_request(user_prompt, history_message);
    let api_key = get_api_key();

    get_response(request, &api_key)
        .await
        .map(|response| response.choices[0].message.content.clone())
        .unwrap_or_else(|e| format!("OpenAI: {}", e))
}

pub async fn generate_images(prompt: &str) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    let api_key = get_api_key();
    let payload = ImageGenerationPayload {
        model: "dall-e-3".to_string(),
        prompt: prompt.to_string(),
        n: Some(1),
        quality: None,
        response_format: None,
        size: None,
        style: None,
    };

    let response = client
        .post("https://api.openai.com/v1/images/generations")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            warn!(?e, warning = "Error getting response from OpenAI",);
            format!("Error getting response from OpenAI: {}", e)
        })?;

    let api_response: ImageGenerationResponse = response.json().await.map_err(|e| {
        warn!(?e, warning = "Error parsing response from OpenAI",);
        format!("Error parsing response from OpenAI: {}", e)
    })?;
    debug!("OpenAI response: {:#?}", api_response);
    
    let image_urls = api_response
        .data
        .into_iter()
        .filter_map(|image_response| image_response.url)
        .collect();

    Ok(image_urls)
}
