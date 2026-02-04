# Mejores Prácticas en Rust 🦀

Guía de buenas prácticas para desarrolladores Python aprendiendo Rust.

## 📚 Índice

1. [Ownership y Borrowing](#ownership-y-borrowing)
2. [Manejo de Errores](#manejo-de-errores)
3. [Tipos y Conversiones](#tipos-y-conversiones)
4. [Colecciones](#colecciones)
5. [Iteradores](#iteradores)
6. [Traits](#traits)
7. [Concurrencia](#concurrencia)
8. [Testing](#testing)

## Ownership y Borrowing

### ✅ Preferir Referencias sobre Clones

```rust
// ❌ Evita clonar innecesariamente
fn procesar(texto: String) -> usize {
    texto.len()
}

let s = String::from("hola");
let len = procesar(s.clone());  // Clone costoso

// ✅ Usa referencias cuando no necesites ownership
fn procesar(texto: &str) -> usize {
    texto.len()
}

let s = String::from("hola");
let len = procesar(&s);  // Más eficiente
```

### ✅ Usa &str en Firmas de Funciones

```rust
// ❌ Menos flexible
fn saludar(nombre: String) -> String {
    format!("Hola, {}", nombre)
}

// ✅ Más flexible (acepta &str, String, etc.)
fn saludar(nombre: &str) -> String {
    format!("Hola, {}", nombre)
}
```

### ✅ Retorna Owned Types, Acepta Borrowed

```rust
// ✅ Patrón común y eficiente
fn procesar(input: &str) -> String {
    input.to_uppercase()
}
```

## Manejo de Errores

### ✅ Usa ? en lugar de unwrap() en Producción

```rust
// ❌ Evita en producción
fn leer_archivo(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()  // Panic si falla
}

// ✅ Propaga errores con ?
fn leer_archivo(path: &str) -> Result<String, std::io::Error> {
    let contenido = std::fs::read_to_string(path)?;
    Ok(contenido)
}
```

### ✅ Crea Errores Personalizados para tu Dominio

```rust
// ✅ Error específico del dominio
#[derive(Debug)]
pub enum MiError {
    ArchivoNoEncontrado(String),
    FormatoInvalido,
    PermisoDenegado,
}

impl std::fmt::Display for MiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MiError::ArchivoNoEncontrado(path) => 
                write!(f, "Archivo no encontrado: {}", path),
            MiError::FormatoInvalido => 
                write!(f, "Formato de archivo inválido"),
            MiError::PermisoDenegado => 
                write!(f, "Permiso denegado"),
        }
    }
}

impl std::error::Error for MiError {}
```

### ✅ Usa unwrap_or/unwrap_or_else para Valores por Defecto

```rust
// ❌ Panic si None
let valor = opcion.unwrap();

// ✅ Valor por defecto
let valor = opcion.unwrap_or(0);

// ✅ Valor por defecto computado
let valor = opcion.unwrap_or_else(|| calcular_default());
```

## Tipos y Conversiones

### ✅ Usa From/Into para Conversiones

```rust
// ✅ Implementa From, obtienes Into gratis
impl From<i32> for MiTipo {
    fn from(valor: i32) -> Self {
        MiTipo { campo: valor }
    }
}

let x: MiTipo = 42.into();  // Into gratis
let y = MiTipo::from(42);   // From explícito
```

### ✅ Usa Newtype Pattern para Tipos Seguros

```rust
// ❌ Fácil confundir parámetros
fn crear_usuario(id: u64, edad: u64) { /* ... */ }
crear_usuario(25, 12345);  // Oops, al revés

// ✅ Newtype previene errores
struct UserId(u64);
struct Edad(u64);

fn crear_usuario(id: UserId, edad: Edad) { /* ... */ }
crear_usuario(Edad(25), UserId(12345));  // Error de compilación
```

## Colecciones

### ✅ Reserva Capacidad si Conoces el Tamaño

```rust
// ❌ Múltiples realocaciones
let mut v = Vec::new();
for i in 0..1000 {
    v.push(i);
}

// ✅ Una sola alocación
let mut v = Vec::with_capacity(1000);
for i in 0..1000 {
    v.push(i);
}
```

### ✅ Usa entry() API con HashMap

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

// ❌ Doble búsqueda
if !map.contains_key(&key) {
    map.insert(key, valor);
}

// ✅ Una sola búsqueda con entry()
map.entry(key).or_insert(valor);

// ✅ Modificar o insertar
map.entry(key)
    .and_modify(|v| *v += 1)
    .or_insert(1);
```

## Iteradores

### ✅ Usa Iteradores en lugar de Loops Indexados

```rust
// ❌ Estilo imperativo
let mut suma = 0;
for i in 0..vec.len() {
    suma += vec[i];
}

// ✅ Estilo funcional con iteradores
let suma: i32 = vec.iter().sum();

// ✅ Map y filter
let resultado: Vec<_> = vec
    .iter()
    .filter(|&&x| x > 0)
    .map(|x| x * 2)
    .collect();
```

### ✅ Usa iter() vs into_iter() Apropiadamente

```rust
let vec = vec![1, 2, 3];

// iter() - referencias inmutables (&T)
for item in vec.iter() {
    println!("{}", item);
}
// vec sigue siendo válido

// into_iter() - consume (T)
for item in vec.into_iter() {
    println!("{}", item);
}
// vec ya no es válido
```

## Traits

### ✅ Deriva Traits Comunes

```rust
// ✅ Deriva automáticamente cuando sea posible
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Usuario {
    pub nombre: String,
    pub edad: u32,
}
```

### ✅ Usa Trait Bounds Claros

```rust
// ❌ Confuso
fn procesar<T>(item: T) where T: Clone + Display + Debug { }

// ✅ Más claro con trait bounds inline (para pocos)
fn procesar<T: Clone + Display + Debug>(item: T) { }

// ✅ Más claro con where clause (para muchos)
fn procesar<T>(item: T) 
where 
    T: Clone + Display + Debug 
{ }
```

## Concurrencia

### ✅ Prefiere Message Passing sobre Estado Compartido

```rust
use std::sync::mpsc;
use std::thread;

// ✅ Channels (message passing)
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(42).unwrap();
});

let valor = rx.recv().unwrap();
```

### ✅ Usa Arc + Mutex Solo Cuando Sea Necesario

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// ✅ Arc<Mutex<T>> para estado compartido mutable
let contador = Arc::new(Mutex::new(0));

let handles: Vec<_> = (0..10)
    .map(|_| {
        let contador = Arc::clone(&contador);
        thread::spawn(move || {
            let mut num = contador.lock().unwrap();
            *num += 1;
        })
    })
    .collect();

for handle in handles {
    handle.join().unwrap();
}
```

## Testing

### ✅ Organiza Tests en Módulo

```rust
// ✅ Tests en módulo separado
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suma() {
        assert_eq!(suma(2, 2), 4);
    }

    #[test]
    #[should_panic(expected = "división por cero")]
    fn test_division_cero() {
        dividir(10, 0);
    }
}
```

### ✅ Usa assert! Apropiado

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn pruebas() {
        // ✅ Para igualdad
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 2, 5);

        // ✅ Para booleanos
        assert!(es_par(4));
        assert!(!es_par(3));

        // ✅ Con mensajes
        assert_eq!(
            resultado, 
            esperado, 
            "Resultado {} no coincide con esperado {}", 
            resultado, 
            esperado
        );
    }
}
```

## Consejos Generales

### ✅ Escucha al Compilador

El compilador de Rust es tu mejor maestro. Lee los mensajes de error cuidadosamente:

```
error[E0382]: borrow of moved value: `s`
  --> src/main.rs:5:20
   |
3  |     let s = String::from("hello");
   |         - move occurs because `s` has type `String`
4  |     let s2 = s;
   |              - value moved here
5  |     println!("{}", s);
   |                    ^ value borrowed here after move
```

### ✅ Usa Clippy

```bash
cargo clippy --all-targets --all-features
```

Clippy te sugerirá mejoras idiomáticas.

### ✅ Formatea con rustfmt

```bash
cargo fmt
```

Mantén el código consistente.

### ✅ Lee el Código de Crates Populares

Aprende de proyectos bien escritos:
- [serde](https://github.com/serde-rs/serde) - Serialización
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime
- [clap](https://github.com/clap-rs/clap) - CLI parsing

## Comparación con Python

| Práctica | Python | Rust |
|----------|--------|------|
| **Mutabilidad** | Por defecto mutable | Inmutable por defecto |
| **Tipos** | Duck typing | Tipos explícitos |
| **Errores** | Excepciones | Result/Option |
| **Iteración** | `for x in lista` | `for x in lista.iter()` |
| **Valores None** | `if x is None:` | `if x.is_none()` / `match` |
| **Clonación** | Automática (shallow) | Explícita con `.clone()` |

---

**Recuerda**: Rust te obliga a pensar en detalles que Python oculta. Esto no es malo, te hace un mejor programador. 🚀
