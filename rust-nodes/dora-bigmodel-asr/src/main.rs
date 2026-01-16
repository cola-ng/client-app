// Dora Node: BigModel ASR (Automatic Speech Recognition)
// Converts user audio to text using ZhipuAI GLM-ASR API

use dora_node_api::arrow::array::{Array, StringArray, UInt8Array};
use dora_node_api::{DoraNode, Event};
use eyre::{Context, Result};
use reqwest::header;
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct AudioInput {
    audio_data: Vec<u8>,
    sample_rate: u32,
    format: String,
    session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AsrOutput {
    text: String,
    confidence: f32,
    words: Vec<WordTiming>,
    session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WordTiming {
    word: String,
    start_time: f64,
    end_time: f64,
    confidence: f32,
}

#[derive(Debug, Deserialize)]
struct BigModelResponse {
    #[allow(dead_code)]
    id: Option<String>,
    #[allow(dead_code)]
    model: Option<String>,
    choices: Option<Vec<Choice>>,
    error: Option<ErrorInfo>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    #[allow(dead_code)]
    index: i32,
    message: Option<Message>,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Message {
    #[allow(dead_code)]
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ErrorInfo {
    code: String,
    message: String,
}

const API_URL: &str = "https://open.bigmodel.cn/api/paas/v4/audio/transcriptions";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let api_key = std::env::var("BIGMODEL_API_KEY")
        .wrap_err("BIGMODEL_API_KEY environment variable not set")?;

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    let (mut node, mut events) = DoraNode::init_from_env()?;

    log::info!("BigModel ASR node started");

    while let Some(event) = events.recv() {
        match event {
            Event::Input { id, data, metadata } => match id.as_str() {
                "audio" => {
                    log::debug!("Received audio input");

                    let raw_data: Vec<u8> = extract_bytes(&data)
                        .ok_or_else(|| eyre::eyre!("Failed to get bytes from arrow data"))?;
                    log::debug!("Received audio input ({} bytes)", raw_data.len());

                    match serde_json::from_slice::<AudioInput>(&raw_data) {
                        Ok(input) => {
                            match perform_asr(&client, &api_key, &input).await {
                                Ok(asr_result) => {
                                    log::info!("ASR result: {}", asr_result.text);

                                    let output_json = serde_json::to_string(&asr_result)?;
                                    let output_array =
                                        StringArray::from(vec![output_json.as_str()]);
                                    node.send_output(
                                        "text".to_string().into(),
                                        metadata.parameters.clone(),
                                        output_array,
                                    )?;

                                    let status = json!({
                                        "node": "bigmodel-asr",
                                        "status": "ok",
                                        "text_length": asr_result.text.len(),
                                    });
                                    let status_array =
                                        StringArray::from(vec![status.to_string().as_str()]);
                                    node.send_output(
                                        "status".to_string().into(),
                                        metadata.parameters.clone(),
                                        status_array,
                                    )?;
                                }
                                Err(e) => {
                                    log::error!("ASR failed: {}", e);

                                    let status = json!({
                                        "node": "bigmodel-asr",
                                        "status": "error",
                                        "error": e.to_string(),
                                    });
                                    let status_array =
                                        StringArray::from(vec![status.to_string().as_str()]);
                                    node.send_output(
                                        "status".to_string().into(),
                                        metadata.parameters.clone(),
                                        status_array,
                                    )?;
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to parse audio input: {}", e);
                        }
                    }
                }
                _ => {
                    log::warn!("Received unknown input: {}", id);
                }
            },
            Event::InputClosed { id } => {
                log::info!("Input {} closed", id);
            }
            Event::Stop(_) => {
                log::info!("Received stop signal");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

async fn perform_asr(client: &Client, api_key: &str, input: &AudioInput) -> Result<AsrOutput> {
    // Determine file extension based on format
    let file_name = match input.format.as_str() {
        "wav" => "audio.wav",
        "mp3" => "audio.mp3",
        "pcm" => "audio.pcm",
        "m4a" => "audio.m4a",
        "flac" => "audio.flac",
        _ => "audio.wav",
    };

    // Determine MIME type
    let mime_type = match input.format.as_str() {
        "wav" => "audio/wav",
        "mp3" => "audio/mpeg",
        "pcm" => "audio/pcm",
        "m4a" => "audio/mp4",
        "flac" => "audio/flac",
        _ => "audio/wav",
    };

    // Build multipart form
    let file_part = Part::bytes(input.audio_data.clone())
        .file_name(file_name.to_string())
        .mime_str(mime_type)?;

    let form = Form::new()
        .text("model", "glm-asr")
        .part("file", file_part);

    let response = client
        .post(API_URL)
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        eyre::bail!("ASR API error: {}", error_text);
    }

    let result: BigModelResponse = response.json().await?;

    // Check for error response
    if let Some(error) = result.error {
        eyre::bail!("ASR API error: {} - {}", error.code, error.message);
    }

    // Extract transcription text from response
    let text = result
        .choices
        .and_then(|choices| choices.into_iter().next())
        .and_then(|choice| choice.message)
        .map(|msg| msg.content)
        .unwrap_or_default();

    Ok(AsrOutput {
        text,
        confidence: 1.0, // BigModel doesn't provide confidence scores
        words: vec![],   // BigModel doesn't provide word-level timing
        session_id: input.session_id.clone(),
    })
}

/// Extract bytes from ArrowData (handles both UInt8Array and StringArray)
fn extract_bytes(data: &dora_node_api::ArrowData) -> Option<Vec<u8>> {
    use dora_node_api::arrow::array::ListArray;
    use dora_node_api::arrow::datatypes::DataType;

    let array = &data.0;
    match array.data_type() {
        DataType::UInt8 => {
            let arr = array.as_any().downcast_ref::<UInt8Array>()?;
            Some(arr.values().to_vec())
        }
        DataType::Utf8 => {
            let arr = array.as_any().downcast_ref::<StringArray>()?;
            if arr.len() > 0 {
                Some(arr.value(0).as_bytes().to_vec())
            } else {
                None
            }
        }
        DataType::List(_) => {
            let list_array = array.as_any().downcast_ref::<ListArray>()?;
            if list_array.len() > 0 {
                let values = list_array.values();
                if let Some(float_array) = values
                    .as_any()
                    .downcast_ref::<dora_node_api::arrow::array::Float32Array>()
                {
                    let bytes: Vec<u8> = float_array
                        .values()
                        .iter()
                        .flat_map(|f| f.to_le_bytes())
                        .collect();
                    Some(bytes)
                } else {
                    log::warn!("List contains non-Float32 data");
                    None
                }
            } else {
                None
            }
        }
        _ => {
            log::warn!("Unsupported data type: {:?}", array.data_type());
            None
        }
    }
}
