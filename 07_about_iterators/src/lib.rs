// Koan 07: Iteradores y Closures

// TODO: Crea un iterador de un vector
pub fn crear_iterador(v: Vec<i32>) -> impl Iterator<Item = i32> {
    todo!() // Hint: v.into_iter()
}

// TODO: Usa map para transformar
pub fn doblar_valores(v: Vec<i32>) -> Vec<i32> {
    todo!() // Hint: v.iter().map(|x| x * 2).collect()
}

// TODO: Usa filter para filtrar
pub fn solo_pares(v: Vec<i32>) -> Vec<i32> {
    todo!() // Hint: v.into_iter().filter(|x| x % 2 == 0).collect()
}

// TODO: Closure básico
pub fn crear_closure() -> impl Fn(i32) -> i32 {
    todo!() // Hint: |x| x + 1
}

// TODO: Usa fold para sumar
pub fn sumar_con_fold(v: Vec<i32>) -> i32 {
    todo!() // Hint: v.iter().fold(0, |acc, x| acc + x)
}

// TODO: Combina map y filter
pub fn transformar_y_filtrar(v: Vec<i32>) -> Vec<i32> {
    todo!() // Hint: v.iter().map(|x| x * 2).filter(|x| x > &5).copied().collect()
}

// TODO: Usa any y all
pub fn tiene_negativos(v: &Vec<i32>) -> bool {
    todo!() // Hint: v.iter().any(|x| *x < 0)
}

// TODO: find - encuentra el primer elemento que cumple condición
pub fn encontrar_par(v: Vec<i32>) -> Option<i32> {
    todo!() // Hint: v.into_iter().find(|x| x % 2 == 0)
}

// TODO: Closure que captura variables
pub fn crear_multiplicador(factor: i32) -> impl Fn(i32) -> i32 {
    todo!() // Hint: move |x| x * factor
}

// TODO: Encadena iteradores
pub fn procesar_complejo(v: Vec<i32>) -> i32 {
    todo!()
    // Hint: v.iter().filter(|x| **x > 0).map(|x| x * x).sum()
}
