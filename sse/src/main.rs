use serde::Serialize;
use reqwest::Client;
use tokio_stream::StreamExt;

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://api.openai.com/v1/chat/completions";
    let model = "gpt-4.1";
    let api_key = "";

    let messages = vec![
        ChatMessage {
            role: "developer".to_string(),
            content: "You are a helpful assistant.".to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: "Hello!".to_string(),
        },
    ];
    let chat_request = ChatRequest {
        model: model.to_string(),
        messages,
        stream: true,
    };

    let client = Client::new();
    let response = client
        .post(base_url)
        .bearer_auth(api_key)
        // .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&chat_request)
        .send()
        .await?;
    println!("Response: {:?}", response);

    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        match item {
            Ok(chunk) => {
                let chunk_str = String::from_utf8_lossy(&chunk);
                // println!("Received: {}", chunk_str);
                for line in chunk_str.lines() {
                    if line.starts_with("data: ") {
                        let data = &line[6..];
                        if data == "[DONE]" {
                            println!("Stream ended.");
                            break;
                        }

                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                            if let Some(choices) = json["choices"].as_array() {
                                if let Some(choice) = choices.first() {
                                    if let Some(content) = choice["delta"]["content"].as_str() {
                                        print!("{}", content);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}