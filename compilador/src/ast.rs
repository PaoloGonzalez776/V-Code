// ast.rs - Árbol de Sintaxis Abstracta (AST) para V-Code
// Define todas las estructuras de datos que representan el código V-Code

use std::fmt;

/// Posición en el código fuente para mensajes de error claros
#[derive(Debug, Clone, PartialEq)]
pub struct Posicion {
    pub linea: usize,
    pub columna: usize,
}

impl Posicion {
    pub fn new(linea: usize, columna: usize) -> Self {
        Posicion { linea, columna }
    }
}

impl fmt::Display for Posicion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "línea {}, columna {}", self.linea, self.columna)
    }
}

/// Programa completo - raíz del AST
#[derive(Debug, Clone, PartialEq)]
pub struct Programa {
    pub declaraciones: Vec<Declaracion>,
}

/// Declaraciones de nivel superior
#[derive(Debug, Clone, PartialEq)]
pub enum Declaracion {
    Escena(Escena),
    Funcion(Funcion),
}

/// Escena VR - equivalente a una clase o módulo principal
#[derive(Debug, Clone, PartialEq)]
pub struct Escena {
    pub nombre: String,
    pub cuerpo: Vec<Sentencia>,
    pub pos: Posicion,
}

/// Definición de función
#[derive(Debug, Clone, PartialEq)]
pub struct Funcion {
    pub nombre: String,
    pub parametros: Vec<Parametro>,
    pub tipo_retorno: Option<Tipo>,
    pub cuerpo: Vec<Sentencia>,
    pub pos: Posicion,
}

/// Parámetro de función
#[derive(Debug, Clone, PartialEq)]
pub struct Parametro {
    pub nombre: String,
    pub tipo_dato: Tipo,
}

/// Tipos de datos en V-Code
#[derive(Debug, Clone, PartialEq)]
pub enum Tipo {
    Numero,      // i64
    Decimal,     // f64
    Texto,       // String
    Booleano,    // bool
    Vector3,     // (f64, f64, f64) - Para VR
    Pose,        // Posición + rotación
    Mano,        // Estado de mano VR
    Controlador, // Input de controlador
}

/// Sentencias - instrucciones ejecutables
#[derive(Debug, Clone, PartialEq)]
pub enum Sentencia {
    Mostrar(Expresion, Posicion),
    Var(String, Expresion, Posicion),
    Asignacion(String, Expresion, Posicion),
    Si(Expresion, Vec<Sentencia>, Option<Vec<Sentencia>>, Posicion),
    Mientras(Expresion, Vec<Sentencia>, Posicion),
    Para(String, Expresion, Expresion, Vec<Sentencia>, Posicion),
    Retornar(Option<Expresion>, Posicion),
    Expresion(Expresion),
}

/// Expresiones - producen valores
#[derive(Debug, Clone, PartialEq)]
pub enum Expresion {
    Numero(i64, Posicion),
    Decimal(f64, Posicion),
    Texto(String, Posicion),
    Booleano(bool, Posicion),
    Variable(String, Posicion),
    Binaria(Box<Expresion>, OperadorBinario, Box<Expresion>, Posicion),
    Unaria(OperadorUnario, Box<Expresion>, Posicion),
    Llamada(String, Vec<Expresion>, Posicion),
}

impl Expresion {
    pub fn posicion(&self) -> &Posicion {
        match self {
            Expresion::Numero(_, pos) => pos,
            Expresion::Decimal(_, pos) => pos,
            Expresion::Texto(_, pos) => pos,
            Expresion::Booleano(_, pos) => pos,
            Expresion::Variable(_, pos) => pos,
            Expresion::Binaria(_, _, _, pos) => pos,
            Expresion::Unaria(_, _, pos) => pos,
            Expresion::Llamada(_, _, pos) => pos,
        }
    }
}

/// Operadores binarios
#[derive(Debug, Clone, PartialEq)]
pub enum OperadorBinario {
    // Aritméticos
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Modulo,
    
    // Comparación
    Igual,
    Diferente,
    Menor,
    MenorIgual,
    Mayor,
    MayorIgual,
    
    // Lógicos
    Y,
    O,
}

impl fmt::Display for OperadorBinario {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            OperadorBinario::Suma => "+",
            OperadorBinario::Resta => "-",
            OperadorBinario::Multiplicacion => "*",
            OperadorBinario::Division => "/",
            OperadorBinario::Modulo => "%",
            OperadorBinario::Igual => "==",
            OperadorBinario::Diferente => "!=",
            OperadorBinario::Menor => "<",
            OperadorBinario::MenorIgual => "<=",
            OperadorBinario::Mayor => ">",
            OperadorBinario::MayorIgual => ">=",
            OperadorBinario::Y => "y",
            OperadorBinario::O => "o",
        };
        write!(f, "{}", s)
    }
}

/// Operadores unarios
#[derive(Debug, Clone, PartialEq)]
pub enum OperadorUnario {
    Negacion,  // -
    No,        // no
}

impl fmt::Display for OperadorUnario {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            OperadorUnario::Negacion => "-",
            OperadorUnario::No => "no",
        };
        write!(f, "{}", s)
    }
}
