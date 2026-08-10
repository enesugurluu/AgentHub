import { BotIcon, ChevronLeftIcon, ChevronRightIcon, UserRoundIcon } from 'lucide-react'
import { useState } from 'react'

import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Progress } from '@/components/ui/progress'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Textarea } from '@/components/ui/textarea'
import { useEngineRegistry } from '@/hooks/useEngineRegistry'
import type { HirePayload } from '@/lib/ipc'
import { AVATAR_COLORS, ROLE_PRESETS, type RolePreset } from '@/lib/presets'
import { cn } from '@/lib/utils'
import { useAgentStore } from '@/store/agents'

const EFFORT_OPTIONS = ['low', 'medium', 'high', 'xhigh', 'max'] as const
const PERMISSION_OPTIONS = ['full', 'standard', 'limited', 'custom'] as const

/**
 * İşe alım sihirbazı (docs 6.1 — 3 adım):
 * 1) Rol seçimi (preset veya özel) → 2) Motor ve yetenekler → 3) Uzmanlık ve kişilik.
 * "İşe Al" → `agent_hire` (DB) → ofiste masa (WP-09) + görev verilebilir (WP-10).
 */
export function HireWizard({
  open,
  onOpenChange,
}: {
  open: boolean
  onOpenChange: (open: boolean) => void
}) {
  const hireAgent = useAgentStore((s) => s.hireAgent)
  const { engines } = useEngineRegistry()

  const [step, setStep] = useState(0)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Adım 1 — rol
  const [preset, setPreset] = useState<RolePreset | null>(null)
  const [customRole, setCustomRole] = useState('')

  // Adım 2 — motor ve yetenekler
  const [motor, setMotor] = useState('')
  const [model, setModel] = useState('')
  const [effort, setEffort] = useState<string>('medium')
  const [budget, setBudget] = useState('')
  const [maxTurns, setMaxTurns] = useState('')
  const [permissions, setPermissions] = useState<string>('standard')

  // Adım 3 — kişilik
  const [name, setName] = useState('')
  const [avatarColor, setAvatarColor] = useState<string>(AVATAR_COLORS[0])
  const [systemPrompt, setSystemPrompt] = useState('')

  const detectedMotorIds = new Set(
    engines.filter((e) => e.detected && e.metadata?.engineType).map((e) => e.metadata!.engineType),
  )
  // Preset'lerdeki motorların kurulu olanları + kurulu diğer motorlar (engine_type bazlı).
  const motorOptions = [
    ...new Set([
      ...engines
        .filter((e) => e.detected && e.metadata?.engineType)
        .map((e) => e.metadata!.engineType),
      ...ROLE_PRESETS.map((p) => p.motor),
    ]),
  ].sort()

  const motorInfo = (engineType: string) =>
    engines.find((e) => e.metadata?.engineType === engineType && e.detected)

  const close = () => {
    onOpenChange(false)
    reset()
  }

  const reset = () => {
    setStep(0)
    setPreset(null)
    setCustomRole('')
    setMotor('')
    setModel('')
    setEffort('medium')
    setBudget('')
    setMaxTurns('')
    setPermissions('standard')
    setName('')
    setAvatarColor(AVATAR_COLORS[0])
    setSystemPrompt('')
    setError(null)
    setSubmitting(false)
  }

  const selectPreset = (p: RolePreset) => {
    setPreset(p)
    setCustomRole('')
    setMotor(p.motor)
    setEffort(p.effort)
    setPermissions(p.permissions)
    setSystemPrompt(p.systemPrompt)
  }

  const stepValid = (): boolean => {
    if (step === 0) return preset !== null || customRole.trim().length > 0
    if (step === 1) {
      if (!motor) return false
      if (!motorInfo(motor)) {
        setError(`'${motor}' motoru kurulu değil — önce Ayarlar → Motorlar'dan kurun.`)
        return false
      }
      if (budget && Number.isNaN(Number(budget))) {
        setError('Bütçe sayısal olmalı (USD).')
        return false
      }
      if (maxTurns && (Number.isNaN(Number(maxTurns)) || Number(maxTurns) <= 0)) {
        setError('Maksimum tur pozitif bir tam sayı olmalı.')
        return false
      }
      return true
    }
    return name.trim().length >= 2
  }

  const next = () => {
    setError(null)
    if (stepValid()) setStep((s) => Math.min(s + 1, 2))
  }

  const submit = async () => {
    setError(null)
    if (!stepValid()) return
    setSubmitting(true)
    const payload: HirePayload = {
      name: name.trim(),
      role: preset?.role ?? customRole.trim(),
      motor,
      model: model.trim() || null,
      effort,
      maxBudgetUsd: budget ? Number(budget) : null,
      maxTurns: maxTurns ? Number(maxTurns) : null,
      permissionsProfile: permissions as HirePayload['permissionsProfile'],
      systemPrompt: systemPrompt.trim() || null,
      avatarColor,
      skills: [],
      mcpServers: [],
    }
    try {
      const record = await hireAgent(payload)
      if (!record) {
        setError('İşe alım başarısız oldu — lütfen tekrar deneyin.')
        return
      }
      close()
    } catch (e) {
      setError(String(e))
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <Dialog
      open={open}
      onOpenChange={(o) => {
        if (!o) close()
        else onOpenChange(o)
      }}
    >
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <BotIcon className="size-4 text-primary" />
            Yeni Ajan İşe Al — Adım {step + 1}/3
          </DialogTitle>
          <DialogDescription>
            {step === 0 && 'Rol seç: hazır şablondan veya sıfırdan özel rol tanımla.'}
            {step === 1 && 'Motor, model ve yetenekleri ayarla (bütçe, tur, izin profili).'}
            {step === 2 && 'İsim, avatar rengi ve sistem promptu ile ajanı kişiselleştir.'}
          </DialogDescription>
        </DialogHeader>

        <Progress value={((step + 1) / 3) * 100} className="h-1.5" />

        {error && <p className="text-xs text-destructive">{error}</p>}

        {/* Adım 1 — Rol seçimi */}
        {step === 0 && (
          <ScrollArea className="max-h-72">
            <div className="grid grid-cols-2 gap-2">
              {ROLE_PRESETS.map((p) => (
                <button
                  key={p.id}
                  type="button"
                  onClick={() => selectPreset(p)}
                  className={cn(
                    'rounded-md border px-3 py-2 text-left transition-colors',
                    preset?.id === p.id
                      ? 'border-primary/60 bg-accent'
                      : 'border-border hover:bg-muted/60',
                  )}
                >
                  <span className="block text-sm font-medium">{p.name}</span>
                  <span className="block text-xs text-muted-foreground">
                    {p.motor} · {p.effort}
                  </span>
                </button>
              ))}
            </div>
            <div className="mt-3 border-t border-border pt-3">
              <Label htmlFor="custom-role">Özel rol</Label>
              <Input
                id="custom-role"
                placeholder="örn. Veri Bilimci"
                value={customRole}
                onChange={(e) => {
                  setCustomRole(e.target.value)
                  setPreset(null)
                }}
                className="mt-1.5"
              />
            </div>
          </ScrollArea>
        )}

        {/* Adım 2 — Motor ve yetenekler */}
        {step === 1 && (
          <div className="flex flex-col gap-3">
            <div className="grid grid-cols-2 gap-3">
              <div>
                <Label>Motor</Label>
                <Select value={motor} onValueChange={(v) => setMotor(v)}>
                  <SelectTrigger className="w-full">
                    <SelectValue placeholder="Motor seç" />
                  </SelectTrigger>
                  <SelectContent>
                    {motorOptions.map((m) => {
                      const installed = detectedMotorIds.has(m)
                      return (
                        <SelectItem key={m} value={m} disabled={!installed}>
                          {m} {installed ? '' : '(kurulu değil)'}
                        </SelectItem>
                      )
                    })}
                  </SelectContent>
                </Select>
                {motor && !motorInfo(motor) && (
                  <p className="pt-1 text-xs text-destructive">
                    '{motor}' kurulu değil — Ayarlar → Motorlar'dan kurun.
                  </p>
                )}
              </div>
              <div>
                <Label>Model</Label>
                <Input
                  placeholder="örn. sonnet"
                  value={model}
                  onChange={(e) => setModel(e.target.value)}
                />
              </div>
            </div>

            <div className="grid grid-cols-2 gap-3">
              <div>
                <Label>Effort</Label>
                <Select value={effort} onValueChange={setEffort}>
                  <SelectTrigger className="w-full">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {EFFORT_OPTIONS.map((e) => (
                      <SelectItem key={e} value={e}>
                        {e}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              <div>
                <Label>İzin profili</Label>
                <Select value={permissions} onValueChange={setPermissions}>
                  <SelectTrigger className="w-full">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {PERMISSION_OPTIONS.map((p) => (
                      <SelectItem key={p} value={p}>
                        {p}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-3">
              <div>
                <Label>Bütçe (USD / görev)</Label>
                <Input
                  type="number"
                  min={0}
                  step="0.1"
                  placeholder="örn. 1.5"
                  value={budget}
                  onChange={(e) => setBudget(e.target.value)}
                />
              </div>
              <div>
                <Label>Max tur</Label>
                <Input
                  type="number"
                  min={1}
                  step={1}
                  placeholder="örn. 20"
                  value={maxTurns}
                  onChange={(e) => setMaxTurns(e.target.value)}
                />
              </div>
            </div>
          </div>
        )}

        {/* Adım 3 — Uzmanlık ve kişilik */}
        {step === 2 && (
          <div className="flex flex-col gap-3">
            <div className="grid grid-cols-2 gap-3">
              <div>
                <Label htmlFor="agent-name">İsim</Label>
                <Input
                  id="agent-name"
                  placeholder="örn. Ayşe"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                />
              </div>
              <div>
                <Label>Avatar rengi</Label>
                <div className="mt-1.5 flex items-center gap-1.5">
                  {AVATAR_COLORS.map((c) => (
                    <button
                      key={c}
                      type="button"
                      aria-label={`renk ${c}`}
                      onClick={() => setAvatarColor(c)}
                      className={cn(
                        'size-6 rounded-full ring-offset-2 transition-shadow',
                        avatarColor === c && 'ring-2 ring-ring',
                      )}
                      style={{ backgroundColor: c }}
                    />
                  ))}
                </div>
              </div>
            </div>
            <div>
              <Label htmlFor="agent-prompt">Sistem promptu</Label>
              <Textarea
                id="agent-prompt"
                rows={5}
                placeholder="Ajanın ana talimatı…"
                value={systemPrompt}
                onChange={(e) => setSystemPrompt(e.target.value)}
                className="mt-1.5"
              />
            </div>
            {preset && (
              <p className="flex items-center gap-1.5 text-xs text-muted-foreground">
                <UserRoundIcon className="size-3.5" />
                Rol: {preset.role} · Motor: {motor} · Effort: {effort} · İzinler: {permissions}
              </p>
            )}
          </div>
        )}

        <DialogFooter>
          {step > 0 && (
            <Button variant="ghost" onClick={() => setStep((s) => Math.max(s - 1, 0))}>
              <ChevronLeftIcon className="size-4" />
              Geri
            </Button>
          )}
          {step < 2 ? (
            <Button onClick={next}>
              İleri
              <ChevronRightIcon className="size-4" />
            </Button>
          ) : (
            <Button onClick={submit} disabled={submitting}>
              {submitting ? 'İşe alınıyor…' : 'İşe Al'}
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
