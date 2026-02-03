// Koan 03: Structs, Enums y Pattern Matching
//
// En Python usas clases para agrupar datos.
// En Rust usas structs (datos) y enums (variantes).

// PASO 1: Struct Básico
// En Python: class Persona: def __init__(self, nombre, edad): ...
// En Rust: struct Persona { nombre: String, edad: u32 }

// TODO: Define un struct Persona con campos nombre (String) y edad (u32)
pub struct Persona {
    // TODO: Agrega campos aquí
    // pub nombre: String,
    // pub edad: u32,
}

// TODO: Implementa una función constructora
pub fn nueva_persona(nombre: String, edad: u32) -> Persona {
    todo!()
    // Hint: Persona { nombre, edad }
}

// PASO 2: Métodos en Structs
// En Python: def saludar(self): return f"Hola, soy {self.nombre}"
// En Rust: impl Persona { fn saludar(&self) -> String { ... } }

impl Persona {
    // TODO: Implementa un método saludar que retorne "Hola, soy [nombre]"
    pub fn saludar(&self) -> String {
        todo!()
        // Hint: format!("Hola, soy {}", self.nombre)
    }

    // TODO: Implementa un método cumplir_años que incremente la edad
    pub fn cumplir_años(&mut self) {
        todo!()
        // Hint: self.edad += 1;
    }
}

// PASO 3: Tuple Structs
// Structs sin nombres de campos

// TODO: Define un tuple struct Punto3D con tres f64
pub struct Punto3D(/* TODO: pub f64, pub f64, pub f64 */);

pub fn crear_punto(x: f64, y: f64, z: f64) -> Punto3D {
    todo!()
    // Hint: Punto3D(x, y, z)
}

// PASO 4: Enums
// En Python: usarías clases o constantes
// En Rust: enum es un tipo algebraico

// TODO: Define un enum Direccion con variantes: Norte, Sur, Este, Oeste
pub enum Direccion {
    // TODO: Agrega variantes
}

// TODO: Retorna la variante Norte
pub fn ir_al_norte() -> Direccion {
    todo!()
    // Hint: Direccion::Norte
}

// PASO 5: Enums con Datos
// Cada variante puede contener datos diferentes

// TODO: Define un enum Mensaje con variantes:
// - Quit (sin datos)
// - Mover { x: i32, y: i32 }
// - Escribir(String)
pub enum Mensaje {
    // TODO: Agrega variantes
}

pub fn crear_mensaje_mover(x: i32, y: i32) -> Mensaje {
    todo!()
    // Hint: Mensaje::Mover { x, y }
}

// PASO 6: Pattern Matching
// En Python: if isinstance(x, Tipo): ...
// En Rust: match x { patrón => resultado }

// TODO: Usa match para procesar un Mensaje
pub fn procesar_mensaje(msg: Mensaje) -> String {
    todo!()
    // Hint:
    // match msg {
    //     Mensaje::Quit => String::from("Saliendo"),
    //     Mensaje::Mover { x, y } => format!("Mover a ({}, {})", x, y),
    //     Mensaje::Escribir(texto) => format!("Texto: {}", texto),
    // }
}

// PASO 7: Option<T>
// En Python: None o valor
// En Rust: Option::Some(valor) o Option::None

// TODO: Retorna Some(42)
pub fn retornar_some() -> Option<i32> {
    todo!()
    // Hint: Some(42)
}

// TODO: Retorna None
pub fn retornar_none() -> Option<i32> {
    todo!()
    // Hint: None
}

// TODO: Usa match para extraer el valor de un Option
pub fn extraer_option(opt: Option<i32>) -> i32 {
    todo!()
    // Hint:
    // match opt {
    //     Some(valor) => valor,
    //     None => 0,
    // }
}

// PASO 8: if let
// Atajo para match con un solo patrón

// TODO: Usa if let para extraer el valor
pub fn usar_if_let(opt: Option<i32>) -> String {
    todo!()
    // Hint:
    // if let Some(valor) = opt {
    //     format!("Valor: {}", valor)
    // } else {
    //     String::from("Sin valor")
    // }
}

// PASO 9: Métodos en Enums
impl Direccion {
    // TODO: Implementa un método que retorne un String con el nombre de la dirección
    pub fn como_string(&self) -> &str {
        todo!()
        // Hint:
        // match self {
        //     Direccion::Norte => "Norte",
        //     Direccion::Sur => "Sur",
        //     Direccion::Este => "Este",
        //     Direccion::Oeste => "Oeste",
        // }
    }
}

// PASO 10: Struct Update Syntax
pub struct Configuracion {
    pub ancho: u32,
    pub alto: u32,
    pub activo: bool,
}

// TODO: Crea una nueva configuración basada en otra, cambiando solo activo
pub fn actualizar_config(config: Configuracion) -> Configuracion {
    todo!()
    // Hint: Configuracion { activo: true, ..config }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_basico() {
        let persona = Persona {
            nombre: String::from("Ana"),
            edad: 30,
        };
        assert_eq!(persona.nombre, "Ana");
        assert_eq!(persona.edad, 30);
    }
}
