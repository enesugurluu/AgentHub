import { CrownIcon } from 'lucide-react'

import { resolveDeskStatus, StatusBadge } from '@/components/OfficeFloor/StatusBadge'
import type { AgentRecord } from '@/lib/ipc'
import { cn } from '@/lib/utils'
import { useTerminalStore } from '@/store/terminal'

/**
 * Ajan masası (docs 5.4): SVG görünümünde HTML kart — masa + avatar (avatarColor) +
 * durum rozeti. Tıklama → Inspector + terminal sekmesi (docs 5.5 çift tık M2'de).
 */
export function AgentDesk({
  agent,
  isPlayer = false,
  selected = false,
  onSelect,
}: {
  agent: AgentRecord
  isPlayer?: boolean
  selected?: boolean
  onSelect: (id: number) => void
}) {
  const session = useTerminalStore((s) => s.sessions[String(agent.id)])
  const status = resolveDeskStatus(session?.status, agent.status)

  return (
    <button
      type="button"
      tabIndex={0}
      aria-label={`${agent.name} (${agent.role}) — ${status}`}
      onClick={() => onSelect(agent.id)}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault()
          onSelect(agent.id)
        }
      }}
      className={cn(
        'flex w-44 flex-col items-center gap-1.5 rounded-xl border bg-card px-4 py-4 text-center shadow-sm transition-all',
        'hover:-translate-y-0.5 hover:shadow-md focus-visible:ring-2 focus-visible:ring-ring',
        'motion-reduce:transition-none motion-reduce:hover:translate-y-0',
        isPlayer ? 'border-primary/40' : 'border-border',
        selected && 'border-primary/70 ring-1 ring-primary/40',
      )}
    >
      {isPlayer && (
        <span className="flex items-center gap-1 text-xs font-medium text-primary">
          <CrownIcon className="size-3.5" />
          CEO Masası — Sen
        </span>
      )}
      <span
        className="flex size-10 items-center justify-center rounded-full text-sm font-semibold text-white"
        style={{ backgroundColor: agent.avatarColor ?? 'oklch(0.45 0.1 260)' }}
      >
        {agent.name.slice(0, 1).toUpperCase()}
      </span>
      <span className="text-sm font-medium">{agent.name}</span>
      <span className="text-xs text-muted-foreground">{agent.role}</span>
      <StatusBadge status={status} />
    </button>
  )
}
