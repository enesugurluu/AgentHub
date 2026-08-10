import { create } from 'zustand'

import {
  type AgentRecord,
  agentFire,
  agentHire,
  type FireOptions,
  type HirePayload,
  listAgents,
} from '@/lib/ipc'

type AgentState = {
  agents: AgentRecord[]
  selectedAgentId: number | null
  loading: boolean
  error: string | null
  fetchAgents: () => Promise<void>
  /** İşe alım (Hire Wizard — WP-07): backend kaydı + liste yenileme + seçim. */
  hireAgent: (payload: HirePayload) => Promise<AgentRecord | null>
  /** İşten çıkarma (FireDialog — WP-08): `fired` + liste yenileme + seçim temizleme. */
  fireAgent: (id: number, options: FireOptions) => Promise<boolean>
  selectAgent: (id: number | null) => void
}

/** `fired` olmayan ajanlar — sidebar/ofis/inspector ortak filtresi (WP-08/09). */
export const selectVisibleAgents = (agents: AgentRecord[]): AgentRecord[] =>
  agents.filter((a) => !a.firedAt)

export const useAgentStore = create<AgentState>((set, get) => ({
  agents: [],
  selectedAgentId: null,
  loading: false,
  error: null,

  fetchAgents: async () => {
    set({ loading: true, error: null })
    try {
      const agents = await listAgents()
      set({ agents, loading: false })
      if (get().selectedAgentId === null && agents.length > 0) {
        set({ selectedAgentId: agents[0].id })
      }
    } catch (e) {
      set({ loading: false, error: String(e) })
    }
  },

  hireAgent: async (payload) => {
    try {
      const record = await agentHire(payload)
      const agents = await listAgents()
      set({ agents, selectedAgentId: record.id, error: null })
      return record
    } catch (e) {
      set({ error: String(e) })
      return null
    }
  },

  fireAgent: async (id, options) => {
    try {
      await agentFire(id, options)
      const agents = await listAgents()
      set({ agents, error: null })
      if (get().selectedAgentId === id) set({ selectedAgentId: null })
      return true
    } catch (e) {
      set({ error: String(e) })
      return false
    }
  },

  selectAgent: (id) => set({ selectedAgentId: id }),
}))
