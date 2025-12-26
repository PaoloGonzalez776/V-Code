# Contribuyendo a V-Code

¡Gracias por tu interés en contribuir a V-Code! Este documento proporciona guías para contribuir al proyecto.

## 🎯 Filosofía del Proyecto

V-Code está siendo construido como un lenguaje de programación **real** para VR, no como un prototipo o proyecto educativo. Cada contribución debe mantener estos estándares:

1. **Calidad de código profesional**: Como si fuera a vivir 10 años
2. **Documentación clara**: Otros desarrolladores deben poder continuar el trabajo
3. **Diseño pensado para VR**: 90-120 FPS no es opcional
4. **Sintaxis en español**: Diseño nativo, no traducción

## 🚀 Áreas de Contribución

### Prioridad Alta

1. **Sistema de Tipos VR**
   - Implementación completa de vector3, quaternion, pose
   - Sistema de tipos para input VR (mano, controlador)
   - Validación de tipos en tiempo de compilación

2. **Backend LLVM**
   - Generación de LLVM IR desde el AST
   - Optimizaciones específicas para VR
   - Target Qualcomm XR2/XR2+ Gen 2

3. **Runtime VR**
   - Loop de renderizado 90-120 FPS
   - Sistema de eventos XR
   - Integración con OpenXR

### Prioridad Media

4. **Librería Estándar**
   - Funciones matemáticas para VR
   - Utilidades de transformación espacial
   - Sistema de logging y debugging

5. **Tooling**
   - Language Server Protocol (LSP)
   - Syntax highlighting
   - Debugger integration

### Prioridad Baja

6. **Optimizaciones**
   - Dead code elimination
   - Constant folding
   - Loop unrolling

## 📝 Proceso de Contribución

### 1. Fork y Clonar

```bash
git clone https://github.com/TU_USUARIO/V-Code-Librerias.git
cd V-Code-Librerias
```

### 2. Crear Rama

```bash
git checkout -b feature/nombre-descriptivo
```

Nomenclatura de ramas:
- `feature/` - Nueva funcionalidad
- `fix/` - Corrección de bug
- `docs/` - Documentación
- `perf/` - Optimización de rendimiento
- `refactor/` - Refactorización sin cambio de funcionalidad

### 3. Desarrollar

- Escribe código claro y documentado
- Mantén la consistencia con el estilo existente
- Agrega tests para nueva funcionalidad
- Actualiza documentación relevante

### 4. Tests

```bash
cd compilador
cargo test
cargo clippy
cargo fmt
```

Todos los tests deben pasar antes de hacer PR.

### 5. Commit

Usa mensajes de commit descriptivos:

```
feat: Agregar soporte para tipo vector3
fix: Corregir precedencia de operadores
docs: Actualizar README con ejemplos VR
perf: Optimizar lexer para archivos grandes
```

### 6. Pull Request

1. Push tu rama al fork
2. Abre PR contra `main`
3. Describe claramente los cambios
4. Referencia issues relacionados

## 💻 Estándares de Código

### Rust

```rust
// ✅ BIEN: Nombres descriptivos, documentación clara
/// Convierte tokens en AST usando recursive descent parsing
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

// ❌ MAL: Nombres cortos, sin documentación
pub struct P {
    t: Vec<Token>,
    p: usize,
}
```

### V-Code

```vcode
// ✅ BIEN: Sintaxis clara en español
escena JuegoVR {
    var puntuacion = 0
    
    cuando controlador.gatillo > 0.5 {
        puntuacion = puntuacion + 1
    }
}

// ❌ MAL: Mezcla español/inglés
escena JuegoVR {
    var score = 0  // ❌ Usar 'puntuacion'
    
    if controller.trigger > 0.5 {  // ❌ Usar 'si' y 'cuando'
        score = score + 1
    }
}
```

## 🧪 Testing

### Tests Unitarios

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lexer_identifica_escena() {
        let codigo = "escena Principal { }";
        let mut lexer = Lexer::nuevo(codigo);
        let tokens = lexer.tokenizar().unwrap();
        
        assert_eq!(tokens[0].token, Token::Escena);
    }
}
```

### Tests de Integración

Agrega ejemplos `.vc` en `ejemplos/tests/` que deben ejecutarse correctamente.

## 📚 Documentación

### Código

- Documenta funciones públicas con `///`
- Explica decisiones de diseño complejas
- Incluye ejemplos de uso

### README

- Actualiza README.md si agregas features
- Mantén ejemplos actualizados
- Documenta cambios en sintaxis

## 🐛 Reportar Bugs

Usa el template de issues con:

1. **Descripción**: ¿Qué sucedió?
2. **Reproducción**: Pasos para reproducir
3. **Esperado**: ¿Qué debería suceder?
4. **Código**: Ejemplo mínimo `.vc`
5. **Entorno**: OS, versión de Rust, etc.

## 💡 Proponer Features

Para features nuevas, abre un issue de discusión primero:

1. **Problema**: ¿Qué problema resuelve?
2. **Propuesta**: ¿Cómo lo resolvería?
3. **Alternativas**: ¿Consideraste otras opciones?
4. **Impacto**: ¿Cómo afecta el rendimiento/API?

## 🎨 Estilo de Código

### Rust

Seguimos [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

```rust
// Snake case para funciones y variables
fn parsear_expresion() -> Result<Expresion, String> { }
let numero_actual = 42;

// PascalCase para tipos
struct TokenPosicionado { }
enum ResultadoEjecucion { }

// SCREAMING_SNAKE_CASE para constantes
const VERSION_MAXIMA: u32 = 100;
```

### Comentarios

```rust
// Comentarios de una línea en español
// Explican el "por qué", no el "qué"

/// Documentación de API en español
/// 
/// # Ejemplos
/// ```
/// let mut parser = Parser::nuevo(tokens);
/// let ast = parser.parsear()?;
/// ```
```

## 🏆 Reconocimientos

Los contribuidores serán listados en el README y en releases notes.

## 📞 Preguntas

- Issues: Para bugs y features
- Discussions: Para preguntas generales
- Direct: Para contribuciones mayores, contacta antes de empezar

## 🔄 Proceso de Review

1. Automatic checks (tests, clippy, fmt)
2. Code review por mantenedores
3. Discussion de cambios si necesario
4. Merge cuando se apruebe

## 📋 Checklist de PR

- [ ] Tests pasan localmente
- [ ] Código formateado con `cargo fmt`
- [ ] Sin warnings de `cargo clippy`
- [ ] Documentación actualizada
- [ ] CHANGELOG.md actualizado (si aplica)
- [ ] Commits tienen mensajes descriptivos
- [ ] PR tiene descripción clara

---

¡Gracias por contribuir a V-Code! Juntos estamos construyendo el lenguaje del futuro de la Realidad Virtual. 🥽✨
