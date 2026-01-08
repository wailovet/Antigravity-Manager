import { request as invoke } from '../utils/request';
import { RateLimitStatus } from '../types/rateLimit';

export async function getProxyRateLimits(): Promise<RateLimitStatus[]> {
    return await invoke('get_proxy_rate_limits');
}

export async function clearProxyRateLimit(accountId: string): Promise<boolean> {
    return await invoke('clear_proxy_rate_limit', { accountId });
}
