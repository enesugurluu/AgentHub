import { Channel, invoke } from '@tauri-apps/api/core'

/**
 * Tauri invoke köprüsü — frontend ile Rust backend arasındaki tüm
 * komutların tip güvenli tanımları. (Docs: Bölüm 3.3 UI komut kanalı)
 */

export type AgentSpawnResult = {
  agentId: string
  executionId: string
}

/** Zeka/effort seviyesi (claude `--effort`; docs 6.1 Adım 2). */
export type Effort = 'low' | 'medium' | 'high' | 'xhigh' | 'max'

/**
 * CLI spawn seçenekleri (Rust `SpawnOptions` ile camelCase birebir eşleşir — WP-02).
 * `workdir`/`env` boş bırakılırsa backend worktree'yi çözer ve doldurur.
 */
export type SpawnOptions = {
  workdir?: string
  env?: [string, string][]
  args?: string[]
  model?: string | null
  effort?: Effort | null
  maxBudgetUsd?: number | null
  maxTurns?: number | null
  nonInteractive?: boolean
  taskFile?: string | null
}

export type EngineMetadata = {
  engineType: string
  version: string | null
  capabilities: string[]
}

/** Adaptör detect bilgisi (kurulu mu + sürüm + capability + kurulum ipucu). */
export type DetectResult = {
  detected: boolean
  version: string | null
  capabilities: string[]
  installHint: string | null
}

export type AgentRecord = {
  id: number
  name: string
  role: string
  motor: string
  model: string | null
  status: string
  worktreePath: string | null
  createdAt: string | null
  avatarColor: string | null
  configJson: string | null
  hiredAt: string | null
  firedAt: string | null
}

export type PermissionProfile = 'full' | 'standard' | 'limited' | 'custom'

/** İşe alım payload'ı (docs 6.1 Adım 2/3) — Rust `HirePayload` ile camelCase eşleşir. */
export type HirePayload = {
  name: string
  role: string
  motor: string
  model?: string | null
  effort?: string | null
  maxBudgetUsd?: number | null
  maxTurns?: number | null
  permissionsProfile: PermissionProfile
  systemPrompt?: string | null
  avatarColor?: string | null
  skills: string[]
  mcpServers: string[]
}

/** İşten çıkarma seçenekleri (docs 6.2). Worktree davranışı WP-05'te bağlanır. */
export type FireOptions = {
  worktreeAction: 'delete' | 'keep' | 'commit_and_keep'
  moveOpenTasksToBacklog: boolean
  keepLogs: boolean
}

/** Kısmi güncelleme (None = alana dokunma). */
export type AgentPatch = {
  name?: string | null
  role?: string | null
  motor?: string | null
  model?: string | null
  status?: string | null
  avatarColor?: string | null
}

export type WorktreeInfo = {
  path: string
  agentId: string
  agentName: string
  branchName: string
  createdAt: number
  parentRepoPath: string
}

export type BranchStrategy =
  | { type: 'existingBranch'; name: string }
  | { type: 'newBranchFrom'; baseBranch: string; name: string }

/** Worktree silme seçenekleri (docs 6.2; WP-05): delete | keep | commit_and_keep. */
export type WorktreeRemoveOptions = {
  action: 'delete' | 'keep' | 'commit_and_keep'
  force?: boolean
}

/**
 * PTY olayları: Rust tarafında serde tag'i ile `type: "output" | "exit"` olarak
 * serialize edilir. Çıktı ham bayt olarak gelir (UTF-8 çok baytlı karakterlerin
 * chunk sınırında bozulmasını önler — xterm Uint8Array'i doğrudan işler).
 */
export type PtyEvent = {
  agentId: string
  executionId: string
  kind: PtyEventKind
}

export type PtyEventKind =
  | { type: 'output'; data: number[] }
  // portable-pty 0.9: ExitStatus::exit_code() her zaman u32 döner (Option değil).
  | { type: 'exit'; code: number }
  // Parser sinyali (WP-04): progress / approval_requested / task_completed / task_failed
  | { type: 'signal'; signal: OutputSignal }

export type OutputSignal =
  | { type: 'progress'; turn: number; cost: number; tokensIn: number; tokensOut: number }
  | { type: 'approvalRequested'; pattern: string }
  | { type: 'taskCompleted'; summary: string }
  | { type: 'taskFailed'; reason: string }

/** Frontend tarafında oluşturulup invoke argümanı olarak backend'e verilir. */
export function createPtyChannel(onEvent: (event: PtyEvent) => void): Channel<PtyEvent> {
  const channel = new Channel<PtyEvent>()
  channel.onmessage = onEvent
  return channel
}

