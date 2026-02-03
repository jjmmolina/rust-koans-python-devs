# Contribuir a Rust Koans 🤝

¡Gracias por tu interés en contribuir! Los aportes son muy bienvenidos.

## 🚀 Cómo Empezar

### 1. Fork y Clonar
```bash
# Fork en GitHub (botón "Fork")
git clone https://github.com/TU_USUARIO/rust-koans-python-devs.git
cd rust-koans-python-devs
```

### 2. Crear Rama
```bash
git checkout -b feature/mi-mejora
```

### 3. Hacer Cambios
```bash
# Editar archivos...
cargo fmt      # Formatear
cargo clippy   # Verificar estilo
cargo test     # Verificar tests
```

### 4. Commit y Push
```bash
git add .
git commit -m "Agrega descripción clara de cambios"
git push origin feature/mi-mejora
```

### 5. Pull Request
- Ve a GitHub y abre un PR
- Describe claramente qué cambió y por qué
- Los mantainers revisarán tu PR

## 📝 Guías de Código

### Para Nuevos Koans

Si quieres agregar un nuevo koan:

1. **Nombre descriptivo**: `XX_about_concepto` (ej: `11_about_async`)
2. **Estructura**:
   ```
   XX_about_concepto/
   ├── Cargo.toml          # Con dependencies necesarias
   ├── src/lib.rs          # Con TODOs y hints (SIN SOLUCIONES)
   └── tests/tests.rs      # Con tests #[ignore]
   ```

3. **Código del Koan**:
   ```rust
   // CONCEPTO: Explicación breve
   // En Python: mostrar equivalente si es posible
   // 
   // TODO: Descripción clara de qué hacer
   // HINT: Pista si es necesario
   // HINT 2: Segunda pista si es muy difícil
   
   pub fn mi_funcion() {
       todo!()  // ← Nunca muestres la solución
   }
   ```

4. **Tests**:
   ```rust
   #[test]
   #[ignore]
   fn test_descripcion_clara() {
       assert_eq!(resultado, esperado);
   }
   ```

5. **Actualizar**:
   - `Cargo.toml` raíz: agregar el nuevo miembro
   - `README.md`: actualizar lista de koans
   - `SOLUCIONES.md`: agregar soluciones con explicaciones

### Mejoras de Documentación

Para mejorar GUIA.md, COMO_FUNCIONA.md, etc:

- ✅ Usa markdown correctamente
- ✅ Mantén ejemplos concisos
- ✅ Usa código resaltado con sintaxis
- ✅ Incluye comparaciones Python/Rust cuando sea relevante
- ✅ Sé claro y conciso

### Mejoras de Código

Para mejoras en código existente:

- ✅ Sigue el formato: `cargo fmt`
- ✅ No hay warnings: `cargo clippy`
- ✅ Mantén TODOs sin soluciones
- ✅ Tests deben pasar: `cargo test`
- ✅ Escribe mensajes de commit claros

## 🧪 Antes de Hacer el PR

```bash
# Formatear
cargo fmt --all

# Linter
cargo clippy --workspace --all-targets --all-features

# Tests
cargo test --workspace

# Verificación final
cargo check --workspace
```

## 📋 Checklist para PRs

- [ ] Código formateado con `cargo fmt`
- [ ] Sin warnings de `cargo clippy`
- [ ] Tests pasan: `cargo test`
- [ ] Documentación actualizada
- [ ] Commits con mensajes claros
- [ ] Sin archivos irrelevantes incluidos

## 🎯 Áreas Donde Ayuda es Bienvenida

- 📝 Mejoras de documentación
- 🐛 Reporte de bugs
- ✨ Sugerencias de nuevos koans
- 🔧 Mejoras de tooling
- 🌍 Traducción a otros idiomas
- 💡 Ejemplos adicionales

## 💬 Código de Conducta

- Sé respetuoso y constructivo
- Inclusivo con todas las personas
- Constructivo en la crítica
- Aprecia diferentes opiniones

## ❓ Preguntas?

- Abre una **Issue** para reportar bugs
- Abre una **Discussion** para preguntas
- Contacta en el PR si tienes dudas

## ✨ Gracias

Cada contribución, por pequeña que sea, ayuda a hacer Rust más accesible. 🦀

---

**¡Esperamos tu PR! Gracias por contribuir a la comunidad Rust.** ❤️
