# Guía de Uso - Rust Koans 🦀

Esta guía te ayudará a navegar por los Rust Koans y aprovecharlos al máximo.

## 🎯 Filosofía de Aprendizaje

**Estos koans NO contienen las soluciones.** En su lugar encontrarás:
- ✅ Explicaciones de **cómo funciona Rust** comparado con Python
- ✅ **Hints y pistas** sobre qué hacer
- ✅ **TODOs claros** que indican qué implementar
- ✅ **Tests** que describen el comportamiento esperado

**Tu trabajo es completar los ejercicios** usando las pistas provistas.

## 🚀 Orden Recomendado

1. **01_about_variables** - Variables, tipos, mutabilidad, shadowing
2. **02_about_ownership** - Ownership, borrowing, referencias
3. **03_about_structs** - Structs, enums, pattern matching
4. **04_about_traits** - Traits, generics, trait bounds
5. **05_about_errors** - Option, Result, manejo de errores
6. **06_about_collections** - Vec, HashMap, String
7. **07_about_iterators** - Iteradores, closures, map/filter
8. **08_about_lifetimes** - Lifetimes, anotaciones
9. **09_about_concurrency** - Threads, Mutex, Arc, channels
10. **10_about_modules** - Módulos, visibilidad, organización

## 🔥 Ejemplo Práctico Paso a Paso

### Paso 1: Navega al primer koan
```bash
cd 01_about_variables
```

### Paso 2: Ejecuta los tests (fallarán - esto es esperado)
```bash
cargo test
```

Verás errores como:
```
thread 'test_get_saludo' panicked at 'not yet implemented'
```

### Paso 3: Abre `src/lib.rs` y lee el código
Verás:
```rust
// PASO 1: Variables Inmutables
// En Python: x = 5
// En Rust: let x = 5; (inmutable por defecto)

// TODO: Crea una variable inmutable 'saludo' con valor "Hola Rust"
pub fn get_saludo() -> &'static str {
    todo!() // Reemplaza con: let saludo = "Hola Rust"; saludo
}
```

### Paso 4: Implementa la solución
```rust
pub fn get_saludo() -> &'static str {
    let saludo = "Hola Rust";
    saludo
}
```

### Paso 5: Ejecuta los tests nuevamente
```bash
cargo test
```

Si pasa 🟢:
```
test test_get_saludo ... ok
```

### Paso 6: Continúa con el siguiente TODO

Repite hasta completar todos los tests del koan.

### Paso 7: Pasa al siguiente koan
```bash
cd ../02_about_ownership
cargo test
```

## 💡 Conceptos Clave por Koan

### 01 - Variables
- Inmutabilidad por defecto: `let x = 5`
- Mutabilidad explícita: `let mut x = 5`
- Shadowing: redeclarar variable con `let`
- Tipos: i32, u8, f64, bool, etc.

### 02 - Ownership
- Cada valor tiene un dueño único
- Move semant ics: asignar transfiere ownership
- Borrowing: `&T` (inmutable) y `&mut T` (mutable)
- No puede haber referencias mutables e inmutables simultáneas

### 03 - Structs & Enums
- Structs para agrupar datos
- Enums para variantes
- Pattern matching con `match`
- Option<T> y Result<T, E>

### 04 - Traits
- Similar a interfaces en otros lenguajes
- Implementación explícita: `impl Trait for Type`
- Generics: `fn funcion<T>(x: T)`
- Trait bounds: `T: Display + Clone`

### 05 - Errors
- Option<T>: Some(T) o None
- Result<T, E>: Ok(T) o Err(E)
- Operador ?: propaga errores
- unwrap(), expect(), unwrap_or()

### 06 - Collections
- Vec<T>: vector dinámico
- HashMap<K, V>: mapa clave-valor
- String vs &str
- push(), get(), insert()

### 07 - Iterators
- Lazy evaluation
- map, filter, fold
- Closures: |x| x * 2
- collect() para materializar

### 08 - Lifetimes
- Anotaciones: 'a, 'b
- Garantizan validez de referencias
- Lifetime elision (inferencia)
- 'static para datos que viven todo el programa

### 09 - Concurrency
- thread::spawn para crear threads
- Arc<T> para compartir datos inmutables
- Mutex<T> para compartir datos mutables
- Channels (mpsc) para comunicación

### 10 - Modules
- mod para definir módulos
- pub para exportar
- use para importar
- Visibilidad: pub vs privado

## ⚠️ Errores Comunes

### Error: "not yet implemented"
**Causa**: No has reemplazado `todo!()`  
**Solución**: Implementa la función según las pistas

### Error: "cannot borrow as mutable"
**Causa**: Intentas modificar algo inmutable  
**Solución**: Usa `let mut` o `&mut`

### Error: "use of moved value"
**Causa**: Ownership fue transferido  
**Solución**: Usa `.clone()` o referencias `&`

### Error: "lifetime parameter"
**Causa**: Rust no puede inferir lifetimes  
**Solución**: Agrega anotaciones de lifetime

## 🛠️ Comandos Útiles

```bash
# Ejecutar todos los tests
cargo test

# Ejecutar tests de un koan
cd 01_about_variables
cargo test

# Ejecutar un test específico
cargo test test_get_saludo

# Ver output detallado
cargo test -- --nocapture

# Verificar sin ejecutar
cargo check

# Formatear código
cargo fmt

# Linter
cargo clippy
```

## 📚 Comparación Python vs Rust

| Concepto | Python | Rust |
|----------|--------|------|
| **Mutabilidad** | Por defecto | `let mut` |
| **Tipos** | Dinámicos | Estáticos con inferencia |
| **Memoria** | GC automático | Ownership + RAII |
| **None** | `None` | `Option::None` |
| **Errores** | `try/except` | `Result<T, E>` |
| **Concurrencia** | GIL limita | Sin GIL, seguro en compile-time |

## 🎓 Recursos Adicionales

- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustlings**: https://github.com/rust-lang/rustlings
- **Rust Playground**: https://play.rust-lang.org/

## 🚀 Próximos Pasos

Una vez completes todos los koans:

1. **Construye un proyecto real** (CLI tool, web server, etc.)
2. **Lee The Rust Book** para profundizar conceptos
3. **Practica con Rustlings** para más ejercicios
4. **Explora crates populares** (tokio, serde, clap, etc.)
5. **Contribuye a proyectos open source** en Rust

---

**¡Disfruta tu viaje aprendiendo Rust!** 🦀
