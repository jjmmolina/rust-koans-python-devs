#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_hablador_perro() {
        let nombre = "Rex";
        let resultado = Perro(nombre.to_string()).hablar();
        assert_eq!(resultado, "Rex dice: Guau!");
    }

    #[test]
    #[ignore]
    fn test_hablador_gato() {
        let nombre = "Misu";
        let resultado = Gato { nombre: nombre.to_string() }.hablar();
        assert_eq!(resultado, "Misu dice: Miau!");
    }

    #[test]
    #[ignore]
    fn test_hacer_hablar() {
        let perro = Perro("Rex".to_string());
        let resultado = hacer_hablar(&perro);
        assert_eq!(resultado, "Rex dice: Guau!");
    }

    #[test]
    #[ignore]
    fn test_mayor_numeros() {
        assert_eq!(mayor(&5, &10), 10);
        assert_eq!(mayor(&100, &50), 100);
    }

    #[test]
    #[ignore]
    fn test_mayor_strings() {
        assert_eq!(mayor(&"abc", &"xyz"), "xyz");
        assert_eq!(mayor(&"rust", &"python"), "rust");
    }

    #[test]
    #[ignore]
    fn test_punto_display() {
        let p = Punto { x: 3, y: 4 };
        assert_eq!(format!("{}", p), "(3, 4)");
    }

    #[test]
    #[ignore]
    fn test_suma_dos() {
        assert_eq!(suma_dos(5), 7);
        assert_eq!(suma_dos(100), 102);
    }

    #[test]
    #[ignore]
    fn test_trait_objects() {
        let animales: Vec<Box<dyn Hablador>> = vec![
            Box::new(Perro("Rex".to_string())),
            Box::new(Gato { nombre: "Misu".to_string() }),
        ];
        assert_eq!(animales.len(), 2);
    }
}
