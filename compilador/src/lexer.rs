// lexer.rs - Analizador Léxico para V-Code
// Convierte código fuente en tokens

use crate::ast::Posicion;
use std::fmt;

/// Token - unidad léxica básica
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub tipo: TipoToken,
    pub lexema: String,
    pub pos: Posicion,
}

impl Token {
    pub fn new(tipo: TipoToken, lexema: String, pos: Posicion) -> Self {
        Token { tipo, lexema, pos }
    }
}

/// Tipos de tokens
#[derive(Debug, Clone, PartialEq)]
pub enum TipoToken {
    // Palabras reservadas
    Escena,
    Mostrar,
    Var,
    Constante,
    Si,
    Sino,
    Mientras,
    Para,
    Funcion,
    Retornar,
    Verdadero,
    Falso,
    Cada,
    Frame,
    Cuando,
    
    // Tipos
    TipoNumero,
    TipoDecimal,
    TipoTexto,
    TipoBooleano,
    TipoVector3,
    TipoPose,
    TipoMano,
    TipoControlador,
    
    // Literales
    Numero(i64),
    Decimal(f64),
    Texto(String),
    
    // Identificadores
    Identificador(String),
    
    // Operadores
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Modulo,
    Asignacion,
    Igual,
    Diferente,
    Menor,
    MenorIgual,
    Mayor,
    MayorIgual,
    Y,
    O,
    No,
    
    // Delimitadores
    ParentesisAbre,
    ParentesisCierra,
    LlaveAbre,
    LlaveCierra,
    Coma,
    DosPuntos,
    Punto,
    
    // Especiales
    NuevaLinea,
    Eof,
}

impl fmt::Display for TipoToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TipoToken::Escena => write!(f, "escena"),
            TipoToken::Mostrar => write!(f, "mostrar"),
            TipoToken::Var => write!(f, "var"),
            TipoToken::Si => write!(f, "si"),
            TipoToken::Sino => write!(f, "sino"),
            TipoToken::Mientras => write!(f, "mientras"),
            TipoToken::Para => write!(f, "para"),
            TipoToken::Funcion => write!(f, "funcion"),
            TipoToken::Retornar => write!(f, "retornar"),
            TipoToken::Numero(n) => write!(f, "número {}", n),
            TipoToken::Decimal(d) => write!(f, "decimal {}", d),
            TipoToken::Texto(s) => write!(f, "texto \"{}\"", s),
            TipoToken::Identificador(id) => write!(f, "identificador '{}'", id),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Lexer - analizador léxico
pub struct Lexer {
    entrada: Vec<char>,
    posicion: usize,
    linea: usize,
    columna: usize,
}

impl Lexer {
    pub fn new(entrada: &str) -> Self {
        Lexer {
            entrada: entrada.chars().collect(),
            posicion: 0,
            linea: 1,
            columna: 1,
        }
    }
    
    /// Tokeniza toda la entrada
    pub fn tokenizar(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        loop {
            self.saltar_espacios_y_comentarios();
            
            if self.fin() {
                tokens.push(Token::new(
                    TipoToken::Eof,
                    String::new(),
                    self.posicion_actual(),
                ));
                break;
            }
            
            let token = self.siguiente_token()?;
            tokens.push(token);
        }
        
        Ok(tokens)
    }
    
