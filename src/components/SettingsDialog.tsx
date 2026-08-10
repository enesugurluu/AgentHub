import { RefreshCcwIcon, RocketIcon } from 'lucide-react'
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
import { useEngineRegistry } from '@/hooks/useEngineRegistry'
import { isTauriRuntime } from '@/lib/ipc'
import { useTerminalStore } from '@/store/terminal'

/** Tek adaptör kartı: durum rozeti + capability + kurulum ipucu + "Kur" (WP-12). */
function EngineCard({ id, onInstall }: { id: string; onInstall: (id: string) => void }) {
  const { engines } = useEngineRegistry()
  const engine = engines.find((e) => e.id === id)

  if (!engine) return null
  const { metadata, detectInfo } = engine

  return (
    <div className="flex flex-col gap-1.5 rounded-md border border-border px-3 py-2">
      <div className="flex items-center gap-2">
        <span className="font-mono text-xs">{engine.id}</span>
        <span className="ml-auto flex items-center gap-1.5">
          {engine.detected ? (
            <Badge variant="success" className="uppercase">
              kurulu
            </Badge>
          ) : (
            <Badge variant="destructive" className="uppercase">
              kurulu değil
            </Badge>
          )}
          {metadata?.version && (
            <Badge variant="secondary" className="font-mono">
              v{metadata.version}
            </Badge>
          )}
        </span>
      </div>

      <div className="flex flex-wrap items-center gap-1.5">
        {metadata?.engineType && (
          <Badge variant="outline" className="font-mono">
            {metadata.engineType}
          </Badge>
        )}
        {metadata?.capabilities.map((cap) => (
          <Badge key={cap} variant="outline" className="font-mono">
            {cap}
          </Badge>
        ))}
      </div>

      {!engine.detected && (
        <div className="flex items-center gap-2 pt-1">
          <p className="min-w-0 flex-1 truncate font-mono text-[10px] text-muted-foreground">
            {detectInfo?.installHint ?? 'kurulum komutu tanımlı değil'}
          </p>
          {detectInfo?.installHint && (
            <Button
              size="sm"
              variant="outline"
              className="h-7 shrink-0"
              onClick={() => onInstall(id)}
            >
              <RocketIcon className="size-3.5" />
              Kur
            </Button>
          )}
        </div>
      )}
    </div>
  )
}

export function SettingsDialog({ trigger }: { trigger: React.ReactNode }) {
  const { engines, loading, error, refresh } = useEngineRegistry()
  const [open, setOpen] = useState(false)
  const [installTarget, setInstallTarget] = useState<string | null>(null)

  const startSession = useTerminalStore((s) => s.startSession)
  const setActive = useTerminalStore((s) => s.setActive)

  // Kur onayı → kurulum terminal sekmesini açar (PtyTerminal otomatik başlatır).
  const confirmInstall = () => {
    if (!installTarget) return
    const agentId = `install-${installTarget}`
    startSession(agentId, 'pty')
    setActive(agentId)
    setInstallTarget(null)
  }

  return (
    <>
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogTrigger asChild>{trigger}</DialogTrigger>
        <DialogContent className="sm:max-w-lg">
          <DialogHeader>
            <DialogTitle>Ayarlar</DialogTitle>
            <DialogDescription>
              Motor adaptörleri, kurulum durumu ve sistem bilgisi. İzin profilleri M2'de.
            </DialogDescription>
          </DialogHeader>

          <Tabs defaultValue="engines">
            <TabsList>
              <TabsTrigger value="engines">Motorlar</TabsTrigger>
              <TabsTrigger value="system">Sistem</TabsTrigger>
            </TabsList>

            <TabsContent value="engines" className="pt-2">
              <div className="flex items-center justify-between">
                <p className="text-xs text-muted-foreground">Kayıtlı {engines.length} adaptör</p>
                <Button size="sm" variant="ghost" onClick={refresh} disabled={loading}>
                  <RefreshCcwIcon className={loading ? 'size-3.5 animate-spin' : 'size-3.5'} />
                  Yenile
                </Button>
              </div>

              {!isTauriRuntime() && (
                <p className="pt-2 text-xs text-muted-foreground">
                  Tauri runtime yok — motor listesi yalnızca masaüstünde görünür.
                </p>
              )}
              {error && <p className="pt-2 text-xs text-destructive">{error}</p>}

              <div className="flex flex-col gap-1.5 pt-2">
                {engines.map((engine) => (
                  <EngineCard key={engine.id} id={engine.id} onInstall={setInstallTarget} />
                ))}
              </div>
            </TabsContent>

            <TabsContent value="system" className="pt-2 text-xs text-muted-foreground">
              <ul className="flex flex-col gap-1.5">
                <li>· Veritabanı: SQLite + WAL (migration v2)</li>
                <li>· PTY: portable-pty 0.9 (ConPTY / POSIX)</li>
                <li>· İzolasyon: Windows Job Objects / Unix process group + worktree</li>
                <li>· Worktree: .git/agenthub-worktrees (otomatik + .env.local port offset)</li>
                <li>· Oturum kaydı: ~/.agentcompany/logs (JSONL)</li>
              </ul>
            </TabsContent>
          </Tabs>
        </DialogContent>
      </Dialog>

      {/* Kurulum onayı */}
      <AlertDialog open={installTarget !== null} onOpenChange={(o) => !o && setInstallTarget(null)}>
        <AlertDialogContent className="sm:max-w-md">
          <AlertDialogHeader>
            <AlertDialogTitle className="flex items-center gap-2">
              <RocketIcon className="size-4 text-primary" />
              Motor kurulsun mu?
            </AlertDialogTitle>
            <AlertDialogDescription>
              Kurulum komutu backend'de çözülür ve ayrı bir terminal sekmesinde akar. Çıktıyı
              izleyip bitince Ayarlar'dan "Yenile" ile durumu güncelle.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Vazgeç</AlertDialogCancel>
            <AlertDialogAction onClick={confirmInstall}>Kur</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  )
}
