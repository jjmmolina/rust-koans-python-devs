# Guía de Soluciones - Rust Koans 🎯

Esta guía contiene explicaciones detalladas y soluciones para todos los koans.

**⚠️ ADVERTENCIA**: Lee esto DESPUÉS de intentar resolver cada koan por tu cuenta. Ver las soluciones primero reduce significativamente el aprendizaje.

---

## 01 - About Variables 📝

### Ejercicio 1.1: Variables Inmutables

**Concepto**: En Rust, las variables son inmutables por defecto (a diferencia de Python).

```python
# PYTHON: por defecto es mutable
nombre = "Juan"
nombre = "Pedro"  # ✅ Permitido
```

```rust
// RUST: por defecto es inmutable
let nombre = "Juan";
nombre = "Pedro";  // ❌ Error: cannot assign twice to immutable variable
```

**Solución**:
```rust
pub fn get_saludo() -> &'static str {
    "Hola Rust"
}
```

**Concepto Clave**: `&'static str` es una referencia a una cadena literal que vive durante todo el programa.

---

### Ejercicio 1.2: Variables Mutables

**Concepto**: Necesitas usar `let mut` para permitir mutabilidad.

```python
# PYTHON
contador = 0
contador += 1  # ✅ Permitido

# RUST sin mut
let contador = 0;
contador += 1;  // ❌ Error
```

**Solución**:
```rust
pub fn incrementar_contador() -> i32 {
    let mut contador = 0;
    contador += 1;
    contador
}
```

**Concepto Clave**: La mutabilidad es **explícita** en Rust. Esto previene cambios accidentales.

---

### Ejercicio 1.3: Tipos Explícitos

**Concepto**: Rust infiere tipos, pero puedes ser explícito.

```python
# PYTHON: tipo dinámico
x = 5      # int
x = "hola" # str - ¡cambiar tipo está permitido!
```

```rust
// RUST: tipado estático
let x: i32 = 5;
let y: &str = "hola";
// No puedes hacer: x = "hola";  ❌ Tipo incorrecto
```

**Solución**:
```rust
pub fn tipos_numeros() -> (i32, f64) {
    let entero: i32 = 42;
    let flotante: f64 = 3.14;
    (entero, flotante)
}
```

**Concepto Clave**: Los tipos principales:
- `i32`, `i64`, `u32`, `u64` - Enteros con/sin signo
- `f32`, `f64` - Flotantes
- `bool` - Booleanos (true/false)
- `char` - Carácter único
- `&str` - Referencia a cadena (inmutable)
- `String` - Cadena mutable (en heap)

---

### Ejercicio 1.4: Shadowing

**Concepto**: Puedes redeclarar una variable con `let` (shadowing).

```python
# PYTHON: reasignación
x = 5
x = "Hola"  # x es ahora string

# RUST: shadowing (más seguro)
let x = 5;
let x = "Hola";  // ✅ Nueva variable shadea la anterior
```

**Solución**:
```rust
pub fn shadowing_ejemplo() -> String {
    let x = 5;
    let x = x + 1;      // shadowing: x = 6
    let x = x * 2;      // shadowing: x = 12
    let x = format!("{}", x);  // shadowing: x = "12"
    x
}
```

**Concepto Clave**: Shadowing es diferente a reasignación. Crea una nueva variable.

---

### Ejercicio 1.5-1.10: Arrays y Tuplas

**Arrays** - Tamaño fijo, mismo tipo:
```rust
let numeros: [i32; 3] = [1, 2, 3];  // array de 3 i32
let primero = numeros[0];           // 1
```

**Tuplas** - Tamaño fijo, tipos diferentes:
```rust
let persona: (&str, i32) = ("Juan", 30);
let nombre = persona.0;  // "Juan"
let edad = persona.1;    // 30

// O con destructuring:
let (nombre, edad) = persona;
```

---

## 02 - About Ownership 🎯

### Ejercicio 2.1-2.3: Move Semantics

**Concepto**: En Rust, solo hay UN dueño de un valor.

```python
# PYTHON: GC automático
lista = [1, 2, 3]
lista2 = lista      # Ambas referencias apuntan a lo mismo
# Python las garbage-collect automáticamente
```

```rust
// RUST: Ownership
let s1 = String::from("Hola");
let s2 = s1;        // s1 se MUEVE a s2
println!("{}", s1); // ❌ Error: s1 ya no es válido
println!("{}", s2); // ✅ OK: s2 es el dueño
```

**Solución**:
```rust
pub fn clonar_string(s: &String) -> String {
    s.clone()  // Crea una copia
}
```

