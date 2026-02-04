// Koan 09: Concurrencia en Rust ("Fearless Concurrency")
//
// En Python: GIL (Global Interpreter Lock) limita a 1 thread ejecutando Python a la vez. 
// Para paralelismo real usas multiprocessing.
//
// En Rust: NO hay GIL. Threads reales del sistema operativo.
// El type system (Send/Sync traits) te IMPIDE compilar data races.

use std::sync::{Arc, Mutex};
use std::thread;

// PASO 1: Threads
// thread::spawn(closure) crea un nuevo hilo.

// TODO: Crea y ejecuta un thread
pub fn crear_thread() -> String {
    let handle = thread::spawn(|| {
        // TODO: Retorna "Hola desde thread"
        todo!()
    });
    // .join() espera a que el thread termine. Si hubo pánico, retorna Err.
    handle.join().unwrap()
}

// PASO 2: Memoria Compartida (Mutex + Arc)
// En Python: threading.Lock().
// En Rust: Mutex<T> envuelve el dato. No puedes acceder al dato sin bloquear (lock).
// Arc<T>: Atomic Reference Count. Como un puntero compartido thread-safe.
// Necesario para que múltiples threads sean "dueños" del mismo Mutex.

// TODO: Usa Mutex para compartir datos
pub fn usar_mutex() -> i32 {
    // Este contador debe ser compartido. Lo envolvemos en Arc y Mutex.
    let contador = Arc::new(Mutex::new(0)); 
    let mut handles = vec![];
    
    for _ in 0..10 {
        // Clona el puntero Arc (barato, incrementa ref count) para el nuevo thread
        let contador_clon = Arc::clone(&contador);
        
        let handle = thread::spawn(move || {
            // .lock() devuelve un LockResult. unwrap() nos da el MutexGuard.
            // Al salir del scope, el MutexGuard se dropea y libera el lock automáticamente.
            let mut num = contador_clon.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Necesitamos lockear una última vez para leer el valor final o desempaquetar
    // Como somos los únicos dueños ahora (si Arc count == 1), podríamos try_unwrap
    let resultado = *contador.lock().unwrap();
    resultado
}

// PASO 3: Channels (Message Passing)
// "No te comuniques compartiendo memoria; comparte memoria comunicándote".
use std::sync::mpsc;

pub fn usar_channel() -> i32 {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send(42).unwrap();
    });
    
    todo!() // Hint: rx.recv().unwrap()
}

// TODO: Arc (Atomic Reference Count) para compartir datos immutables
pub fn usar_arc() -> Vec<i32> {
    let datos = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];
    
    for _ in 0..3 {
        let datos = Arc::clone(&datos);
        let handle = thread::spawn(move || {
            datos.len()
        });
        handles.push(handle);
    }
    
    let mut resultados = vec![];
    for handle in handles {
        resultados.push(handle.join().unwrap() as i32);
    }
    
    resultados
}
