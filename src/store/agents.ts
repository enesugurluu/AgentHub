import { create } from 'zustand'

import { type AgentRecord, listAgents } from '@/lib/ipc'

type AgentState = {
  agents: AgentRecord[]
  selectedAgentId: number | null
  loading: boolean
  error: string | null
  fetchAgents: () => Promise<void>
  selectAgent: (id: number | null) => void
  /** DB'de kayıt yoksa "default" iskelet ajana geri düşer. */
  firstAgentId: () => string
}

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

  selectAgent: (id) => set({ selectedAgentId: id }),

  firstAgentId: () => {
    const { agents, selectedAgentId } = get()
    if (selectedAgentId !== null && agents.some((a) => a.id === selectedAgentId)) {
      return String(selectedAgentId)
    }
    if (agents.length > 0) return String(agents[0].id)
    return 'default-agent-id'
  },
}))