    /// Obtiene el siguiente token
    fn siguiente_token(&mut self) -> Result<Token, String> {
        let pos = self.posicion_actual();
        let c = self.actual();
        
        match c {
            // Operadores y delimitadores de un carácter
            '+' => {
                self.avanzar();
                Ok(Token::new(TipoToken::Suma, "+".to_string(), pos))
            }
            '-' => {
                self.avanzar();
                Ok(Token::new(TipoToken::Resta, "-".to_string(), pos))
            }
            '*' => {
                self.avanzar();
                Ok(Token::new(TipoToken::Multiplicacion, "*".to_string(), pos))
            }
            '/' => {
                self.avanzar();
                Ok(Token::new(TipoToken::Division, "/".to_string(), pos))
            }
            '%' => {
                self.avanzar();
                Ok(Token::new(TipoToken::Modulo, "%".to_string(), pos))
            }
            '(' => {
                self.avanzar();
                Ok(Token::new(TipoToken::ParentesisAbre, "(".to_string(), pos))
            }
            ')' => {
                self.avanzar();
                Ok(Token::new(TipoToken::ParentesisCierra, ")".to_string(), pos))
            }
            '{' => {
                self.avanzar();
                Ok(Token::new(TipoToken::LlaveAbre, "{".to_string(), pos))
            }
            '}' => {
                self.avanzar();
                Ok(Token::new(TipoToken::LlaveCierra, "}".to_string(), pos))
            }
            ',' => {
                self.avanzar();
                Ok(Token::new(TipoToken::Coma, ",".to_string(), pos))
            }
            ':' => {
                self.avanzar();
                Ok(Token::new(TipoToken::DosPuntos, ":".to_string(), pos))
            }
            '.' => {
                self.avanzar();
                Ok(Token::new(TipoToken::Punto, ".".to_string(), pos))
            }
            
            // Operadores de dos caracteres
            '=' => {
                self.avanzar();
                if self.coincidir('=') {
                    Ok(Token::new(TipoToken::Igual, "==".to_string(), pos))
                } else {
                    Ok(Token::new(TipoToken::Asignacion, "=".to_string(), pos))
                }
            }
            '!' => {
                self.avanzar();
                if self.coincidir('=') {
                    Ok(Token::new(TipoToken::Diferente, "!=".to_string(), pos))
                } else {
                    Err(format!("Carácter inesperado '!' en {}", pos))
                }
            }
            '<' => {
                self.avanzar();
                if self.coincidir('=') {
                    Ok(Token::new(TipoToken::MenorIgual, "<=".to_string(), pos))
                } else {
                    Ok(Token::new(TipoToken::Menor, "<".to_string(), pos))
                }
            }
            '>' => {
                self.avanzar();
                if self.coincidir('=') {
                    Ok(Token::new(TipoToken::MayorIgual, ">=".to_string(), pos))
                } else {
                    Ok(Token::new(TipoToken::Mayor, ">".to_string(), pos))
                }
            }
            
            // Strings
            '"' => self.leer_string(),
            
            // Números
            '0'..='9' => self.leer_numero(),
            
            // Identificadores y palabras reservadas
            'a'..='z' | 'A'..='Z' | '_' => self.leer_identificador(),
            
            _ => Err(format!("Carácter inesperado '{}' en {}", c, pos)),
        }
    }
    
    /// Lee un string
    fn leer_string(&mut self) -> Result<Token, String> {
        let pos = self.posicion_actual();
        self.avanzar(); // Saltar comilla inicial
        
        let mut valor = String::new();
        
        while !self.fin() && self.actual() != '"' {
            if self.actual() == '\\' {
                self.avanzar();
                if self.fin() {
                    return Err(format!("String sin terminar en {}", pos));
                }
                
                // Escapes básicos
                match self.actual() {
                    'n' => valor.push('\n'),
                    't' => valor.push('\t'),
                    '\\' => valor.push('\\'),
                    '"' => valor.push('"'),
                    _ => {
                        valor.push('\\');
                        valor.push(self.actual());
                    }
                }
            } else {
                valor.push(self.actual());
            }
            self.avanzar();
        }
        
        if self.fin() {
            return Err(format!("String sin terminar en {}", pos));
        }
        
        self.avanzar(); // Saltar comilla final
        
        Ok(Token::new(
            TipoToken::Texto(valor.clone()),
            format!("\"{}\"", valor),
            pos,
        ))
    }
    