**Concepto Clave**: `clone()` es explícito. No hay copias implícitas.

---

### Ejercicio 2.4-2.6: Borrowing & References

**Concepto**: Puedes prestar un valor sin transferir ownership.

```rust
// Referencia inmutable (&T)
let s = String::from("Hola");
let len = obtener_longitud(&s);  // Prestamos s
println!("{}", s);               // ✅ s sigue siendo válido

fn obtener_longitud(s: &String) -> usize {
    s.len()  // Leemos pero no tomamos ownership
}

// Referencia mutable (&mut T)
let mut s = String::from("Hola");
agregar_exclamacion(&mut s);  // Prestamos mutuablemente
println!("{}", s);            // "Hola!"

fn agregar_exclamacion(s: &mut String) {
    s.push_str("!");  // Modificamos
}
```

**Regla Importante**:
- ✅ Muchas referencias inmutables: `&T` (todos pueden leer)
- ✅ Una referencia mutable: `&mut T` (solo uno puede escribir)
- ❌ Nunca ambas simultáneamente

---

### Ejercicio 2.7-2.10: Slices

**Concepto**: Vista a parte de un array/String sin transferir ownership.

```rust
let s = String::from("Hola Mundo");
let hola = &s[0..4];      // "Hola"
let mundo = &s[5..10];    // "Mundo"

// Para strings es más común usar &str:
let s = "Hola";           // &str literal
let s: &str = &String::from("Hola");
```

---

## 03 - About Structs 🏗️

### Ejercicio 3.1-3.3: Defining Structs

**Concepto**: Grupos de datos relacionados.

```python
# PYTHON: diccionario o clase
persona = {"nombre": "Juan", "edad": 30}
# o
class Persona:
    def __init__(self, nombre, edad):
        self.nombre = nombre
        self.edad = edad
```

```rust
// RUST: struct
struct Persona {
    nombre: String,
    edad: u32,
}

let juan = Persona {
    nombre: String::from("Juan"),
    edad: 30,
};
```

**Solución**:
```rust
struct Persona {
    nombre: String,
    edad: u32,
    email: String,
}

pub fn nueva_persona(nombre: &str, edad: u32) -> Persona {
    Persona {
        nombre: nombre.to_string(),
        edad,
        email: format!("{}@example.com", nombre.to_lowercase()),
    }
}
```

---

### Ejercicio 3.4-3.6: Enums

**Concepto**: Tipo que puede ser una de varias variantes.

```python
# PYTHON: no tiene enum nativo
class Estado:
    PENDIENTE = 1
    COMPLETADO = 2
    CANCELADO = 3
```

```rust
// RUST: enum
enum Estado {
    Pendiente,
    Completado,
    Cancelado,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}
```

---

### Ejercicio 3.7-3.10: Pattern Matching

**Concepto**: Descomponer y comparar valores.

```rust
let estado = Estado::Completado;

match estado {
    Estado::Pendiente => println!("Esperando..."),
    Estado::Completado => println!("¡Hecho!"),
    Estado::Cancelado => println!("Cancelado"),
}

// Con Option:
let valor: Option<i32> = Some(5);
match valor {
    Some(x) => println!("Valor: {}", x),
    None => println!("No hay valor"),
}

// if let (más corto):
if let Some(x) = valor {
    println!("Valor: {}", x);
}
```

---

## 04 - About Traits 🎯

### Concepto General: Polimorfismo

**Traits** = Interfaces que definen comportamiento.

```python
# PYTHON: duck typing
class Perro:
    def hablar(self):
        return "Guau"

class Gato:
    def hablar(self):
        return "Miau"

def hacer_hablar(animal):
    print(animal.hablar())
```

```rust
// RUST: traits explícitos
trait Hablador {
    fn hablar(&self) -> String;
}

struct Perro(String);
impl Hablador for Perro {
    fn hablar(&self) -> String {
        format!("{} dice: Guau!", self.0)
    }
}

// Genericidad
fn hacer_hablar<T: Hablador>(t: &T) {
    println!("{}", t.hablar());
}

// O trait objects
fn hacer_hablar_dinamico(t: &dyn Hablador) {
    println!("{}", t.hablar());
}
```

---

## 05 - About Errors ⚠️

### Option<T> y Result<T, E>

**Option** - Valor presente o no:
```rust
enum Option<T> {
    Some(T),
    None,
}

let valor = Some(5);
match valor {
    Some(x) => println!("Tengo {}", x),
    None => println!("Sin valor"),
}
```

