import { X, Clock, AlertCircle } from 'lucide-react';
import { createPortal } from 'react-dom';
import { Account, ModelQuota } from '../../types/account';
import { RateLimitStatus } from '../../types/rateLimit';
import { formatDate, formatDurationSeconds, formatRateLimitModels } from '../../utils/format';
import { useTranslation } from 'react-i18next';

interface AccountDetailsDialogProps {
    account: Account | null;
    rateLimit?: RateLimitStatus | null;
    onClose: () => void;
}

export default function AccountDetailsDialog({ account, rateLimit, onClose }: AccountDetailsDialogProps) {
    const { t } = useTranslation();
    if (!account) return null;
    const quotaUnknown = !account.quota?.is_forbidden && (!account.quota?.models || account.quota.models.length === 0);
    const quotaUnknownReason = quotaUnknown
        ? (account.quota_last_attempt_at ? t('accounts.quota_unknown_reason_recent') : t('accounts.quota_unknown_reason_initial'))
        : '';
    const quotaUnknownLastAttempt = quotaUnknown && account.quota_last_attempt_at
        ? formatDate(account.quota_last_attempt_at)
        : null;
    const rateLimitModel = rateLimit ? formatRateLimitModels(rateLimit.model, rateLimit.models) : null;

    return createPortal(
        <div className="modal modal-open z-[100]">
            {/* Draggable Top Region */}
            <div data-tauri-drag-region className="fixed top-0 left-0 right-0 h-8 z-[110]" />

            <div className="modal-box relative max-w-3xl bg-white dark:bg-base-100 shadow-2xl rounded-2xl p-0 overflow-hidden">
                {/* Header */}
                <div className="px-6 py-5 border-b border-gray-100 dark:border-base-200 bg-gray-50/50 dark:bg-base-200/50 flex justify-between items-center">
                    <div className="flex items-center gap-3">
                        <h3 className="font-bold text-lg text-gray-900 dark:text-base-content">{t('accounts.details.title')}</h3>
                        <div className="px-2.5 py-0.5 rounded-full bg-gray-100 dark:bg-base-200 border border-gray-200 dark:border-base-300 text-xs font-mono text-gray-500 dark:text-gray-400">
                            {account.email}
                        </div>
                    </div>
                    <button
                        onClick={onClose}
                        className="btn btn-sm btn-circle btn-ghost text-gray-400 hover:bg-gray-100 dark:hover:bg-base-200 hover:text-gray-600 dark:hover:text-base-content transition-colors"
                    >
                        <X size={18} />
                    </button>
                </div>

                {/* Content */}
                <div className="p-6 grid grid-cols-1 md:grid-cols-2 gap-4 max-h-[60vh] overflow-y-auto">
                    {rateLimit && (
                        <div className="md:col-span-2 p-4 rounded-xl border border-amber-100 dark:border-amber-900/40 bg-amber-50/60 dark:bg-amber-900/20">
                            <div className="flex items-center gap-2">
                                <AlertCircle className="w-4 h-4 text-amber-500" />
                                <span className="text-sm font-semibold text-amber-700 dark:text-amber-300">
                                    {t('accounts.rate_limit_active')}
                                </span>
                                <span className="text-xs text-amber-600/80 dark:text-amber-300/80">
                                    {t(`accounts.rate_limit_reasons.${rateLimit.reason}`)}
                                </span>
                            </div>
                            <div className="mt-2 grid grid-cols-1 sm:grid-cols-2 gap-2 text-xs text-amber-700/90 dark:text-amber-200/80">
                                <div className="flex items-center gap-1.5">
                                    <Clock size={12} />
                                    {t('accounts.rate_limit_badge', { time: formatDurationSeconds(rateLimit.remaining_seconds) })}
                                </div>
                                {rateLimitModel && (
                                    <div>
                                        {t('accounts.rate_limit_model')}: {rateLimitModel}
                                    </div>
                                )}
                                <div>
                                    {t('accounts.rate_limit_reset')}: {formatDate(rateLimit.reset_at) || t('common.unknown')}
                                </div>
                            </div>
                        </div>
                    )}
                    {quotaUnknown && (
                        <div className="md:col-span-2 p-4 rounded-xl border border-slate-200 dark:border-slate-800/60 bg-slate-50/70 dark:bg-slate-900/30">
                            <div className="flex items-center gap-2">
                                <AlertCircle className="w-4 h-4 text-slate-500" />
                                <span className="text-sm font-semibold text-slate-700 dark:text-slate-200">
                                    {t('accounts.quota_unknown_title')}
                                </span>
                                <span className="text-xs text-slate-500 dark:text-slate-300">
                                    {quotaUnknownReason}
                                </span>
                            </div>
                            <div className="mt-2 grid grid-cols-1 sm:grid-cols-2 gap-2 text-xs text-slate-600 dark:text-slate-300">
                                <div>{t('accounts.quota_unknown_hint')}</div>
                                {quotaUnknownLastAttempt && (
                                    <div>
                                        {t('accounts.quota_unknown_last_attempt')}: {quotaUnknownLastAttempt}
                                    </div>
                                )}
                            </div>
                        </div>
                    )}
                    {account.quota?.models?.map((model: ModelQuota) => (
                        <div key={model.name} className="p-4 rounded-xl border border-gray-100 dark:border-base-200 bg-white dark:bg-base-100 hover:border-blue-100 dark:hover:border-blue-900 hover:shadow-sm transition-all group">
                            <div className="flex justify-between items-start mb-3">
                                <span className="text-sm font-medium text-gray-700 dark:text-gray-300 group-hover:text-blue-700 dark:group-hover:text-blue-400 transition-colors">
                                    {model.name}
                                </span>
                                <span
                                    className={`text-xs font-bold px-2 py-0.5 rounded-md ${model.percentage >= 50 ? 'bg-green-50 text-green-700 dark:bg-green-900/30 dark:text-green-400' :
                                        model.percentage >= 20 ? 'bg-orange-50 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400' :
                                            'bg-red-50 text-red-700 dark:bg-red-900/30 dark:text-red-400'
                                        }`}
                                >
                                    {model.percentage}%
                                </span>
                            </div>

                            {/* Progress Bar */}
                            <div className="h-1.5 w-full bg-gray-100 dark:bg-base-200 rounded-full overflow-hidden mb-3">
                                <div
                                    className={`h-full rounded-full transition-all duration-500 ${model.percentage >= 50 ? 'bg-emerald-500' :
                                        model.percentage >= 20 ? 'bg-orange-400' :
                                            'bg-red-500'
                                        }`}
                                    style={{ width: `${model.percentage}%` }}
                                ></div>
                            </div>

                            <div className="flex items-center gap-1.5 text-[10px] text-gray-400 dark:text-gray-500 font-mono">
                                <Clock size={10} />
                                <span>{t('accounts.reset_time')}: {formatDate(model.reset_time) || t('common.unknown')}</span>
                            </div>
                        </div>
                    )) || (
                            <div className="col-span-2 py-10 text-center text-gray-400 flex flex-col items-center">
                                <AlertCircle className="w-8 h-8 mb-2 opacity-20" />
                                <span>{t('accounts.no_data')}</span>
                            </div>
                        )}
                </div>
            </div>
            <div className="modal-backdrop bg-black/40 backdrop-blur-sm" onClick={onClose}></div>
        </div>,
        document.body
    );
}
