# Changelog

Todos los cambios notables en este proyecto serán documentados en este archivo.

## [0.1.0] - 2026-02-03

### ✨ Características Principales

- **10 Koans Completos**: Desde variables hasta módulos, cubriendo todos los conceptos clave de Rust
- **Enfoque TDD**: Todos los ejercicios siguen Test-Driven Development
- **Comparaciones Python ↔ Rust**: Cada koan incluye comparaciones para facilitar aprendizaje
- **Tests Completos**: 8+ tests por koan para validar aprendizaje
- **Documentación Extensa**: GUIA.md, COMO_FUNCIONA.md, SOLUCIONES.md

### 🛠 Tooling & Configuración

- **VS Code Setup Completo**:
  - settings.json con formateo automático
  - launch.json para debugging
  - tasks.json con custom tasks
  - extensions.json con recomendaciones

- **GitHub Actions CI/CD**:
  - Verificación automática de código
  - Tests en cada push
  - Clippy linting
  - Rustfmt checking

- **Scripts de Progreso**:
  - check_progress.ps1 (Windows PowerShell)
  - check_progress.sh (Linux/Mac Bash)

### 📚 Documentación

- **README.md**: Introducción completa con badges y guía rápida
- **GUIA.md**: Tutorial paso a paso con ejemplos
- **COMO_FUNCIONA.md**: Explicación profunda de metodología TDD
- **SOLUCIONES.md**: Soluciones completas con explicaciones (41KB)
- **CONTRIBUTING.md**: Guía completa para contribuidores
- **QUICKSTART.md**: Guía rápida de 5 minutos
- **MI_PROGRESO.md**: Template para seguimiento personal

### 🎯 Koans Implementados

1. **01_about_variables** - Variables, tipos, mutabilidad, shadowing
2. **02_about_ownership** - Ownership, borrowing, referencias, slices
3. **03_about_structs** - Structs, enums, pattern matching
4. **04_about_traits** - Traits, generics, trait bounds, trait objects
5. **05_about_errors** - Option, Result, manejo de errores
6. **06_about_collections** - Vec, HashMap, String
7. **07_about_iterators** - Iteradores, closures, map/filter/fold
8. **08_about_lifetimes** - Lifetimes, anotaciones de referencias
9. **09_about_concurrency** - Threads, Mutex, Arc, channels
10. **10_about_modules** - Módulos, visibilidad, organización

### 🎁 Características Especiales

- **Sin Soluciones Directas**: Los TODOs guían sin revelar respuestas
- **Hints Progresivos**: Pistas claras para resolver cada ejercicio
- **Orden Progresivo**: Cada koan se construye sobre anteriores
- **Python Friendly**: Diseñado específicamente para desarrolladores Python
- **Workspace Cargo**: Fácil ejecutar todos los tests

### 📋 Requisitos

- Rust 1.75+
- VS Code recomendado
- Conocimientos básicos de Python

### 🚀 Inicio Rápido

```bash
git clone https://github.com/jjmmolina/rust-koans-python-devs.git
cd rust-koans-python-devs
cd 01_about_variables
cargo test
```

### 📖 Documentación Adicional

- [GUIA.md](GUIA.md) - Tutorial completo
- [COMO_FUNCIONA.md](COMO_FUNCIONA.md) - Metodología TDD
- [SOLUCIONES.md](SOLUCIONES.md) - Soluciones y explicaciones
- [QUICKSTART.md](QUICKSTART.md) - 5 minutos para empezar
- [CONTRIBUTING.md](CONTRIBUTING.md) - Cómo contribuir

### 🙏 Inspiración

- Ruby Koans
- Go Koans
- Rustlings
- The Rust Book

---

## Roadmap Futuro

- [ ] Koans adicionales sobre Async/Await
- [ ] Koans sobre Macros
- [ ] Interactividad mejorada en CLI
- [ ] Sistema de badges de progreso
- [ ] Traducción a otros idiomas
- [ ] Videos tutoriales
- [ ] Comunidad Discord

---

**Versión Actual**: 0.1.0  
**Última Actualización**: 2026-02-03  
**Licencia**: MIT
