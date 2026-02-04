// Koan 04: Traits en Rust
//
// En Python: Polimorfismo implícito ("Duck Typing"). Si camina como pato...
// En Rust: Polimorfismo explícito. Debes declarar "Este Struct implementa el Trait Pato".
// Traits son como Interfaces en Java/C# o clases base abstractas en Python (ABC).

// PASO 1: Definir un Trait
// En Python:
// class Hablador(ABC):
//     @abstractmethod
//     def hablar(self): pass
// En Rust:
// trait Hablador { fn hablar(&self) -> String; }

// TODO: Define un trait Hablador con método hablar() -> String
pub trait Hablador {
    // TODO: fn hablar(&self) -> String;
}

// PASO 2: Implementar un Trait
pub struct Perro {
    pub nombre: String,
}

// TODO: Implementa Hablador para Perro
// impl Hablador for Perro {
//     fn hablar(&self) -> String {
//         format!("{} dice: Guau!", self.nombre)
//     }
// }

pub struct Gato {
    pub nombre: String,
}

// TODO: Implementa Hablador para Gato
// impl Hablador for Gato {
//     fn hablar(&self) -> String {
//         format!("{} dice: Miau!", self.nombre)
//     }
// }

// PASO 3: Trait Bounds
// TODO: Función que acepta cualquier tipo que implemente Hablador
pub fn hacer_hablar<T: Hablador>(animal: &T) -> String {
    todo!()
    // Hint: animal.hablar()
}

// PASO 4: Trait con Implementación por Defecto
pub trait Saludable {
    fn saludar(&self) -> String {
        String::from("Hola!")
    }
}

// TODO: Implementa Saludable para Persona (puede usar implementación por defecto)
pub struct Persona {
    pub nombre: String,
}

// impl Saludable for Persona {}  // Usa implementación por defecto
// O puedes sobrescribirla:
// impl Saludable for Persona {
//     fn saludar(&self) -> String {
//         format!("Hola, soy {}", self.nombre)
//     }
// }

// PASO 5: Generics
// TODO: Define una función genérica que retorne el mayor de dos valores
pub fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    todo!()
    // Hint: if a > b { a } else { b }
}

// PASO 6: Trait std::fmt::Display
use std::fmt;

pub struct Punto {
    pub x: i32,
    pub y: i32,
}

// TODO: Implementa Display para Punto
// impl fmt::Display for Punto {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// PASO 7: Trait Debug (Auto-generación de código)
// En Python: __repr__ suele ser automático o fácil de añadir.
// En Rust: #[derive(Debug)] genera automáticamente el código para imprimir usando {:?}.
// Es comparable a usar @dataclass en Python para obtener __repr__ gratis.

// TODO: Deriva Debug para Rectangulo
// #[derive(Debug)]
pub struct Rectangulo {
    pub ancho: u32,
    pub alto: u32,
}

// PASO 8: Múltiples Trait Bounds
// TODO: Función con múltiples trait bounds
pub fn imprimir_y_comparar<T>(a: &T, b: &T) -> bool
where
    T: fmt::Display + PartialEq,
{
    todo!()
    // Hint: println!("{} vs {}", a, b); a == b
}

// PASO 9: Associated Types
pub trait Contenedor {
    type Item;
    fn agregar(&mut self, item: Self::Item);
    fn obtener(&self) -> Option<&Self::Item>;
}

// TODO: Implementa Contenedor para Caja
pub struct Caja {
    pub contenido: Option<String>,
}

// impl Contenedor for Caja {
//     type Item = String;
//     fn agregar(&mut self, item: String) {
//         self.contenido = Some(item);
//     }
//     fn obtener(&self) -> Option<&String> {
//         self.contenido.as_ref()
//     }
// }

// PASO 10: Trait Objects
// TODO: Retorna un vector de trait objects
pub fn crear_animales() -> Vec<Box<dyn Hablador>> {
    todo!()
    // Hint:
    // vec![
    //     Box::new(Perro { nombre: String::from("Rex") }),
    //     Box::new(Gato { nombre: String::from("Misu") }),
    // ]
}
