# Voxtype Custom — Estado Final

Fecha: 2026-08-31

## Estado del sistema

**Todos los modos funcionan correctamente.** El usuario puede alternar idiomas, grabar, cambiar entre dictado general y modo bridge.

---

## Modos disponibles

| Modo | Hotkey | Comando | Estado |
|------|--------|---------|--------|
| Dictado normal | `SUPER+N` | `voxtype record toggle` | ✅ Funcional |
| Bridge toggle | `SUPER+H` | `voxtype-bridge-toggle` | ✅ Funcional |
| Seleccionar par | `SUPER+B` | `voxtype-bridge-select` | ✅ Funcional |
| Push-to-talk | `F9` | `voxtype record start/stop` | ✅ Funcional |

---

## Bridge

| Parámetro | Valor |
|-----------|-------|
| Estado | Habilitado |
| Par actual | `es:en` |
| Modelo | `openai/gpt-oss-20b` |
| Hotkey toggle | `SUPER+H` |
| Hotkey par | `SUPER+B` (selector de 2 pasos, 8 idiomas) |
| Notificación | Omarchy Quattro QuickShell |
| Bidireccional | Sí (español → inglés, inglés → español) |

### Idiomas del selector

es, ru, en, ja, zh, it, pt, fr

---

## Issues resueltos

| Issue | Descripción | Estado |
|-------|-------------|--------|
| SIGILL | AVX2 no disponible en Ivy Bridge | ✅ Resuelto con `-C target-cpu=ivybridge` |
| remote_model | Config usaba modelo incorrecto | ✅ Resuelto |
| /tmp lleno | 6.2 GB de archivos temporales | ✅ Limpieza, ahora ~150 MB |
| Brave pause | Pausa inconsistente | ✅ Verificado, funciona |
| Notificaciones | No llegaban | ✅ Integrado con Omarchy Quattro QuickShell |

---

## Ramas pendientes

| Rama | Contenido | PR |
|------|-----------|-----|
| `fix/ivy-bridge-avx` | Fix AVX2 para Ivy Bridge | Pendiente |
| `feat/bridge` | Puente de traducción bidireccional | Pendiente |

---

## Archivos clave

| Archivo | Descripción |
|---------|-------------|
| `src/bridge/mod.rs` | Implementación del bridge (~120 líneas) |
| `src/config/bridge.rs` | Config del bridge |
| `~/.local/bin/voxtype-bridge-toggle` | Script de toggle del bridge |
| `~/.local/bin/voxtype-bridge-select` | Script de selección de par |
| `~/.config/voxtype/config.toml` | Config principal |

---

## Verificación

```bash
# Daemon corriendo
pgrep -a voxtype

# Test dictado
SUPER+N → hablar → soltar → texto aparece

# Test bridge
SUPER+H → notificación aparece

# Test selector
SUPER+B → elegir idiomas → par guardado

# Test push-to-talk
F9 (mantener) → hablar → soltar → texto aparece

# Verificar /tmp
du -sh /tmp
```

---

## Próximos pasos

1. Merge de PRs contra upstream (`fix/ivy-bridge-avx`, `feat/bridge`)
2. Considerar soporte para más pares de idiomas
3. Evaluar streaming con Gemini Live (ver STATUS anterior)
