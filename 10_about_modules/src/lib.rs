// Koan 10: Módulos y Organización (Visibilidad)
//
// En Python: Todo es público por defecto. Usamos _guion para "sugerir" privado.
// En Rust: Todo es PRIVADO por defecto. Debes usar declarativamente 'pub'.

// PASO 1: Módulos y Visibilidad
// mod nombre { ... } define un namespace.
// Solo lo marcado con 'pub' es accesible desde fuera del módulo.

// TODO: Define un módulo público, con función publica
pub mod geometria {
    // TODO: Función pública
    pub fn calcular_area_rectangulo(ancho: u32, alto: u32) -> u32 {
        todo!() // Hint: ancho * alto
    }
    
    // Función privada (no pub)
    // Intentar llamar a esto desde fuera causará error de compilación.
    fn funcion_privada() {
        println!("Solo visible dentro del módulo");
    }
}

// PASO 2: Imports (use)
// 'use' crea atajos a items de otros módulos.
// Similar a 'from module import function' en Python.

// TODO: Usa 'use' para importar
use geometria::calcular_area_rectangulo;

pub fn usar_modulo() -> u32 {
    todo!() // Hint: calcular_area_rectangulo(10, 20)
}

// TODO: Módulo anidado
pub mod matematicas {
    pub mod algebra {
        pub fn sumar(a: i32, b: i32) -> i32 {
            a + b
        }
    }
}

// TODO: super para acceder al módulo padre
pub mod hijo {
    use super::matematicas;
    
    pub fn usar_padre() -> i32 {
        matematicas::algebra::sumar(5, 10)
    }
}

// PASO 5: Encapsulamiento Real
// A diferencia de Python donde 'self.saldo' es accesible si quieres...
// En Rust, si un campo no es 'pub', ¡es realmente inaccesible desde otro módulo!

// TODO: Struct con campos privados
pub struct Cuenta {
    saldo: f64,  // privado por defecto
}

impl Cuenta {
    pub fn nueva(saldo_inicial: f64) -> Cuenta {
        Cuenta { saldo: saldo_inicial }
    }
    
    // Mutador seguro
    pub fn depositar(&mut self, cantidad: f64) {
        self.saldo += cantidad;
    }
    
    // Getter
    pub fn obtener_saldo(&self) -> f64 {
        self.saldo
    }
}
