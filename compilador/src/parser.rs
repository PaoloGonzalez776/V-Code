// parser.rs - Analizador Sintáctico para V-Code
// Convierte tokens en un AST

use crate::ast::*;
use crate::lexer::{Token, TipoToken};

pub struct Parser {
    tokens: Vec<Token>,
    actual: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, actual: 0 }
    }
    
    /// Parsea el programa completo
    pub fn parsear(&mut self) -> Result<Programa, String> {
        let mut declaraciones = Vec::new();
        
        while !self.fin() {
            declaraciones.push(self.declaracion()?);
        }
        
        Ok(Programa { declaraciones })
    }
    
    /// Parsea una declaración de nivel superior
    fn declaracion(&mut self) -> Result<Declaracion, String> {
        match &self.token_actual().tipo {
            TipoToken::Escena => {
                let escena = self.escena()?;
                Ok(Declaracion::Escena(escena))
            }
            TipoToken::Funcion => {
                let funcion = self.funcion()?;
                Ok(Declaracion::Funcion(funcion))
            }
            _ => Err(format!(
                "Se esperaba 'escena' o 'funcion', se encontró {} en {}",
                self.token_actual().tipo,
                self.token_actual().pos
            )),
        }
    }
    
    /// Parsea una escena
    fn escena(&mut self) -> Result<Escena, String> {
        let pos = self.token_actual().pos.clone();
        self.consumir(TipoToken::Escena, "Se esperaba 'escena'")?;
        
        let nombre = match &self.token_actual().tipo {
            TipoToken::Identificador(id) => {
                let n = id.clone();
                self.avanzar();
                n
            }
            _ => return Err(format!(
                "Se esperaba nombre de escena, se encontró {} en {}",
                self.token_actual().tipo,
                self.token_actual().pos
            )),
        };
        
        self.consumir(TipoToken::LlaveAbre, "Se esperaba '{' después del nombre de escena")?;
        
        let mut cuerpo = Vec::new();
        while !self.verificar(&TipoToken::LlaveCierra) && !self.fin() {
            cuerpo.push(self.sentencia()?);
        }
        
        self.consumir(TipoToken::LlaveCierra, "Se esperaba '}' al final de escena")?;
        
        Ok(Escena { nombre, cuerpo, pos })
    }
    
    /// Parsea una función
    fn funcion(&mut self) -> Result<Funcion, String> {
        let pos = self.token_actual().pos.clone();
        self.consumir(TipoToken::Funcion, "Se esperaba 'funcion'")?;
        
        let nombre = match &self.token_actual().tipo {
            TipoToken::Identificador(id) => {
                let n = id.clone();
                self.avanzar();
                n
            }
            _ => return Err(format!(
                "Se esperaba nombre de función, se encontró {} en {}",
                self.token_actual().tipo,
                self.token_actual().pos
            )),
        };
        
        self.consumir(TipoToken::ParentesisAbre, "Se esperaba '(' después del nombre de función")?;
        
        let mut parametros = Vec::new();
        if !self.verificar(&TipoToken::ParentesisCierra) {
            loop {
                let nombre_param = match &self.token_actual().tipo {
                    TipoToken::Identificador(id) => {
                        let n = id.clone();
                        self.avanzar();
                        n
                    }
                    _ => return Err(format!(
                        "Se esperaba nombre de parámetro, se encontró {} en {}",
                        self.token_actual().tipo,
                        self.token_actual().pos
                    )),
                };
                
                self.consumir(TipoToken::DosPuntos, "Se esperaba ':' después del nombre de parámetro")?;
                
                let tipo_dato = self.tipo()?;
                
                parametros.push(Parametro {
                    nombre: nombre_param,
                    tipo_dato,
                });
                
                if !self.coincidir(&TipoToken::Coma) {
                    break;
                }
            }
        }
        
        self.consumir(TipoToken::ParentesisCierra, "Se esperaba ')' después de parámetros")?;
        
        let tipo_retorno = if self.coincidir(&TipoToken::DosPuntos) {
            Some(self.tipo()?)
        } else {
            None
        };
        
        self.consumir(TipoToken::LlaveAbre, "Se esperaba '{' antes del cuerpo de función")?;
        
        let mut cuerpo = Vec::new();
        while !self.verificar(&TipoToken::LlaveCierra) && !self.fin() {
            cuerpo.push(self.sentencia()?);
        }
        
        self.consumir(TipoToken::LlaveCierra, "Se esperaba '}' al final de función")?;
        
        Ok(Funcion {
            nombre,
            parametros,
            tipo_retorno,
            cuerpo,
            pos,
        })
    }
    
    /// Parsea un tipo de dato
    fn tipo(&mut self) -> Result<Tipo, String> {
        let tipo = match &self.token_actual().tipo {
            TipoToken::TipoNumero => Tipo::Numero,
            TipoToken::TipoDecimal => Tipo::Decimal,
            TipoToken::TipoTexto => Tipo::Texto,
            TipoToken::TipoBooleano => Tipo::Booleano,
            TipoToken::TipoVector3 => Tipo::Vector3,
            TipoToken::TipoPose => Tipo::Pose,
            TipoToken::TipoMano => Tipo::Mano,
            TipoToken::TipoControlador => Tipo::Controlador,
            _ => return Err(format!(
                "Se esperaba un tipo de dato, se encontró {} en {}",
                self.token_actual().tipo,
                self.token_actual().pos
            )),
        };
        
        self.avanzar();
        Ok(tipo)
    }
    
    /// Parsea una sentencia
    fn sentencia(&mut self) -> Result<Sentencia, String> {
        match &self.token_actual().tipo {
            TipoToken::Mostrar => self.sentencia_mostrar(),
            TipoToken::Var => self.sentencia_var(),
            TipoToken::Si => self.sentencia_si(),
            TipoToken::Mientras => self.sentencia_mientras(),
            TipoToken::Para => self.sentencia_para(),
            TipoToken::Retornar => self.sentencia_retornar(),
            TipoToken::Identificador(_) => {
                // Puede ser asignación o expresión
                let pos_guardada = self.actual;
                let nombre = match &self.token_actual().tipo {
                    TipoToken::Identificador(id) => id.clone(),
                    _ => unreachable!(),
                };
                self.avanzar();
                
                if self.verificar(&TipoToken::Asignacion) {
                    self.avanzar();
                    let valor = self.expresion()?;
                    Ok(Sentencia::Asignacion(nombre, valor, self.token_en(pos_guardada).pos.clone()))
                } else {
                    // Es una expresión (probablemente llamada a función)
                    self.actual = pos_guardada;
                    let expr = self.expresion()?;
                    Ok(Sentencia::Expresion(expr))
                }
            }
            _ => {
                let expr = self.expresion()?;
                Ok(Sentencia::Expresion(expr))
            }
        }
    }
    
    /// Parsea sentencia mostrar
    fn sentencia_mostrar(&mut self) -> Result<Sentencia, String> {
        let pos = self.token_actual().pos.clone();
        self.consumir(TipoToken::Mostrar, "Se esperaba 'mostrar'")?;
        let expr = self.expresion()?;
        Ok(Sentencia::Mostrar(expr, pos))
    }
    
    /// Parsea sentencia var
    fn sentencia_var(&mut self) -> Result<Sentencia, String> {
        let pos = self.token_actual().pos.clone();
        self.consumir(TipoToken::Var, "Se esperaba 'var'")?;
        
        let nombre = match &self.token_actual().tipo {
            TipoToken::Identificador(id) => {
                let n = id.clone();
                self.avanzar();
                n
            }
            _ => return Err(format!(
                "Se esperaba nombre de variable, se encontró {} en {}",
                self.token_actual().tipo,
                self.token_actual().pos
            )),
        };
        
        self.consumir(TipoToken::Asignacion, "Se esperaba '=' después del nombre de variable")?;
        
        let valor = self.expresion()?;
        
        Ok(Sentencia::Var(nombre, valor, pos))
    }
    
    /// Parsea sentencia si
    fn sentencia_si(&mut self) -> Result<Sentencia, String> {
        let pos = self.token_actual().pos.clone();
        self.consumir(TipoToken::Si, "Se esperaba 'si'")?;
        
        let condicion = self.expresion()?;
        
        self.consumir(TipoToken::LlaveAbre, "Se esperaba '{' después de condición")?;
        
        let mut entonces = Vec::new();
        while !self.verificar(&TipoToken::LlaveCierra) && !self.fin() {
            entonces.push(self.sentencia()?);
        }
        
        self.consumir(TipoToken::LlaveCierra, "Se esperaba '}' al final del bloque 'si'")?;
        
        let sino = if self.coincidir(&TipoToken::Sino) {
            self.consumir(TipoToken::LlaveAbre, "Se esperaba '{' después de 'sino'")?;
            
            let mut bloque_sino = Vec::new();
            while !self.verificar(&TipoToken::LlaveCierra) && !self.fin() {
                bloque_sino.push(self.sentencia()?);
            }
            
            self.consumir(TipoToken::LlaveCierra, "Se esperaba '}' al final del bloque 'sino'")?;
            
            Some(bloque_sino)
        } else {
            None
        };
        
        Ok(Sentencia::Si(condicion, entonces, sino, pos))
    }
    
    /// Parsea sentencia mientras
    fn sentencia_mientras(&mut self) -> Result<Sentencia, String> {
        let pos = self.token_actual().pos.clone();
        self.consumir(TipoToken::Mientras, "Se esperaba 'mientras'")?;
        
        let condicion = self.expresion()?;
        
        self.consumir(TipoToken::LlaveAbre, "Se esperaba '{' después de condición")?;
        
        let mut cuerpo = Vec::new();
        while !self.verificar(&TipoToken::LlaveCierra) && !self.fin() {
            cuerpo.push(self.sentencia()?);
        }
        
        self.consumir(TipoToken::LlaveCierra, "Se esperaba '}' al final del bloque 'mientras'")?;
        
        Ok(Sentencia::Mientras(condicion, cuerpo, pos))
    }
    
    /// Parsea sentencia para
    fn sentencia_para(&mut self) -> Result<Sentencia, String> {
        let pos = self.token_actual().pos.clone();
        self.consumir(TipoToken::Para, "Se esperaba 'para'")?;
        
        let variable = match &self.token_actual().tipo {
            TipoToken::Identificador(id) => {
                let n = id.clone();
                self.avanzar();
                n
            }
            _ => return Err(format!(
                "Se esperaba nombre de variable, se encontró {} en {}",
                self.token_actual().tipo,
                self.token_actual().pos
            )),
        };
        
        self.consumir(TipoToken::Asignacion, "Se esperaba '=' después del nombre de variable")?;
        
        let inicio = self.expresion()?;
        
        self.consumir(TipoToken::Coma, "Se esperaba ',' entre inicio y fin")?;
        
        let fin = self.expresion()?;
        
        self.consumir(TipoToken::LlaveAbre, "Se esperaba '{' después del rango")?;
        
        let mut cuerpo = Vec::new();
        while !self.verificar(&TipoToken::LlaveCierra) && !self.fin() {
            cuerpo.push(self.sentencia()?);
        }
        
        self.consumir(TipoToken::LlaveCierra, "Se esperaba '}' al final del bloque 'para'")?;
        
        Ok(Sentencia::Para(variable, inicio, fin, cuerpo, pos))
    }
    
    /// Parsea sentencia retornar
    fn sentencia_retornar(&mut self) -> Result<Sentencia, String> {
        let pos = self.token_actual().pos.clone();
        self.consumir(TipoToken::Retornar, "Se esperaba 'retornar'")?;
        
        let valor = if self.verificar(&TipoToken::LlaveCierra) {
            None
        } else {
            Some(self.expresion()?)
        };
        
        Ok(Sentencia::Retornar(valor, pos))
    }
    
    /// Parsea una expresión
    fn expresion(&mut self) -> Result<Expresion, String> {
        self.o_logico()
    }
    
    /// Parsea OR lógico
    fn o_logico(&mut self) -> Result<Expresion, String> {
        let mut expr = self.y_logico()?;
        
        while self.coincidir(&TipoToken::O) {
            let pos = self.token_anterior().pos.clone();
            let derecha = self.y_logico()?;
            expr = Expresion::Binaria(Box::new(expr), OperadorBinario::O, Box::new(derecha), pos);
        }
        
        Ok(expr)
    }
    
    /// Parsea AND lógico
    fn y_logico(&mut self) -> Result<Expresion, String> {
        let mut expr = self.igualdad()?;
        
        while self.coincidir(&TipoToken::Y) {
            let pos = self.token_anterior().pos.clone();
            let derecha = self.igualdad()?;
            expr = Expresion::Binaria(Box::new(expr), OperadorBinario::Y, Box::new(derecha), pos);
        }
        
        Ok(expr)
    }
    
    /// Parsea igualdad y diferencia
    fn igualdad(&mut self) -> Result<Expresion, String> {
        let mut expr = self.comparacion()?;
        
        while let Some(op) = self.coincidir_varios(&[TipoToken::Igual, TipoToken::Diferente]) {
            let pos = self.token_anterior().pos.clone();
            let operador = match op {
                TipoToken::Igual => OperadorBinario::Igual,
                TipoToken::Diferente => OperadorBinario::Diferente,
                _ => unreachable!(),
            };
            let derecha = self.comparacion()?;
            expr = Expresion::Binaria(Box::new(expr), operador, Box::new(derecha), pos);
        }
        
        Ok(expr)
    }
    
    /// Parsea comparaciones
    fn comparacion(&mut self) -> Result<Expresion, String> {
        let mut expr = self.suma_resta()?;
        
        while let Some(op) = self.coincidir_varios(&[
            TipoToken::Menor,
            TipoToken::MenorIgual,
            TipoToken::Mayor,
            TipoToken::MayorIgual,
        ]) {
            let pos = self.token_anterior().pos.clone();
            let operador = match op {
                TipoToken::Menor => OperadorBinario::Menor,
                TipoToken::MenorIgual => OperadorBinario::MenorIgual,
                TipoToken::Mayor => OperadorBinario::Mayor,
                TipoToken::MayorIgual => OperadorBinario::MayorIgual,
                _ => unreachable!(),
            };
            let derecha = self.suma_resta()?;
            expr = Expresion::Binaria(Box::new(expr), operador, Box::new(derecha), pos);
        }
        
        Ok(expr)
    }
    
    /// Parsea suma y resta
    fn suma_resta(&mut self) -> Result<Expresion, String> {
        let mut expr = self.multiplicacion_division()?;
        
        while let Some(op) = self.coincidir_varios(&[TipoToken::Suma, TipoToken::Resta]) {
            let pos = self.token_anterior().pos.clone();
            let operador = match op {
                TipoToken::Suma => OperadorBinario::Suma,
                TipoToken::Resta => OperadorBinario::Resta,
                _ => unreachable!(),
            };
            let derecha = self.multiplicacion_division()?;
            expr = Expresion::Binaria(Box::new(expr), operador, Box::new(derecha), pos);
        }
        
        Ok(expr)
    }
    
    /// Parsea multiplicación, división y módulo
    fn multiplicacion_division(&mut self) -> Result<Expresion, String> {
        let mut expr = self.unario()?;
        
        while let Some(op) = self.coincidir_varios(&[
            TipoToken::Multiplicacion,
            TipoToken::Division,
            TipoToken::Modulo,
        ]) {
            let pos = self.token_anterior().pos.clone();
            let operador = match op {
                TipoToken::Multiplicacion => OperadorBinario::Multiplicacion,
                TipoToken::Division => OperadorBinario::Division,
                TipoToken::Modulo => OperadorBinario::Modulo,
                _ => unreachable!(),
            };
            let derecha = self.unario()?;
            expr = Expresion::Binaria(Box::new(expr), operador, Box::new(derecha), pos);
        }
        
        Ok(expr)
    }
    
    /// Parsea operadores unarios
    fn unario(&mut self) -> Result<Expresion, String> {
        if let Some(op) = self.coincidir_varios(&[TipoToken::No, TipoToken::Resta]) {
            let pos = self.token_anterior().pos.clone();
            let operador = match op {
                TipoToken::No => OperadorUnario::No,
                TipoToken::Resta => OperadorUnario::Negacion,
                _ => unreachable!(),
            };
            let derecha = self.unario()?;
            Ok(Expresion::Unaria(operador, Box::new(derecha), pos))
        } else {
            self.llamada()
        }
    }
    
    /// Parsea llamadas a función
    fn llamada(&mut self) -> Result<Expresion, String> {
        let expr = self.primario()?;
        
        if let Expresion::Variable(nombre, pos) = expr {
            if self.verificar(&TipoToken::ParentesisAbre) {
                self.avanzar();
                
                let mut argumentos = Vec::new();
                if !self.verificar(&TipoToken::ParentesisCierra) {
                    loop {
                        argumentos.push(self.expresion()?);
                        if !self.coincidir(&TipoToken::Coma) {
                            break;
                        }
                    }
                }
                
                self.consumir(TipoToken::ParentesisCierra, "Se esperaba ')' después de argumentos")?;
                
                Ok(Expresion::Llamada(nombre, argumentos, pos))
            } else {
                Ok(Expresion::Variable(nombre, pos))
            }
        } else {
            Ok(expr)
        }
    }
    
    /// Parsea expresiones primarias
    fn primario(&mut self) -> Result<Expresion, String> {
        let token = self.token_actual().clone();
        
        match &token.tipo {
            TipoToken::Numero(n) => {
                self.avanzar();
                Ok(Expresion::Numero(*n, token.pos))
            }
            TipoToken::Decimal(d) => {
                self.avanzar();
                Ok(Expresion::Decimal(*d, token.pos))
            }
            TipoToken::Texto(s) => {
                self.avanzar();
                Ok(Expresion::Texto(s.clone(), token.pos))
            }
            TipoToken::Verdadero => {
                self.avanzar();
                Ok(Expresion::Booleano(true, token.pos))
            }
            TipoToken::Falso => {
                self.avanzar();
                Ok(Expresion::Booleano(false, token.pos))
            }
            TipoToken::Identificador(id) => {
                self.avanzar();
                Ok(Expresion::Variable(id.clone(), token.pos))
            }
            TipoToken::ParentesisAbre => {
                self.avanzar();
                let expr = self.expresion()?;
                self.consumir(TipoToken::ParentesisCierra, "Se esperaba ')' después de expresión")?;
                Ok(expr)
            }
            _ => Err(format!(
                "Expresión inesperada: {} en {}",
                token.tipo, token.pos
            )),
        }
    }
    
    // Utilidades
    
    fn token_actual(&self) -> &Token {
        &self.tokens[self.actual]
    }
    
    fn token_anterior(&self) -> &Token {
        &self.tokens[self.actual - 1]
    }
    
    fn token_en(&self, pos: usize) -> &Token {
        &self.tokens[pos]
    }
    
    fn fin(&self) -> bool {
        matches!(self.token_actual().tipo, TipoToken::Eof)
    }
    
    fn avanzar(&mut self) {
        if !self.fin() {
            self.actual += 1;
        }
    }
    
    fn verificar(&self, tipo: &TipoToken) -> bool {
        if self.fin() {
            false
        } else {
            std::mem::discriminant(&self.token_actual().tipo) == std::mem::discriminant(tipo)
        }
    }
    
    fn coincidir(&mut self, tipo: &TipoToken) -> bool {
        if self.verificar(tipo) {
            self.avanzar();
            true
        } else {
            false
        }
    }
    
    fn coincidir_varios(&mut self, tipos: &[TipoToken]) -> Option<TipoToken> {
        for tipo in tipos {
            if self.verificar(tipo) {
                let t = self.token_actual().tipo.clone();
                self.avanzar();
                return Some(t);
            }
        }
        None
    }
    
    fn consumir(&mut self, tipo: TipoToken, mensaje: &str) -> Result<(), String> {
        if self.verificar(&tipo) {
            self.avanzar();
            Ok(())
        } else {
            Err(format!(
                "{}, se encontró {} en {}",
                mensaje,
                self.token_actual().tipo,
                self.token_actual().pos
            ))
        }
    }
}
