import { MaximizeIcon, MinusIcon, PlusIcon, ZoomInIcon } from 'lucide-react'
import { useState } from 'react'
import { TransformComponent, TransformWrapper, useControls } from 'react-zoom-pan-pinch'

import { AgentDesk } from '@/components/OfficeFloor/AgentDesk'
import { Button } from '@/components/ui/button'
import { selectVisibleAgents, useAgentStore } from '@/store/agents'
import { useTerminalStore } from '@/store/terminal'

/** Çalışan masası konumu (docs 5.6 — deterministik grid; M2'de kalıcı konumlar). */
function deskPosition(index: number): { left: string; top: string } {
  const col = index % 3
  const row = Math.floor(index / 3)
  return {
    left: `${14 + col * 27}%`,
    top: `${22 + row * 30}%`,
  }
}

function ZoomControls() {
  const { zoomIn, zoomOut, resetTransform } = useControls()
  return (
    <div className="absolute right-3 bottom-3 z-20 flex flex-col gap-1 rounded-md border border-border bg-background/90 p-1 shadow-sm backdrop-blur">
      <Button
        variant="ghost"
        size="icon"
        className="size-7"
        title="Yakınlaştır"
        onClick={() => zoomIn(0.25)}
      >
        <PlusIcon className="size-3.5" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        className="size-7"
        title="Uzaklaştır"
        onClick={() => zoomOut(0.25)}
      >
        <MinusIcon className="size-3.5" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        className="size-7"
        title="Sıfırla"
        onClick={() => resetTransform()}
      >
        <MaximizeIcon className="size-3.5" />
      </Button>
    </div>
  )
}

/**
 * Ofis katı v1 (docs Bölüm 5; WP-09): SVG zemin (ızgara + dekor) üzerinde
 * HTML masa kartları; zoom/pan (react-zoom-pan-pinch); tıklama → Inspector +
 * terminal sekmesi. `fired` ajanlar çizilmez; boş ofiste "İşe Al" CTA'sı.
 */
export function OfficeFloor() {
  const { agents, selectedAgentId, selectAgent } = useAgentStore()
  const setActive = useTerminalStore((s) => s.setActive)
  const [hireCta, setHireCta] = useState(false)

  const visibleAgents = selectVisibleAgents(agents)

  const openDesk = (id: number) => {
    selectAgent(id)
    setActive(String(id))
  }

  return (
    <section className="relative flex h-full min-h-0 flex-col overflow-hidden">
      {/* Zemin deseni (SVG) */}
      <svg className="pointer-events-none absolute inset-0 h-full w-full" aria-hidden="true">
        <defs>
          <pattern id="office-grid" width="28" height="28" patternUnits="userSpaceOnUse">
            <path
              d="M 28 0 L 0 0 0 28"
              fill="none"
              stroke="currentColor"
              strokeOpacity="0.05"
              strokeWidth="1"
            />
          </pattern>
        </defs>
        <rect width="100%" height="100%" fill="url(#office-grid)" />
        {/* Dekor: kahve köşesi + bitki + raf */}
        <g fill="none" stroke="currentColor" strokeOpacity="0.08">
          <rect x="16" y="16" width="90" height="14" rx="4" />
          <rect x="16" y="30" width="90" height="90" rx="4" />
          <circle cx="53" cy="74" r="10" />
          <path d="M53 64 L53 84 M47 70 L59 78 M47 78 L59 70" />
          <rect x="12" y="52" width="120" height="6" rx="2" />
        </g>
      </svg>

      <TransformWrapper
        initialScale={1}
        minScale={0.5}
        maxScale={2.5}
        centerOnInit
        doubleClick={{ disabled: true }}
        wheel={{ step: 0.15 }}
      >
        <TransformComponent wrapperClass="h-full w-full" contentClass="h-full w-full">
          <div className="relative h-full w-full">
            {/* CEO masası — merkez */}
            <div className="absolute top-[46%] left-1/2 -translate-x-1/2 -translate-y-1/2">
              <AgentDesk
                agent={{
                  id: 0,
                  name: 'Sen',
                  role: 'CEO',
                  motor: 'claude',
                  model: null,
                  status: 'idle',
                  worktreePath: null,
                  createdAt: null,
                  avatarColor: null,
                  configJson: null,
                  hiredAt: null,
                  firedAt: null,
                }}
                isPlayer
                selected={false}
                onSelect={() => {
                  selectAgent(null)
                  setActive(null)
                }}
              />
            </div>

            {/* Çalışan masaları */}
            {visibleAgents.map((agent, i) => {
              const pos = deskPosition(i)
              return (
                <div key={agent.id} className="absolute" style={pos}>
                  <AgentDesk
                    agent={agent}
                    selected={selectedAgentId === agent.id}
                    onSelect={openDesk}
                  />
                </div>
              )
            })}

            {/* Boş ofis CTA */}
            {visibleAgents.length === 0 && (
              <div className="absolute top-[10%] left-1/2 -translate-x-1/2 rounded-lg border border-dashed border-border bg-background/80 px-8 py-6 text-center text-sm text-muted-foreground backdrop-blur">
                <p>Masalar boş.</p>
                <button
                  type="button"
                  className="mt-1 text-primary underline-offset-2 hover:underline"
                  onClick={() => setHireCta(true)}
                >
                  İşe alım sihirbazı
                </button>
                {hireCta && (
                  <p className="mt-2 text-xs">
                    Sol paneldeki <span className="font-semibold">+</span> butonu ile ilk ajanı işe
                    alabilirsin.
                  </p>
                )}
              </div>
            )}
          </div>
        </TransformComponent>
      </TransformWrapper>

      <ZoomControls />

      {/* Erişilebilirlik notu (docs 5.8): klavye ile seçim AgentDesk'te */}
      <div className="pointer-events-none absolute bottom-2 left-3 z-20 hidden text-[10px] text-muted-foreground/60 md:block">
        <ZoomInIcon className="inline size-3" /> Tekerlek: zoom · sürükle: pan · masa: seç
      </div>
    </section>
  )
}
