import { useEffect, useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { Terminal } from 'xterm'
import { FitAddon } from 'xterm-addon-fit'
import 'xterm/css/xterm.css'

type PtyOutputEvent = {
  id: string
  data: string
}

function isTauriRuntime() {
  return typeof window !== 'undefined' && typeof (window as any).__TAURI_INTERNALS__ !== 'undefined'
}

export function PtyTerminal() {
  const containerRef = useRef<HTMLDivElement | null>(null)
  const terminalRef = useRef<Terminal | null>(null)
  const fitAddonRef = useRef<FitAddon | null>(null)
  const [sessionId, setSessionId] = useState<string | null>(null)
  const [starting, setStarting] = useState(false)
  const [tauriAvailable] = useState(isTauriRuntime())

  useEffect(() => {
    const terminal = new Terminal({
      cursorBlink: true,
      convertEol: true,
      fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
      fontSize: 13,
      scrollback: 5000,
    })
    const fitAddon = new FitAddon()

    terminal.loadAddon(fitAddon)
    terminalRef.current = terminal
    fitAddonRef.current = fitAddon

    if (containerRef.current) {
      terminal.open(containerRef.current)
      fitAddon.fit()
    }

    const onResize = () => fitAddonRef.current?.fit()
    window.addEventListener('resize', onResize)

    return () => {
      window.removeEventListener('resize', onResize)
      terminal.dispose()
    }
  }, [])

  useEffect(() => {
    if (!sessionId) return

    const terminal = terminalRef.current
    if (!terminal) return

    const disposable = terminal.onData((data) => {
      if (!tauriAvailable) {
        terminal.write(data)
        return
      }

      void invoke('pty_write', { id: sessionId, data })
    })

    return () => disposable.dispose()
  }, [sessionId, tauriAvailable])

  useEffect(() => {
    if (!tauriAvailable) return
    let unlisten: (() => void) | undefined

    void listen<PtyOutputEvent>('pty://output', (event) => {
      if (event.payload.id !== sessionId) return
      terminalRef.current?.write(event.payload.data)
    }).then((fn) => {
      unlisten = fn
    })

    return () => {
      unlisten?.()
    }
  }, [sessionId, tauriAvailable])

  const startShell = async () => {
    if (starting || sessionId) return
    setStarting(true)

    try {
      if (!tauriAvailable) {
        terminalRef.current?.reset()
        terminalRef.current?.writeln('Tauri runtime not detected. Running in local echo mode.')
        setSessionId('local-echo')
        return
      }

      const terminal = terminalRef.current
      const cols = terminal?.cols ?? 80
      const rows = terminal?.rows ?? 24

      const id = await invoke<string>('pty_spawn', {
        program: 'powershell.exe',
        args: ['-NoLogo'],
        cols,
        rows,
      })

      terminal?.reset()
      setSessionId(id)
    } finally {
      setStarting(false)
    }
  }

  const stopShell = async () => {
    if (!sessionId) return
    const id = sessionId
    setSessionId(null)
    if (!tauriAvailable) return
    await invoke('pty_stop', { id })
  }

  return (
    <div style={{ display: 'grid', gap: 12 }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
        <button type="button" onClick={startShell} disabled={starting || !!sessionId}>
          Start Shell
        </button>
        <button type="button" onClick={stopShell} disabled={!sessionId}>
          Stop
        </button>
        <div style={{ fontFamily: 'monospace', fontSize: 12, opacity: 0.8 }}>
          {sessionId ?? 'no session'}
        </div>
      </div>
      <div
        ref={containerRef}
        style={{
          height: 460,
          width: '100%',
          border: '1px solid rgba(255, 255, 255, 0.15)',
          borderRadius: 8,
          overflow: 'hidden',
        }}
      />
    </div>
  )
}
