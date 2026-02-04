// Koan 08: Lifetimes ('a)
// Concepto único de Rust. No existe en Python.
//
// Problema: Rust debe saber cuánto tiempo vive CADA referencia para evitar Dangling Pointers.
// Solución: Lifetimes conectan el tiempo de vida de los argumentos con el retorno.
//
// "Si la referencia de entrada vive 'a, entonces el retorno vivirá al menos 'a".

// PASO 1: Lifetimes en Funciones
// El compilador no sabe si retornamos s1 o s2.
// Debemos decirle: "El retorno vive tanto como el MÍNIMO de s1 y s2".
// Nota: <'a> declara el lifetime, &'a usa el lifetime.

// TODO: Función con lifetime annotation explícito
pub fn mas_largo<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    todo!() // Hint: if s1.len() > s2.len() { s1 } else { s2 }
}

// PASO 2: Lifetimes en Structs
// Si un struct guarda una referencia, debe saber cuánto vive esa referencia
// para no sobrevivir a sus datos (y quedar apuntando a basura).

// TODO: Struct con lifetime 'a
pub struct Extracto<'a> {
    pub parte: &'a str,
}

// TODO: Implementa un constructor
pub fn crear_extracto(texto: &str, inicio: usize, fin: usize) -> Extracto {
    todo!() // Hint: Extracto { parte: &texto[inicio..fin] }
}

// TODO: Método con lifetime
impl<'a> Extracto<'a> {
    pub fn obtener_parte(&self) -> &str {
        todo!() // Hint: self.parte
    }
}

// TODO: Múltiples lifetimes
pub fn primer_elemento<'a, 'b>(s1: &'a str, _s2: &'b str) -> &'a str {
    todo!() // Hint: s1
}

// TODO: Lifetime elision (inferido automáticamente)
pub fn primera_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    s
}

// PASO 5: Lifetime 'static
// Referencias que viven TODO el programa.
// Todas las string literals ("hola") son 'static porque están hardcoded en el binario.

// TODO: Static lifetime
pub fn obtener_static() -> &'static str {
    todo!() // Hint: "Soy estático"
}

// PASO 6: Lifetime Elision
// Rust infiere lifetimes en casos obvios (como referencia -> referencia).
// No necesitas escribir 'a aquí, pero está ahí implícitamente.

// TODO: Lifetime elision (inferido automáticamente)
pub fn primera_palabra(s: &str) -> &str {
