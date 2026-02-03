#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_crear_vector() {
        let vec = crear_vector();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    #[ignore]
    fn test_agregar_elemento() {
        let mut vec = vec![1, 2];
        agregar_elemento(&mut vec, 3);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    #[ignore]
    fn test_obtener_elemento() {
        let vec = vec![10, 20, 30];
        assert_eq!(obtener_elemento(&vec, 1), Some(&20));
        assert_eq!(obtener_elemento(&vec, 10), None);
    }

    #[test]
    #[ignore]
    fn test_usar_mapa() {
        let mapa = usar_mapa();
        assert_eq!(mapa.get("nombre"), Some(&"Juan".to_string()));
        assert_eq!(mapa.get("edad"), Some(&"30".to_string()));
    }

    #[test]
    #[ignore]
    fn test_buscar_en_mapa() {
        let mut mapa = std::collections::HashMap::new();
        mapa.insert("rust".to_string(), 10);
        assert_eq!(buscar_en_mapa(&mapa, "rust"), Some(&10));
        assert_eq!(buscar_en_mapa(&mapa, "python"), None);
    }

    #[test]
    #[ignore]
    fn test_concatenar() {
        assert_eq!(concatenar("Hola ", "Mundo"), "Hola Mundo");
    }

    #[test]
    #[ignore]
    fn test_mayusculas() {
        assert_eq!(mayusculas("hola"), "HOLA");
    }

    #[test]
    #[ignore]
    fn test_longitud_string() {
        assert_eq!(longitud_string("Rust"), 4);
    }
}
