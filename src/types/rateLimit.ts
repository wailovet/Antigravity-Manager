export type RateLimitReason = 'quota_exhausted' | 'rate_limit_exceeded' | 'server_error' | 'unknown';

export interface RateLimitStatus {
    account_id: string;
    model: string;
    models?: string[];
    reason: RateLimitReason;
    reset_at: number;
    remaining_seconds: number;
}
