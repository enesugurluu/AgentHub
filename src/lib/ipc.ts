import { Channel, invoke } from '@tauri-apps/api/core'

/**
 * Tauri invoke köprüsü — frontend ile Rust backend arasındaki tüm
 * komutların tip güvenli tanımları. (Docs: Bölüm 3.3 UI komut kanalı)
 */

export type AgentSpawnResult = {
  agentId: string
  executionId: string
}

export type EngineMetadata = {
  engineType: string
  version: string | null
  capabilities: string[]
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
  cols: number
  rows: number
  channel: Channel<PtyEvent>
}): Promise<AgentSpawnResult> {
  return invoke<AgentSpawnResult>('agent_spawn_engine', args)
}

export function agentStop(args: { agentId: string; executionId: string }): Promise<void> {
  return invoke('agent_stop', args)
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

export function listAgents(): Promise<AgentRecord[]> {
  return invoke<AgentRecord[]>('agent_list_all')
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

export function worktreeRemove(worktreePath: string, force?: boolean): Promise<void> {
  return invoke('worktree_remove', { worktreePath, force })
}
