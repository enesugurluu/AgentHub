import { ClipboardListIcon } from 'lucide-react'
import { useState } from 'react'

import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Textarea } from '@/components/ui/textarea'
import type { AgentRecord } from '@/lib/ipc'
import { useTasksStore } from '@/store/tasks'
import { useTerminalStore } from '@/store/terminal'

/**
 * "Görev Ver" (docs 13.1; WP-10): görev tanımla → `task_create` → `task_assign`
 * (worktree garanti + AGENT_TASK.md + non-interactive spawn). Çıktı ajanın
 * terminal sekmesine akar; tamamlanma backend'de algılanır.
 */
export function TaskDialog({
  agent,
  open,
  onOpenChange,
}: {
  agent: AgentRecord
  open: boolean
  onOpenChange: (open: boolean) => void
}) {
  const createAndAssign = useTasksStore((s) => s.createAndAssign)
  const [title, setTitle] = useState('')
  const [description, setDescription] = useState('')
  const [acceptance, setAcceptance] = useState('')
  const [priority, setPriority] = useState('3')
  const [budget, setBudget] = useState('')
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const submit = async () => {
    setError(null)
    if (title.trim().length < 3) {
      setError('Görev başlığı en az 3 karakter olmalı.')
      return
    }
    const channel = useTerminalStore.getState().channels[String(agent.id)]
    if (!channel) {
      setError('Terminal kanalı hazır değil — önce ajanın sekmesini aç.')
      return
    }
    setSubmitting(true)
    const ok = await createAndAssign({
      agentId: agent.id,
      title: title.trim(),
      description: description.trim() || undefined,
      acceptanceCriteria: acceptance.trim() || undefined,
      priority: Number(priority) || 3,
      budget: budget ? Number(budget) : null,
      cols: 80,
      rows: 24,
      channel,
    })
    setSubmitting(false)
    if (!ok) {
      setError('Görev atanamadı — tekrar deneyin.')
      return
    }
    setTitle('')
    setDescription('')
    setAcceptance('')
    onOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <ClipboardListIcon className="size-4 text-primary" />
            Görev Ver — {agent.name}
          </DialogTitle>
          <DialogDescription>
            Görev worktree'ye `AGENT_TASK.md` olarak yazılır ve ajan non-interactive çalıştırılır
            (bütçe/turn limitleriyle).
          </DialogDescription>
        </DialogHeader>

        {error && <p className="text-xs text-destructive">{error}</p>}

        <div className="flex flex-col gap-3">
          <div>
            <Label htmlFor="task-title">Başlık</Label>
            <Input
              id="task-title"
              placeholder="örn. JWT auth akışı kur"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              className="mt-1.5"
            />
          </div>
          <div>
            <Label htmlFor="task-desc">Açıklama</Label>
            <Textarea
              id="task-desc"
              rows={3}
              placeholder="Görevin tanımı…"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="mt-1.5"
            />
          </div>
          <div>
            <Label htmlFor="task-accept">Kabul kriterleri</Label>
            <Textarea
              id="task-accept"
              rows={2}
              placeholder="Testler geçiyor, lint temiz…"
              value={acceptance}
              onChange={(e) => setAcceptance(e.target.value)}
              className="mt-1.5"
            />
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <Label htmlFor="task-priority">Öncelik (1–5)</Label>
              <Input
                id="task-priority"
                type="number"
                min={1}
                max={5}
                step={1}
                value={priority}
                onChange={(e) => setPriority(e.target.value)}
                className="mt-1.5"
              />
            </div>
            <div>
              <Label htmlFor="task-budget">Bütçe (USD)</Label>
              <Input
                id="task-budget"
                type="number"
                min={0}
                step="0.1"
                placeholder="örn. 0.5"
                value={budget}
                onChange={(e) => setBudget(e.target.value)}
                className="mt-1.5"
              />
            </div>
          </div>
        </div>

        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            Vazgeç
          </Button>
          <Button onClick={submit} disabled={submitting}>
            {submitting ? 'Atanıyor…' : 'Görevi Ata ve Başlat'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
