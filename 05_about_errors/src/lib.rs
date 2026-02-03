// Koan 05: Manejo de Errores en Rust
//
// En Python usas try/except para excepciones.
// En Rust usas Result<T, E> y Option<T> para errores.

// PASO 1: Option<T>
// None en Python vs None en Rust

// TODO: Retorna Some si el número es positivo
pub fn positivo(n: i32) -> Option<i32> {
    todo!()
    // Hint: if n > 0 { Some(n) } else { None }
}

// PASO 2: unwrap y expect
// TODO: Usa unwrap para extraer el valor (panic si es None)
pub fn usar_unwrap(opt: Option<i32>) -> i32 {
    todo!()
    // Hint: opt.unwrap()
}

// TODO: Usa expect con un mensaje personalizado
pub fn usar_expect(opt: Option<i32>) -> i32 {
    todo!()
    // Hint: opt.expect("No hay valor!")
}

// PASO 3: Result<T, E>
// TODO: Retorna Ok si b != 0, Err si b == 0
pub fn dividir(a: i32, b: i32) -> Result<i32, String> {
    todo!()
    // Hint:
    // if b == 0 {
    //     Err(String::from("División por cero"))
    // } else {
    //     Ok(a / b)
    // }
}

// PASO 4: Operador ?
// Propaga errores automáticamente
pub fn leer_y_sumar(a_str: &str, b_str: &str) -> Result<i32, std::num::ParseIntError> {
    todo!()
    // Hint:
    // let a: i32 = a_str.parse()?;
    // let b: i32 = b_str.parse()?;
    // Ok(a + b)
}

// PASO 5: match con Result
// TODO: Usa match para manejar Result
pub fn manejar_resultado(res: Result<i32, String>) -> String {
    todo!()
    // Hint:
    // match res {
    //     Ok(valor) => format!("Éxito: {}", valor),
    //     Err(error) => format!("Error: {}", error),
    // }
}

// PASO 6: Errores Personalizados
use std::fmt;

#[derive(Debug)]
pub struct MiError {
    pub mensaje: String,
}

impl fmt::Display for MiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MiError: {}", self.mensaje)
    }
}

impl std::error::Error for MiError {}

// TODO: Retorna MiError
pub fn crear_error() -> Result<(), MiError> {
    todo!()
    // Hint: Err(MiError { mensaje: String::from("Algo salió mal") })
}

// PASO 7: map y and_then
// TODO: Usa map para transformar un Option
pub fn doblar_si_existe(opt: Option<i32>) -> Option<i32> {
    todo!()
    // Hint: opt.map(|x| x * 2)
}

// TODO: Usa and_then para encadenar operaciones
pub fn dividir_y_doblar(a: i32, b: i32) -> Result<i32, String> {
    todo!()
    // Hint: dividir(a, b).map(|x| x * 2)
}

// PASO 8: unwrap_or y unwrap_or_else
// TODO: Usa unwrap_or para proporcionar un valor por defecto
pub fn obtener_o_defecto(opt: Option<i32>) -> i32 {
    todo!()
    // Hint: opt.unwrap_or(0)
}

// PASO 9: Convertir entre Option y Result
// TODO: Convierte Option a Result
pub fn option_a_result(opt: Option<i32>) -> Result<i32, String> {
    todo!()
    // Hint: opt.ok_or(String::from("Sin valor"))
}

// PASO 10: panic!
// TODO: Usa panic! para errores irrecuperables
pub fn forzar_panic(debe_fallar: bool) {
    if debe_fallar {
        todo!() // Hint: panic!("¡Error irrecuperable!")
    }
}
