// Dora Node: BigModel TTS (Text-to-Speech)
// Converts AI text responses to speech using ZhipuAI GLM-TTS API

use dora_node_api::arrow::array::{Array, StringArray, UInt8Array};
use dora_node_api::{ArrowData, DoraNode, Event};
use eyre::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Comprehensive response format (from english-teacher/json_data)
#[derive(Debug, Serialize, Deserialize)]
struct ComprehensiveResponse {
    session_id: String,
    use_lang: String,
    original_en: String,
    original_zh: String,
    reply_en: String,
    reply_zh: String,
    issues: Vec<TextIssue>,
    timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct TextIssue {
    #[serde(rename = "type")]
    issue_type: String,
    original: String,
    suggested: String,
    description_en: String,
    description_zh: String,
    severity: String,
    #[serde(default)]
    start_position: Option<i32>,
    #[serde(default)]
    end_position: Option<i32>,
}

/// Simple text input format
#[derive(Debug, Serialize, Deserialize)]
struct TextInput {
    text: String,
    session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AudioMetadata {
    duration_ms: u64,
    format: String,
    sample_rate: u32,
    bytes: usize,
}

const API_URL: &str = "https://open.bigmodel.cn/api/paas/v4/audio/speech";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let api_key = std::env::var("BIGMODEL_API_KEY")
        .wrap_err("BIGMODEL_API_KEY environment variable not set")?;

    let voice = std::env::var("BIGMODEL_VOICE").unwrap_or_else(|_| "alloy".to_string());
    let speed: f32 = std::env::var("BIGMODEL_SPEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.0);
    let volume: f32 = std::env::var("BIGMODEL_VOLUME")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.0);
    let response_format =
        std::env::var("BIGMODEL_AUDIO_FORMAT").unwrap_or_else(|_| "wav".to_string());

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    let (mut node, mut events) = DoraNode::init_from_env()?;

    log::info!(
        "BigModel TTS node started (voice: {}, speed: {}, format: {})",
        voice,
        speed,
        response_format
    );

