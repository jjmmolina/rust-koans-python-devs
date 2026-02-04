use about_structs::*;

#[test]
#[ignore] // Elimina #[ignore] cuando definas el struct Persona con campos pub nombre y pub edad
fn test_nueva_persona() {
    let persona = nueva_persona(String::from("Ana"), 30);
    assert_eq!(
        persona.nombre, 
        "Ana",
        "❌ Define: pub struct Persona {{ pub nombre: String, pub edad: u32 }}"
    );
    assert_eq!(persona.edad, 30);
    println!("✅ Struct creado: {} tiene {} años", persona.nombre, persona.edad);
}

#[test]
#[ignore] // Elimina cuando implementes el método saludar en impl Persona
fn test_saludar() {
    let persona = nueva_persona(String::from("Carlos"), 25);
    let saludo = persona.saludar();
    assert_eq!(
        saludo, 
        "Hola, soy Carlos",
        "❌ Usa format!(\"Hola, soy {{}}\", self.nombre) en el método"
    );
    println!("✅ Método saludar: '{}'", saludo);
}

#[test]
#[ignore] // Elimina cuando implementes cumplir_años
fn test_cumplir_años() {
    let mut persona = nueva_persona(String::from("María"), 20);
    persona.cumplir_años();
    assert_eq!(
        persona.edad, 
        21,
        "❌ Incrementa self.edad con self.edad += 1; (necesitas &mut self)"
    );
    println!("✅ ¡Feliz cumpleaños! Ahora {} tiene {} años", persona.nombre, persona.edad);
}

#[test]
#[ignore] // Elimina cuando definas Punto3D como tuple struct
fn test_crear_punto() {
    let punto = crear_punto(1.0, 2.0, 3.0);
    assert_eq!(punto.0, 1.0, "❌ Define: pub struct Punto3D(pub f64, pub f64, pub f64);");
    assert_eq!(punto.1, 2.0);
    assert_eq!(punto.2, 3.0);
    println!("✅ Punto3D creado: ({}, {}, {})", punto.0, punto.1, punto.2);
}

#[test]
#[ignore] // Elimina cuando definas el enum Direccion
fn test_direccion() {
    let dir = ir_al_norte();
    assert_eq!(
        dir.como_string(), 
        "Norte",
        "❌ Define: pub enum Direccion {{ Norte, Sur, Este, Oeste }}"
    );
    println!("✅ Dirección: {}", dir.como_string());
}

#[test]
#[ignore] // Elimina cuando definas el enum Mensaje con datos
fn test_mensaje_mover() {
    let msg = crear_mensaje_mover(10, 20);
    let procesado = procesar_mensaje(msg);
    assert_eq!(
        procesado, 
        "Mover a (10, 20)",
        "❌ Enum con datos: Mensaje::Mover {{ x: i32, y: i32 }}"
    );
    println!("✅ Mensaje procesado: '{}'", procesado);
}

#[test]
#[ignore]
fn test_option_some() {
    assert_eq!(retornar_some(), Some(42));
}

#[test]
#[ignore]
fn test_option_none() {
    assert_eq!(retornar_none(), None);
}

#[test]
#[ignore]
fn test_extraer_option() {
    assert_eq!(extraer_option(Some(42)), 42);
    assert_eq!(extraer_option(None), 0);
}

#[test]
#[ignore]
fn test_if_let() {
    assert_eq!(usar_if_let(Some(42)), "Valor: 42");
    assert_eq!(usar_if_let(None), "Sin valor");
}

#[test]
#[ignore]
fn test_actualizar_config() {
    let config1 = Configuracion {
        ancho: 800,
        alto: 600,
        activo: false,
    };
    let config2 = actualizar_config(config1);
    assert_eq!(config2.ancho, 800);
    assert_eq!(config2.alto, 600);
    assert_eq!(config2.activo, true);
}
