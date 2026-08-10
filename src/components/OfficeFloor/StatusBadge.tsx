import {
  AlertTriangleIcon,
  BrainIcon,
  CoffeeIcon,
  HandIcon,
  KeyboardIcon,
  MessagesSquareIcon,
} from 'lucide-react'

import { cn } from '@/lib/utils'

/** Ofis durumları (docs 5.4 tablosu — renk + ikon + animasyon sınıfı). */
export type DeskStatus = 'idle' | 'thinking' | 'working' | 'blocked' | 'error' | 'meeting'

const STATUS_CONFIG: Record<
  DeskStatus,
  { icon: typeof CoffeeIcon; color: string; animation: string; label: string }
> = {
  idle: { icon: CoffeeIcon, color: 'text-muted-foreground', animation: '', label: 'Boşta' },
  thinking: {
    icon: BrainIcon,
    color: 'text-blue-400',
    animation: 'animate-pulse',
    label: 'Düşünüyor',
  },
  working: {
    icon: KeyboardIcon,
    color: 'text-emerald-400',
    animation: 'animate-pulse',
    label: 'Çalışıyor',
  },
  blocked: {
    icon: HandIcon,
    color: 'text-amber-500',
    animation: 'animate-pulse',
    label: 'Onay bekliyor',
  },
  error: {
    icon: AlertTriangleIcon,
    color: 'text-red-500',
    animation: 'animate-pulse',
    label: 'Hata',
  },
  meeting: {
    icon: MessagesSquareIcon,
    color: 'text-purple-400',
    animation: 'animate-pulse',
    label: 'Toplantıda',
  },
}

/** Ajanın canlı durumunu çözer (docs 5.4): oturum durumu önceliklidir. */
export function resolveDeskStatus(
  sessionStatus: string | undefined,
  agentStatus: string,
): DeskStatus {
  if (sessionStatus === 'running') return 'working'
  if (sessionStatus === 'starting') return 'thinking'
  if (sessionStatus === 'waiting') return 'blocked'
  if (sessionStatus === 'exited') return 'idle'
  if (agentStatus === 'thinking') return 'thinking'
  if (agentStatus === 'waiting') return 'blocked'
  if (agentStatus === 'error') return 'error'
  return 'idle'
}

/** Masa üstü durum rozeti — `role="status"` ile erişilebilir (docs 5.8). */
export function StatusBadge({
  status,
  showLabel = false,
}: {
  status: DeskStatus
  showLabel?: boolean
}) {
  const cfg = STATUS_CONFIG[status]
  const Icon = cfg.icon
  return (
    <span
      role="status"
      aria-label={cfg.label}
      title={cfg.label}
      className={cn('flex items-center gap-1 text-xs', cfg.color)}
    >
      <Icon className={cn('size-3.5', cfg.animation, 'motion-reduce:animate-none')} />
      {showLabel && <span className="capitalize">{status}</span>}
    </span>
  )
}
