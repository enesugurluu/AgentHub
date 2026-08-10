import { RefreshCcwIcon } from 'lucide-react'
import { useCallback, useEffect, useState } from 'react'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  type EngineMetadata,
  isTauriRuntime,
  ptyAdapterMetadata,
  ptyListEngineAdapters,
} from '@/lib/ipc'

type AdapterInfo = {
  id: string
  metadata: EngineMetadata | null
}

export function SettingsDialog({ trigger }: { trigger: React.ReactNode }) {
  const [open, setOpen] = useState(false)
  const [adapters, setAdapters] = useState<AdapterInfo[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const refresh = useCallback(async () => {
    if (!isTauriRuntime()) {
      setError('Tauri runtime yok — web önizlemesinde adaptör listesi görüntülenemez.')
      setAdapters([])
      return
    }
    setLoading(true)
    setError(null)
    try {
      const ids = await ptyListEngineAdapters('all')
      const infos = await Promise.all(
        ids.map(async (id) => {
          try {
            const metadata = await ptyAdapterMetadata(id)
            return { id, metadata }
          } catch {
            return { id, metadata: null }
          }
        }),
      )
      setAdapters(infos)
    } catch (e) {
      setError(String(e))
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    if (open) void refresh()
  }, [open, refresh])

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>{trigger}</DialogTrigger>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>Ayarlar</DialogTitle>
          <DialogDescription>
            Motor adaptörleri ve sistem durumu. İşe alım/izin profilleri Faz 2+.
          </DialogDescription>
        </DialogHeader>

        <Tabs defaultValue="engines">
          <TabsList>
            <TabsTrigger value="engines">Motorlar</TabsTrigger>
            <TabsTrigger value="system">Sistem</TabsTrigger>
          </TabsList>

          <TabsContent value="engines" className="pt-2">
            <div className="flex items-center justify-between">
              <p className="text-xs text-muted-foreground">Kayıtlı {adapters.length} adaptör</p>
              <Button size="sm" variant="ghost" onClick={refresh} disabled={loading}>
                <RefreshCcwIcon className={loading ? 'size-3.5 animate-spin' : 'size-3.5'} />
                Yenile
              </Button>
            </div>

            {error && <p className="pt-2 text-xs text-destructive">{error}</p>}

            <div className="flex flex-col gap-1.5 pt-2">
              {adapters.map((adapter) => (
                <div
                  key={adapter.id}
                  className="flex items-center gap-2 rounded-md border border-border px-3 py-2"
                >
                  <span className="font-mono text-xs">{adapter.id}</span>
                  <span className="ml-auto flex items-center gap-2">
                    {adapter.metadata?.engineType && (
                      <Badge variant="outline" className="font-mono">
                        {adapter.metadata.engineType}
                      </Badge>
                    )}
                    {adapter.metadata?.version && (
                      <Badge variant="secondary" className="font-mono">
                        v{adapter.metadata.version}
                      </Badge>
                    )}
                    {adapter.metadata?.capabilities.map((cap) => (
                      <Badge key={cap} variant="outline" className="font-mono">
                        {cap}
                      </Badge>
                    ))}
                  </span>
                </div>
              ))}
            </div>
          </TabsContent>

          <TabsContent value="system" className="pt-2 text-xs text-muted-foreground">
            <ul className="flex flex-col gap-1.5">
              <li>· Veritabanı: SQLite + WAL (src-tauri/src/db.rs)</li>
              <li>· PTY: portable-pty 0.9 (ConPTY / POSIX)</li>
              <li>· İzolasyon: Windows Job Objects / Unix process group</li>
              <li>· Worktree: .git/agenthub-worktrees (güvenli path)</li>
            </ul>
          </TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  )
}
