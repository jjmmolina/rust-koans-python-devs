# Rust Koans - Aprende Rust con TDD 🦀

[![Rust Version](https://img.shields.io/badge/Rust-1.75+-orange?style=flat&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![CI](https://github.com/jjmmolina/rust-koans-python-devs/workflows/CI%20-%20Build%20&%20Test/badge.svg)](https://github.com/jjmmolina/rust-koans-python-devs/actions)
[![GitHub Stars](https://img.shields.io/github/stars/jjmmolina/rust-koans-python-devs?style=social)](https://github.com/jjmmolina/rust-koans-python-devs)

> Aprende Rust mediante Test-Driven Development (TDD) con un enfoque especial para desarrolladores Python.

## 🎯 ¿Qué son los Koans?

Los Koans son ejercicios de programación que siguen el ciclo TDD:

```
🔴 Red → 🟢 Green → 🔵 Refactor
```

**🎓 Filosofía de Aprendizaje:**
- ❌ **NO** te damos las soluciones directamente
- ✅ Te damos **tests** que describen el comportamiento esperado
- ✅ Te damos **pistas y hints** sobre cómo resolverlo en Rust
- ✅ Te mostramos **comparaciones con Python** para facilitar la comprensión
- ✅ Tú **implementas** la solución siguiendo los TODOs

**Este es un viaje de descubrimiento, no de copiar y pegar.** 🚀

## 🎯 ¿Para quién es esto?

✅ Desarrolladores Python que quieren aprender Rust  
✅ Personas que prefieren aprender haciendo  
✅ Quienes buscan entender Rust mediante comparaciones con Python  
✅ Desarrolladores que quieren dominar TDD en Rust  

## 📋 Requisitos

- Rust 1.75 o superior ([Instalar](https://www.rust-lang.org/tools/install))
- VS Code (recomendado) con extensión rust-analyzer
- Conocimientos básicos de Python
- Ganas de aprender 😊

## 🚀 Inicio Rápido

1. **Clona el repositorio**:
   ```bash
   git clone https://github.com/jjmmolina/rust-koans-python-devs.git
   cd rust-koans-python-devs
   ```

2. **Ejecuta todos los tests** para ver cuántos fallan:
   ```bash
   cargo test
   ```
   Verás muchos tests rojos 🔴 - ¡esto es lo esperado!

3. **Empieza con el primer koan**:
   ```bash
   cd 01_about_variables
   cargo test
   ```

4. **Abre** `src/lib.rs` y busca los TODOs:
   - Lee las comparaciones con Python
   - Sigue las pistas en los comentarios
   - Reemplaza `todo!()` y placeholders por las soluciones correctas

5. **Ejecuta el test** hasta que pase 🟢:
   ```bash
   cargo test
   ```

6. **Repite** con cada koan siguiendo el orden numérico

7. **Lee la [GUIA.md](GUIA.md)** para un ejemplo paso a paso completo

## 📚 Documentación Rápida

| Documento | Tiempo | Para Qué |
|-----------|--------|----------|
| **[QUICKSTART.md](QUICKSTART.md)** | 5 min ⚡ | Empieza ahora mismo |
| **[GUIA.md](GUIA.md)** | 30 min 📖 | Tutorial completo paso a paso |
| **[COMO_FUNCIONA.md](COMO_FUNCIONA.md)** | 20 min 🧠 | Entiende la metodología TDD |
| **[CHEATSHEET.md](CHEATSHEET.md)** | 10 min 📋 | Referencia Python ↔ Rust |
| **[MEJORES_PRACTICAS.md](MEJORES_PRACTICAS.md)** | 15 min 💎 | Patrones y código idiomático |
| **[SOLUCIONES.md](SOLUCIONES.md)** | ⚠️ Después | Soluciones (úsalo después de intentar) |

## 📖 Cómo Usar los Koans

### 🎯 Estructura de Cada Koan

Cada archivo `lib.rs` contiene **ejercicios graduales**:

```rust
// PASO 1: Explicación del concepto
// En Python: ejemplo_python()
// En Rust: ejemplo_rust()
// 
// Concepto clave: Explicación detallada de por qué es diferente

// TODO: Descripción clara de qué hacer
pub fn mi_funcion() -> String {
    todo!()  // ← Reemplaza esto con tu código
    // Hint: Pista de cómo hacerlo
}
```

### 💪 Estrategia de Aprendizaje Óptima

1. **Lee primero SIN escribir código**: Entiende el contexto Python vs Rust
2. **Lee el test**: Comprende qué se espera (el test es tu especificación)
3. **Implementa gradualmente**: Un TODO a la vez, no saltes pasos
4. **Ejecuta tests frecuentemente**: `cargo test` después de cada cambio
5. **Lee los mensajes de error**: El compilador de Rust es tu mejor maestro
6. **Experimenta**: Prueba variaciones, rompe cosas, aprende de los errores

### 🎓 Tipos de Tests

- **Tests normales**: Pasan cuando resuelves el TODO
- **Tests con `#[ignore]`**: Requieren completar estructuras previas (descomenta cuando estés listo)
- **Tests con mensajes**: Te guían con feedback específico si fallan

**Tu trabajo:**
1. Lee el comentario de comparación Python/Rust
2. Lee el TODO
3. Consulta los hints si los hay
4. Implementa la solución
5. Ejecuta `cargo test` para verificar

**NO mires las soluciones en internet hasta que lo intentes primero!** 💪

## 📚 Estructura del Proyecto

```
rust-koans/
├── 01_about_variables/      # Variables, tipos, mutabilidad, shadowing
│   ├── src/lib.rs           # ← Edita este archivo
│   └── tests/tests.rs       # ← Los tests que deben pasar
├── 02_about_ownership/      # Ownership, borrowing, referencias
├── 03_about_structs/        # Structs, enums, pattern matching
├── 04_about_traits/         # Traits, generics, trait bounds
├── 05_about_errors/         # Result, Option, error handling
├── 06_about_collections/    # Vec, HashMap, String
├── 07_about_iterators/      # Iteradores, closures, map/filter
├── 08_about_lifetimes/      # Lifetimes, referencias avanzadas
├── 09_about_concurrency/    # Threads, channels, Arc, Mutex
└── 10_about_modules/        # Módulos, crates, visibilidad
```

**Orden recomendado:** Sigue el orden numérico (01 → 10) ya que cada koan construye sobre los anteriores.

## 💡 Consejos para Desarrolladores Python

| Concepto | Python | Rust |
|----------|--------|------|
| **Mutabilidad** | Por defecto mutable | Por defecto inmutable (`let mut`) |
| **Ownership** | Todo por referencia | Sistema único de ownership |
| **None** | `None` | `Option<T>` con `Some(T)` o `None` |
| **Excepciones** | `try/except` | `Result<T, E>` con `Ok(T)` o `Err(E)` |
| **Clases** | `class Person:` | `struct Person {}` + `impl Person {}` |
| **Herencia** | Herencia de clases | Composición + Traits |
| **Duck Typing** | Implícito | Traits explícitos |
| **GC** | Garbage collector | Ownership + RAII |

**Diferencias clave:**
- 🔸 Rust **no tiene garbage collector** - usa ownership
- 🔸 Rust es **de tipos estáticos** con inferencia de tipos
- 🔸 Rust requiere **manejo explícito de errores**
- 🔸 Rust **previene race conditions** en tiempo de compilación
- 🔸 Rust tiene **zero-cost abstractions**

## 📖 Documentación Adicional

| Documento | Descripción |
|-----------|------------|
| **[COMO_FUNCIONA.md](COMO_FUNCIONA.md)** | 📚 Explicación detallada de la metodología TDD y cómo funcionan los koans |
| **[GUIA.md](GUIA.md)** | 🚀 Tutorial paso a paso con ejemplos completos |
| **[QUICKSTART.md](QUICKSTART.md)** | ⚡ Comienza en 5 minutos |
| **[CHEATSHEET.md](CHEATSHEET.md)** | 📋 Python vs Rust - Referencia rápida |
| **[SOLUCIONES.md](SOLUCIONES.md)** | ⚠️ Soluciones completas (úsalo solo DESPUÉS de intentarlo) |
| **[CHANGELOG.md](CHANGELOG.md)** | 📝 Historial de versiones |
| **[CONTRIBUTING.md](CONTRIBUTING.md)** | 🤝 Cómo contribuir al proyecto |
| **[SECURITY.md](SECURITY.md)** | 🔒 Política de seguridad |
| **[MI_PROGRESO.md](MI_PROGRESO.md)** | ✅ Template para seguir tu progreso |

## 🛠️ Comandos Útiles
 del workspace
cargo test --workspace

# Ejecutar tests de un koan específico
cd 01_about_variables
cargo test

# Ejecutar un test específico
cargo test test_nombre

# Ver output detallado
cargo test -- --nocapture

# Verificar compilación sin ejecutar
cargo check

# Formatear código
cargo fmt

# Ejecutar clippy (linter)
cargo clippy --all-targets --all-features

# Ejecutar específicamente los tests ignorados
cargo test -- --ignored --nocapture
```

### VS Code Tasks (si usas extensión tasks)

Presiona `Ctrl+Shift+P` y escribe:
- `Tasks: Run Task` → `Run Current Koan Tests`
- `Tasks: Run Task` → `Check Progress`
- `Tasks: Run Task` → `Lint Code (Clippy)Ejecutar clippy (linter)
cargo clippy
```

## 🎓 Progreso

Usa el script de progreso para ver cuántos koans has completado:

```bash
# Windows PowerShell
.\check_progress.ps1

# Linux/Mac
./check_progress.sh
```

## 🤝 Contribuir

¡Las contribuciones son bienvenidas! Lee [CONTRIBUTING.md](CONTRIBUTING.md) para más detalles.

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Ver [LICENSE](LICENSE) para más detalles.

## 🙏 Agradecimientos

Inspirado por:
- [Ruby Koans](https://github.com/edgecase/ruby_koans)
- [Go Koans](https://github.com/cdarwin/go-koans)
## ✨ Características

✅ **10 Koans Progresivos** - Desde variables hasta módulos  
✅ **Enfoque TDD** - Test-Driven Development desde el principio  
✅ **Comparaciones Python ↔ Rust** - Aprende más rápido reconociendo patrones  
✅ **Sin Soluciones Directas** - Aprende descubriendo, no copiando  
✅ **Bien Documentado** - GUIA.md, COMO_FUNCIONA.md, SOLUCIONES.md  
✅ **CI/CD Integrado** - GitHub Actions verifica el código automáticamente  
✅ **Configuración VS Code** - Settings, tasks, debug automático  
✅ **Scripts de Progreso** - Verifica tu avance en PowerShell o Bash  

## 🎬 Quick Start Video Style

```
$ git clone https://github.com/jjmmolina/rust-koans-python-devs.git
$ cd rust-koans-python-devs
$ cd 01_about_variables
$ cargo test              # ❌ Tests fallan
$ code src/lib.rs         # 📝 Edita el archivo
# ... implementa la solución ...
$ cargo test              # ✅ ¡Tests pasan!
$ cd ../02_about_ownership
# ... repite 9 veces más ...
$ cargo test --workspace  # ✅ ¡Todos los tests pasan!
```

## 🤔 FAQ

**P: ¿Cuánto tiempo toma completar todos los koans?**  
R: Depende de tu experiencia:
- Con conocimientos de Python: 20-40 horas
- Sin experiencia en sistemas/tipos: 40-60 horas
- Completando todos los ejercicios de bonus: 60-80 horas

**P: ¿Puedo ver las soluciones?**  
R: Sí, están en [SOLUCIONES.md](SOLUCIONES.md), pero **intenta resolver primero**. El verdadero aprendizaje viene del proceso de descubrimiento, los errores del compilador y el ciclo Red-Green-Refactor.

**P: ¿Este proyecto es para principiantes?**  
R: Sí, pero necesitas:
- ✅ Conocimientos básicos de Python (variables, funciones, bucles, clases)
- ✅ Entender qué son los tests (aunque TDD sea nuevo para ti)
- ✅ Paciencia con el compilador de Rust (es estricto pero útil)

**P: ¿Los mensajes de error del compilador de Rust son difíciles?**  
R: Al principio sí, pero el compilador de Rust es **el mejor maestro**. Lee los errores cuidadosamente:
- Te dice exactamente dónde está el problema
- Te sugiere soluciones
- Te enseña los conceptos mientras programas

**P: ¿Qué hago si me quedo atascado?**  
R: En este orden:
1. Lee el mensaje de error del compilador (¡seriamente!)
2. Lee los comentarios y pistas en el código
3. Revisa la sección de conceptos en [GUIA.md](GUIA.md)
4. Busca en [CHEATSHEET.md](CHEATSHEET.md) la sintaxis
5. Experimenta (romper cosas es parte del aprendizaje)
6. Si después de 30 min sigues atascado, revisa [SOLUCIONES.md](SOLUCIONES.md)

**P: ¿Puedo contribuir nuevos koans?**  
R: ¡Sí! Lee [CONTRIBUTING.md](CONTRIBUTING.md) para detalles.

---

**Creado con ❤️ para la comunidad de desarrolladores Python que quieren aprender Rust**

**Si te gusta este proyecto, dale una ⭐ en GitHub!
## 📚 Recursos Adicionales

- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustlings**: https://github.com/rust-lang/rustlings
- **Rust Playground**: https://play.rust-lang.org/

---

**Creado con ❤️ para la comunidad de desarrolladores Python que quieren aprender Rust**

¡Empieza tu viaje con Rust hoy! 🦀
