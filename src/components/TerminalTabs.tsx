import { TerminalIcon } from 'lucide-react'

import { PtyTerminal } from '@/components/PtyTerminal'
import { cn } from '@/lib/utils'
import { useAgentStore } from '@/store/agents'
import { useTerminalStore } from '@/store/terminal'

/**
 * Alt terminal alanı (Docs Bölüm 5.3 TerminalTabs).
 * FAZ0'da seçili ajan için tek sekme + çalışan oturumlar için ek sekmeler.
 */
export function TerminalTabs() {
  const { agents, selectedAgentId } = useAgentStore()
  const sessions = useTerminalStore((s) => s.sessions)
  const activeSessionAgentId = useTerminalStore((s) => s.activeSessionAgentId)
  const setActive = useTerminalStore((s) => s.setActive)

  const agentName = (id: string) => {
    const agent = agents.find((a) => String(a.id) === id)
    return agent?.name ?? `ajan-${id}`
  }

  // Sıra: seçili ajan önce, sonra çalışan oturumlar.
  const tabs = new Map<string, string>()
  if (selectedAgentId !== null)
    tabs.set(String(selectedAgentId), agentName(String(selectedAgentId)))
  for (const agentId of Object.keys(sessions)) {
    if (!tabs.has(agentId)) tabs.set(agentId, agentName(agentId))
  }

  if (tabs.size === 0) {
    return (
      <footer className="flex h-56 shrink-0 items-center justify-center gap-2 border-t border-border text-sm text-muted-foreground">
        <TerminalIcon className="size-4" />
        Soldan bir ajan seç ve terminalde oturum başlat
      </footer>
    )
  }

  const activeId =
    activeSessionAgentId && tabs.has(activeSessionAgentId)
      ? activeSessionAgentId
      : (tabs.keys().next().value as string)

  return (
    <footer className="flex h-64 shrink-0 flex-col border-t border-border">
      <div className="flex h-9 shrink-0 items-center gap-1 border-b border-border px-2">
        {[...tabs.entries()].map(([agentId, name]) => (
          <button
            key={agentId}
            type="button"
            onClick={() => setActive(agentId)}
            className={cn(
              'flex h-7 items-center gap-1.5 rounded-t-md px-3 text-xs font-medium transition-colors',
              agentId === activeId
                ? 'border-x border-t border-border bg-background text-foreground'
                : 'text-muted-foreground hover:text-foreground',
            )}
          >
            <span
              className={cn(
                'size-1.5 rounded-full',
                sessions[agentId]?.status === 'running'
                  ? 'bg-emerald-400'
                  : 'bg-muted-foreground/40',
              )}
            />
            {name}
          </button>
        ))}
      </div>
      <div className="min-h-0 flex-1 p-2">
        {/* Her sekmenin terminali mount'lu kalır: key yalnızca ajan id'sidir.
            Böylece (a) spawn sonrası executionId set edilince remount olmaz
            (backend'e verilen Channel dispose edilmiş terminale bağlı kalmaz)
            ve (b) sekme değişince çıktı/scrollback kaybolmaz. Gizli sekmeler
            display:none ile tutulur; xterm arka planda yazmaya devam eder. */}
        {[...tabs.keys()].map((tabAgentId) => {
          const isActive = tabAgentId === activeId
          return (
            <div key={tabAgentId} className={cn('h-full min-h-0', isActive ? 'block' : 'hidden')}>
              <PtyTerminal
                agentId={tabAgentId}
                isActive={isActive}
                engine={sessions[tabAgentId]?.engineType === 'claude' ? 'claude' : 'pty'}
              />
            </div>
          )
        })}
      </div>
    </footer>
  )
}
