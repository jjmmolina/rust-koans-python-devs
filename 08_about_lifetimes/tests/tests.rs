#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_mas_largo() {
        let str1 = "hola";
        let str2 = "mundo!";
        assert_eq!(mas_largo(str1, str2), "mundo!");
    }

    #[test]
    #[ignore]
    fn test_mas_largo_igual() {
        let str1 = "rust";
        let str2 = "lang";
        assert_eq!(mas_largo(str1, str2).len(), 4);
    }

    #[test]
    #[ignore]
    fn test_extracto_nuevo() {
        let texto = String::from("Hola Rust");
        let extracto = Extracto::nuevo(&texto, "Hola");
        assert_eq!(extracto.texto_completo, "Hola Rust");
        assert_eq!(extracto.parte, "Hola");
    }

    #[test]
    #[ignore]
    fn test_extracto_anuncio() {
        let texto = String::from("¡Aprende Rust!");
        let extracto = Extracto::nuevo(&texto, "Rust");
        assert_eq!(extracto.anuncio(), "¡Lee más: Rust!");
    }

    #[test]
    #[ignore]
    fn test_primera_palabra() {
        assert_eq!(primera_palabra("Hola mundo"), "Hola");
        assert_eq!(primera_palabra("Rust"), "Rust");
    }

    #[test]
    #[ignore]
    fn test_lifetime_struct() {
        let texto1 = String::from("Hola");
        let texto2 = String::from("Mundo");
        
        let holder = LifetimeHolder {
            referencia: &texto1,
        };
        
        assert_eq!(holder.obtener(), &texto1);
    }
}
