use about_variables::*;

#[test]
fn test_get_saludo() {
    let resultado = get_saludo();
    assert_eq!(
        resultado, 
        "Hola Rust",
        "❌ La función debe retornar 'Hola Rust'. Recuerda: let saludo = \"Hola Rust\"; saludo"
    );
}

#[test]
fn test_incrementar_contador() {
    let resultado = incrementar_contador();
    assert_eq!(
        resultado, 
        1,
        "❌ Debes declarar 'let mut contador = 0;' y luego incrementarlo con 'contador += 1;'"
    );
}

#[test]
fn test_shadowing() {
    let resultado = shadowing_ejemplo();
    assert_eq!(
        resultado, 
        3,
        "❌ Usa shadowing: let espacio = \"   \"; let espacio = espacio.len(); (3 espacios)"
    );
}

#[test]
fn test_tipo_explicito() {
    let resultado = tipo_explicito();
    assert_eq!(
        resultado, 
        42,
        "❌ Declara explícitamente: let numero: i32 = 42;"
    );
}

#[test]
fn test_max_u8() {
    let max = max_u8();
    assert_eq!(
        max, 
        255,
        "❌ Usa la constante u8::MAX (un u8 va de 0 a 255)"
    );
}

#[test]
fn test_min_i32() {
    let min = min_i32();
    assert_eq!(
        min, 
        -2147483648,
        "❌ Usa la constante i32::MIN (un i32 con signo va de -2147483648 a 2147483647)"
    );
}

#[test]
fn test_get_verdadero() {
    assert_eq!(get_verdadero(), true);
}

#[test]
fn test_crear_tupla() {
    let tupla = crear_tupla();
    assert_eq!(tupla.0, 42, "❌ El primer elemento debe ser 42");
    assert_eq!(tupla.1, "Rust", "❌ El segundo elemento debe ser \"Rust\"");
    assert_eq!(tupla.2, true, "❌ El tercer elemento debe ser true");
    println!("✅ Tupla creada correctamente: ({}, {}, {})", tupla.0, tupla.1, tupla.2);
}

#[test]
fn test_extraer_de_tupla() {
    let resultado = extraer_de_tupla();
    assert_eq!(
        resultado, 
        "dos",
        "❌ Accede al segundo elemento con tupla.1 (indexación empieza en 0)"
    );
}

#[test]
fn test_array_ceros() {
    assert_eq!(array_ceros(), [0, 0, 0, 0, 0]);
}

#[test]
fn test_acceder_array() {
    assert_eq!(acceder_array(), 30);
}

#[test]
fn test_max_puntos() {
    assert_eq!(MAX_PUNTOS, 100);
}

#[test]
fn test_string_a_numero() {
    let resultado = string_a_numero();
    assert_eq!(
        resultado, 
        42,
        "❌ Usa .parse::<i32>().unwrap() para convertir \"42\" a número"
    );
    println!("✅ Conversión exitosa: \"42\" → 42");
}

#[test]
fn test_numero_a_string() {
    let resultado = numero_a_string();
    assert_eq!(
        resultado, 
        "42",
        "❌ Usa .to_string() para convertir 42 a String"
    );
    println!("✅ Conversión exitosa: 42 → \"42\"");
}
