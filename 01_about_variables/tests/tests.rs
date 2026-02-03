use about_variables::*;

#[test]
fn test_get_saludo() {
    assert_eq!(get_saludo(), "Hola Rust");
}

#[test]
fn test_incrementar_contador() {
    assert_eq!(incrementar_contador(), 1);
}

#[test]
fn test_shadowing() {
    assert_eq!(shadowing_ejemplo(), 3);
}

#[test]
fn test_tipo_explicito() {
    assert_eq!(tipo_explicito(), 42);
}

#[test]
fn test_max_u8() {
    assert_eq!(max_u8(), 255);
}

#[test]
fn test_min_i32() {
    assert_eq!(min_i32(), -2147483648);
}

#[test]
fn test_get_verdadero() {
    assert_eq!(get_verdadero(), true);
}

#[test]
fn test_crear_tupla() {
    let tupla = crear_tupla();
    assert_eq!(tupla.0, 42);
    assert_eq!(tupla.1, "Rust");
    assert_eq!(tupla.2, true);
}

#[test]
fn test_extraer_de_tupla() {
    assert_eq!(extraer_de_tupla(), "dos");
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
    assert_eq!(string_a_numero(), 42);
}

#[test]
fn test_numero_a_string() {
    assert_eq!(numero_a_string(), "42");
}
