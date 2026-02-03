#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    #[ignore]
    fn test_crear_thread() {
        let handle = crear_thread();
        let resultado = handle.join().unwrap();
        assert_eq!(resultado, 10);
    }

    #[test]
    #[ignore]
    fn test_usar_mutex() {
        let contador = Arc::new(Mutex::new(0));
        let resultado = usar_mutex(contador);
        assert_eq!(resultado, 5);
    }

    #[test]
    #[ignore]
    fn test_usar_channel() {
        let valor = usar_channel();
        assert_eq!(valor, "Hola desde el thread");
    }

    #[test]
    #[ignore]
    fn test_usar_arc() {
        let resultado = usar_arc();
        assert_eq!(resultado, 30); // 3 threads * 10 cada uno
    }

    #[test]
    #[ignore]
    fn test_procesar_paralelo() {
        let numeros = vec![1, 2, 3, 4];
        let resultado = procesar_paralelo(numeros);
        assert_eq!(resultado, vec![2, 4, 6, 8]);
    }

    #[test]
    #[ignore]
    fn test_contador_compartido() {
        let handles = contador_compartido();
        for handle in handles {
            handle.join().unwrap();
        }
        // Test implícito: no debe causar deadlock
    }
}
