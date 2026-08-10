import { SearchIcon, SquareIcon, TriangleAlertIcon } from 'lucide-react'
import { useEffect, useRef, useState } from 'react'
import { Terminal } from 'xterm'
import { FitAddon } from 'xterm-addon-fit'
import { SearchAddon } from 'xterm-addon-search'
import { SerializeAddon } from 'xterm-addon-serialize'
import { WebLinksAddon } from 'xterm-addon-web-links'
import { WebglAddon } from 'xterm-addon-webgl'
import 'xterm/css/xterm.css'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  agentSpawn,
  agentSpawnEngine,
  agentStop,
  agentWrite,
  createPtyChannel,
  isTauriRuntime,
  type PtyEvent,
  ptyResize,
  transcriptAppendSessionBuffer,
} from '@/lib/ipc'
import { useTerminalStore } from '@/store/terminal'

export type EngineChoice = 'pty' | 'claude'

const ENGINE_LABELS: Record<EngineChoice, string> = {
  pty: 'Shell (PTY)',
  claude: 'Claude Code',
}

function defaultShellProgram(): { program: string; args: string[] } {
  if (navigator.userAgent.includes('Windows')) {
    return { program: 'powershell.exe', args: ['-NoLogo'] }
  }
  return { program: 'bash', args: ['-l'] }
}

