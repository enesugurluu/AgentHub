import { create } from 'zustand'

import { type AgentRecord, agentHire, type HirePayload, listAgents } from '@/lib/ipc'

type AgentState = {
  agents: AgentRecord[]
  selectedAgentId: number | null
  loading: boolean
  error: string | null
  fetchAgents: () => Promise<void>
  /** İşe alım (Hire Wizard — WP-07): backend kaydı + liste yenileme + seçim. */
  hireAgent: (payload: HirePayload) => Promise<AgentRecord | null>
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

  selectAgent: (id) => set({ selectedAgentId: id }),
}))
