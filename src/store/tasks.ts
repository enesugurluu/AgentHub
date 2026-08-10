import { create } from 'zustand'

import { type PtyEvent, type TaskRecord, taskAssign, taskCreate, taskList } from '@/lib/ipc'

type TasksState = {
  tasks: TaskRecord[]
  loading: boolean
  error: string | null
  fetchTasks: () => Promise<void>
  /** Görev oluştur + ajana ata + spawn et (docs 13.1; WP-10). */
  createAndAssign: (args: {
    agentId: number
    title: string
    description?: string
    acceptanceCriteria?: string
    priority?: number
    budget?: number | null
    cols: number
    rows: number
    channel: import('@tauri-apps/api/core').Channel<PtyEvent>
  }) => Promise<boolean>
}

export const useTasksStore = create<TasksState>((set) => ({
  tasks: [],
  loading: false,
  error: null,

  fetchTasks: async () => {
    set({ loading: true, error: null })
    try {
      const tasks = await taskList(null)
      set({ tasks, loading: false })
    } catch (e) {
      set({ loading: false, error: String(e) })
    }
  },

  createAndAssign: async ({
    agentId,
    title,
    description,
    acceptanceCriteria,
    priority,
    budget,
    cols,
    rows,
    channel,
  }) => {
    try {
      const task = await taskCreate({
        title,
        description: description || null,
        acceptanceCriteria: acceptanceCriteria || null,
        priority: priority ?? 3,
        budget: budget ?? null,
      })
      await taskAssign({ agentId, taskId: task.id, cols, rows, channel })
      const tasks = await taskList(null)
      set({ tasks, error: null })
      return true
    } catch (e) {
      set({ error: String(e) })
      return false
    }
  },
}))