export function PtyTerminal({
  agentId,
  engine: initialEngine = 'pty',
  isActive = true,
}: {
  agentId: string
  engine?: EngineChoice
  isActive?: boolean
}) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  const terminalRef = useRef<Terminal | null>(null)
  const fitAddonRef = useRef<FitAddon | null>(null)
  const searchAddonRef = useRef<SearchAddon | null>(null)
  const channelRef = useRef<ReturnType<typeof createPtyChannel> | null>(null)
  const engineRef = useRef<EngineChoice>(initialEngine)

  const [engine, setEngine] = useState<EngineChoice>(initialEngine)
  const [starting, setStarting] = useState(false)
  const [searchOpen, setSearchOpen] = useState(false)
  const [searchText, setSearchText] = useState('')
  const [tauriAvailable] = useState(isTauriRuntime())
  const [startError, setStartError] = useState<string | null>(null)

  const session = useTerminalStore((s) => s.sessions[agentId])
  const setActive = useTerminalStore((s) => s.setActive)
  const startSession = useTerminalStore((s) => s.startSession)
  const markRunning = useTerminalStore((s) => s.markRunning)
  const markError = useTerminalStore((s) => s.markError)
  const stopSession = useTerminalStore((s) => s.stopSession)

  engineRef.current = engine

  // ---- Terminal kurulumu (bir kez) -----------------------------------------
  useEffect(() => {
    const terminal = new Terminal({
      cursorBlink: true,
      convertEol: true,
      fontFamily:
        'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "JetBrains Mono", monospace',
      fontSize: 13,
      scrollback: 10000,
      allowProposedApi: true,
    })

    const fitAddon = new FitAddon()
    terminal.loadAddon(fitAddon)
    fitAddonRef.current = fitAddon

    // WebGL renderer: 10K+ satır logda akıcı kalır. GPU yoksa canvas'a düşer.
    try {
      const webglAddon = new WebglAddon()
      webglAddon.onContextLoss(() => {
        // Context kaybında eski renderer'a dön
        webglAddon.dispose()
      })
      terminal.loadAddon(webglAddon)
    } catch {
      // WebGL desteklenmiyorsa varsayılan canvas renderer kullanılır.
    }

    terminal.loadAddon(new WebLinksAddon())
    const searchAddon = new SearchAddon()
    terminal.loadAddon(searchAddon)
    searchAddonRef.current = searchAddon
    // JSONL session_buffer kaydı için (docs 12.2; WP-11 — FAZ0 serialize ertelemesi).
    const serializeAddon = new SerializeAddon()
    terminal.loadAddon(serializeAddon)

    terminalRef.current = terminal

    if (containerRef.current) {
      terminal.open(containerRef.current)
      requestAnimationFrame(() => fitAddon.fit())
    }

    // WP-11: önceki oturumun serialize edilmiş buffer'ı varsa geri yükle
    // (yalnızca bu sekmede henüz canlı çıktı yokken — çift yazma önlenir).
    const savedBuffer = useTerminalStore.getState().buffers[agentId]
    const sessionState = useTerminalStore.getState().sessions[agentId]
    if (savedBuffer && (!sessionState || sessionState.outputBytes === 0)) {
      try {
        terminal.write(savedBuffer)
      } catch {
        // geri yükleme başarısızsa yoksay
      }
    }

    // PTY kanalı: backend her olayı sadece bu oturuma gönderir.
    const channel = createPtyChannel((event: PtyEvent) => {
      const sessionState = useTerminalStore.getState().sessions[event.agentId]
      if (!sessionState) return
      if (sessionState.executionId && event.executionId !== sessionState.executionId) return

      if (event.kind.type === 'output') {
        const bytes = new Uint8Array(event.kind.data)
        terminal.write(bytes)
        useTerminalStore.getState().bumpOutput(event.agentId, bytes.length)
      } else if (event.kind.type === 'exit') {
        useTerminalStore.getState().markExited(event.agentId)
        // JSONL session_buffer kaydı + in-memory geri yükleme (docs 12.2; WP-11).
        try {
          const buffer = serializeAddon.serialize()
          useTerminalStore.getState().setBuffer(event.agentId, buffer)
          void transcriptAppendSessionBuffer({
            agentId: event.agentId,
            executionId: event.executionId,
            text: buffer,
          }).catch(() => {
            // transcript yoksa sessiz geç
          })
        } catch {
          // serialize desteklenmiyorsa sessiz geç
        }
        terminal.writeln('')
        terminal.writeln(`\x1b[90m[agent ${event.agentId} exited] (code ${event.kind.code})\x1b[0m`)
      }
    })
    channelRef.current = channel

    // Klavye: Ctrl/Cmd+Shift+F arama aç/kapa
    const onKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === 'f') {
        e.preventDefault()
        setSearchOpen((open) => !open)
      }
    }
    window.addEventListener('keydown', onKeyDown)

    const onResize = () => {
      try {
        fitAddon.fit()
      } catch {
        // container görünür değilse yok say
      }
    }
    window.addEventListener('resize', onResize)

    // Container boyut değişimini izle ve PTY boyutunu backend'e bildir.
    let resizeObserver: ResizeObserver | null = null
    if (typeof ResizeObserver !== 'undefined' && containerRef.current) {
      resizeObserver = new ResizeObserver(() => {
        const el = containerRef.current
        const term = terminalRef.current
        const fit = fitAddonRef.current
        if (!el || !term || !fit) return
        // Gizli sekme (display:none): fit() ölçülebilir boyut bulamaz, atla.
        if (el.getBoundingClientRect().width === 0) return
        try {
          fit.fit()
        } catch {
          return
        }
        const sessionState = useTerminalStore.getState().sessions[agentId]
        if (sessionState?.executionId && sessionState.status === 'running') {
          void ptyResize({
            agentId,
            executionId: sessionState.executionId,
            cols: term.cols,
            rows: term.rows,
          }).catch((e) => console.error('resize failed:', e))
        }
      })
      resizeObserver.observe(containerRef.current)
    }

    return () => {
      window.removeEventListener('keydown', onKeyDown)
      window.removeEventListener('resize', onResize)
      resizeObserver?.disconnect()
      terminal.dispose()
      terminalRef.current = null
      fitAddonRef.current = null
      searchAddonRef.current = null
    }
  }, [agentId])

  // ---- Sekme tekrar görünür olunca: boyut senkronu + scroll -----------------
  useEffect(() => {
    if (!isActive) return
    const raf = requestAnimationFrame(() => {
      const term = terminalRef.current
      const fit = fitAddonRef.current
      if (!term || !fit) return
      try {
        fit.fit()
      } catch {
        // container henüz ölçülebilir değil
      }
      term.scrollToBottom()
      // Gizliyken değişmiş olabilecek boyutu backend PTY'sine yeniden bildir.
      const sessionState = useTerminalStore.getState().sessions[agentId]
      if (sessionState?.executionId && sessionState.status === 'running') {
        void ptyResize({
          agentId,
          executionId: sessionState.executionId,
          cols: term.cols,
          rows: term.rows,
        }).catch((e) => console.error('resize failed:', e))
      }
    })
    return () => cancelAnimationFrame(raf)
  }, [isActive, agentId])

  // ---- stdin köprüsü --------------------------------------------------------
  useEffect(() => {
    const terminal = terminalRef.current
    if (!terminal) return

    const disposable = terminal.onData((data) => {
      if (!tauriAvailable) {
        terminal.write(data)
        return
      }
      const sessionState = useTerminalStore.getState().sessions[agentId]
      if (!sessionState?.executionId || sessionState.status !== 'running') return
      void agentWrite({
        agentId,
        executionId: sessionState.executionId,
        data,
      }).catch((e) => console.error('write failed:', e))
    })

    return () => disposable.dispose()
  }, [agentId, tauriAvailable])

  // ---- Spawn / Stop ----------------------------------------------------------
  const startShell = async () => {
    if (starting || session?.status === 'running') return
    setStarting(true)
    setStartError(null)
    setActive(agentId)

    try {
      const terminal = terminalRef.current
      if (!terminal) return
      terminal.reset()
      terminal.writeln('')

      if (!tauriAvailable) {
        terminal.writeln('Tauri runtime not detected. Running in local echo mode.')
        startSession(agentId, engineRef.current)
        markRunning(agentId, 'local-echo')
        return
      }

      const cols = terminal.cols
      const rows = terminal.rows
      startSession(agentId, engineRef.current)
      setActive(agentId)

      const channel = channelRef.current
      if (!channel) throw new Error('pty channel not initialized')

      const result =
        engineRef.current === 'claude'
          ? await agentSpawnEngine({
              agentId,
              engineType: 'claude',
              cols,
              rows,
              channel,
            })
          : await agentSpawn({
              agentId,
              ...defaultShellProgram(),
              cols,
              rows,
              channel,
            })

      markRunning(result.agentId, result.executionId)
    } catch (e) {
      console.error('agent spawn failed:', e)
      setStartError(String(e))
      markError(agentId, String(e))
      const terminal = terminalRef.current
      terminal?.writeln(`\x1b[31m[spawn error] ${String(e)}\x1b[0m`)
    } finally {
      setStarting(false)
    }
  }

  const stopShell = async () => {
    if (!session?.executionId) return
    const aId = agentId
    const eId = session.executionId
    stopSession(aId)
    if (!tauriAvailable) return
    try {
      await agentStop({ agentId: aId, executionId: eId })
    } catch (e) {
      console.error('Failed to stop agent:', e)
    }
  }

  const running = session?.status === 'running' || session?.status === 'starting'

  const runSearch = () => {
    if (!searchAddonRef.current || !searchText) return
    searchAddonRef.current.findNext(searchText, { incremental: true })
  }

  return (
    <div className="flex h-full min-h-0 flex-col gap-2">
      <div className="flex items-center gap-2 px-1">
        <div className="flex items-center gap-1 rounded-md bg-muted p-0.5">
          {(Object.keys(ENGINE_LABELS) as EngineChoice[]).map((choice) => (
            <button
              key={choice}
              type="button"
              onClick={() => setEngine(choice)}
              className={`rounded px-2.5 py-1 text-xs font-medium transition-colors ${
                engine === choice
                  ? 'bg-background text-foreground shadow-sm'
                  : 'text-muted-foreground hover:text-foreground'
              }`}
              disabled={running}
            >
              {ENGINE_LABELS[choice]}
            </button>
          ))}
        </div>

        <Button size="sm" onClick={startShell} disabled={starting || running}>
          {starting ? 'Starting…' : running ? 'Running' : 'Start'}
        </Button>
        <Button size="sm" variant="outline" onClick={stopShell} disabled={!running}>
          <SquareIcon className="size-3" />
          Stop
        </Button>

        <Button
          size="icon"
          variant="ghost"
          className="size-7"
          onClick={() => setSearchOpen((open) => !open)}
          title="Search (Ctrl+Shift+F)"
        >
          <SearchIcon className="size-3.5" />
        </Button>

        <div className="ml-auto flex items-center gap-2 font-mono text-xs text-muted-foreground">
          <Badge variant={running ? 'success' : 'secondary'} className="uppercase">
            {session?.status ?? 'idle'}
          </Badge>
          <span className="tabular-nums">
            {session ? `${(session.outputBytes / 1024).toFixed(1)} KB` : ''}
          </span>
        </div>
      </div>

      {searchOpen && (
        <div className="flex items-center gap-2 px-1">
          <input
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter') runSearch()
            }}
            placeholder="Search buffer…"
            className="h-7 w-56 rounded border border-input bg-transparent px-2 text-xs outline-none focus:border-ring"
          />
          <Button size="sm" variant="ghost" onClick={runSearch}>
            Find
          </Button>
        </div>
      )}

      {startError && (
        <div className="flex items-center gap-2 px-1 text-xs text-destructive">
          <TriangleAlertIcon className="size-3.5" />
          {startError}
        </div>
      )}

      <div
        ref={containerRef}
        className="min-h-0 flex-1 overflow-hidden rounded-md border border-border bg-black/60"
      />
    </div>
  )
}
