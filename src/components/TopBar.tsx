import { MoonIcon, SettingsIcon, SunIcon, WalletIcon } from 'lucide-react'
import { useEffect } from 'react'
import { SettingsDialog } from '@/components/SettingsDialog'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import { useSettingsStore } from '@/store/settings'

export function TopBar() {
  const theme = useSettingsStore((s) => s.theme)
  const toggleTheme = useSettingsStore((s) => s.toggleTheme)

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
        <span className="rounded px-2 py-0.5 text-xs font-medium ring-1 ring-border">
          Proje: repo kökü
        </span>
      </div>

      <div className="ml-auto flex items-center gap-2">
        <div className="flex items-center gap-1.5 rounded-md border border-border px-2.5 py-1 text-xs">
          <WalletIcon className="size-3.5 text-muted-foreground" />
          <span className="tabular-nums text-muted-foreground">$0.00</span>
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
