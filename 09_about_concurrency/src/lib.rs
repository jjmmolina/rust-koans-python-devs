// Koan 09: Concurrencia en Rust
// Threads, Mutex, Arc, Channels

use std::sync::{Arc, Mutex};
use std::thread;

// TODO: Crea y ejecuta un thread
pub fn crear_thread() -> String {
    let handle = thread::spawn(|| {
        // TODO: Retorna "Hola desde thread"
        todo!()
    });
    handle.join().unwrap()
}

// TODO: Usa Mutex para compartir datos
pub fn usar_mutex() -> i32 {
    let contador = Mutex::new(0);
    let mut handles = vec![];
    
    for _ in 0..10 {
        let contador = Arc::new(Mutex::new(0)); // TODO: Necesitas Arc para compartir
        let handle = thread::spawn(move || {
            let mut num = contador.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    todo!() // Hint: Retorna el valor final del contador
}

// TODO: Usa channels para comunicación
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
