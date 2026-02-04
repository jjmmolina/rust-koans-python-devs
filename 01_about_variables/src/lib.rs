// Koan 01: Variables y Tipos en Rust
// 
// ¡Bienvenido! Este es tu primer paso en Rust.
// 
// CONCEPTOS CLAVE PARA PYTHONISTAS:
// 1. Inmutabilidad: En Python, los nombres son referencias. En Rust, las variables (bindings) 
//    son inmutables por defecto. Es como si todo fuera una tupla por defecto.
// 2. Tipado Estático: Rust necesita saber los tipos en compilación, aunque tiene una 
//    potente inferencia de tipos (type inference) que nos ahorra escribir mucho.
// 3. Shadowing: Puedes reutilizar el mismo nombre de variable, incluso cambiando su tipo.

// PASO 1: Variables Inmutables
// Rust prefiere la inmutabilidad para evitar errores y facilitar la concurrencia.

// TODO: Crea una variable llamada 'saludo' con el valor "Hola Rust" y retórnala.
// Nota: En Rust, la última expresión de una función es el valor de retorno (sin 'return').
// Observa que no hay punto y coma al final de esa línea.
pub fn get_saludo() -> &'static str {
    // Escribe tu código aquí
    todo!("Debes declarar la variable 'saludo' y retornarla")
}

// PASO 2: Variables Mutables
// A veces necesitamos cambiar valores. Para eso usamos 'mut'.
// Python:
//   contador = 0
//   contador += 1
// Rust:
//   let mut contador = 0; ...

// TODO: Declara una variable mutable 'contador' inicializada en 0.
// Incrementa su valor en 1 y retórnalo.
pub fn incrementar_contador() -> i32 {
    let mut contador = 0;
    // Agrega la línea para incrementar
    
    // Retorna el contador
    todo!("Incrementa y retorna el contador")
}

// PASO 3: Shadowing (Sombreado)
// En Python, reasignar una variable con otro tipo es normal y cambia la referencia:
//   x = "hola"
//   x = len(x)
// En Rust, no puedes reasignar un tipo diferente a la misma variable mutable.
// Pero puedes declarar una NUEVA variable con el mismo nombre usando 'let'.
// IMPORTANTE: Esto crea una variable totalmente nueva que oculta ("shadows") a la anterior.
// La anterior deja de ser accesible en este scope, pero sigue existiendo hasta que salga del scope.

pub fn shadowing_ejemplo() -> usize {
    let espacio = "   "; // Tipo: &str
    // TODO: Usa shadowing para crear una nueva variable 'espacio' que contenga
    // la longitud del string anterior (espacio.len()).
    // El tipo de la nueva variable será 'usize'.
    
    // tu código aquí
    
    todo!("Redeclara 'espacio' con su longitud y retórnalo")
}

// PASO 4: Tipos Explícitos
// Aunque Rust infiere tipos, a veces queremos (o necesitamos) ser explícitos.
// Esto es similar a los Type Hints en Python, pero obligatorio en la compilación.

// TODO: Declara una variable 'numero' explícitamente como i32 con valor 42 y retórnala.
pub fn tipo_explicito() -> i32 {
    // Ejemplo: let x: f64 = 3.14;
    
    todo!("Declara 'let numero: i32 = ...' y retórnalo")
}

// PASO 5: Tipos Numéricos
// Rust tiene tipos específicos para enteros y flotantes con diferentes tamaños en bits.
// i = signed (con signo), u = unsigned (sin signo).
// i8, u8, i32, u32, f64, etc.
// 
// Python maneja enteros de precisión arbitraria automáticamente. En Rust, debes elegir
// el tamaño adecuado (o usar el default i32) para rendimiento y memoria.
// ¡CUIDADO!: En modo debug, Rust entra en pánico si hay desbordamiento (overflow).
// En Python los números crecen indefinidamente, en Rust tienes límites fijos (ej: u8 llega hasta 255).

// TODO: Retorna el valor máximo posible para un entero sin signo de 8 bits (u8).
// Tip: Los tipos primitivos tienen constantes asociadas como MAX y MIN.
pub fn max_u8() -> u8 {
    todo!("Busca la constante MAX de u8")
}

