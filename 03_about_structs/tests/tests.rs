use about_structs::*;

#[test]
#[ignore] // Elimina #[ignore] cuando completes el struct
fn test_nueva_persona() {
    let persona = nueva_persona(String::from("Ana"), 30);
    assert_eq!(persona.nombre, "Ana");
    assert_eq!(persona.edad, 30);
}

#[test]
#[ignore]
fn test_saludar() {
    let persona = nueva_persona(String::from("Carlos"), 25);
    assert_eq!(persona.saludar(), "Hola, soy Carlos");
}

#[test]
#[ignore]
fn test_cumplir_años() {
    let mut persona = nueva_persona(String::from("María"), 20);
    persona.cumplir_años();
    assert_eq!(persona.edad, 21);
}

#[test]
#[ignore]
fn test_crear_punto() {
    let punto = crear_punto(1.0, 2.0, 3.0);
    assert_eq!(punto.0, 1.0);
    assert_eq!(punto.1, 2.0);
    assert_eq!(punto.2, 3.0);
}

#[test]
#[ignore]
fn test_direccion() {
    let dir = ir_al_norte();
    assert_eq!(dir.como_string(), "Norte");
}

#[test]
#[ignore]
fn test_mensaje_mover() {
    let msg = crear_mensaje_mover(10, 20);
    assert_eq!(procesar_mensaje(msg), "Mover a (10, 20)");
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
