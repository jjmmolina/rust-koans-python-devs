# Cheat Sheet - Rust vs Python 📚

Una referencia rápida para desarrolladores Python aprendiendo Rust.

## Variables y Mutabilidad

```python
# PYTHON
x = 5
x = 10  # ✅ Permitido
y = "texto"
```

```rust
// RUST
let x = 5;
x = 10;  // ❌ Error: cannot assign twice to immutable variable

// Solución: usar mut
let mut x = 5;
x = 10;  // ✅ OK

let y = "texto";  // &str
let mut s = String::from("texto");  // String
s.push_str("!");  // OK
```

## Funciones

```python
# PYTHON
def sumar(a, b):
    return a + b

def saludo(nombre="Juan"):
    return f"Hola {nombre}"
```

```rust
// RUST
fn sumar(a: i32, b: i32) -> i32 {
    a + b  // Sin ; retorna el valor
}

fn saludo(nombre: &str) -> String {
    format!("Hola {}", nombre)
}
```

## Tipos de Datos

| Tipo | Python | Rust |
|------|--------|------|
| Entero | `x = 42` | `let x: i32 = 42;` |
| Flotante | `x = 3.14` | `let x: f64 = 3.14;` |
| String | `x = "hola"` | `let x = "hola";` // &str |
| Booleano | `x = True` | `let x = true;` |
| Lista | `x = [1,2,3]` | `let x = vec![1,2,3];` |
| Tupla | `x = (1, "a")` | `let x = (1, "a");` |
| Diccionario | `x = {"a": 1}` | `HashMap::new()` |

## Control de Flujo

```python
# PYTHON
if x > 5:
    print("Mayor")
else:
    print("Menor")

for i in range(5):
    print(i)

while x > 0:
    x -= 1
```

```rust
// RUST
if x > 5 {
    println!("Mayor");
} else {
    println!("Menor");
}

for i in 0..5 {
    println!("{}", i);
}

while x > 0 {
    x -= 1;
}

// Match (pattern matching)
match x {
    1 => println!("uno"),
    2 => println!("dos"),
    _ => println!("otro"),
}
```

## Ownership (El Concepto Más Importante)

```python
# PYTHON: GC automático
lista = [1, 2, 3]
lista2 = lista  # Ambas apuntan a lo mismo
# Python se encarga de memory

lista3 = lista.copy()  # Copia explícita
```

```rust
// RUST: Ownership único
let s1 = String::from("Hola");
let s2 = s1;  // s1 se MUEVE a s2, s1 ya no es válido

let s3 = s1.clone();  // Copia explícita
println!("{}", s1);   // ❌ Error: s1 fue movido

// Referencias (borrowing)
let s4 = &s2;     // Préstamo inmutable
let s5 = &mut s2; // Préstamo mutable (solo uno a la vez)
```

## Estructuras

```python
# PYTHON
class Persona:
    def __init__(self, nombre, edad):
        self.nombre = nombre
        self.edad = edad
    
    def cumpleaños(self):
        self.edad += 1

p = Persona("Juan", 30)
p.cumpleaños()
```

```rust
// RUST
struct Persona {
    nombre: String,
    edad: u32,
}

impl Persona {
    fn nuevo(nombre: &str, edad: u32) -> Persona {
        Persona {
            nombre: nombre.to_string(),
            edad,
        }
    }
    
    fn cumpleaños(&mut self) {
        self.edad += 1;
    }
}

let mut p = Persona::nuevo("Juan", 30);
p.cumpleaños();
```

## Enums y Pattern Matching

```python
# PYTHON
status = "pendiente"
if status == "pendiente":
    print("Esperando")
elif status == "completado":
    print("¡Hecho!")
```

```rust
// RUST
enum Status {
    Pendiente,
    Completado,
    Error(String),
}

let status = Status::Pendiente;

match status {
    Status::Pendiente => println!("Esperando"),
    Status::Completado => println!("¡Hecho!"),
    Status::Error(msg) => println!("Error: {}", msg),
}
```

## Manejo de Errores

```python
# PYTHON
try:
    resultado = dividir(10, 2)
except ZeroDivisionError:
    print("División por cero")
```

