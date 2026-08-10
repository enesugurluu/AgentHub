import { UserXIcon } from 'lucide-react'
import { useState } from 'react'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import { Button } from '@/components/ui/button'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import type { AgentRecord, FireOptions } from '@/lib/ipc'
import { cn } from '@/lib/utils'
import { useAgentStore } from '@/store/agents'
import { useTerminalStore } from '@/store/terminal'

/**
 * İşten çıkarma onay akışı (docs 6.2; WP-08):
 * açık görevler → backlog (varsayılan) · worktree: sil/koru/commit'le-sakla ·
 * konuşma loglarını sakla (varsayılan). "Kalıcı Sil" yalnızca `fired` kayıtlar için.
 */
export function FireDialog({
  agent,
  open,
  onOpenChange,
}: {
  agent: AgentRecord
  open: boolean
  onOpenChange: (open: boolean) => void
}) {
  const fireAgent = useAgentStore((s) => s.fireAgent)
  const sessions = useTerminalStore((s) => s.sessions)
  const [moveTasks, setMoveTasks] = useState(true)
  const [keepLogs, setKeepLogs] = useState(true)
  const [worktreeAction, setWorktreeAction] = useState<'delete' | 'keep' | 'commit_and_keep'>(
    'delete',
  )
  const [error, setError] = useState<string | null>(null)

  const running = sessions[String(agent.id)]?.status === 'running'

  const confirm = async () => {
    setError(null)
    const options: FireOptions = {
      worktreeAction,
      moveOpenTasksToBacklog: moveTasks,
      keepLogs,
    }
    const ok = await fireAgent(agent.id, options)
    if (!ok) {
      setError('İşten çıkarma başarısız oldu — tekrar deneyin.')
      return
    }
    onOpenChange(false)
  }

  return (
    <AlertDialog open={open} onOpenChange={onOpenChange}>
      <AlertDialogContent className="sm:max-w-md">
        <AlertDialogHeader>
          <AlertDialogTitle className="flex items-center gap-2">
            <UserXIcon className="size-4 text-destructive" />
            {agent.name} işten çıkarılsın mı?
          </AlertDialogTitle>
          <AlertDialogDescription>
            Ajan `fired` durumuna geçer, ofisten ve listeden kalkar. Geçmiş kayıtlar (events)
            denetim için saklanır.
          </AlertDialogDescription>
        </AlertDialogHeader>

        {running && (
          <p className="rounded-md border border-amber-500/40 bg-amber-500/10 px-3 py-2 text-xs text-amber-500">
            Ajanın aktif bir oturumu var — onaylarsan önce durdurulacak.
          </p>
        )}
        {error && <p className="text-xs text-destructive">{error}</p>}

        <div className="flex flex-col gap-3">
          <div>
            <Label className="text-xs">Açık görevler</Label>
            <div className="mt-1.5 flex items-center gap-2 text-sm">
              <Switch
                checked={moveTasks}
                onCheckedChange={setMoveTasks}
                aria-label="Görevleri Backlog'a geri al"
              />
              Görevleri Backlog'a geri al
            </div>
          </div>

          <div>
            <Label className="text-xs">Worktree</Label>
            <div className="mt-1.5 flex flex-col gap-1.5">
              {(
                [
                  { id: 'delete', label: 'Sil (değişiklikler ziyan olur)' },
                  { id: 'keep', label: 'Koru (başka biri devralabilir)' },
                  { id: 'commit_and_keep', label: 'Değişiklikleri commit’le ve sakla' },
                ] as const
              ).map((opt) => (
                <label
                  key={opt.id}
                  className={cn(
                    'flex cursor-pointer items-center gap-2 rounded-md border px-3 py-2 text-sm transition-colors',
                    worktreeAction === opt.id ? 'border-primary/60 bg-accent' : 'border-border',
                  )}
                >
                  <input
                    type="radio"
                    name="worktree-action"
                    className="accent-primary"
                    checked={worktreeAction === opt.id}
                    onChange={() => setWorktreeAction(opt.id)}
                  />
                  {opt.label}
                </label>
              ))}
            </div>
          </div>

          <div>
            <Label className="text-xs">Konuşma geçmişi</Label>
            <div className="mt-1.5 flex items-center gap-2 text-sm">
              <Switch
                checked={keepLogs}
                onCheckedChange={setKeepLogs}
                aria-label="Oturum loglarını sakla"
              />
              Oturum loglarını sakla (JSONL)
            </div>
          </div>
        </div>

        <AlertDialogFooter>
          <AlertDialogCancel>Vazgeç</AlertDialogCancel>
          <AlertDialogAction
            className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            onClick={(e) => {
              e.preventDefault()
              void confirm()
            }}
          >
            İşten Çıkar
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}

/** Inspector'da kullanılan açma düğmesi. */
export function FireButton({ agent }: { agent: AgentRecord }) {
  const [open, setOpen] = useState(false)
  return (
    <>
      <Button variant="destructive" size="sm" className="w-full" onClick={() => setOpen(true)}>
        <UserXIcon className="size-3.5" />
        İşten Çıkar
      </Button>
      <FireDialog agent={agent} open={open} onOpenChange={setOpen} />
    </>
  )
}
