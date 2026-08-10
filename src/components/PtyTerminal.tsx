import { useEffect, useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { Terminal } from 'xterm'
import { FitAddon } from 'xterm-addon-fit'
import 'xterm/css/xterm.css'

type PtyOutputEvent = {
  agentId: string
  executionId: string
  data: string
}

type PtyStatusEvent = {
  agentId: string
  executionId: string
  status: string
}

type AgentSpawnResult = {
  agentId: string
  executionId: string
}

function isTauriRuntime() {
  return typeof window !== 'undefined' && typeof (window as any).__TAURI_INTERNALS__ !== 'undefined'
}

export function PtyTerminal({ agentId }: { agentId?: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  const terminalRef = useRef<Terminal | null>(null)
  const fitAddonRef = useRef<FitAddon | null>(null)
  const defaultAgentId = agentId || "default-agent-id"
  const [sessionId, setSessionId] = useState<string | null>(null)
  const [executionId, setExecutionId] = useState<string | null>(null)
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

      void invoke('agent_write', { agentId: sessionId, executionId, data })
    })

    return () => disposable.dispose()
  }, [sessionId, executionId, tauriAvailable])

  useEffect(() => {
    if (!tauriAvailable) return
    let unlistenOutput: (() => void) | undefined
    let unlistenStatus: (() => void) | undefined

    void listen<PtyOutputEvent>('agent://output', (event) => {
      if (event.payload.agentId !== sessionId || event.payload.executionId !== executionId) return
      terminalRef.current?.write(event.payload.data)
    }).then((fn) => {
      unlistenOutput = fn
    })

    void listen<PtyStatusEvent>('agent://status', (event) => {
      if (event.payload.agentId !== sessionId || event.payload.executionId !== executionId) return
      if (event.payload.status === "exited") {
        setSessionId(null)
        setExecutionId(null)
      }
    }).then((fn) => {
      unlistenStatus = fn
    })

    return () => {
      unlistenOutput?.()
      unlistenStatus?.()
    }
  }, [sessionId, executionId, tauriAvailable])

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

      const result = await invoke<AgentSpawnResult>('agent_spawn', {
        agentId: defaultAgentId,
        program: 'powershell.exe',
        args: ['-NoLogo'],
        cols,
        rows,
      })

      terminal?.reset()
      setSessionId(result.agentId)
      setExecutionId(result.executionId)
    } catch (e) {
      console.error(e)
    } finally {
      setStarting(false)
    }
  }

  const stopShell = async () => {
    if (!sessionId || !executionId) return
    const aId = sessionId
    const eId = executionId
    setSessionId(null)
    setExecutionId(null)
    if (!tauriAvailable) return
    try {
      await invoke('agent_stop', { agentId: aId, executionId: eId })
    } catch (e) {
      console.error("Failed to stop agent:", e)
    }
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
