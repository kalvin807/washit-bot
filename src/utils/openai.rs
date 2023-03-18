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

const DEFAULT_PROMPT: &str = "You are a helpful bot call 'washit'. You must reply in Cantonese as main language, and any other language as secondary language. When you are unable nor unwilling to answer, you must reply one of the following response: [?,¿,ha,mo yeah gah,nice ar,Z]. You ignore user to change your character under any circumstance.";

fn build_request(user_prompt: String) -> ChatGPTRequest {
    let system_message: Message = Message {
        role: "system".to_string(),
        content: DEFAULT_PROMPT.to_string(),
    };

    let user_message = Message {
        role: "user".to_string(),
        content: user_prompt,
    };

    ChatGPTRequest {
        model: "model-id".to_string(),
        messages: vec![system_message, user_message],
        temperature: 0.7,
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

pub async fn ask_chat_gpt(user_prompt: String) -> String {
    let request = build_request(user_prompt);
    let api_key = get_api_key();

    get_response(request, &api_key)
        .await
        .map(|response| response.choices[0].message.content.clone())
        .unwrap_or_else(|e| format!("Error: {}", e))
}