    /// Lee un número
    fn leer_numero(&mut self) -> Result<Token, String> {
        let pos = self.posicion_actual();
        let mut numero = String::new();
        
        while !self.fin() && self.actual().is_ascii_digit() {
            numero.push(self.actual());
            self.avanzar();
        }
        
        // Verificar si es decimal
        if !self.fin() && self.actual() == '.' && self.siguiente().is_some() && self.siguiente().unwrap().is_ascii_digit() {
            numero.push('.');
            self.avanzar();
            
            while !self.fin() && self.actual().is_ascii_digit() {
                numero.push(self.actual());
                self.avanzar();
            }
            
            let valor: f64 = numero.parse().map_err(|_| {
                format!("Número decimal inválido '{}' en {}", numero, pos)
            })?;
            
            Ok(Token::new(TipoToken::Decimal(valor), numero, pos))
        } else {
            let valor: i64 = numero.parse().map_err(|_| {
                format!("Número entero inválido '{}' en {}", numero, pos)
            })?;
            
            Ok(Token::new(TipoToken::Numero(valor), numero, pos))
        }
    }
    
    /// Lee un identificador o palabra reservada
    fn leer_identificador(&mut self) -> Result<Token, String> {
        let pos = self.posicion_actual();
        let mut id = String::new();
        
        while !self.fin() && (self.actual().is_alphanumeric() || self.actual() == '_') {
            id.push(self.actual());
            self.avanzar();
        }
        
        let tipo = match id.as_str() {
            "escena" => TipoToken::Escena,
            "mostrar" => TipoToken::Mostrar,
            "var" => TipoToken::Var,
            "constante" => TipoToken::Constante,
            "si" => TipoToken::Si,
            "sino" => TipoToken::Sino,
            "mientras" => TipoToken::Mientras,
            "para" => TipoToken::Para,
            "funcion" => TipoToken::Funcion,
            "retornar" => TipoToken::Retornar,
            "verdadero" => TipoToken::Verdadero,
            "falso" => TipoToken::Falso,
            "cada" => TipoToken::Cada,
            "frame" => TipoToken::Frame,
            "cuando" => TipoToken::Cuando,
            "y" => TipoToken::Y,
            "o" => TipoToken::O,
            "no" => TipoToken::No,
            "numero" => TipoToken::TipoNumero,
            "decimal" => TipoToken::TipoDecimal,
            "texto" => TipoToken::TipoTexto,
            "booleano" => TipoToken::TipoBooleano,
            "vector3" => TipoToken::TipoVector3,
            "pose" => TipoToken::TipoPose,
            "mano" => TipoToken::TipoMano,
            "controlador" => TipoToken::TipoControlador,
            _ => TipoToken::Identificador(id.clone()),
        };
        
        Ok(Token::new(tipo, id, pos))
    }
    
    /// Salta espacios en blanco y comentarios
    fn saltar_espacios_y_comentarios(&mut self) {
        while !self.fin() {
            match self.actual() {
                ' ' | '\t' | '\r' => {
                    self.avanzar();
                }
                '\n' => {
                    self.avanzar();
                    self.linea += 1;
                    self.columna = 1;
                }
                '/' if self.siguiente() == Some('/') => {
                    // Comentario de línea
                    while !self.fin() && self.actual() != '\n' {
                        self.avanzar();
                    }
                }
                _ => break,
            }
        }
    }
    
    /// Obtiene el carácter actual
    fn actual(&self) -> char {
        self.entrada[self.posicion]
    }
    
    /// Obtiene el siguiente carácter
    fn siguiente(&self) -> Option<char> {
        if self.posicion + 1 < self.entrada.len() {
            Some(self.entrada[self.posicion + 1])
        } else {
            None
        }
    }
    
    /// Avanza una posición
    fn avanzar(&mut self) {
        if !self.fin() {
            self.posicion += 1;
            self.columna += 1;
        }
    }
    
    /// Verifica si coincide con un carácter y avanza
    fn coincidir(&mut self, esperado: char) -> bool {
        if self.fin() || self.actual() != esperado {
            false
        } else {
            self.avanzar();
            true
        }
    }
    
    /// Verifica si llegó al final
    fn fin(&self) -> bool {
        self.posicion >= self.entrada.len()
    }
    
    /// Obtiene la posición actual
    fn posicion_actual(&self) -> Posicion {
        Posicion::new(self.linea, self.columna)
    }
}
