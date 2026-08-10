import { create } from 'zustand'

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
  setActive: (agentId: string | null) => void
  startSession: (agentId: string, engineType: string) => void
  markRunning: (agentId: string, executionId: string) => void
  markExited: (agentId: string) => void
  markError: (agentId: string, error: string) => void
  stopSession: (agentId: string) => void
  bumpOutput: (agentId: string, bytes: number) => void
  getSession: (agentId: string) => SessionState | undefined
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

  setActive: (agentId) => set({ activeSessionAgentId: agentId }),

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
