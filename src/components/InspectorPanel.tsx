import {
  BotIcon,
  CircleDollarSignIcon,
  ClipboardListIcon,
  FolderGit2Icon,
  KeyRoundIcon,
} from 'lucide-react'
import { useState } from 'react'

import { FireButton } from '@/components/Settings/FireDialog'
import { TaskDialog } from '@/components/Tasks/TaskDialog'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Separator } from '@/components/ui/separator'
import { selectVisibleAgents, useAgentStore } from '@/store/agents'
import { useTerminalStore } from '@/store/terminal'

export function InspectorPanel() {
  const { agents, selectedAgentId } = useAgentStore()
  const sessions = useTerminalStore((s) => s.sessions)
  const [taskOpen, setTaskOpen] = useState(false)

  // `fired` ajanlar Inspector'da da görünmez (WP-08).
  const agent = selectVisibleAgents(agents).find((a) => a.id === selectedAgentId) ?? null
  const session = agent ? sessions[String(agent.id)] : undefined

  if (!agent) {
    return (
      <aside className="flex h-full items-center justify-center border-l border-border px-4 text-center text-xs text-muted-foreground">
        Ajan seçilmedi
      </aside>
    )
  }

  const rows: { icon: React.ReactNode; label: string; value: string }[] = [
    { icon: <BotIcon className="size-3.5" />, label: 'Motor', value: agent.motor },
    { icon: <KeyRoundIcon className="size-3.5" />, label: 'Model', value: agent.model ?? '—' },
    {
      icon: <FolderGit2Icon className="size-3.5" />,
      label: 'Worktree',
      value: agent.worktreePath ?? '—',
    },
    {
      icon: <CircleDollarSignIcon className="size-3.5" />,
      label: 'Oturum',
      value: session?.executionId ?? 'yok',
    },
  ]

  return (
    <aside className="flex h-full min-h-0 flex-col border-l border-border">
      <Card className="flex-1 overflow-y-auto rounded-none border-0 shadow-none">
        <CardHeader>
          <div className="flex items-center gap-2">
            <span className="flex size-8 items-center justify-center rounded-full bg-muted text-sm font-semibold">
              {agent.name.slice(0, 1).toUpperCase()}
            </span>
            <div>
              <CardTitle className="text-sm">{agent.name}</CardTitle>
              <p className="text-xs text-muted-foreground">{agent.role}</p>
            </div>
          </div>
          <div className="pt-1">
            <Badge variant={session?.status === 'running' ? 'success' : 'secondary'}>
              {session?.status ?? agent.status}
            </Badge>
          </div>
        </CardHeader>
        <CardContent className="flex flex-col gap-2 text-xs">
          <Separator />
          {rows.map((row) => (
            <div key={row.label} className="flex items-center gap-2">
              <span className="text-muted-foreground">{row.icon}</span>
              <span className="w-16 text-muted-foreground">{row.label}</span>
              <span className="min-w-0 flex-1 truncate font-mono">{row.value}</span>
            </div>
          ))}
          <Separator className="mt-2" />
          <div className="flex flex-col gap-2 pt-1">
            {session?.status !== 'running' ? (
              <Button
                variant="outline"
                size="sm"
                className="w-full"
                onClick={() => setTaskOpen(true)}
              >
                <ClipboardListIcon className="size-3.5" />
                Görev Ver
              </Button>
            ) : (
              <p className="text-xs text-muted-foreground">
                Ajan çalışıyor — yeni görev için önce durdurun.
              </p>
            )}
            <FireButton agent={agent} />
          </div>
          {agent && <TaskDialog agent={agent} open={taskOpen} onOpenChange={setTaskOpen} />}
          <p className="pt-1 leading-relaxed text-muted-foreground">
            Ajan detayı, izin profili ve bütçe yönetimi Faz 3'te bu panele gelir.
          </p>
        </CardContent>
      </Card>
    </aside>
  )
}
