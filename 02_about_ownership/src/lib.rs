// Koan 02: Ownership y Borrowing en Rust
//
// En Python, todo se maneja por referencia con garbage collector.
// En Rust, cada valor tiene un único dueño y se libera cuando el dueño sale del scope.

// PASO 1: Ownership Básico
// En Python: s1 = "hello"; s2 = s1; # ambos apuntan al mismo objeto
// En Rust: let s1 = String::from("hello"); let s2 = s1; // s1 ya no es válido (moved)

// TODO: Retorna un String sin que ocurra un move
pub fn crear_string() -> String {
    todo!()
    // Hint: String::from("Hola")
}

// PASO 2: Clone
// En Python: import copy; s2 = copy.deepcopy(s1)
// En Rust: let s2 = s1.clone();

// TODO: Clona un String y retorna ambos
pub fn clonar_string() -> (String, String) {
    let s1 = String::from("original");
    // TODO: Clona s1 en s2 y retorna (s1, s2)
    todo!()
}

// PASO 3: Referencias Inmutables
// En Python: no hay distinción explícita
// En Rust: &variable crea una referencia inmutable

// TODO: Calcula la longitud sin tomar ownership
pub fn longitud_string(s: &String) -> usize {
    todo!()
    // Hint: s.len()
}

// TODO: Usa la función anterior
pub fn usar_referencia() -> usize {
    let texto = String::from("Rust");
    // TODO: Pasa &texto a longitud_string
    todo!()
}

// PASO 4: Referencias Mutables
// En Python: lista.append(4) modifica en su lugar
// En Rust: necesitas &mut para modificar

// TODO: Agrega texto a un String mutable
pub fn agregar_texto(s: &mut String) {
    // TODO: Usa s.push_str(" es genial!")
    todo!()
}

pub fn usar_referencia_mutable() -> String {
    let mut texto = String::from("Rust");
    agregar_texto(&mut texto);
    texto
}

// PASO 5: Reglas de Borrowing
// 1. Puedes tener múltiples referencias inmutables (&T)
// 2. O una sola referencia mutable (&mut T)
// 3. Pero NO ambas al mismo tiempo

// TODO: Crea múltiples referencias inmutables
pub fn multiples_referencias() -> (usize, usize) {
    let s = String::from("hello");
    // TODO: Crea dos referencias inmutables y retorna sus longitudes
    // let r1 = &s;
    // let r2 = &s;
    // (r1.len(), r2.len())
    todo!()
}

// PASO 6: Slices
// En Python: texto[0:5]
// En Rust: &texto[0..5]

// TODO: Retorna una slice de los primeros 4 caracteres
pub fn primera_palabra(s: &str) -> &str {
    // TODO: Retorna &s[0..4]
    // Asume que s tiene al menos 4 caracteres
    todo!()
}

// PASO 7: String vs &str
// String: owned, heap-allocated, mutable
// &str: borrowed, puede ser stack o heap, inmutable

// TODO: Convierte &str a String
pub fn str_a_string(s: &str) -> String {
    todo!()
    // Hint: s.to_string() o String::from(s)
}

// TODO: Convierte String a &str
pub fn string_a_str(s: String) -> String {
    let referencia: &str = &s;
    // TODO: Retorna s (no referencia)
    todo!()
}

// PASO 8: Ownership con Funciones
// Pasar un valor a una función transfiere ownership

pub fn tomar_ownership(s: String) -> usize {
    s.len()
    // s se destruye aquí
}

// TODO: Llama tomar_ownership sin perder acceso al String
pub fn mantener_acceso() -> (usize, String) {
    let texto = String::from("Rust");
    // TODO: Clona texto antes de pasarlo, o usa referencia
    // Opción 1: let len = tomar_ownership(texto.clone());
    // Opción 2: Cambia la firma de tomar_ownership (no en este ejercicio)
    todo!()
}

// PASO 9: Retornar Ownership
// Las funciones pueden devolver ownership

// TODO: Crea y retorna un String
pub fn crear_y_retornar() -> String {
    let s = String::from("creado");
    // TODO: Retorna s (se transfiere ownership)
    todo!()
}

// PASO 10: Dangling References
// Rust previene referencias que apuntan a memoria liberada

// Este código NO COMPILA (descomentado)
// pub fn dangling() -> &String {
//     let s = String::from("hello");
//     &s // ¡Error! s se destruye, pero intentamos retornar una referencia
// }

// TODO: Arregla el código anterior retornando el String, no una referencia
pub fn no_dangling() -> String {
    let s = String::from("hello");
    // TODO: Retorna s en lugar de &s
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_basico() {
        let s1 = String::from("hello");
        let s2 = s1; // s1 ya no es válido
        // println!("{}", s1); // ¡Error de compilación!
        assert_eq!(s2, "hello");
    }
}
