import { open } from '@tauri-apps/plugin-dialog'
import { FolderOpenIcon } from 'lucide-react'
import { useEffect, useState } from 'react'

import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { isTauriRuntime } from '@/lib/ipc'
import { useProjectsStore } from '@/store/projects'

/**
 * İlk açılış onboarding'i (WP-06/ADR-7): repo seçilmemişse mini-dialog gösterir.
 * "Proje Seç" → dialog klasör seçici → `repo_select` (backend doğrular + kalıcı).
 * "Şimdi Atla" → `onboarding_skipped=1` (sonraki açılışta sorulmaz).
 */
export function OnboardingDialog() {
  const repoPath = useProjectsStore((s) => s.repoPath)
  const onboardingSkipped = useProjectsStore((s) => s.onboardingSkipped)
  const loading = useProjectsStore((s) => s.loading)
  const selectRepo = useProjectsStore((s) => s.selectRepo)
  const skipOnboarding = useProjectsStore((s) => s.skipOnboarding)
  const [openState, setOpenState] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (isTauriRuntime() && !loading && repoPath === null && !onboardingSkipped) {
      setOpenState(true)
    } else {
      setOpenState(false)
    }
  }, [repoPath, onboardingSkipped, loading])

  const handlePick = async () => {
    if (!isTauriRuntime()) return
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Proje (git repo) seç',
    })
    if (typeof selected === 'string') {
      const result = await selectRepo(selected)
      if (result === null) {
        setError('Seçilen yol geçerli bir git deposu değil — tekrar deneyin.')
        return
      }
      setOpenState(false)
    }
  }

  const handleSkip = async () => {
    await skipOnboarding()
    setOpenState(false)
  }

  return (
    <Dialog open={openState} onOpenChange={setOpenState}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <FolderOpenIcon className="size-4 text-primary" />
            Hoş geldin — projeyi seç
          </DialogTitle>
          <DialogDescription>
            Ajanlar, izole git worktree'lerinde çalışır. Çalıştıkları ana repoyu seç: her ajan kendi
            branch + worktree'sini buradan türetir.
          </DialogDescription>
        </DialogHeader>

        {error && <p className="text-xs text-destructive">{error}</p>}

        <DialogFooter>
          <Button variant="ghost" onClick={handleSkip}>
            Şimdi Atla
          </Button>
          <Button onClick={handlePick}>
            <FolderOpenIcon className="size-4" />
            Proje Seç
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
