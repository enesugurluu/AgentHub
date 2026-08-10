import { CrownIcon } from 'lucide-react'

import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'
import { useAgentStore } from '@/store/agents'
import { useTerminalStore } from '@/store/terminal'

/**
 * Ofis katı (Docs Bölüm 5): FAZ0'da ajan masalarının statik bir izdüşümü.
 * Faz 2'de SVG/Canvas interaktif ofis katına dönüşecek.
 */
export function OfficeFloor() {
  const { agents, selectedAgentId, selectAgent } = useAgentStore()
  const sessions = useTerminalStore((s) => s.sessions)
  const setActive = useTerminalStore((s) => s.setActive)

  return (
    <section className="relative flex h-full min-h-0 flex-col overflow-hidden">
      <div
        className="pointer-events-none absolute inset-0 opacity-[0.04]"
        style={{
          backgroundImage:
            'linear-gradient(var(--foreground) 1px, transparent 1px), linear-gradient(90deg, var(--foreground) 1px, transparent 1px)',
          backgroundSize: '28px 28px',
        }}
      />
      <div className="relative z-10 flex h-full flex-col items-center justify-center gap-6 p-6">
        {/* CEO masası */}
        <div className="flex items-center gap-3 rounded-xl border border-primary/30 bg-card px-6 py-4 shadow-lg">
          <CrownIcon className="size-5 text-primary" />
          <div>
            <p className="text-sm font-semibold">CEO Masası — Sen</p>
            <p className="text-xs text-muted-foreground">
              Kanban'dan görev dağıt, onay akışını yönet (Faz 3)
            </p>
          </div>
        </div>

        {/* Çalışan masaları */}
        <div className="grid grid-cols-2 gap-4 lg:grid-cols-3 xl:grid-cols-4">
          {agents.length === 0 && (
            <div className="col-span-full rounded-lg border border-dashed border-border px-8 py-6 text-center text-sm text-muted-foreground">
              Masalar boş — Faz 2'de işe alım sihirbazı ile doldurulacak
            </div>
          )}
          {agents.map((agent) => {
            const session = sessions[String(agent.id)]
            const running = session?.status === 'running'
            return (
              <button
                key={agent.id}
                type="button"
                onClick={() => {
                  selectAgent(agent.id)
                  setActive(String(agent.id))
                }}
                className={cn(
                  'flex w-44 flex-col items-center gap-1.5 rounded-xl border bg-card px-4 py-4 text-center shadow-sm transition-all hover:-translate-y-0.5 hover:shadow-md',
                  selectedAgentId === agent.id
                    ? 'border-primary/60 ring-1 ring-primary/40'
                    : 'border-border',
                )}
              >
                <span
                  className={cn(
                    'flex size-10 items-center justify-center rounded-full text-lg',
                    running ? 'bg-emerald-500/15' : 'bg-muted',
                  )}
                >
                  {agent.name.slice(0, 1).toUpperCase()}
                </span>
                <span className="text-sm font-medium">{agent.name}</span>
                <span className="text-xs text-muted-foreground">{agent.role}</span>
                <Badge variant={running ? 'success' : 'secondary'} className="mt-1 uppercase">
                  {session?.status ?? agent.status}
                </Badge>
              </button>
            )
          })}
        </div>
      </div>
    </section>
  )
}
