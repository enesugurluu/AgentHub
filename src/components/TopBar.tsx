import { open } from '@tauri-apps/plugin-dialog'
import { FolderGit2Icon, MoonIcon, SettingsIcon, SunIcon, WalletIcon } from 'lucide-react'
import { useEffect } from 'react'
import { SettingsDialog } from '@/components/SettingsDialog'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import { isTauriRuntime } from '@/lib/ipc'
import { useProjectsStore } from '@/store/projects'
import { useSettingsStore } from '@/store/settings'
import { useTerminalStore } from '@/store/terminal'

export function TopBar() {
  const theme = useSettingsStore((s) => s.theme)
  const toggleTheme = useSettingsStore((s) => s.toggleTheme)
  const repoPath = useProjectsStore((s) => s.repoPath)
  const loadRepoPath = useProjectsStore((s) => s.loadRepoPath)
  const selectRepo = useProjectsStore((s) => s.selectRepo)
  // WP-13: Progress.cost birikimi — M2'de settings'ten gerçek bütçe.
  const sessions = useTerminalStore((s) => s.sessions)
  const totalCost = Object.values(sessions).reduce((acc, s) => acc + s.totalCostUsd, 0)

  useEffect(() => {
    void loadRepoPath()
  }, [loadRepoPath])

  useEffect(() => {
    const root = document.documentElement
    if (theme === 'dark') {
      root.classList.add('dark')
      root.classList.remove('light')
    } else {
      root.classList.remove('dark')
      root.classList.add('light')
    }
  }, [theme])

  return (
    <header className="flex h-12 shrink-0 items-center gap-3 border-b border-border px-4">
      <div className="flex items-center gap-2">
        <span className="text-base font-semibold tracking-tight">
          agent<span className="text-primary">Hub</span>
        </span>
        <Badge variant="secondary" className="font-mono">
          FAZ0
        </Badge>
      </div>

      <Separator orientation="vertical" className="h-6" />

      <div className="flex items-center gap-1.5 text-sm text-muted-foreground">
        <button
          type="button"
          onClick={async () => {
            if (!isTauriRuntime()) return
            const selected = await open({
              directory: true,
              multiple: false,
              title: 'Proje (git repo) seç',
            })
            if (typeof selected === 'string') {
              await selectRepo(selected)
            }
          }}
          title={repoPath ? `Proje: ${repoPath}` : 'Proje seç (henüz seçilmedi)'}
          className="flex items-center gap-1.5 rounded px-2 py-0.5 text-xs font-medium ring-1 ring-border transition-colors hover:bg-muted/60"
        >
          <FolderGit2Icon className="size-3.5" />
          Proje:{' '}
          <span className="max-w-48 truncate font-mono">
            {repoPath ? (repoPath.length > 36 ? `…${repoPath.slice(-36)}` : repoPath) : 'repo kökü'}
          </span>
        </button>
      </div>

      <div className="ml-auto flex items-center gap-2">
        <div
          className="flex items-center gap-1.5 rounded-md border border-border px-2.5 py-1 text-xs"
          title="Canlı maliyet (yaklaşık — tam dashboard M2'de)"
        >
          <WalletIcon className="size-3.5 text-muted-foreground" />
          <span className="tabular-nums text-muted-foreground">≈ ${totalCost.toFixed(2)}</span>
          <span className="text-muted-foreground/50">/</span>
          <span className="tabular-nums">$50.00</span>
        </div>
        <Button variant="outline" size="icon" title="Tema" onClick={toggleTheme}>
          {theme === 'dark' ? <SunIcon className="size-4" /> : <MoonIcon className="size-4" />}
        </Button>
        <SettingsDialog
          trigger={
            <Button variant="outline" size="icon" title="Ayarlar">
              <SettingsIcon className="size-4" />
            </Button>
          }
        />
      </div>
    </header>
  )
}
