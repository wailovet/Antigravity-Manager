use super::gemini_models::UsageMetadata;
use super::models::Usage;

/// 将 Gemini UsageMetadata 转换为 OpenAI Usage
pub fn to_openai_usage(usage_metadata: &UsageMetadata) -> Usage {
    let prompt_tokens = usage_metadata.prompt_token_count.unwrap_or(0);
    let completion_tokens = usage_metadata.candidates_token_count.unwrap_or(0);
    
    let total_tokens = usage_metadata
        .total_token_count
        .unwrap_or(prompt_tokens + completion_tokens);

    Usage {
        prompt_tokens,
        completion_tokens,
        total_tokens,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::gemini_models::UsageMetadata;

    #[test]
    fn test_to_openai_usage() {
        let usage = UsageMetadata {
            prompt_token_count: Some(100),
            candidates_token_count: Some(50),
            total_token_count: Some(150),
            cached_content_token_count: None,
        };

        let openai_usage = to_openai_usage(&usage);
        assert_eq!(openai_usage.prompt_tokens, 100);
        assert_eq!(openai_usage.completion_tokens, 50);
        assert_eq!(openai_usage.total_tokens, 150);
    }
}
