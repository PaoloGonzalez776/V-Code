# Arquitectura de V-Code

Este documento describe la arquitectura interna del compilador e intérprete de V-Code.

## 🏗️ Vista General

V-Code está construido como un compilador tradicional de múltiples fases, con la arquitectura diseñada para evolucionar de intérprete a compilador completo con backend LLVM.

```
┌─────────────────────────────────────────────────────────────┐
│                    CÓDIGO FUENTE (.vc)                      │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  FASE 1: ANÁLISIS LÉXICO (Lexer)                           │
│  - Tokenización                                             │
│  - Reconocimiento de palabras reservadas                   │
│  - Manejo de literales y símbolos                          │
└──────────────────────┬──────────────────────────────────────┘
                       │ Tokens
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  FASE 2: ANÁLISIS SINTÁCTICO (Parser)                      │
│  - Construcción del AST                                     │
│  - Precedencia de operadores                                │
│  - Validación sintáctica                                    │
└──────────────────────┬──────────────────────────────────────┘
                       │ AST
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  FASE 3: ANÁLISIS SEMÁNTICO (Futuro)                       │
│  - Type checking                                            │
│  - Symbol resolution                                        │
│  - Semantic validation                                      │
└──────────────────────┬──────────────────────────────────────┘
                       │ Validated AST
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  FASE 4: EJECUCIÓN/COMPILACIÓN                             │
│  Actual: Intérprete directo del AST                        │
│  Futuro: LLVM IR → Native Code                             │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
               SALIDA / VR RUNTIME
```

## 📦 Módulos Principales

### 1. Lexer (`src/lexer.rs`)

**Responsabilidad**: Convertir texto plano en tokens estructurados.

#### Componentes

```rust
pub struct Lexer {
    entrada: Vec<char>,        // Código fuente como caracteres
    posicion_actual: usize,    // Cursor actual
    posicion: Posicion,        // Línea y columna para errores
}

pub enum Token {
    // Palabras reservadas
    Escena, Mostrar, Var, Si, Mientras, ...
    
    // Literales
    Numero(i64),
    Decimal(f64),
    Texto(String),
    
    // Símbolos
    LlaveAbre, LlaveCierra, ...
}

pub struct TokenPosicionado {
    token: Token,
    posicion: Posicion,  // Para mensajes de error claros
}
```

#### Características

- **Posicionamiento preciso**: Cada token guarda su línea y columna
- **Manejo de escapes**: Strings con `\n`, `\t`, `\"`, etc.
- **Comentarios**: Soporta `//` para comentarios de línea
- **Números**: Distingue enteros y decimales
- **Palabras reservadas**: Mapa directo a tokens específicos

#### Ejemplo de Flujo

```
Entrada: "escena Principal { var x = 10 }"
         ↓
Tokens:  [Escena, Identificador("Principal"), LlaveAbre, 
          Var, Identificador("x"), Asignacion, Numero(10), 
          LlaveCierra]
```

### 2. Parser (`src/parser.rs`)

**Responsabilidad**: Convertir lista de tokens en AST estructurado.

#### Estrategia: Recursive Descent Parsing

El parser usa **recursive descent** con **precedencia de operadores**:

```
Expresión
    └─ Igualdad (==, !=)
        └─ Comparación (>, <)
            └─ Suma (+, -)
                └─ Multiplicación (*, /)
                    └─ Primaria (literal, identificador, llamada)
```

#### Componentes Principales

```rust
pub struct Parser {
    tokens: Vec<TokenPosicionado>,
    posicion: usize,
}

// Métodos de parsing por nivel
fn parsear_programa() -> Result<Programa, String>
fn parsear_escena() -> Result<Escena, String>
fn parsear_instruccion() -> Result<Instruccion, String>
fn parsear_expresion() -> Result<Expresion, String>
fn parsear_igualdad() -> Result<Expresion, String>
fn parsear_comparacion() -> Result<Expresion, String>
// ... etc
```

#### Precedencia de Operadores

| Nivel | Operadores | Asociatividad |
|-------|-----------|---------------|
| 1     | `()` llamadas | Izquierda |
| 2     | `*` `/` | Izquierda |
| 3     | `+` `-` | Izquierda |
| 4     | `>` `<` | Izquierda |
| 5     | `==` `!=` | Izquierda |
| 6     | `y` | Izquierda |
| 7     | `o` | Izquierda |

#### Ejemplo de Construcción

```
Tokens: [Si, Numero(5), Mayor, Numero(3), LlaveAbre, ...]
        ↓
AST:    Instruccion::Si {
            condicion: Expresion::Binaria {
                izquierda: Expresion::Numero(5),
                operador: OperadorBinario::Mayor,
                derecha: Expresion::Numero(3),
            },
            bloque_entonces: [...],
            bloque_sino: None,
        }
```

