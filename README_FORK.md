---
created: 2026-08-31
tags: [voxtype, fork, handoff]
---

# Voxtype Custom — Fork Summary

## Resumen
Fork de [peteonrails/voxtype](https://github.com/peteonrails/voxtype) para Ivy Bridge (sin AVX2) con puente de traducción bidireccional.

## Ramas
- `dev`: Espejo de upstream, base para todo el trabajo custom
- `fix/ivy-bridge-avx`: Solo fix para model_manager.rs (evita overwrites al Groq)
- `feat/bridge`: Puente de traducción + fix, rama de uso diario
- `fix/url-and-backend`: Fixes menores (URL doble /v1, backend→mode)

## Build
```bash
cd voxtype-custom
RUSTFLAGS="-C target-cpu=ivybridge" cargo build --release
cp target/release/voxtype ~/.local/bin/voxtype
systemctl --user restart voxtype
```

## Contexto completo
Ver: `06 - Voxtype - Hilo y Contexto para Agente Nuevo.md` en Herramientas/Voxtype/

## Estado
- Servicio: active, state idle
- Bridge: enabled, pair configurable via WIN+B
- Binario: ~/.local/bin/voxtype 1.0.1 feat/bridge c0a799a

## Rebase mensual
```bash
git checkout dev && git fetch upstream && git rebase upstream/dev && git push --force-with-lease
git checkout fix/ivy-bridge-avx && git rebase dev && git push --force-with-lease
git checkout feat/bridge && git rebase dev && git push --force-with-lease
```

## PR Strategy
- Crear ramas desde `dev`, cherry-pick commits necesarios
- No enviar `feat/bridge` entero
- PRs pequeños y enfocados