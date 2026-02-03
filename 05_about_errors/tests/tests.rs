#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_dividir_ok() {
        assert_eq!(dividir(10.0, 2.0), Ok(5.0));
    }

    #[test]
    #[ignore]
    fn test_dividir_err() {
        assert!(dividir(10.0, 0.0).is_err());
    }

    #[test]
    #[ignore]
    fn test_obtener_primer_elemento_some() {
        let vec = vec![1, 2, 3];
        assert_eq!(obtener_primer_elemento(&vec), Some(&1));
    }

    #[test]
    #[ignore]
    fn test_obtener_primer_elemento_none() {
        let vec: Vec<i32> = vec![];
        assert_eq!(obtener_primer_elemento(&vec), None);
    }

    #[test]
    #[ignore]
    fn test_raiz_cuadrada_ok() {
        assert_eq!(raiz_cuadrada(4.0).unwrap(), 2.0);
    }

    #[test]
    #[ignore]
    fn test_raiz_cuadrada_err() {
        assert!(raiz_cuadrada(-4.0).is_err());
    }

    #[test]
    #[ignore]
    fn test_leer_y_sumar_ok() {
        assert_eq!(leer_y_sumar("5", "3"), Ok(8));
    }

    #[test]
    #[ignore]
    fn test_leer_y_sumar_err() {
        assert!(leer_y_sumar("no_numero", "3").is_err());
    }

    #[test]
    #[ignore]
    fn test_map_option() {
        let valor = Some(5);
        let resultado = valor.map(|x| x * 2);
        assert_eq!(resultado, Some(10));
    }

    #[test]
    #[ignore]
    fn test_and_then_option() {
        let valor = Some(5);
        let resultado = valor.and_then(|x| if x > 0 { Some(x * 2) } else { None });
        assert_eq!(resultado, Some(10));
    }

    #[test]
    #[ignore]
    fn test_unwrap_or() {
        let valor: Option<i32> = None;
        assert_eq!(valor.unwrap_or(100), 100);
    }
}
