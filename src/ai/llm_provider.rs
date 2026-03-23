//! LLM-backed shape generation via OpenAI-compatible, Ollama, or Gemini APIs.
//! Sends a system prompt requesting SVG `<path>` output, then parses the response.

use serde::{Deserialize, Serialize};

// ── Provider config ─────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AiBackend {
    Ollama,
    OpenAi,
    Gemini,
}

impl Default for AiBackend {
    fn default() -> Self {
        AiBackend::Ollama
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AiConfig {
    pub backend: AiBackend,
    /// Base URL (e.g. "http://localhost:11434" for Ollama, "https://api.openai.com" for OpenAI)
    pub endpoint: String,
    /// Model name (e.g. "llama3.1:8b", "gpt-4o-mini")
    pub model: String,
    /// API key (only needed for OpenAI / compatible)
    pub api_key: String,
    /// Max tokens for the response
    pub max_tokens: u32,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            backend: AiBackend::Ollama,
            endpoint: "http://localhost:11434".to_string(),
            model: "llama3.1:8b".to_string(),
            api_key: String::new(),
            max_tokens: 32768,
        }
    }
}

// ── System prompt ───────────────────────────────────────────────────────

const SYSTEM_PROMPT: &str = r#"You are an expert SVG artist creating laser-cut artwork. You produce ONLY SVG <path> tags.

ABSOLUTE RULES (breaking any = unusable output):
1. Output ONLY <path> tags. NO text, markdown, explanation, code fences, or XML headers.
2. Use ONLY absolute commands: M, C, Q, L, Z (uppercase). NO relative commands (m,c,q,l,z).
3. ALL coordinates between 0 and 100 (viewBox 0 0 100 100).
4. First character of output must be '<'.

QUALITY RULES (critical for good results):
- Use C (cubic bezier) for ALL curved shapes. Avoid L for organic forms.
- Every curve needs proper control points for smooth, natural shapes.
- The cut outline should be ONE single closed path that forms the complete silhouette.
- Plan the composition: subject centered, filling 70-90% of the 100x100 space.
- Use bilateral symmetry where anatomically appropriate.
- Each path should be smooth and continuous, not jagged zigzags.

LAYER SYSTEM — every <path> needs a class attribute:
- class="cut" → ONE outer silhouette path. Must be closed (Z). This is laser-cut through material.
- class="engrave" → Major internal features (eyes, beak, limbs, major lines). Medium laser power.
- class="fine" → Texture, feathers, fur, bark, scales, hatching. Light laser power for contrast.

COMPOSITION APPROACH:
1. FIRST: Design the outer silhouette as one elegant closed curve (class="cut").
2. THEN: Add 3-6 engrave paths for major anatomical features.
3. FINALLY: Add 4-8 fine paths for texture and detail.

EXAMPLE — "cat sitting" (note: all curves use C for smoothness):
<path class="cut" d="M50,2 C40,2 30,8 28,18 C26,24 20,26 15,24 C10,22 5,18 5,24 C5,30 12,32 18,30 C22,28 24,30 24,35 C24,45 18,55 18,70 C18,82 22,92 30,96 C35,98 45,98 50,96 C55,98 65,98 70,96 C78,92 82,82 82,70 C82,55 76,45 76,35 C76,30 78,28 82,30 C88,32 95,30 95,24 C95,18 90,22 85,24 C80,26 74,24 72,18 C70,8 60,2 50,2 Z" />
<path class="engrave" d="M38,25 C35,22 35,18 38,16 C41,14 45,16 45,20 C45,24 41,28 38,25 Z" />
<path class="engrave" d="M62,25 C59,28 55,24 55,20 C55,16 59,14 62,16 C65,18 65,22 62,25 Z" />
<path class="engrave" d="M47,30 C48,32 52,32 53,30 C52,33 48,33 47,30 Z" />
<path class="engrave" d="M42,38 C45,42 55,42 58,38" />
<path class="engrave" d="M35,60 C35,55 40,52 50,52 C60,52 65,55 65,60" />
<path class="fine" d="M30,40 C32,42 34,40 M36,44 C38,46 40,44 M42,40 C44,42 46,40" />
<path class="fine" d="M54,40 C56,42 58,40 M60,44 C62,46 64,44 M66,40 C68,42 70,40" />
<path class="fine" d="M40,65 C42,68 44,65 M46,67 C48,70 50,67 M52,65 C54,68 56,65 M58,67 C60,70 62,67" />
<path class="fine" d="M25,50 C27,52 29,50 M28,55 C30,57 32,55 M75,50 C73,52 71,50 M72,55 C70,57 68,55" />
<path class="fine" d="M38,80 C39,85 41,80 M44,82 C45,87 47,82 M53,82 C54,87 56,82 M59,80 C60,85 62,80" />"#;

