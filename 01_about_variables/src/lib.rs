// Koan 01: Variables y Tipos en Rust
// 
// En Python, las variables son dinámicas y mutables por defecto.
// En Rust, las variables son inmutables por defecto y tipadas estáticamente.

// PASO 1: Variables Inmutables
// En Python: x = 5
// En Rust: let x = 5; (inmutable por defecto)

// TODO: Crea una variable inmutable 'saludo' con valor "Hola Rust"
pub fn get_saludo() -> &'static str {
    todo!() // Reemplaza con: let saludo = "Hola Rust"; saludo
}

// PASO 2: Variables Mutables
// En Python: x = 5; x = 10 (siempre mutable)
// En Rust: let mut x = 5; x = 10; (debes declarar mut)

// TODO: Crea una variable mutable 'contador' con valor inicial 0, increm éntala y retorna
pub fn incrementar_contador() -> i32 {
    todo!()
    // Hint: let mut contador = 0;
    // contador += 1;
    // contador
}

// PASO 3: Shadowing
// En Python: no existe este concepto
// En Rust: puedes redeclarar una variable con let

// TODO: Usa shadowing para convertir un string en su longitud
pub fn shadowing_ejemplo() -> usize {
    let espacio = "   ";
    // TODO: Usa shadowing para convertir espacio en su longitud
    // let espacio = espacio.len();
    todo!()
}

// PASO 4: Tipos Explícitos
// En Python: x: int = 5 (opcional, type hints)
// En Rust: let x: i32 = 5; (tipos inferidos o explícitos)

// TODO: Declara una variable con tipo explícito i32
pub fn tipo_explicito() -> i32 {
    todo!()
    // Hint: let numero: i32 = 42;
}

// PASO 5: Tipos Numéricos
// Rust tiene muchos tipos numéricos: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64

// TODO: Retorna el valor máximo de un u8 (unsigned 8-bit)
pub fn max_u8() -> u8 {
    todo!()
    // Hint: u8::MAX
}

// TODO: Retorna el valor mínimo de un i32
pub fn min_i32() -> i32 {
    todo!()
    // Hint: i32::MIN
}

// PASO 6: Booleanos
// En Python: True, False
// En Rust: true, false (minúsculas)

// TODO: Retorna true
pub fn get_verdadero() -> bool {
    todo!()
}

// PASO 7: Tuplas
// En Python: tupla = (1, 2, 3)
// En Rust: let tupla: (i32, i32, i32) = (1, 2, 3);

// TODO: Crea y retorna una tupla (i32, &str, bool)
pub fn crear_tupla() -> (i32, &'static str, bool) {
    todo!()
    // Hint: (42, "Rust", true)
}

// TODO: Extrae el segundo elemento de una tupla
pub fn extraer_de_tupla() -> &'static str {
    let tupla = (1, "dos", 3.0);
    // TODO: Extrae "dos" usando tupla.1
    todo!()
}

// PASO 8: Arrays
// En Python: lista = [1, 2, 3] (dinámica)
// En Rust: let array = [1, 2, 3]; (tamaño fijo)

// TODO: Crea un array de 5 elementos con valor 0
pub fn array_ceros() -> [i32; 5] {
    todo!()
    // Hint: [0; 5] crea [0, 0, 0, 0, 0]
}

// TODO: Accede al tercer elemento (índice 2)
pub fn acceder_array() -> i32 {
    let numeros = [10, 20, 30, 40, 50];
    // TODO: Retorna numeros[2]
    todo!()
}

// PASO 9: Constantes
// En Python: CONSTANTE = 42 (por convención)
// En Rust: const CONSTANTE: i32 = 42; (constante real)

// TODO: Define una constante MAX_PUNTOS con valor 100
pub const MAX_PUNTOS: i32 = todo!(); // Reemplaza todo!() con 100

// PASO 10: Conversión de Tipos
// En Python: int("42"), str(42)
// En Rust: "42".parse(), 42.to_string()

// TODO: Convierte un string a i32
pub fn string_a_numero() -> i32 {
    let texto = "42";
    // TODO: Usa texto.parse::<i32>().unwrap()
    todo!()
}

// TODO: Convierte un i32 a String
pub fn numero_a_string() -> String {
    let numero = 42;
    // TODO: Usa numero.to_string()
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inmutabilidad() {
        // Este test demuestra que las variables son inmutables por defecto
        let x = 5;
        // x = 10; // ¡Esto no compila!
        assert_eq!(x, 5);
    }
}