### 3. AST (`src/ast.rs`)

**Responsabilidad**: Representar la estructura semántica del programa.

#### Jerarquía Principal

```
Programa
├── Escena[]
│   ├── nombre: String
│   └── cuerpo: Instruccion[]
└── Funcion[]
    ├── nombre: String
    ├── parametros: Parametro[]
    ├── tipo_retorno: Option<Tipo>
    └── cuerpo: Instruccion[]

Instruccion (enum)
├── Mostrar(Expresion)
├── DeclaracionVar { nombre, tipo, valor }
├── Asignacion { nombre, valor }
├── Si { condicion, entonces, sino }
├── Mientras { condicion, cuerpo }
├── Para { variable, inicio, fin, cuerpo }
├── CadaFrame { cuerpo }
├── LlamadaFuncion { nombre, argumentos }
└── Retornar(Option<Expresion>)

Expresion (enum)
├── Numero(i64)
├── Decimal(f64)
├── Texto(String)
├── Booleano(bool)
├── Identificador(String)
├── Binaria { izq, op, der }
├── Unaria { op, expr }
└── LlamadaFuncion { nombre, argumentos }
```

#### Tipos de Datos

```rust
pub enum Tipo {
    // Primitivos (implementados)
    Numero, Decimal, Booleano, Texto,
    
    // VR (preparados, no implementados)
    Vector3, Quaternion, Pose, Mano, Controlador,
    
    // Colecciones
    Lista(Box<Tipo>),
    
    // Custom
    Personalizado(String),
}
```

#### Valores en Runtime

```rust
pub enum Valor {
    Numero(i64),
    Decimal(f64),
    Booleano(bool),
    Texto(String),
    Nulo,
    
    // Futuro: Vector3(f32, f32, f32), etc.
}
```

### 4. Ejecutor (`src/ejecutor.rs`)

**Responsabilidad**: Interpretar y ejecutar el AST.

#### Arquitectura del Runtime

```rust
pub struct Ejecutor {
    entorno: Entorno,                      // Stack de scopes
    funciones: HashMap<String, Funcion>,   // Registry de funciones
    
    // Preparado para VR:
    // frame_callbacks: Vec<FrameCallback>,
    // input_state: InputState,
    // scene_graph: SceneGraph,
}

pub struct Entorno {
    scopes: Vec<HashMap<String, Valor>>,   // Stack para closures
}
```

#### Sistema de Scopes

```
Global Scope
    └── Escena Scope
        └── Si Scope
            └── Mientras Scope
                └── Variable local
```

Búsqueda de variables: desde scope actual hacia arriba hasta encontrar o error.

#### Flujo de Ejecución

```rust
ejecutar_programa()
    ├── Registrar funciones
    └── Para cada escena:
        └── ejecutar_escena()
            ├── Crear scope
            ├── Para cada instrucción:
            │   └── ejecutar_instruccion()
            │       ├── Mostrar → evaluar + print
            │       ├── Var → evaluar + store
            │       ├── Si → evaluar condición + ejecutar rama
            │       ├── Mientras → loop hasta falso
            │       ├── Para → iterar rango
            │       └── LlamadaFuncion → ejecutar función
            └── Destruir scope
```

#### Evaluación de Expresiones

```rust
evaluar_expresion(expr) -> Result<Valor, String> {
    match expr {
        Numero(n) => Ok(Valor::Numero(n)),
        Identificador(name) => entorno.obtener(name),
        Binaria { izq, op, der } => {
            let v_izq = evaluar(izq)?;
            let v_der = evaluar(der)?;
            aplicar_operador(v_izq, op, v_der)
        }
        ...
    }
}
```

## 🎯 Futuras Fases (Roadmap)

### Fase 5: Análisis Semántico

```rust
// src/semantic.rs (futuro)

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    type_checker: TypeChecker,
}

impl SemanticAnalyzer {
    pub fn analizar(&mut self, ast: &Programa) -> Result<AnalyzedProgram, Vec<Error>> {
        self.check_types(ast)?;
        self.check_scopes(ast)?;
        self.check_vr_constraints(ast)?;  // ej: 'cada frame' solo en escenas
        Ok(AnalyzedProgram { ast, metadata })
    }
}
```

### Fase 6: Generación de IR

```rust
// src/ir.rs (futuro)

pub struct IRGenerator {
    llvm_context: LLVMContext,
    module: Module,
}

impl IRGenerator {
    pub fn generar(&self, ast: &AnalyzedProgram) -> Result<Module, Error> {
        // Generar LLVM IR optimizado
        // Target: ARM64 (Qualcomm XR2)
    }
}
```

### Fase 7: VR Runtime

