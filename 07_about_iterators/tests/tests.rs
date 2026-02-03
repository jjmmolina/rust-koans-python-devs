#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_doblar_valores() {
        let vec = vec![1, 2, 3];
        assert_eq!(doblar_valores(&vec), vec![2, 4, 6]);
    }

    #[test]
    #[ignore]
    fn test_solo_pares() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(solo_pares(&vec), vec![2, 4]);
    }

    #[test]
    #[ignore]
    fn test_suma_total() {
        let vec = vec![1, 2, 3, 4];
        assert_eq!(suma_total(&vec), 10);
    }

    #[test]
    #[ignore]
    fn test_crear_multiplicador() {
        let multiplicar_por_3 = crear_multiplicador(3);
        assert_eq!(multiplicar_por_3(5), 15);
    }

    #[test]
    #[ignore]
    fn test_aplicar_operacion() {
        let resultado = aplicar_operacion(5, |x| x * 2);
        assert_eq!(resultado, 10);
    }

    #[test]
    #[ignore]
    fn test_procesar_numeros() {
        let vec = vec![1, 2, 3, 4, 5, 6];
        // Filtra pares y duplica
        let resultado = procesar_numeros(&vec);
        assert_eq!(resultado, vec![4, 8, 12]);
    }

    #[test]
    #[ignore]
    fn test_encontrar_primero() {
        let vec = vec![1, 3, 5, 8, 10];
        assert_eq!(encontrar_primero(&vec), Some(8));
    }

    #[test]
    #[ignore]
    fn test_contar_mayores() {
        let vec = vec![1, 5, 10, 15, 3];
        assert_eq!(contar_mayores(&vec, 5), 2);
    }
}
