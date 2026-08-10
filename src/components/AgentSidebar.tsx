import { BotIcon, PlusIcon } from 'lucide-react'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { ScrollArea } from '@/components/ui/scroll-area'
import { cn } from '@/lib/utils'
import { useAgentStore } from '@/store/agents'
import { useTerminalStore } from '@/store/terminal'

const STATUS_VARIANT: Record<string, 'success' | 'warning' | 'secondary' | 'destructive'> = {
  idle: 'secondary',
  running: 'success',
  thinking: 'warning',
  waiting: 'warning',
  error: 'destructive',
  done: 'success',
}

export function AgentSidebar() {
  const { agents, selectedAgentId, selectAgent, loading } = useAgentStore()
  const sessions = useTerminalStore((s) => s.sessions)
  const setActive = useTerminalStore((s) => s.setActive)

  return (
    <aside className="flex h-full min-h-0 flex-col border-r border-border">
      <div className="flex items-center justify-between px-3 py-2.5">
        <h2 className="text-xs font-semibold tracking-wide text-muted-foreground uppercase">
          Ajanlar
        </h2>
        <Button variant="ghost" size="icon" className="size-6" title="İşe al (Faz 2)">
          <PlusIcon className="size-3.5" />
        </Button>
      </div>

      <ScrollArea className="min-h-0 flex-1">
        <div className="flex flex-col gap-1 px-2 pb-2">
          {loading && agents.length === 0 ? (
            <p className="px-2 py-1 text-xs text-muted-foreground">Yükleniyor…</p>
          ) : agents.length === 0 ? (
            <div className="px-2 py-1">
              <p className="text-xs text-muted-foreground">
                Kayıtlı ajan yok. Veritabanı seed verisiyle gelir.
              </p>
            </div>
          ) : (
            agents.map((agent) => {
              const session = sessions[String(agent.id)]
              const selected = selectedAgentId === agent.id
              return (
                <button
                  key={agent.id}
                  type="button"
                  onClick={() => {
                    selectAgent(agent.id)
                    setActive(String(agent.id))
                  }}
                  className={cn(
                    'flex items-center gap-2 rounded-md px-2 py-2 text-left transition-colors',
                    selected ? 'bg-accent text-accent-foreground' : 'hover:bg-muted/60',
                  )}
                >
                  <BotIcon
                    className={cn(
                      'size-4 shrink-0',
                      session?.status === 'running' ? 'text-emerald-400' : 'text-muted-foreground',
                    )}
                  />
                  <span className="min-w-0 flex-1">
                    <span className="block truncate text-sm font-medium">{agent.name}</span>
                    <span className="block truncate text-xs text-muted-foreground">
                      {agent.role} · {agent.motor}
                    </span>
                  </span>
                  <Badge
                    variant={STATUS_VARIANT[session?.status ?? agent.status] ?? 'secondary'}
                    className="uppercase"
                  >
                    {session?.status ?? agent.status}
                  </Badge>
                </button>
              )
            })
          )}
        </div>
      </ScrollArea>
    </aside>
  )
}
