use about_ownership::*;

#[test]
fn test_crear_string() {
    let s = crear_string();
    assert_eq!(
        s, 
        "Hola",
        "❌ Crea un String con String::from(\"Hola\")"
    );
    println!("✅ String creado con ownership: {:?}", s);
}

#[test]
fn test_clonar_string() {
    let (s1, s2) = clonar_string();
    assert_eq!(s1, "original", "❌ s1 debe mantener el valor 'original'");
    assert_eq!(s2, "original", "❌ s2 debe ser un clon de s1");
    println!("✅ Clone exitoso: s1='{}' y s2='{}' son independientes", s1, s2);
}

#[test]
fn test_longitud_string() {
    let texto = String::from("Rust");
    let longitud = longitud_string(&texto);
    assert_eq!(
        longitud, 
        4,
        "❌ La función debe aceptar &String (referencia) y retornar s.len()"
    );
    // texto sigue siendo válido porque solo lo prestamos
    assert_eq!(texto, "Rust", "✅ La referencia NO mueve el ownership");
    println!("✅ Borrowing exitoso: texto sigue siendo válido: '{}'", texto);
}

#[test]
fn test_usar_referencia() {
    let resultado = usar_referencia();
    assert_eq!(
        resultado, 
        4,
        "❌ Pasa &texto a la función longitud_string"
    );
}

#[test]
fn test_agregar_texto() {
    let mut s = String::from("Rust");
    agregar_texto(&mut s);
    assert_eq!(
        s, 
        "Rust es genial!",
        "❌ Usa s.push_str(\" es genial!\") dentro de la función"
    );
    println!("✅ Referencia mutable: String modificado a '{}'", s);
}

#[test]
fn test_usar_referencia_mutable() {
    let resultado = usar_referencia_mutable();
    assert_eq!(
        resultado, 
        "Rust es genial!",
        "❌ Pasa &mut texto a agregar_texto"
    );
}

#[test]
fn test_multiples_referencias() {
    let (len1, len2) = multiples_referencias();
    assert_eq!(len1, 5, "❌ Primera referencia debe dar longitud 5");
    assert_eq!(len2, 5, "❌ Segunda referencia debe dar longitud 5");
    println!("✅ Múltiples referencias inmutables son permitidas");
}

#[test]
fn test_primera_palabra() {
    let resultado = primera_palabra("Hola mundo");
    assert_eq!(
        resultado, 
        "Hola",
        "❌ Retorna un slice de los primeros 4 caracteres: &s[0..4]"
    );
    println!("✅ Slice exitoso: primera palabra = '{}'", resultado);
}

#[test]
fn test_str_a_string() {
    let s = str_a_string("test");
    assert_eq!(s, "test", "❌ Usa s.to_string() o String::from(s)");
    assert_eq!(s.len(), 4);
    println!("✅ Conversión &str → String exitosa");
}

#[test]
fn test_string_a_str() {
    let s = string_a_str(String::from("test"));
    assert_eq!(
        s, 
        "test",
        "❌ Retorna el String original (referencia interna como &str luego retorna String)"
    );
    println!("✅ Entendiste la diferencia entre String y &str");
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
