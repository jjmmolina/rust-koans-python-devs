// Koan 10: Módulos y Organización
// mod, pub, use, crate

// TODO: Define un módulo público
pub mod geometria {
    // TODO: Función pública
    pub fn calcular_area_rectangulo(ancho: u32, alto: u32) -> u32 {
        todo!() // Hint: ancho * alto
    }
    
    // Función privada (no pub)
    fn funcion_privada() {
        println!("Solo visible dentro del módulo");
    }
}

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

// TODO: Re-exportar con pub use
pub use matematicas::algebra::sumar as suma_publica;

// TODO: Struct con campos privados
pub struct Cuenta {
    saldo: f64,  // privado
}

impl Cuenta {
    pub fn nueva(saldo_inicial: f64) -> Cuenta {
        Cuenta { saldo: saldo_inicial }
    }
    
    pub fn depositar(&mut self, cantidad: f64) {
        self.saldo += cantidad;
    }
    
    pub fn obtener_saldo(&self) -> f64 {
        self.saldo
    }
}