**Result** - Éxito o error:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("División por cero".to_string())
    } else {
        Ok(a / b)
    }
}
```

**El operador `?`** - Propaga errores:
```rust
fn leer_y_sumar(s1: &str, s2: &str) -> Result<i32, std::num::ParseIntError> {
    let n1: i32 = s1.parse()?;  // Si error, retorna
    let n2: i32 = s2.parse()?;  // Si error, retorna
    Ok(n1 + n2)
}
```

---

## 06 - About Collections 📦

### Vec<T> - Vector dinámico:
```rust
let mut vec = vec![1, 2, 3];
vec.push(4);
let primer = vec[0];        // 1
let ultimo = vec.last();    // Some(&4)
```

### HashMap<K, V> - Mapa clave-valor:
```rust
use std::collections::HashMap;

let mut mapa = HashMap::new();
mapa.insert("nombre", "Juan");
let valor = mapa.get("nombre");  // Some(&"Juan")
```

### String vs &str:
```rust
let literal: &str = "Hola";           // Stack, inmutable, tamaño conocido
let dinamica: String = String::from("Mundo");  // Heap, mutable
```

---

## 07 - About Iterators 🔄

### Iterators y Closures:

```rust
let numeros = vec![1, 2, 3, 4, 5];

// map: transformar cada elemento
let dobles: Vec<i32> = numeros.iter()
    .map(|x| x * 2)
    .collect();

// filter: seleccionar elementos
let pares: Vec<i32> = numeros.iter()
    .filter(|&&x| x % 2 == 0)
    .collect();

// fold: acumular
let suma: i32 = numeros.iter()
    .fold(0, |acc, x| acc + x);
```

---

## 08 - About Lifetimes ⏱️

### Lifetimes - "¿Por cuánto tiempo vive esta referencia?"

```rust
// Función que retorna referencia a uno de sus parámetros
fn mas_largo<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Struct que contiene referencias
struct Extracto<'a> {
    texto_completo: &'a str,
    parte: &'a str,
}

impl<'a> Extracto<'a> {
    fn nuevo(texto: &'a str, parte: &'a str) -> Extracto<'a> {
        Extracto { texto_completo: texto, parte }
    }
}
```

---

## 09 - About Concurrency 🔄

### Threads:
```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Desde el thread!");
});

handle.join().unwrap();
```

### Mutex (Mutual Exclusion):
```rust
use std::sync::Mutex;

let contador = Mutex::new(0);
{
    let mut num = contador.lock().unwrap();
    *num += 1;
}
```

### Arc (Atomic Reference Counting):
```rust
use std::sync::Arc;

let dato = Arc::new(vec![1, 2, 3]);
for _ in 0..3 {
    let dato_clone = Arc::clone(&dato);
    thread::spawn(move || {
        println!("{:?}", dato_clone);
    });
}
```

---

## 10 - About Modules 📦

### Módulos - Organización de código:

```rust
mod geometria {
    pub mod circulo {
        pub fn area(radio: f64) -> f64 {
            std::f64::consts::PI * radio * radio
        }
    }
    
    pub mod rectangulo {
        pub fn area(ancho: f64, alto: f64) -> f64 {
            ancho * alto
        }
    }
}

pub use geometria::circulo;  // Re-export
```

### Visibilidad:
```rust
pub fn publico() {}      // Accesible desde fuera
fn privado() {}          // Solo en el módulo

pub struct Cuenta {
    pub nombre: String,
    saldo: f64,          // privado
}
```

---

## 🎓 Conceptos Transversales

### Ownership - Las 3 Reglas:

1. Cada valor tiene UN dueño
2. Cuando el dueño sale del scope, el valor se droppea
3. Puedes prestar el valor mediante referencias

### Trait Bounds - Restricciones genéricas:

```rust
fn imprime<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}

fn clona<T: Clone>(t: &T) -> T {
    t.clone()
}
```

### Error Handling - Patrones comunes:

```rust
// unwrap - Paniquea si es None/Err
let valor = opcion.unwrap();

// unwrap_or - Valor por defecto
let valor = opcion.unwrap_or(0);

// ? operator - Propaga errores
let valor = opcion?;

// map - Transformar
let valor = opcion.map(|x| x * 2);
```

---

## 🏆 Siguientes Pasos

Ahora que entiendes el "por qué", puedes:

1. Explorar la [Rust Book](https://doc.rust-lang.org/book/)
2. Resolver [Rustlings](https://github.com/rust-lang/rustlings)
3. Crear un proyecto real
4. Leer código Rust en GitHub
5. Contribuir a proyectos open source

**¡Felicidades por completar los Rust Koans!** 🦀🎉
