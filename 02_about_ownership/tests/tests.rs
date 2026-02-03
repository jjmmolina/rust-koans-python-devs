use about_ownership::*;

#[test]
fn test_crear_string() {
    let s = crear_string();
    assert_eq!(s, "Hola");
}

#[test]
fn test_clonar_string() {
    let (s1, s2) = clonar_string();
    assert_eq!(s1, "original");
    assert_eq!(s2, "original");
}

#[test]
fn test_longitud_string() {
    let texto = String::from("Rust");
    assert_eq!(longitud_string(&texto), 4);
    // texto sigue siendo válido
    assert_eq!(texto, "Rust");
}

#[test]
fn test_usar_referencia() {
    assert_eq!(usar_referencia(), 4);
}

#[test]
fn test_agregar_texto() {
    let mut s = String::from("Rust");
    agregar_texto(&mut s);
    assert_eq!(s, "Rust es genial!");
}

#[test]
fn test_usar_referencia_mutable() {
    assert_eq!(usar_referencia_mutable(), "Rust es genial!");
}

#[test]
fn test_multiples_referencias() {
    let (len1, len2) = multiples_referencias();
    assert_eq!(len1, 5);
    assert_eq!(len2, 5);
}

#[test]
fn test_primera_palabra() {
    assert_eq!(primera_palabra("Hola mundo"), "Hola");
}

#[test]
fn test_str_a_string() {
    let s = str_a_string("test");
    assert_eq!(s, "test");
    assert_eq!(s.len(), 4);
}

#[test]
fn test_string_a_str() {
    let s = string_a_str(String::from("test"));
    assert_eq!(s, "test");
}

#[test]
fn test_mantener_acceso() {
    let (len, texto) = mantener_acceso();
    assert_eq!(len, 4);
    assert_eq!(texto, "Rust");
}

#[test]
fn test_crear_y_retornar() {
    let s = crear_y_retornar();
    assert_eq!(s, "creado");
}

#[test]
fn test_no_dangling() {
    let s = no_dangling();
    assert_eq!(s, "hello");
}
