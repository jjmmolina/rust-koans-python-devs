# Quick Start Guide - 5 Minutos ⚡

La forma más rápida de empezar con Rust Koans.

## ✅ Paso 1: Instalación (1 minuto)

```bash
# 1. Instala Rust si no lo tienes
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clona el repositorio
git clone https://github.com/jjmmolina/rust-koans-python-devs.git
cd rust-koans-python-devs
```

## 🚀 Paso 2: Primer Koan (4 minutos)

```bash
# Entra al primer koan
cd 01_about_variables

# Ejecuta los tests (verás fallos - es normal)
cargo test
```

**Salida esperada**:
```
thread 'test_get_saludo' panicked at 'not yet implemented'
thread 'test_incrementar_contador' panicked at 'not yet implemented'
...
test result: FAILED. 0 passed; 10 failed; 0 ignored; 0 measured
```

## 📝 Paso 3: Resuelve el Primer Ejercicio (3 minutos)

**Abre** `src/lib.rs` en VS Code:

```rust
// PASO 1: Variables Inmutables
// En Python: saludo = "Hola Rust"
// En Rust: let saludo = "Hola Rust"; (inmutable por defecto)
//
// TODO: Retorna la cadena "Hola Rust"
pub fn get_saludo() -> &'static str {
    todo!()  // ← Reemplaza esto
}
```

**Tu tarea**: Reemplaza `todo!()` con la solución.

**Solución**:
```rust
pub fn get_saludo() -> &'static str {
    "Hola Rust"
}
```

## ✨ Paso 4: Verifica (30 segundos)

```bash
cargo test test_get_saludo
```

**Salida**:
```
test test_get_saludo ... ok ✅
```

## 🎉 ¡Lo hiciste!

Repetiste el ciclo TDD:
1. 🔴 **RED**: Tests fallaban
2. 🟢 **GREEN**: Implementaste la solución
3. Tests pasaron ✅

## 📖 ¿Qué hacer ahora?

### Opción A: Continúa con el mismo Koan
```bash
# Resuelve el siguiente TODO
# test_incrementar_contador, test_tipos_numeros, etc.
cargo test
```

### Opción B: Lee la Documentación
- Abre [GUIA.md](GUIA.md) para tutorial completo
- Abre [COMO_FUNCIONA.md](COMO_FUNCIONA.md) para entender TDD

### Opción C: Ve al Siguiente Koan
```bash
cd ../02_about_ownership
cargo test
```

## 💡 Tips Importantes

### ❌ Errores Comunes

**Error**: `cannot assign twice to immutable variable`
```
fn incrementar_contador() -> i32 {
    let contador = 0;  // ← Inmutable
    contador += 1;     // ❌ Error
}
```

**Solución**: Usa `let mut`
```rust
fn incrementar_contador() -> i32 {
    let mut contador = 0;  // ✅ Mutable
    contador += 1;
    contador
}
```

### ✅ Workflow Efectivo

```bash
# 1. Lee el TODO en src/lib.rs
# 2. Implementa una solución
# 3. Ejecuta el test
cargo test test_nombre

# 4. Si falla, lee el error
# 5. Corrige y repite
```

### 🔍 Debugging

**Ver el error completo**:
```bash
cargo test -- --nocapture
```

**Ver información de compilador**:
```bash
cargo check
```

**Linting**:
```bash
cargo clippy
```

## 🎯 El Viaje Completo

```
01_about_variables     (Variables, tipos)
    ↓
02_about_ownership     (Ownership, borrowing)
    ↓
03_about_structs       (Datos complejos)
    ↓
04_about_traits        (Abstracción)
    ↓
05_about_errors        (Manejo de errores)
    ↓
06_about_collections   (Vec, HashMap)
    ↓
07_about_iterators     (Funcional)
    ↓
08_about_lifetimes     (¡Muy Rust!)
    ↓
09_about_concurrency   (Threads seguros)
    ↓
10_about_modules       (Organización)
    ↓
🦀 ¡Sabes Rust!
```

## 📚 Recursos Útiles

| Recurso | Para Qué |
|---------|----------|
| [GUIA.md](GUIA.md) | Tutorial completo paso a paso |
| [COMO_FUNCIONA.md](COMO_FUNCIONA.md) | Entiende la metodología TDD |
| [SOLUCIONES.md](SOLUCIONES.md) | Referencia (después de intentar) |
| [The Rust Book](https://doc.rust-lang.org/book/) | Profundización teórica |
| [Rust Playground](https://play.rust-lang.org/) | Experimenta online |

## ❓ FAQ Rápido

**P: ¿Cuánto tiempo toma?**
R: Completo → 20-40 horas (si vienes de Python)

**P: ¿Puedo ver soluciones?**
R: Sí, en SOLUCIONES.md, pero intenta primero

**P: ¿En qué orden?**
R: 01 → 02 → 03 → ... → 10 (el orden importa)

**P: ¿Qué pasa si me atasco?**
R: Lee el TODO cuidadosamente, revisa GUIA.md, consulta SOLUCIONES.md

**P: ¿Hay solución única?**
R: No, múltiples formas funcionan. La mejor es la que entiendes

---

**¡Ya estás listo para empezar!** Abre VS Code y comienza. 🚀

¿Preguntas? Lee [GUIA.md](GUIA.md) o abre una Issue en GitHub.
