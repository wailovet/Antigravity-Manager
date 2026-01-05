// Gemini v1internal 数据模型
// Copied from ../claude/models.rs to isolate OpenAI dependency
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V1InternalRequest {
    pub project: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub request: serde_json::Value,
    pub model: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    #[serde(rename = "requestType")]
    pub request_type: String,
}

/// Gemini Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiContent {
    pub role: String,
    pub parts: Vec<GeminiPart>,
}

/// Gemini Part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thoughtSignature")]
    pub thought_signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functionCall")]
    pub function_call: Option<FunctionCall>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functionResponse")]
    pub function_response: Option<FunctionResponse>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inlineData")]
    pub inline_data: Option<InlineData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionResponse {
    pub name: String,
    pub response: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineData {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub data: String,
}

/// Gemini 完整响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<Candidate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: Option<UsageMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modelVersion")]
    pub model_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "responseId")]
    pub response_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "finishReason")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groundingMetadata")]
    pub grounding_metadata: Option<GroundingMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "promptTokenCount")]
    pub prompt_token_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "candidatesTokenCount")]
    pub candidates_token_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cachedContentTokenCount")]
    pub cached_content_token_count: Option<u32>,
}

// ========== Grounding Metadata (for googleSearch results) ==========

/// Gemini Grounding Metadata - contains search results from googleSearch tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundingMetadata {
    #[serde(rename = "webSearchQueries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search_queries: Option<Vec<String>>,

    #[serde(rename = "groundingChunks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounding_chunks: Option<Vec<GroundingChunk>>,

    #[serde(rename = "groundingSupports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounding_supports: Option<Vec<GroundingSupport>>,

    #[serde(rename = "searchEntryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_entry_point: Option<SearchEntryPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundingChunk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<WebSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundingSupport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<TextSegment>,
    #[serde(rename = "groundingChunkIndices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounding_chunk_indices: Option<Vec<i32>>,
    #[serde(rename = "confidenceScores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_scores: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSegment {
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
    #[serde(rename = "endIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEntryPoint {
    #[serde(rename = "renderedContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendered_content: Option<String>,
}
