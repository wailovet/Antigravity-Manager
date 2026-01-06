import { request as invoke } from '../utils/request';
import { RateLimitStatus } from '../types/rateLimit';

export async function getProxyRateLimits(): Promise<RateLimitStatus[]> {
    return await invoke('get_proxy_rate_limits');
}
