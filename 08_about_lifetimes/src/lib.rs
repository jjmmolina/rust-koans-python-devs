// Koan 08: Lifetimes en Rust
// Anotaciones de lifetimes garantizan que las referencias son válidas

// TODO: Función con lifetime annotation
pub fn mas_largo<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    todo!() // Hint: if s1.len() > s2.len() { s1 } else { s2 }
}

// TODO: Struct con lifetime
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

// TODO: Static lifetime
pub fn obtener_static() -> &'static str {
    todo!() // Hint: "Esta es una string literal con lifetime 'static"
}

// TODO: Lifetime en structs con métodos
pub struct Analizador<'a> {
    pub texto: &'a str,
}

impl<'a> Analizador<'a> {
    pub fn nuevo(texto: &'a str) -> Self {
        todo!() // Hint: Analizador { texto }
    }
}
