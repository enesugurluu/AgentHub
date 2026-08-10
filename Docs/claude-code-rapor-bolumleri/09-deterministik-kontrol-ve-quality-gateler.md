## 9. Deterministik Kontrol ve Quality Gate'ler

### 9.1 Temel Prensip

> **"AI çıktısının kalitesi, doğrulama katmanlarının kalitesiyle sınırlıdır."**

Modelin "bu kod çalışıyor" demesine güvenmek yerine **çalıştığını kanıtlayan** otomatik katmanlar gerekir. 1 veya 0 sonucu veren, "bence çalışıyor"a hiçbir zaman güvenmeyen sistemler kurmalısınız.

### 9.2 Deterministik Kontrol Katmanları (Kalite Kapıları)

Her kod değişikliği sırayla şu kapılardan geçmelidir:

```
[Claude kod yazar]
       ↓
[Gate 1] Type Check (tsc --noEmit / mypy)
       ↓ ✅ Geçtiyse devam
[Gate 2] Lint (ESLint / Ruff)
       ↓ ✅
[Gate 3] Format (Prettier / Black — otomatik düzeltmeler)
       ↓ ✅
[Gate 4] Unit Test (Vitest/pytest — değişiklikle ilgili testler)
       ↓ ✅
[Gate 5] Tam Test Suite (pytest / pnpm test:all)
       ↓ ✅
[Gate 6] Zero-Context AI Review (ayrı model/oturum)
       ↓ ✅
[Gate 7] (Opsiyonel) Entegrasyon/E2E Test (Playwright/Cypress)
       ↓ ✅
[Commit] → [PR] → [İnsan Review] → [Merge]
```

### 9.3 Otomatik Doğrulama Döngüsü Betiği

Bu betik Claude'u bir döngü içinde çalıştırır, testler kırmızıysa hatayı geri besler, yeşil olana kadar düzelttirir.

**`scripts/auto-dev.py`:**
```python
#!/usr/bin/env python3
"""
Deterministik ajan döngüsü: Kod üret → Test et → Hata varsa geri besle.
"""
import subprocess
import sys
import json
import time
from pathlib import Path

MAX_ROUNDS = 8

def run_command(cmd, cwd=None):
    """Komut çalıştır, çıkış kodu ve çıktıyı döndür."""
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd=cwd)
    return result.returncode, result.stdout + result.stderr

def check_gate(name, cmd, cwd=None, auto_fix_cmd=None):
    """Bir quality gate'i çalıştır. Başarısızsa opsiyonel oto-fix komutunu çalıştır."""
    print(f"\n{'='*60}")
    print(f"[GATE] {name}")
    print(f"{'='*60}")
    code, output = run_command(cmd, cwd)
    if code == 0:
        print(f"✅ {name} geçti")
        return True, output
    print(f"❌ {name} başarısız (exit code: {code})")
    print(output[:2000])
    if auto_fix_cmd:
        print(f"🔧 Otomatik düzeltme deneniyor: {auto_fix_cmd}")
        run_command(auto_fix_cmd, cwd)
        code2, output2 = run_command(cmd, cwd)
        if code2 == 0:
            print(f"✅ Otomatik düzeltme sonrası {name} geçti")
            return True, output2
        output += "\n\nOto-fix sonrası çıktı:\n" + output2
    return False, output

def main():
    task_description = sys.argv[1] if len(sys.argv) > 1 else "TASK.md"
    task_file = Path(task_description)
    if not task_file.exists():
        print(f"Görev dosyası bulunamadı: {task_file}")
        sys.exit(1)

    print(f"🚀 Otonom geliştirme döngüsü başlatılıyor...")
    print(f"📋 Görev: {task_file}")
    print(f"🎯 Maksimum tur: {MAX_ROUNDS}")

    rounds = 0
    while rounds < MAX_ROUNDS:
        rounds += 1
        print(f"\n{'#'*60}")
        print(f"### TUR {rounds}/{MAX_ROUNDS}")
        print(f"{'#'*60}")

        # Adım 1: Claude'a kodu yazdır/güncelle
        print(f"\n[1/4] Claude kodu üretiyor/güncelliyor...")
        claude_prompt = f"""
{task_file.read_text()}

Önceki hataları ve durumu da dikkate al. Şu anda tur {rounds}'dayız.
Kod üzerinde gerekli değişiklikleri yap. Bittiğinde sadece bitir, açıklama yapma.
"""
        # Claude Code'u headless (--print) çalıştır
        code, claude_out = run_command(f'claude --print "{claude_prompt}"')
        print(f"Claude çıktı üretti (kod: {code})")

        # Hızlı bir feedback olarak CLAUDE.md kontrolü: hangi dosyalar değişti?
        diff_code, diff = run_command("git diff --stat")
        print(f"\nDeğişiklikler:\n{diff}")

        # Adım 2: Quality gate'leri çalıştır
        gates = [
            ("Type Check", "pnpm typecheck", "pnpm install"),
            ("Lint", "pnpm lint", "pnpm lint:fix"),
            ("Unit Test", "pnpm test -- --run"),
        ]

        all_passed = True
        failures = {}
        for name, cmd, *rest in gates:
            auto_fix = rest[0] if rest else None
            passed, output = check_gate(name, cmd, auto_fix_cmd=auto_fix)
            if not passed:
                all_passed = False
                failures[name] = output[-3000:]  # Son 3000 karakter

        if all_passed:
            print(f"\n{'🎉'*20}")
            print(f"✅ TÜM KAPILAR GEÇTİ — {rounds} turda tamamlandı!")
            print(f"{'🎉'*20}")
            # Başarılı checkpoint commit
            run_command(f'git commit -am "chore: otonom döngü başarılı (tur {rounds})"')
            return 0

        # Adım 3: Hataları Claude'a geri besle
        print(f"\n[4/4] Hatalar Claude'a geri besleniyor...")
        feedback = f"Tur {rounds} sonucu bazı kapılar başarısız oldu. Lütfen hataları düzelt:\n\n"
        for gate_name, err_output in failures.items():
            feedback += f"=== HATA: {gate_name} ===\n{err_output}\n\n"
        feedback += "\nBu hataları düzelt ve tekrar dene. Testleri kolaylaştırma, kodu düzelt."

        # Feedback'i bir dosyaya yaz (Claude'un okuması için)
        Path("FEEDBACK.md").write_text(feedback)
        print(f"Hatalar FEEDBACK.md'ye yazıldı, sonraki tur düzeltme yapacak.")

        # Çok fazla dosya değiştiyse, kanıtlama amaçlı ara commit at
        if rounds % 3 == 0:
            run_command(f'git commit -am "chore: checkpoint tur {rounds} (hatalar devam ediyor)"')

        # Küçük bir nefeslenme
        time.sleep(2)

    print(f"\n❌ Maksimum tur sayısına ({MAX_ROUNDS}) ulaşıldı.")
    print(f"Son durum FEEDBACK.md ve git geçmişinde mevcuttur.")
    print(f"Lütfen manuel müdahale yapın.")
    return 1

if __name__ == "__main__":
    sys.exit(main())
```

