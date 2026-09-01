---
# voxtype — Translation Bridge Issue Draft

**Title:** `feat: optional translation bridge (ru<->es etc) — prototype working`

**Body:**

Hello — found voxtype two days ago through Lex Fridman's four-hour conversation with DHH. Using it daily since.

Need: a bridge mode for conversations across a language pair. Normal mode types what is spoken. Bridge mode types the counterpart language.

Example `ru ↔ es`: Russian input → Spanish output, Spanish input → Russian output. Same for `en ↔ es`.

How it works: after Whisper transcription (Groq `whisper-large-v3-turbo`, `language=auto`), when enabled it makes one Groq Chat call (`openai/gpt-oss-20b`, same `remote_api_key`). One prompt does detection and translation: "if Russian translate to Spanish, if Spanish translate to Russian, output only translation." About 500ms. When off, it is straight transcription.

Built: `src/bridge/mod.rs` after `transcribe/remote.rs` — same `reqwest`, 15s timeout, fallback to original text when offline. Config `[bridge] enabled=true, pair="es:en"` — enabled by default. Wired in `daemon.rs` after transcription. About 120 lines, `fmt/clippy/test` green. Branch `bridge` rebasing on `dev`. Short phrases like "sí" or "да" remain ambiguous, so fallback is passthrough.

Added: pair selector with 8 languages (es, ru, en, ja, zh, it, pt, fr) via `SUPER+B` two-step menu. Toggle bridge with `SUPER+H` (notifications via Omarchy Quattro QuickShell).

If this is useful, I can tidy it to project style and open a PR against `dev`. If not, I will keep it as a fork tracking upstream.

Separate: running on a T430 (Ivy Bridge) required `-C target-cpu=ivybridge` for the AVX2 SIGILL — small fix available if useful.
---