// TODO: Retorna el valor mínimo de un entero con signo de 32 bits (i32).
pub fn min_i32() -> i32 {
    todo!("Busca la constante MIN de i32")
}

// PASO 6: Booleanos
// En Python: True, False (Capitalizados)
// En Rust: true, false (Minúsculas)

// TODO: Retorna el valor booleano 'verdadero'.
pub fn get_verdadero() -> bool {
    todo!()
}

// PASO 7: Tuplas
// Las tuplas agrupan valores de diferentes tipos. Tienen longitud fija.
// En Python: tupla = (1, "hola", False)
// En Rust: let tupla: (i32, &str, bool) = (1, "hola", false);

// TODO: Crea y retorna una tupla que contenga:
// 1. El número 42
// 2. El string "Rust"
// 3. El booleano true
pub fn crear_tupla() -> (i32, &'static str, bool) {
    todo!("Retorna (42, \"Rust\", true)")
}

// TODO: Extrae el segundo elemento de una tupla.
// En Rust, usamos punto (.) en lugar de corchetes [] para indexar tuplas.
pub fn extraer_de_tupla() -> &'static str {
    let tupla = (1, "dos", 3.0);
    
    // tu código aquí
    todo!("Retorna el segundo elemento usando .1")
}
, pueden crecer y viven en el Heap.
// En Rust, los Arrays [1, 2, 3] tienen tamaño FIJO inmutable y se guardan en el Stack (más rápido).
// Para listas dinámicas como en Python, Rust tiene 'Vectors' (Vec), que viven en el Heap
// En Rust, los Arrays [1, 2, 3] tienen tamaño FIJO y se guardan en el stack.
// Para listas dinámicas, Rust tiene 'Vectors' (lo veremos más adelante).

// TODO: Crea un array de 5 elementos, todos inicializados a 0.
// Tip: Rust tiene una sintaxis corta para esto: [valor_inicial; repeticiones]
pub fn array_ceros() -> [i32; 5] {
    todo!("Usa la sintaxis [0; 5]")
}

// TODO: Accede al tercer elemento (índice 2).
// El acceso a arrays sí usa corchetes [].
pub fn acceder_array() -> i32 {
    let numeros = [10, 20, 30, 40, 50];
    
    // tu código aquí
    todo!("Retorna el elemento en el índice 2")
}

// PASO 9: Constantes
// En Python: CONSTANTE = 42 (por convención, pero se puede cambiar)
// En Rust: const CONSTANTE: Type = 42; (siempre inmutable, requiere tipo explícito)
// Las constantes viven todo el tiempo de vida del programa y se 'inlinean' donde se usan.

// TODO: Define una constante llamada MAX_PUNTOS con tipo i32 y valor 100.
// Nota: 'todo!()' no se puede usar para inicializar constantes directamente en este contexto de ejercicio,
// así que tendrás que reemplazar toda la línea, o poner el valor dentro del cuerpo si fuera una función.
// Pero aquí es una constante global al módulo.
pub const MAX_PUNTOS: i32 = 100; // CÁMBIAME: Este valor está hardcoded para que compile, asegúrate de entenderlo.

// PASO 10: Conversión de Tipos (Casting/Parsing)
// Rust es estricto. No suma int + float automáticamente.
// Python: int("42")
// Rust: "42".parse()
// Nota: Usar .unwrap() en producción es peligroso (crashea si falla), pero común en prototipos/ejemplos.
// En Python un error de conversión lanza excepción, en Rust retorna un Result.

// TODO: Convierte el string "42" a un i32.
// .parse() retorna un Result, porque podría fallar.
// Por ahora usaremos .unwrap() para forzar el valor o paniquear si falla.
pub fn string_a_numero() -> i32 {
    let texto = "42";
    // Hint: texto.parse::<i32>().unwrap()
    // ::<> es el "turbofish syntax" para especificar el tipo genérico
    todo!("Parsea el texto a i32")
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