// ── API call ────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaChatMessage>,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OllamaOptions {
    num_predict: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: Option<OllamaChatMsgContent>,
    #[serde(default)]
    error: Option<String>,
}

#[derive(Deserialize)]
struct OllamaChatMsgContent {
    content: String,
}

#[derive(Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Deserialize)]
struct OllamaGenerateResponse {
    response: Option<String>,
    #[serde(default)]
    error: Option<String>,
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Option<Vec<OpenAiChoice>>,
    error: Option<OpenAiError>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiMsgContent,
}

#[derive(Deserialize)]
struct OpenAiMsgContent {
    content: String,
}

#[derive(Deserialize)]
struct OpenAiError {
    message: String,
}

/// Call the configured LLM and return raw SVG text.
/// This is a **blocking** HTTP call — should be run in a background thread.
pub fn call_llm(config: &AiConfig, user_prompt: &str) -> Result<String, String> {
    let full_prompt = format!(
        r#"Create a laser-cut artwork of: {prompt}

Step-by-step plan:
1. SILHOUETTE: Design one smooth closed path (class="cut") for the complete outer outline. Use C (cubic bezier) curves for organic shapes. The silhouette should be immediately recognizable as "{prompt}".
2. FEATURES: Add 3-6 paths (class="engrave") for the main internal features — eyes, beak/mouth, limbs, major structural lines.
3. TEXTURE: Add 4-8 paths (class="fine") for surface detail — feathers, fur, bark, scales, hatching lines.

Output ONLY <path> tags now. Use C curves, not L lines. Coordinates 0-100."#,
        prompt = user_prompt
    );

    match config.backend {
        AiBackend::Ollama => call_ollama(config, &full_prompt),
        AiBackend::OpenAi => call_openai(config, &full_prompt),
        AiBackend::Gemini => call_gemini(config, &full_prompt),
    }
}

fn call_ollama(config: &AiConfig, prompt: &str) -> Result<String, String> {
    let base = config.endpoint.trim_end_matches('/');

    // Try /api/chat first (newer Ollama versions, works with more models)
    match call_ollama_chat(base, config, prompt) {
        Ok(text) => return Ok(text),
        Err(e) => {
            // If 404, try the legacy /api/generate endpoint
            if e.contains("404") || e.contains("not found") {
                match call_ollama_generate(base, config, prompt) {
                    Ok(text) => return Ok(text),
                    Err(e2) => return Err(format!(
                        "Ollama: both /api/chat and /api/generate failed.\n\
                         Chat error: {}\nGenerate error: {}\n\n\
                         Make sure Ollama is running (ollama serve) and the model is a TEXT model \
                         (e.g. llama3.1:8b, mistral, codellama). Image models like Flux cannot generate SVG.",
                        e, e2
                    )),
                }
            }
            // Non-404 error: could be model not found, connection refused, etc.
            if e.contains("connection refused") || e.contains("Connection refused") {
                return Err(format!(
                    "Cannot connect to Ollama at {}. \
                     Make sure Ollama is running: ollama serve",
                    base
                ));
            }
            return Err(e);
        }
    }
}

