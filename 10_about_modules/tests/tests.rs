#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_usar_circulo() {
        assert_eq!(usar_circulo(), 314.0);
    }

    #[test]
    #[ignore]
    fn test_usar_rectangulo() {
        assert_eq!(usar_rectangulo(), 200.0);
    }

    #[test]
    #[ignore]
    fn test_cuenta_crear() {
        let cuenta = Cuenta::crear("Juan", 100);
        assert_eq!(cuenta.obtener_balance(), 100);
    }

    #[test]
    #[ignore]
    fn test_cuenta_depositar() {
        let mut cuenta = Cuenta::crear("Ana", 50);
        cuenta.depositar(25);
        assert_eq!(cuenta.obtener_balance(), 75);
    }

    #[test]
    #[ignore]
    fn test_cuenta_titular_privado() {
        let cuenta = Cuenta::crear("Pedro", 200);
        // El siguiente código NO debe compilar (titular es privado):
        // let _ = cuenta.titular;
        assert_eq!(cuenta.obtener_balance(), 200);
    }

    #[test]
    #[ignore]
    fn test_usar_constante() {
        assert_eq!(usar_constante(), 100);
    }

    #[test]
    #[ignore]
    fn test_modulos_anidados() {
        // geometria::circulo::area() debe ser accesible
        assert_eq!(geometria::circulo::area(10.0), 314.0);
    }
}