export function isTauriRuntime(): boolean {
  return (
    typeof window !== 'undefined' &&
    typeof (window as unknown as { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__ !==
      'undefined'
  )
}

// ---- invoke sarmalayıcıları -------------------------------------------------

export function agentSpawn(args: {
  agentId: string
  program: string
  args: string[]
  cols: number
  rows: number
  channel: Channel<PtyEvent>
}): Promise<AgentSpawnResult> {
  return invoke<AgentSpawnResult>('agent_spawn', args)
}

/**
 * Motor tipine göre spawn (ör. "claude", "pty"): Rust tarafında adaptör,
 * `SpawnOptions` ile CLI komutunu kendisi kurar.
 */
export function agentSpawnEngine(args: {
  agentId: string
  engineType: string
  options?: SpawnOptions
  cols: number
  rows: number
  channel: Channel<PtyEvent>
}): Promise<AgentSpawnResult> {
  return invoke<AgentSpawnResult>('agent_spawn_engine', {
    agentId: args.agentId,
    engineType: args.engineType,
    options: args.options ?? {},
    cols: args.cols,
    rows: args.rows,
    channel: args.channel,
  })
}

export function agentStop(args: { agentId: string; executionId: string }): Promise<void> {
  return invoke('agent_stop', args)
}

/**
 * Motor kurulumu (docs 7.5) — komut backend'de adaptörün `install_command()`'undan
 * çözülür; frontend asla program/args göndermez (FAZ0 S5). Oturum
 * `install-<engineType>` agentId'siyle terminal sekmesinde akar (WP-12).
 */
export function agentInstallEngine(args: {
  engineType: string
  cols: number
  rows: number
  channel: Channel<PtyEvent>
}): Promise<AgentSpawnResult> {
  return invoke<AgentSpawnResult>('agent_install_engine', {
    engineType: args.engineType,
    cols: args.cols,
    rows: args.rows,
    channel: args.channel,
  })
}

export function agentWrite(args: {
  agentId: string
  executionId: string
  data: string
}): Promise<void> {
  return invoke('agent_write', args)
}

export function ptyResize(args: {
  agentId: string
  executionId: string
  cols: number
  rows: number
}): Promise<void> {
  return invoke('pty_resize', args)
}

export function ptyListEngineAdapters(query?: string): Promise<string[]> {
  return invoke<string[]>('pty_list_engine_adapters', { query: query ?? 'all' })
}

export function ptyFindByEngineType(engineType: string): Promise<EngineMetadata[]> {
  return invoke<EngineMetadata[]>('pty_find_by_engine_type', { engineType })
}

/** Tek adaptörün metadata'sı — Settings UI id → metadata çözümlemesi. */
export function ptyAdapterMetadata(id: string): Promise<EngineMetadata> {
  return invoke<EngineMetadata>('pty_adapter_metadata', { id })
}

/** Tek adaptörün detect bilgisi (kurulu mu + install_hint) — WP-07/12. */
export function ptyAdapterDetectInfo(id: string): Promise<DetectResult> {
  return invoke<DetectResult>('pty_adapter_detect_info', { id })
}

export function listAgents(): Promise<AgentRecord[]> {
  return invoke<AgentRecord[]>('agent_list_all')
}

/** Yeni ajan kaydı (Hire Wizard — WP-07). */
export function agentHire(payload: HirePayload): Promise<AgentRecord> {
  return invoke<AgentRecord>('agent_hire', { payload })
}

/** Ajanı işten çıkar (Fire onay akışı — WP-08). */
export function agentFire(id: number, options: FireOptions): Promise<AgentRecord> {
  return invoke<AgentRecord>('agent_fire', { id, options })
}

/** Kalıcı silme — yalnızca `fired` kayıtlar. */
export function agentDelete(id: number): Promise<void> {
  return invoke('agent_delete', { id })
}

/** Kısmi güncelleme. */
export function agentUpdate(id: number, patch: AgentPatch): Promise<AgentRecord> {
  return invoke<AgentRecord>('agent_update', { id, patch })
}

/** Tek ajan kaydı. */
export function agentGet(id: number): Promise<AgentRecord> {
  return invoke<AgentRecord>('agent_get', { id })
}

/** settings tablosu (repo_path, main_branch, ... — WP-06). */
export function settingsGet(key: string): Promise<string | null> {
  return invoke<string | null>('settings_get', { key })
}

export function settingsSet(key: string, value: string): Promise<void> {
  return invoke('settings_set', { key, value })
}

/**
 * Repo yolu seçimi (WP-06): canonicalize + `.git` doğrulaması + worktree kökü
 * reddi; başarılıysa `settings.repo_path`'e yazılır ve canonical yol döner.
 */
export function repoSelect(path: string): Promise<string> {
  return invoke<string>('repo_select', { path })
}

export function worktreeCreate(args: {
  repoPath: string
  agentId: string
  agentName: string
  branchStrategy: BranchStrategy
}): Promise<WorktreeInfo> {
  return invoke<WorktreeInfo>('worktree_create', args)
}

export function worktreeList(repoPath: string): Promise<WorktreeInfo[]> {
  return invoke<WorktreeInfo[]>('worktree_list', { repoPath })
}

export function worktreeRemove(
  worktreePath: string,
  options?: WorktreeRemoveOptions,
): Promise<void> {
  return invoke('worktree_remove', { worktreePath, options })
}

/** Ajanın yönetilen worktree'si (spawn öncesi UI bilgisi; WP-07 masa ataması). */
export function worktreeForAgent(repoPath: string, agentId: string): Promise<WorktreeInfo> {
  return invoke<WorktreeInfo>('worktree_for_agent', { repoPath, agentId })
}
