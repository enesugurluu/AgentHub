import { create } from 'zustand'

export type Theme = 'dark' | 'light'

type SettingsState = {
  theme: Theme
  toggleTheme: () => void
}

export const useSettingsStore = create<SettingsState>((set) => ({
  theme: 'dark',
  toggleTheme: () => set((state) => ({ theme: state.theme === 'dark' ? 'light' : 'dark' })),
}))
