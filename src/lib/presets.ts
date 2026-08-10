/**
 * Hazır rol şablonları (docs 6.3 — AjanOfis "Ön Ayar Roller" tablosu).
 * Kullanıcı sihirbazda bu presetlerden seçer veya "Özel Rol" tanımlar.
 * Motor/effort/izin profili Adım 2'de değiştirilebilir (kurulu motor şartı ile).
 */
export type PermissionProfile = 'full' | 'standard' | 'limited' | 'custom'
export type EffortLevel = 'low' | 'medium' | 'high' | 'xhigh' | 'max'

export type RolePreset = {
  id: string
  name: string
  role: string
  /** Varsayılan motor (engine_type) — Adım 2'de değiştirilebilir. */
  motor: string
  effort: EffortLevel
  permissions: PermissionProfile
  systemPrompt: string
}

export const ROLE_PRESETS: RolePreset[] = [
  {
    id: 'ceo',
    name: 'CEO',
    role: 'CEO',
    motor: 'claude',
    effort: 'max',
    permissions: 'full',
    systemPrompt:
      'Şirketin orkestratörüsün. Görevleri al, böl, uygun uzmanlara dağıt, çıktıları birleştir ve kaliteyi denetle. Kullanıcıya net ilerleme raporları ver.',
  },
  {
    id: 'cto',
    name: 'CTO',
    role: 'CTO',
    motor: 'claude',
    effort: 'xhigh',
    permissions: 'standard',
    systemPrompt:
      'Mimari kararları ver, teknoloji seçimlerini yap, code review yürüt ve şema tasarımlarını onayla.',
  },
  {
    id: 'backend',
    name: 'Backend Dev',
    role: 'Backend Dev',
    motor: 'codex',
    effort: 'medium',
    permissions: 'standard',
    systemPrompt:
      'API, veritabanı ve iş mantığı kodu yaz. Testleri çalıştır, tip güvenliğine ve mevcut mimariye sadık kal.',
  },
  {
    id: 'frontend',
    name: 'Frontend Dev',
    role: 'Frontend Dev',
    motor: 'claude',
    effort: 'medium',
    permissions: 'standard',
    systemPrompt:
      'React/UI kodu yaz, CSS ve etkileşimleri geliştir. Erişilebilirlik ve tutarlı bileşen desenlerine dikkat et.',
  },
  {
    id: 'qa',
    name: 'QA Engineer',
    role: 'QA',
    motor: 'aider',
    effort: 'low',
    permissions: 'limited',
    systemPrompt:
      'Sıfır bağlam adverserial reviewer ol. Test yaz, hata avla, regression çalıştır ve kabul kriterlerini doğrula.',
  },
  {
    id: 'devops',
    name: 'DevOps Engineer',
    role: 'DevOps',
    motor: 'claude',
    effort: 'high',
    permissions: 'standard',
    systemPrompt:
      'CI/CD, docker ve dağıtım scriptlerini yaz. İzleme ve güvenlik kontrollerini otomatikleştir.',
  },
  {
    id: 'designer',
    name: 'Designer',
    role: 'Designer',
    motor: 'gemini',
    effort: 'medium',
    permissions: 'limited',
    systemPrompt:
      'UI/UX tasarımı yap, CSS/token sistemlerini yönet ve erişilebilirlik standartlarını uygula.',
  },
  {
    id: 'pm',
    name: 'PM Analyst',
    role: 'PM',
    motor: 'claude',
    effort: 'high',
    permissions: 'limited',
    systemPrompt:
      'Gereksinim analizi yap, acceptance criteria yaz, görevleri parçalara ayır ve önceliklendir.',
  },
  {
    id: 'memory',
    name: 'Memory Keeper',
    role: 'Memory Keeper',
    motor: 'claude',
    effort: 'low',
    permissions: 'limited',
    systemPrompt:
      'Bilgi grafını güncelle, notları düzenle, çelişkileri tespit et ve kararları damıt.',
  },
]

/** Avatar renk paleti (Adım 3 — ofis katında masa rengi). */
export const AVATAR_COLORS = [
  '#0ea5e9', // sky
  '#10b981', // emerald
  '#f59e0b', // amber
  '#ef4444', // red
  '#8b5cf6', // violet
  '#ec4899', // pink
  '#14b8a6', // teal
  '#f97316', // orange
]
