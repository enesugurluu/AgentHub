import { create } from 'zustand'

import type { PtyEvent } from '@/lib/ipc'

export type SessionStatus = 'idle' | 'starting' | 'running' | 'exited' | 'error'

export type SessionState = {
  agentId: string | null
  executionId: string | null
  status: SessionStatus
  engineType: string
  /** terminal kayıtlarında gösterilecek toplam karakter sayısı */
  outputBytes: number
  error: string | null
}

type TerminalStore = {
  sessions: Record<string, SessionState>
  activeSessionAgentId: string | null
  /** Serialize edilmiş terminal buffer'ları (sekme geri açılışta geri yükleme — WP-11). */
  buffers: Record<string, string>
  /** Ajan başına PTY kanalı (PtyTerminal mount'ta kaydeder) — task_assign bunu kullanır (WP-10). */
  channels: Record<string, import('@tauri-apps/api/core').Channel<PtyEvent>>
  registerChannel: (
    agentId: string,
    channel: import('@tauri-apps/api/core').Channel<PtyEvent>,
  ) => void
  setActive: (agentId: string | null) => void
  startSession: (agentId: string, engineType: string) => void
  markRunning: (agentId: string, executionId: string) => void
  markExited: (agentId: string) => void
  markError: (agentId: string, error: string) => void
  stopSession: (agentId: string) => void
  bumpOutput: (agentId: string, bytes: number) => void
  getSession: (agentId: string) => SessionState | undefined
  setBuffer: (agentId: string, text: string) => void
}

const initialSession = (agentId: string, engineType: string): SessionState => ({
  agentId,
  executionId: null,
  status: 'starting',
  engineType,
  outputBytes: 0,
  error: null,
})

export const useTerminalStore = create<TerminalStore>((set, get) => ({
  sessions: {},
  activeSessionAgentId: null,
  buffers: {},
  channels: {},

  registerChannel: (agentId, channel) =>
    set((state) => ({ channels: { ...state.channels, [agentId]: channel } })),

  setActive: (agentId) => set({ activeSessionAgentId: agentId }),

  setBuffer: (agentId, text) =>
    set((state) => ({ buffers: { ...state.buffers, [agentId]: text } })),

  startSession: (agentId, engineType) =>
    set((state) => ({
      sessions: {
        ...state.sessions,
        [agentId]: initialSession(agentId, engineType),
      },
      activeSessionAgentId: agentId,
    })),

  markRunning: (agentId, executionId) =>
    set((state) => {
      const session = state.sessions[agentId]
      if (!session) return state
      return {
        sessions: {
          ...state.sessions,
          [agentId]: { ...session, executionId, status: 'running' },
        },
      }
    }),

  markExited: (agentId) =>
    set((state) => {
      const session = state.sessions[agentId]
      if (!session) return state
      return {
        sessions: {
          ...state.sessions,
          [agentId]: { ...session, status: 'exited' },
        },
      }
    }),

  markError: (agentId, error) =>
    set((state) => {
      const session = state.sessions[agentId]
      if (!session) return state
      return {
        sessions: {
          ...state.sessions,
          [agentId]: { ...session, status: 'error', error },
        },
      }
    }),

  stopSession: (agentId) =>
    set((state) => {
      const next = { ...state.sessions }
      delete next[agentId]
      return { sessions: next }
    }),

  bumpOutput: (agentId, bytes) =>
    set((state) => {
      const session = state.sessions[agentId]
      if (!session) return state
      return {
        sessions: {
          ...state.sessions,
          [agentId]: { ...session, outputBytes: session.outputBytes + bytes },
        },
      }
    }),

  getSession: (agentId) => get().sessions[agentId],
}))