### 9.4 Generatör ve Reviewer Ayrımı Prensibi

**Kural: Kodu yazan ajan, kendi kodunun denetçisi olamaz.**

Bu psikolojik bir gerçektir — insanlar için de, AI için de: kendi ürettiğiniz çıktıdaki hataları görmek, sıfır bir gözle bakmaktan çok daha zordur.

**Uygulama Şekilleri:**

1. **Aynı model, ayrı oturum (ücretsiz/basit):**
   - Terminal 1: `claude` (kod yazdır)
   - Terminal 2: Ayrı bir dizinde veya worktree'de `claude` ile sadece review
   - Reviewer oturumuna üretim sürecine dair hiçbir bağlam verme — sadece kodu ver

2. **Farklı model, çapraz doğrulama (güçlü):**
   - Üretici: Claude Sonnet (yaratıcı, bağlamı iyi tutar)
   - Denetleyici: GPT-4o (robotik, sorgulamayan, talimatlara tam itaat)

3. **Uzmanlaşmış Reviewer prompt'u:**
   ```
   Sıfır bağlamla kod incelemesi yapıyorsun. Bu kodun ne için yazıldığını,
   kim tarafından yazıldığını bilmiyorsun. Sadece kodun kendisine bakarak
   şu kategorilerde hata ara:
   1. Runtime hatalar (null/undefined, type errors)
   2. Güvenlik açıkları (injection, auth eksikleri, XSS)
   3. Off-by-one ve sınır koşulu hataları
   4. Yarış koşulları (race conditions)
   5. Kaynak sızıntıları (connection, file handle)
   6. Test eksiklikleri
   7. Performans sorunları (N+1 query, gereksiz döngüler)

   Her sorun için DOSYA:SATIR numarası ver ve neden hata olduğunu açıkla.
   Eğer hiç hata bulamazsan "LGTM" yaz ve neden güvendiğini belirt.
   ```

### 9.5 Test İlkesi: Testi de Ayrı Bir Ajan Yazsın

Testleri kod yazan ajana yazdırmak **ahlaki tehlike (moral hazard)** yaratır: ajan, kendi kodunu geçecek testleri yazar, kodun çalıştığını değil kendi yazdığının geçtiğini kanıtlamış olur.

**Doğru akış:**
1. **Ajan A:** Testleri yaz (kodu henüz yazma!) — TDD yaklaşımı
2. **Ajan B:** Implementasyonu yap (testler kırmızıdan yeşile döne kadar)
3. **Ajan C (Reviewer):** Hem testi hem implementasyonu incele

Uygulamada basit olması için şu yeterlidir:
- Önce Claude'a test yazdırın
- `/clear` ile temizleyin
- "Şu testleri geçecek şekilde implementasyon yap" deyin
- Sonra ayrı bir oturumda ikisini birden inceletin

---
