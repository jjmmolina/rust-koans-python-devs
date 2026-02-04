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
- **Inmutabilidad por defecto**: `let x = 5` (no se puede reasignar)
- **Mutabilidad explícita**: `let mut x = 5` (se puede modificar)
- **Shadowing**: Redeclarar variable con `let` (¡puede cambiar tipo!)
- **Tipos numéricos**: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
- **Desbordamiento**: En debug mode, Rust entra en pánico con overflow (Python no)

### 02 - Ownership (El concepto más importante)
- **Cada valor tiene UN dueño único**: Cuando el dueño sale del scope, el valor se libera
- **Move semantics**: Asignar transfiere ownership (diferente de Python)
- **Borrowing inmutable**: `&T` permite múltiples lectores
- **Borrowing mutable**: `&mut T` permite UN escritor (sin lectores simultáneos)
- **String vs &str**: String es dueño (heap), &str es prestado (vista)
- **No dangling references**: El compilador garantiza que las referencias son válidas

### 03 - Structs & Enums
- **Structs**: Agrupan datos relacionados (como clases sin métodos)
- **impl blocks**: Definen métodos (separados de la definición de datos)
- **Tuple structs**: Structs sin nombres de campos
- **Enums**: Tipos con variantes (¡pueden contener datos!)
- **Pattern matching**: `match` para manejar todas las variantes
- **Option<T>**: Reemplazo seguro de null (Some/None)

### 04 - Traits
- **Traits**: Definen comportamiento compartido (interfaces)
- **Implementación**: `impl Trait for Type` (explícito, no duck typing)
- **Trait bounds**: `T: Display` restringe tipos genéricos
- **Derive**: `#[derive(Debug)]` genera código automáticamente
- **Display vs Debug**: Display para usuarios, Debug para programadores
### 05 - Errors
- **No excepciones**: Rust usa valores de retorno (Result/Option)
- **Option<T>**: Representa ausencia de valor (Some/None)
- **Result<T, E>**: Representa éxito (Ok) o error (Err)
- **Operador ?**: Propaga errores automáticamente (unwrap + return Err)
- **unwrap/expect**: Para prototipos (panic en error)
- **Métodos funcionales**: map, and_then, unwrap_or

### 06 - Collections
- **Vec<T>**: Lista dinámica en el heap (como list de Python pero tipada)
- **HashMap<K, V>**: Diccionario (como dict de Python)
- **String**: Collection de bytes UTF-8 (owned)
- **Diferencia clave**: Rust es homogéneo, Python permite listas mixtas

### 07 - Iterators
- **Lazy evaluation**: No se ejecuta hasta llamar a collect/sum/etc
- **Transformadores**: map, filter, take, skip
- **Consumidores**: collect, sum, fold, for_each
- **Closures**: Funciones anónimas `|args| expresión`
- **Captura de entorno**: `move` fuerza ownership al closure

### 08 - Lifetimes
- **Concepto único de Rust**: No existe en Python
- **Anotaciones**: `'a`, `'b` conectan tiempo de vida input-output
- **Prevención**: Evita dangling pointers en tiempo de compilación
- **Lifetime elision**: Rust infiere en casos comunes
- **'static**: Referencias que viven todo el programa (string literals)

### 09 - Concurrency
- **Sin GIL**: Threads reales (no como Python CPython)
- **Send/Sync traits**: El type system previene data races
- **Arc<T>**: Reference counting atómico (shared ownership)
- **Mutex<T>**: Mutual exclusion (lock protege datos)
- **Channels**: Message passing (mpsc - multiple producer, single consumer)

### 10 - Modules
- **Privado por defecto**: Contrario a Python (público por defecto)
- **pub**: Hace items públicos explícitamente
- **use**: Importa items (como `from module import`)
- **super/crate**: Navegación de módulos
- **Encapsulamiento real**: Los campos privados son inaccesibles

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

## ⚠️ Errores Comunes y Cómo Resolverlos

### Error: "not yet implemented" (panic at todo!())
**Causa**: No has reemplazado `todo!()`  
**Solución**: Lee las pistas en los comentarios e implementa la función

**Ejemplo**:
```rust
// ❌ Esto causa el error
pub fn saludo() -> String {
    todo!()
}

// ✅ Solución
pub fn saludo() -> String {
    String::from("Hola")
}
```

### Error: "cannot borrow as mutable"
**Causa**: Intentas modificar algo declarado como inmutable  
**Solución**: Usa `let mut` o cambia la referencia a `&mut`

**Ejemplo**:
```rust
// ❌ Error
let x = 5;
x += 1;  // cannot assign twice to immutable variable

// ✅ Solución
let mut x = 5;
x += 1;
```

### Error: "use of moved value"
**Causa**: Ownership fue transferido y ya no puedes usar la variable original  
**Solución**: Usa `.clone()` para duplicar o usa referencias `&`

**Ejemplo**:
```rust
// ❌ Error
let s1 = String::from("hello");
let s2 = s1;  // s1 moved aquí
println!("{}", s1);  // Error: s1 ya no es válido

// ✅ Solución 1: Clone
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{} {}", s1, s2);  // Ambos válidos

// ✅ Solución 2: Referencias
let s1 = String::from("hello");
let s2 = &s1;
println!("{} {}", s1, s2);  // s1 sigue siendo dueño
```

### Error: "cannot borrow `x` as mutable more than once"
**Causa**: Intentas tener múltiples referencias mutables simultáneas  
**Solución**: Solo usa una referencia mutable a la vez, o usa scopes

**Ejemplo**:
```rust
// ❌ Error
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;  // Error: ya existe &mut s

// ✅ Solución: Usa scopes
let mut s = String::from("hello");
{
    let r1 = &mut s;
    r1.push_str(" world");
}  // r1 sale del scope aquí
let r2 = &mut s;  // OK ahora
```

### Error: "missing lifetime specifier"
**Causa**: Rust no puede inferir cuánto viven las referencias  
**Solución**: Agrega anotaciones de lifetime explícitas

**Ejemplo**:
```rust
// ❌ Error
fn longest(x: &str, y: &str) -> &str {  // Missing lifetime
    if x.len() > y.len() { x } else { y }
}

// ✅ Solución
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

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