```rust
// src/vr_runtime.rs (futuro)

pub struct VRRuntime {
    xr_session: XRSession,
    compositor: Compositor,
    input_manager: InputManager,
    frame_scheduler: FrameScheduler,  // 90-120 FPS garantizado
}

impl VRRuntime {
    pub fn run_loop(&mut self) {
        loop {
            let frame_state = self.xr_session.wait_frame();
            self.input_manager.update();
            self.execute_frame_callbacks();  // 'cada frame' blocks
            self.render_scene();
            self.xr_session.end_frame();
        }
    }
}
```

## 🔄 Pipeline de Datos Completo

```
hello.vc
    │
    │ [fs::read_to_string]
    ▼
"escena Principal { mostrar \"Hola\" }"
    │
    │ [Lexer::tokenizar]
    ▼
[Token::Escena, Token::Identificador("Principal"), ...]
    │
    │ [Parser::parsear]
    ▼
Programa {
    escenas: [
        Escena {
            nombre: "Principal",
            cuerpo: [
                Instruccion::Mostrar(
                    Expresion::Texto("Hola")
                )
            ]
        }
    ]
}
    │
    │ [Ejecutor::ejecutar_programa]
    ▼
Entorno: { }  // scope vacío
    │
    │ [ejecutar_instruccion(Mostrar)]
    ▼
println!("📺 Hola")
```

## 🧠 Decisiones de Diseño

### ¿Por qué Rust?

1. **Performance**: Zero-cost abstractions, crucial para VR
2. **Seguridad**: Memory safety sin garbage collector
3. **Concurrencia**: Para procesamiento paralelo futuro
4. **LLVM**: Integración natural con LLVM backend
5. **Tooling**: Cargo, rustfmt, clippy son excepcionales

### ¿Por qué Intérprete Primero?

1. **Desarrollo rápido**: Iterar más rápido en diseño del lenguaje
2. **Testing**: Más fácil probar características
3. **Debugging**: AST visible y mutable
4. **Educativo**: Entender el lenguaje antes de optimizar

Transición a compilado es directa: AST → LLVM IR.

### ¿Por qué Sin Dependencias de Parsing?

1. **Control total**: Mensajes de error personalizados
2. **Performance**: Sin overhead de frameworks genéricos
3. **Educativo**: Implementación transparente
4. **Extensibilidad**: Fácil agregar features VR-específicas

### Scope por Stack vs Heap

Usamos **stack de HashMaps** en vez de un solo HashMap con prefijos porque:

```rust
// ✅ Stack de scopes - O(depth) lookup
scopes: Vec<HashMap<String, Valor>>

// ❌ Flat con prefijos - O(1) lookup pero complejo
symbols: HashMap<String, Valor>  // "escena::funcion::var"
```

El stack es más natural para closures y mantiene semántica clara.

## 🎮 VR-Specific Design

### Frame Timing Garantizado

```rust
// Futuro: Scheduler que garantiza timing
pub struct FrameScheduler {
    target_fps: u32,           // 90, 120
    budget_ns: u64,            // 11.1ms @ 90fps
    current_frame: u64,
}

impl FrameScheduler {
    fn should_skip_work(&self) -> bool {
        self.elapsed() > self.budget_ns * 0.9  // 90% del budget
    }
}
```

### Input State Management

```rust
// Futuro: Estado de input disponible en cada frame
pub struct InputState {
    hmd_pose: Pose,
    left_hand: HandState,
    right_hand: HandState,
    controllers: [ControllerState; 2],
    gestures: Vec<Gesture>,
}
```

## 📊 Performance Considerations

### Actual (Intérprete)

- **Lexer**: O(n) - un solo pass
- **Parser**: O(n) - recursive descent
- **Ejecución**: Variable según código

### Futuro (Compilado)

- **Compilación**: O(n) una vez
- **Ejecución**: Código nativo, comparable a C/C++
- **Frame budget**: Sub-10ms garantizado

## 🧪 Testing Strategy

```rust
// Unit tests - Cada módulo
#[cfg(test)]
mod tests {
    #[test]
    fn test_lexer_keywords() { }
    
    #[test]
    fn test_parser_precedence() { }
}

// Integration tests - Pipeline completo
#[test]
fn test_ejecutar_programa() {
    let codigo = "escena Test { mostrar 42 }";
    let resultado = ejecutar(codigo);
    assert_eq!(resultado, Ok(()));
}
```

## 📚 Referencias

- **Crafting Interpreters** - Robert Nystrom
- **Engineering a Compiler** - Cooper & Torczon
- **LLVM Documentation** - llvm.org
- **OpenXR Specification** - khronos.org/openxr

---

Esta arquitectura está diseñada para crecer de un intérprete educativo a un compilador de producción para VR. Cada decisión está pensada para el largo plazo. 🚀
