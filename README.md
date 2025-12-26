# V-Code: Lenguaje de Programación para Realidad Virtual

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Language](https://img.shields.io/badge/language-Rust-orange)
![Platform](https://img.shields.io/badge/platform-VR-green)

V-Code es un lenguaje de programación moderno diseñado específicamente para desarrollo en Realidad Virtual, con sintaxis en español y VR como ciudadano de primera clase.

## 🎯 Características Principales

- **🇪🇸 Sintaxis en español**: Palabras reservadas y convenciones naturales en español
- **🥽 VR-First**: Diseñado desde cero para 90-120 FPS en entornos VR
- **⚡ Alto rendimiento**: Arquitectura preparada para compilación LLVM
- **🔒 Seguridad de tipos**: Sin null implícitos, manejo explícito de errores
- **📦 Sin dependencias externas**: Lexer y parser implementados desde cero
- **🚀 Extensible**: Arquitectura modular lista para crecer

## 🏗️ Arquitectura

```
Código V-Code (.vc)
      ↓
  LEXER (lexer.rs)
      ↓
   Tokens
      ↓
  PARSER (parser.rs)
      ↓
    AST (ast.rs)
      ↓
  EJECUTOR (ejecutor.rs)
      ↓
   Salida / VR Runtime
```

### Componentes

1. **Lexer** (`src/lexer.rs`)
   - Análisis léxico completo
   - Tokens posicionados para errores claros
   - Soporte para comentarios y strings con escapes

2. **Parser** (`src/parser.rs`)
   - Recursive descent parsing
   - Precedencia de operadores
   - Manejo robusto de errores

3. **AST** (`src/ast.rs`)
   - Árbol de sintaxis abstracta extensible
   - Tipos preparados para VR (vector3, pose, controlador)
   - Visitor pattern para optimizaciones futuras

4. **Ejecutor** (`src/ejecutor.rs`)
   - Intérprete con manejo de scopes
   - Sistema de funciones
   - Arquitectura preparada para VR runtime (90-120 FPS)

## 📦 Instalación

### Prerrequisitos

- Rust 1.70+ 
- Cargo

### Compilar el Proyecto

```bash
cd compilador
cargo build --release
```

## 🚀 Uso

### Ejecutar un Programa

```bash
cargo run ejemplos/hola.vc
```

### Ejecutar con el Binario Compilado

```bash
./target/release/vcode ejemplos/hola.vc
```

## 📝 Sintaxis de V-Code

### Hola Mundo

```vcode
escena Principal {
    mostrar "V-Code ha nacido"
}
```

### Variables y Tipos

```vcode
escena DemoVariables {
    // Números
    var edad = 25
    var altura = 1.75
    
    // Texto
    var nombre = "V-Code"
    
    // Booleanos
    var activo = verdadero
    
    // Operaciones
    var suma = 10 + 20
    mostrar suma
}
```

### Estructuras de Control

```vcode
escena ControlFlujo {
    // Condicionales
    si edad > 18 {
        mostrar "Mayor de edad"
    } sino {
        mostrar "Menor de edad"
    }
    
    // Bucle mientras
    var i = 0
    mientras i < 5 {
        mostrar i
        i = i + 1
    }
    
    // Bucle para
    para j = 0, 10 {
        mostrar j
    }
}
```

### Funciones

```vcode
funcion saludar(nombre: texto): texto {
    retornar "Hola, " + nombre
}

funcion sumar(a: numero, b: numero): numero {
    retornar a + b
}

escena Principal {
    var mensaje = saludar("V-Code")
    mostrar mensaje
    
    var resultado = sumar(10, 20)
    mostrar resultado
}
```

### Características VR (En Desarrollo)

```vcode
escena EspacioVR {
    // Callback de frame para renderizado VR
    cada frame {
        // Se ejecuta 90-120 veces por segundo
        mostrar "Frame actualizado"
    }
}

// Preparado para futuras versiones:
// 
// escena InteraccionVR {
//     var mano_derecha: mano
//     var posicion: vector3
//     
//     cuando controlador.gatillo > 0.5 {
//         // Acción al presionar gatillo
//     }
// }
```

## 🔤 Palabras Reservadas

### Core del Lenguaje
- `escena` - Define una escena VR
- `mostrar` - Muestra contenido
- `var` - Declara variable mutable
- `constante` - Declara constante inmutable

### Control de Flujo
- `si` - Condicional if
- `sino` - Condicional else
- `mientras` - Bucle while
- `para` - Bucle for
- `cada` - Iterador especial
- `frame` - Frame VR (usado con `cada`)

### Funciones
- `funcion` - Define función
- `retornar` - Retorna valor

### Tipos de Datos
- `numero` - Entero (i64)
- `decimal` - Punto flotante (f64)
- `booleano` - true/false
- `texto` - String
- `vector3` - Vector 3D (preparado)
- `pose` - Posición + rotación (preparado)
- `mano` - Estado de mano VR (preparado)
- `controlador` - Input de controlador (preparado)

### Valores Booleanos
- `verdadero` - true
- `falso` - false

## 🧪 Ejecutar Tests

```bash
cd compilador
cargo test
```

## 📊 Estado del Proyecto

### ✅ Implementado (v0.1.0)

- [x] Lexer completo con español
- [x] Parser con precedencia de operadores
- [x] AST extensible
- [x] Intérprete funcional
- [x] Sistema de variables y scopes
- [x] Funciones definidas por usuario
- [x] Estructuras de control (si, mientras, para)
- [x] Operadores aritméticos y lógicos
- [x] CLI profesional
- [x] Manejo de errores claro

### 🚧 En Desarrollo

- [ ] Tipos VR completos (vector3, pose, mano)
- [ ] Sistema de eventos XR
- [ ] Loop de renderizado 90-120 FPS
- [ ] Input de controladores y gestos
- [ ] Compilación a LLVM IR
- [ ] Compilación a WebAssembly
- [ ] Sistema de módulos
- [ ] Librería estándar VR

### 🔮 Roadmap Futuro

- [ ] Integración con OpenXR
- [ ] Soporte para Qualcomm XR2/XR2+ Gen 2
- [ ] Sistema de física VR
- [ ] Audio espacial integrado
- [ ] Editor VR integrado
- [ ] Hot reload de código
- [ ] Profiling y optimización automática
- [ ] IA asistente para desarrollo VR

## 🎯 Visión del Proyecto

V-Code no es solo un lenguaje de scripting para VR - es un lenguaje compilado de alto rendimiento diseñado para competir con los ecosistemas de Apple Vision Pro, Meta Quest y Android XR.

### Principios de Diseño

1. **VR es Primera Clase**: No es una librería agregada, está en el núcleo
2. **Performance Crítico**: Sub-20ms motion-to-photon latency
3. **Desarrollador Primero**: Sintaxis clara, errores útiles, tooling excelente
4. **Español Nativo**: No es traducción, es diseño en español
5. **Compilado y Rápido**: LLVM backend para código nativo optimizado
6. **Sin Runtime Bloat**: Control total sobre memoria y timing

## 🔧 Arquitectura Técnica

### Pipeline de Compilación (Actual - Fase Intérprete)

```
.vc source → Lexer → Tokens → Parser → AST → Interpreter → Output
```

### Pipeline de Compilación (Futuro - Fase Compilador)

```
.vc source → Lexer → Tokens → Parser → AST → Type Checker →
→ IR Generator → LLVM Optimizer → Native Code → VR Runtime
```

### Stack Tecnológico

- **Lenguaje**: Rust (para el compilador)
- **Target**: Qualcomm XR2/XR2+ Gen 2
- **Backend**: LLVM (planificado)
- **Runtime**: Custom VR runtime con OpenXR
- **Distribución**: WASM para web, nativo para dispositivos

## 📁 Estructura del Repositorio

```
vcode/
├── compilador/
│   ├── Cargo.toml          # Configuración del proyecto Rust
│   └── src/
│       ├── main.rs         # CLI y punto de entrada
│       ├── lexer.rs        # Análisis léxico
│       ├── parser.rs       # Análisis sintáctico
│       ├── ast.rs          # Árbol de sintaxis abstracta
│       └── ejecutor.rs     # Runtime e intérprete
└── ejemplos/
    ├── hola.vc             # Hola mundo
    ├── variables.vc        # Demo de variables
    ├── control_flujo.vc    # Demo de control de flujo
    ├── funciones.vc        # Demo de funciones
    └── vr_demo.vc          # Demo de conceptos VR
```

## 🤝 Contribuir

Este es un proyecto en desarrollo activo. Para contribuir:

1. Fork el repositorio
2. Crea una rama para tu feature (`git checkout -b feature/amazing-feature`)
3. Commit tus cambios (`git commit -m 'Add amazing feature'`)
4. Push a la rama (`git push origin feature/amazing-feature`)
5. Abre un Pull Request

## 📄 Licencia

MIT License - Ver archivo LICENSE para detalles

## 👨‍💻 Autor

**Paolo Gonzalez**
- GitHub: [@PaoloGonzalez776](https://github.com/PaoloGonzalez776)
- Proyecto: [V-Code-Librerias](https://github.com/PaoloGonzalez776/V-Code-Librerias)

## 🙏 Reconocimientos

V-Code está inspirado en la necesidad de un lenguaje verdaderamente nativo para VR que pueda competir con los ecosistemas de Apple, Meta y Google, pero con identidad propia y sintaxis en español.

## 📞 Contacto y Soporte

- Issues: https://github.com/PaoloGonzalez776/V-Code-Librerias/issues
- Discussions: https://github.com/PaoloGonzalez776/V-Code-Librerias/discussions

---

**V-Code - El lenguaje del futuro de la Realidad Virtual** 🥽✨