    while let Some(event) = events.recv() {
        match event {
            Event::Input { id, data, metadata } => {
                let raw_data = extract_bytes(&data);
                match id.as_str() {
                    "text" => {
                        log::debug!("Received text input");

                        let text_to_convert = if let Ok(comprehensive_response) =
                            serde_json::from_slice::<ComprehensiveResponse>(&raw_data)
                        {
                            comprehensive_response.reply_en
                        } else if let Ok(text_input) =
                            serde_json::from_slice::<TextInput>(&raw_data)
                        {
                            text_input.text
                        } else {
                            String::from_utf8_lossy(&raw_data).to_string()
                        };

                        if text_to_convert.trim().is_empty() {
                            log::debug!("Empty text, skipping TTS");
                            continue;
                        }

                        log::info!("Converting to speech: {}", text_to_convert);

                        match perform_tts(
                            &client,
                            &api_key,
                            &voice,
                            speed,
                            volume,
                            &response_format,
                            &text_to_convert,
                        )
                        .await
                        {
                            Ok((audio_bytes, audio_metadata)) => {
                                log::info!("TTS generated {} bytes", audio_bytes.len());

                                // Convert WAV to f32 samples
                                let (audio_samples, actual_sample_rate) =
                                    decode_wav_to_samples(&audio_bytes)?;

                                log::info!(
                                    "Decoded {} bytes to {} samples at {}Hz",
                                    audio_bytes.len(),
                                    audio_samples.len(),
                                    actual_sample_rate
                                );

                                let audio_array =
                                    dora_node_api::arrow::array::ListArray::from_iter_primitive::<
                                        dora_node_api::arrow::datatypes::Float32Type,
                                        _,
                                        _,
                                    >(std::iter::once(Some(
                                        audio_samples.iter().map(|&s| Some(s)),
                                    )));

                                let mut output_params = metadata.parameters.clone();
                                output_params.insert(
                                    "sample_rate".to_string(),
                                    dora_node_api::Parameter::Integer(actual_sample_rate as i64),
                                );

                                node.send_output(
                                    "audio_bytes".to_string().into(),
                                    output_params.clone(),
                                    audio_array,
                                )?;

                                let metadata_json = serde_json::to_string(&audio_metadata)?;
                                let audio_metadata =
                                    StringArray::from(vec![metadata_json.as_str()]);

                                node.send_output(
                                    "audio_metadata".to_string().into(),
                                    output_params.clone(),
                                    audio_metadata,
                                )?;

                                let status = json!({
                                    "node": "bigmodel-tts",
                                    "status": "ok",
                                });
                                let status_array =
                                    StringArray::from(vec![status.to_string().as_str()]);
                                node.send_output(
                                    "status".to_string().into(),
                                    output_params,
                                    status_array,
                                )?;
                            }
                            Err(e) => {
                                log::error!("TTS failed: {}", e);

                                let status = json!({
                                    "node": "bigmodel-tts",
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
                    _ => {
                        log::warn!("Received unknown input: {}", id);
                    }
                }
            }
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

fn extract_bytes(data: &ArrowData) -> Vec<u8> {
    if let Some(array) = data.0.as_any().downcast_ref::<StringArray>() {
        if array.len() > 0 {
            return array.value(0).as_bytes().to_vec();
        }
    }
    if let Some(array) = data.0.as_any().downcast_ref::<UInt8Array>() {
        return array.values().to_vec();
    }
    Vec::new()
}

async fn perform_tts(
    client: &Client,
    api_key: &str,
    voice: &str,
    speed: f32,
    volume: f32,
    response_format: &str,
    text: &str,
) -> Result<(Vec<u8>, AudioMetadata)> {
    let payload = json!({
        "model": "glm-tts",
        "input": text,
        "voice": voice,
        "speed": speed,
        "volume": volume,
        "response_format": response_format
    });

    let response = client
        .post(API_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        eyre::bail!("TTS API error: {}", error_text);
    }

    let audio_bytes = response.bytes().await?.to_vec();

    // Calculate duration from WAV header (assuming 24000Hz sample rate by default)
    let (sample_rate, duration_ms) = parse_wav_info(&audio_bytes).unwrap_or((24000, 0));

    let metadata = AudioMetadata {
        duration_ms,
        format: response_format.to_string(),
        sample_rate,
        bytes: audio_bytes.len(),
    };

    Ok((audio_bytes, metadata))
}

fn parse_wav_info(data: &[u8]) -> Option<(u32, u64)> {
    if data.len() < 44 {
        return None;
    }

    // Check WAV header
    if &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
        return None;
    }

    // Get sample rate from bytes 24-27
    let sample_rate = u32::from_le_bytes([data[24], data[25], data[26], data[27]]);

    // Get data size from bytes 40-43 (for standard WAV)
    let data_size = u32::from_le_bytes([data[40], data[41], data[42], data[43]]) as u64;

    // Get bytes per sample (bits per sample / 8 * channels)
    let bits_per_sample = u16::from_le_bytes([data[34], data[35]]) as u64;
    let channels = u16::from_le_bytes([data[22], data[23]]) as u64;
    let bytes_per_sample = (bits_per_sample / 8) * channels;

    if bytes_per_sample == 0 || sample_rate == 0 {
        return None;
    }

    let total_samples = data_size / bytes_per_sample;
    let duration_ms = (total_samples * 1000) / sample_rate as u64;

    Some((sample_rate, duration_ms))
}

fn decode_wav_to_samples(data: &[u8]) -> Result<(Vec<f32>, u32)> {
    if data.len() < 44 {
        eyre::bail!("WAV data too short");
    }

    // Check WAV header
    if &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
        eyre::bail!("Invalid WAV header");
    }

    let channels = u16::from_le_bytes([data[22], data[23]]);
    let sample_rate = u32::from_le_bytes([data[24], data[25], data[26], data[27]]);
    let bits_per_sample = u16::from_le_bytes([data[34], data[35]]);

    // Find data chunk
    let mut pos = 12;
    let mut data_start = 44;
    let mut data_size = 0u32;

    while pos + 8 <= data.len() {
        let chunk_id = &data[pos..pos + 4];
        let chunk_size = u32::from_le_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]]);

        if chunk_id == b"data" {
            data_start = pos + 8;
            data_size = chunk_size;
            break;
        }
        pos += 8 + chunk_size as usize;
    }

    if data_size == 0 {
        eyre::bail!("No data chunk found in WAV");
    }

    let audio_data = &data[data_start..data_start + data_size as usize];
    let mut samples = Vec::new();

    match bits_per_sample {
        16 => {
            for chunk in audio_data.chunks_exact(2 * channels as usize) {
                // Take first channel or average for mono output
                if channels == 1 {
                    let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                    samples.push(sample as f32 / 32768.0);
                } else {
                    let left = i16::from_le_bytes([chunk[0], chunk[1]]);
                    let right = i16::from_le_bytes([chunk[2], chunk[3]]);
                    samples.push((left as f32 + right as f32) / 2.0 / 32768.0);
                }
            }
        }
        24 => {
            for chunk in audio_data.chunks_exact(3 * channels as usize) {
                if channels == 1 {
                    let sample = i32::from_le_bytes([0, chunk[0], chunk[1], chunk[2]]) >> 8;
                    samples.push(sample as f32 / 8388608.0);
                } else {
                    let left = i32::from_le_bytes([0, chunk[0], chunk[1], chunk[2]]) >> 8;
                    let right = i32::from_le_bytes([0, chunk[3], chunk[4], chunk[5]]) >> 8;
                    samples.push((left as f32 + right as f32) / 2.0 / 8388608.0);
                }
            }
        }
        32 => {
            for chunk in audio_data.chunks_exact(4 * channels as usize) {
                if channels == 1 {
                    let sample = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    samples.push(sample);
                } else {
                    let left = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    let right = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
                    samples.push((left + right) / 2.0);
                }
            }
        }
        _ => {
            eyre::bail!("Unsupported bits per sample: {}", bits_per_sample);
        }
    }

    Ok((samples, sample_rate))
}
