// Koan 06: Colecciones en Rust
//
// En Python: lists, dicts y sets están integrados en el lenguaje.
// En Rust: Vec<T>, HashMap<K,V> vienen de la std lib.
// Diferencia clave: En Rust las colecciones son HOMOGÉNEAS (mismo tipo de datos).

// PASO 1: Vectores (Vec<T>)
// Equivalente a la lista de Python [1, 2, 3], pero tipada.
// Crecen dinámicamente en el Heap.

// TODO: Crea un vector vacío de i32
pub fn crear_vector() -> Vec<i32> {
    todo!() // Hint: Vec::new() o vec![]
}

// TODO: Agrega elementos a un vector
pub fn agregar_elementos() -> Vec<i32> {
    let mut v = Vec::new();
    // TODO: Usa v.push() para agregar 1, 2, 3
    todo!()
}

// TODO: Accede al segundo elemento (índice 1)
pub fn acceder_vector(v: &Vec<i32>) -> i32 {
    todo!() // Hint: v[1] o v.get(1).unwrap()
}

// TODO: Itera sobre un vector
pub fn sumar_vector(v: &Vec<i32>) -> i32 {
    let mut suma = 0;
    // TODO: for elemento in v { suma += elemento; }
    todo!()
}

// PASO 5: HashMap
// Equivalente al dict de Python {'clave': valor}.
// Requiere importar std::collections::HashMap.
// Las claves deben implementar Hash y Eq.

// TODO: Crea un HashMap
use std::collections::HashMap;

pub fn crear_mapa() -> HashMap<String, i32> {
    todo!() // Hint: HashMap::new()
}

// TODO: Inserta pares clave-valor
pub fn usar_mapa() -> HashMap<String, i32> {
   PASO 6: Acceso seguro a Mapas
// En Python: map['clave'] lanza KeyError si no existe, o usas map.get().
// En Rust: map.get() retorna Option<&V>, obligándote a manejar el caso "no encontrado".

//  let mut map = HashMap::new();
    // TODO: map.insert(String::from("azul"), 10);
    todo!()
}

// TODO: Accede a un valor del HashMap
pub fn obtener_del_mapa(map: &HashMap<String, i32>, clave: &str) -> Option<i32> {
    todo!() // Hint: map.get(clave).copied()
}

// TODO: String vs &str
pub fn crear_string() -> String {
    todo!() // Hint: String::from("Hola") o "Hola".to_string()
}

// TODO: Concatena strings
pub fn concatenar(s1: String, s2: &str) -> String {
    todo!() // Hint: format!("{}{}", s1, s2) o s1 + s2
}

// TODO: Obtén longitud de String
pub fn longitud_string(s: &String) -> usize {
    todo!() // Hint: s.len()
}
