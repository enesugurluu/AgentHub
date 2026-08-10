import { create } from 'zustand'

import { isTauriRuntime, repoSelect, settingsGet, settingsSet } from '@/lib/ipc'

type ProjectsState = {
  /** Doğrulanmış repo yolu (`settings.repo_path`); null = henüz seçilmedi. */
  repoPath: string | null
  /** Onboarding "Şimdi Atla" işareti (`settings.onboarding_skipped`). */
  onboardingSkipped: boolean
  loading: boolean
  error: string | null
  /** Açılışta settings'ten repo_path + onboarding bayrağını yükler (WP-06). */
  loadRepoPath: () => Promise<void>
  /** Dialog'dan gelen yolu doğrular + settings'e yazar (backend `repo_select`). */
  selectRepo: (path: string) => Promise<string | null>
  /** Onboarding "Şimdi Atla" — bir daha sormamak için settings'e işaret koyar. */
  skipOnboarding: () => Promise<void>
}

export const useProjectsStore = create<ProjectsState>((set) => ({
  repoPath: null,
  onboardingSkipped: false,
  loading: false,
  error: null,

  loadRepoPath: async () => {
    if (!isTauriRuntime()) return
    set({ loading: true, error: null })
    try {
      const [repoPath, onboardingSkipped] = await Promise.all([
        settingsGet('repo_path'),
        settingsGet('onboarding_skipped'),
      ])
      set({
        repoPath,
        onboardingSkipped: onboardingSkipped === '1',
        loading: false,
      })
    } catch (e) {
      set({ loading: false, error: String(e) })
    }
  },

  selectRepo: async (path) => {
    try {
      const repoPath = await repoSelect(path)
      set({ repoPath, error: null })
      return repoPath
    } catch (e) {
      set({ error: String(e) })
      return null
    }
  },

  skipOnboarding: async () => {
    try {
      await settingsSet('onboarding_skipped', '1')
    } catch {
      // Kayıt başarısızsa sessiz geç — sonraki açılışta tekrar sorulur.
    }
  },
}))