fn call_ollama_chat(base: &str, config: &AiConfig, prompt: &str) -> Result<String, String> {
    let url = format!("{}/api/chat", base);
    let body = OllamaChatRequest {
        model: config.model.clone(),
        messages: vec![
            OllamaChatMessage {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            OllamaChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
        stream: false,
        options: OllamaOptions {
            num_predict: config.max_tokens,
            temperature: 0.7,
        },
    };

    let resp = ureq::post(&url)
        .set("Content-Type", "application/json")
        .send_json(serde_json::to_value(&body).map_err(|e| e.to_string())?)
        .map_err(|e| format!("{}", e))?;

    let parsed: OllamaChatResponse = resp
        .into_json()
        .map_err(|e| format!("Ollama chat parse error: {}", e))?;

    if let Some(err) = parsed.error {
        return Err(format!("Ollama error: {}", err));
    }

    parsed
        .message
        .map(|m| m.content)
        .ok_or_else(|| "Ollama returned empty chat response".to_string())
}

fn call_ollama_generate(base: &str, config: &AiConfig, prompt: &str) -> Result<String, String> {
    let url = format!("{}/api/generate", base);
    let body = OllamaGenerateRequest {
        model: config.model.clone(),
        prompt: prompt.to_string(),
        system: SYSTEM_PROMPT.to_string(),
        stream: false,
        options: OllamaOptions {
            num_predict: config.max_tokens,
            temperature: 0.7,
        },
    };

    let resp = ureq::post(&url)
        .set("Content-Type", "application/json")
        .send_json(serde_json::to_value(&body).map_err(|e| e.to_string())?)
        .map_err(|e| format!("{}", e))?;

    let parsed: OllamaGenerateResponse = resp
        .into_json()
        .map_err(|e| format!("Ollama generate parse error: {}", e))?;

    if let Some(err) = parsed.error {
        return Err(format!("Ollama error: {}", err));
    }

    parsed.response.ok_or_else(|| "Ollama returned empty response".to_string())
}

fn call_openai(config: &AiConfig, prompt: &str) -> Result<String, String> {
    let url = format!(
        "{}/v1/chat/completions",
        config.endpoint.trim_end_matches('/')
    );
    let body = OpenAiRequest {
        model: config.model.clone(),
        messages: vec![
            OpenAiMessage {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            OpenAiMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
        max_tokens: config.max_tokens,
        temperature: 0.7,
    };

    let mut req = ureq::post(&url).set("Content-Type", "application/json");

    if !config.api_key.is_empty() {
        req = req.set("Authorization", &format!("Bearer {}", config.api_key));
    }

    let resp = req
        .send_json(serde_json::to_value(&body).map_err(|e| e.to_string())?)
        .map_err(|e| format!("OpenAI request failed: {}", e))?;

    let parsed: OpenAiResponse = resp
        .into_json()
        .map_err(|e| format!("OpenAI parse error: {}", e))?;

    if let Some(err) = parsed.error {
        return Err(format!("OpenAI error: {}", err.message));
    }

    parsed
        .choices
        .and_then(|c| c.into_iter().next())
        .map(|c| c.message.content)
        .ok_or_else(|| "OpenAI returned no choices".to_string())
}

// ── Gemini ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(rename = "systemInstruction")]
    system_instruction: GeminiContent,
    #[serde(rename = "generationConfig")]
    generation_config: GeminiGenerationConfig,
}

#[derive(Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize, Deserialize)]
struct GeminiPart {
    text: String,
}

#[derive(Serialize)]
struct GeminiGenerationConfig {
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
    error: Option<GeminiError>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiContentResp>,
}

#[derive(Deserialize)]
struct GeminiContentResp {
    parts: Option<Vec<GeminiPart>>,
}

#[derive(Deserialize)]
struct GeminiError {
    message: String,
}

fn call_gemini(config: &AiConfig, prompt: &str) -> Result<String, String> {
    // Gemini endpoint: https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={key}
    let base = config.endpoint.trim_end_matches('/');
    let url = format!(
        "{}/v1beta/models/{}:generateContent?key={}",
        base, config.model, config.api_key
    );

    let body = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![GeminiPart {
                text: prompt.to_string(),
            }],
        }],
        system_instruction: GeminiContent {
            parts: vec![GeminiPart {
                text: SYSTEM_PROMPT.to_string(),
            }],
        },
        generation_config: GeminiGenerationConfig {
            max_output_tokens: config.max_tokens,
            temperature: 0.7,
        },
    };

    let resp = ureq::post(&url)
        .set("Content-Type", "application/json")
        .send_json(serde_json::to_value(&body).map_err(|e| e.to_string())?)
        .map_err(|e| format!("Gemini request failed: {}", e))?;

    let parsed: GeminiResponse = resp
        .into_json()
        .map_err(|e| format!("Gemini parse error: {}", e))?;

    if let Some(err) = parsed.error {
        return Err(format!("Gemini error: {}", err.message));
    }

    parsed
        .candidates
        .and_then(|c| c.into_iter().next())
        .and_then(|c| c.content)
        .and_then(|c| c.parts)
        .and_then(|p| p.into_iter().next())
        .map(|p| p.text)
        .ok_or_else(|| "Gemini returned no content".to_string())
}