```rust
// RUST - Option (puede ser Some o None)
fn obtener_primer(vec: &Vec<i32>) -> Option<i32> {
    if vec.is_empty() {
        None
    } else {
        Some(vec[0])
    }
}

match obtener_primer(&vec) {
    Some(val) => println!("Valor: {}", val),
    None => println!("Vacío"),
}

// Result (Ok o Err)
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("División por cero".to_string())
    } else {
        Ok(a / b)
    }
}

match dividir(10.0, 2.0) {
    Ok(resultado) => println!("Resultado: {}", resultado),
    Err(error) => println!("Error: {}", error),
}

// El operador ? (propaga errores)
fn calculo() -> Result<i32, String> {
    let a: i32 = "5".parse()?;  // Si hay error, retorna
    let b: i32 = "3".parse()?;
    Ok(a + b)
}
```

## Iteradores

```python
# PYTHON
numeros = [1, 2, 3, 4, 5]
dobles = [x * 2 for x in numeros]
pares = [x for x in numeros if x % 2 == 0]
suma = sum(numeros)
```

```rust
// RUST
let numeros = vec![1, 2, 3, 4, 5];

let dobles: Vec<i32> = numeros.iter()
    .map(|x| x * 2)
    .collect();

let pares: Vec<i32> = numeros.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|x| x.clone())
    .collect();

let suma: i32 = numeros.iter().sum();

// O más corto
let suma = numeros.iter().fold(0, |acc, x| acc + x);
```

## Closures

```python
# PYTHON
multiplicador = lambda x: x * 3
resultado = multiplicador(5)  # 15

funciones = [lambda x: x * 2, lambda x: x * 3]
```

```rust
// RUST
let multiplicador = |x| x * 3;
let resultado = multiplicador(5);  // 15

let mut funciones: Vec<Box<dyn Fn(i32) -> i32>> = vec![
    Box::new(|x| x * 2),
    Box::new(|x| x * 3),
];
```

## Traits (similar a interfaces)

```python
# PYTHON
class Animal:
    def hablar(self):
        raise NotImplementedError

class Perro(Animal):
    def hablar(self):
        return "Guau"
```

```rust
// RUST
trait Animal {
    fn hablar(&self) -> String;
}

struct Perro;
impl Animal for Perro {
    fn hablar(&self) -> String {
        "Guau".to_string()
    }
}

// Genéricos con traits
fn hacer_hablar<T: Animal>(animal: &T) {
    println!("{}", animal.hablar());
}
```

## Lifetimes (Concepto Único de Rust)

```rust
// Sin lifetime: ¿cuál referencia retornamos?
fn mas_largo(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
// ❌ Error: compiler doesn't know which lifetime

// Con lifetime: explícito
fn mas_largo<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
// ✅ OK: el resultado vive tanto como s1 y s2
```

## Concurrencia

```python
# PYTHON
import threading
def worker():
    print("Trabajando")

thread = threading.Thread(target=worker)
thread.start()
thread.join()
```

```rust
// RUST
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Trabajando");
    });
    
    handle.join().unwrap();
}

// Compartir datos con Arc<Mutex<T>>
use std::sync::{Arc, Mutex};

let contador = Arc::new(Mutex::new(0));
for _ in 0..5 {
    let contador = Arc::clone(&contador);
    thread::spawn(move || {
        let mut num = contador.lock().unwrap();
        *num += 1;
    });
}
```

## Módulos

```python
# PYTHON - modulos.py
def saludar():
    return "Hola"

# main.py
from modulos import saludar
print(saludar())
```

```rust
// RUST - modulos en el mismo archivo
mod saludar {
    pub fn decir_hola() -> String {
        "Hola".to_string()
    }
}

fn main() {
    println!("{}", saludar::decir_hola());
}

// O en otro archivo: src/saludar.rs
pub fn decir_hola() -> String {
    "Hola".to_string()
}

// En main.rs
mod saludar;
use saludar::decir_hola;
```

---

## 🎯 Resumen Conceptos Clave

| Concepto | Python | Rust |
|----------|--------|------|
| **Tipado** | Dinámico | Estático con inferencia |
| **Mutabilidad** | Por defecto sí | Por defecto no |
| **Ownership** | GC automático | Explícito |
| **Errores** | Excepciones | Result<T, E> |
| **None** | `None` | `Option<None>` |
| **Concurrencia** | GIL limitante | Sin GIL seguro |
| **Performance** | Interpretado | Compilado |
| **Memory** | Automático | Manual (seguro) |

---

**¡Usa este cheat sheet mientras completas los koans!** 🦀
