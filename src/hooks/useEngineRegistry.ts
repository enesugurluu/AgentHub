import { useCallback, useEffect, useState } from 'react'

import {
  type DetectResult,
  type EngineMetadata,
  isTauriRuntime,
  ptyAdapterDetectInfo,
  ptyAdapterMetadata,
  ptyListEngineAdapters,
} from '@/lib/ipc'

export type EngineInfo = {
  id: string
  metadata: EngineMetadata | null
  detectInfo: DetectResult | null
  /** `detectInfo.detected` — kurulu mu? */
  detected: boolean
}

/**
 * Adaptör registry'sini yükler: id + metadata + detectInfo (kurulu mu + install_hint).
 * Settings "Motorlar" (WP-12) ve Hire Wizard Adım 2 (WP-07) tek kaynak olarak kullanır.
 */
export function useEngineRegistry() {
  const [engines, setEngines] = useState<EngineInfo[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const refresh = useCallback(async () => {
    if (!isTauriRuntime()) {
      setEngines([])
      setError('Tauri runtime yok — motor listesi yalnızca masaüstünde görünür.')
      return
    }
    setLoading(true)
    setError(null)
    try {
      const ids = await ptyListEngineAdapters('all')
      const infos = await Promise.all(
        ids.map(async (id) => {
          const [metadata, detectInfo] = await Promise.all([
            ptyAdapterMetadata(id).catch(() => null),
            ptyAdapterDetectInfo(id).catch(() => null),
          ])
          return { id, metadata, detectInfo, detected: detectInfo?.detected ?? false }
        }),
      )
      setEngines(infos)
    } catch (e) {
      setError(String(e))
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    void refresh()
  }, [refresh])

  return { engines, loading, error, refresh }
}
