// ejecutor.rs - Intérprete/Ejecutor para V-Code
// Ejecuta el AST

use crate::ast::*;
use std::collections::HashMap;
use std::fmt;

/// Valor en tiempo de ejecución
#[derive(Debug, Clone, PartialEq)]
pub enum Valor {
    Numero(i64),
    Decimal(f64),
    Texto(String),
    Booleano(bool),
    Nulo,
}

impl fmt::Display for Valor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Valor::Numero(n) => write!(f, "{}", n),
            Valor::Decimal(d) => write!(f, "{}", d),
            Valor::Texto(s) => write!(f, "{}", s),
            Valor::Booleano(b) => write!(f, "{}", if *b { "verdadero" } else { "falso" }),
            Valor::Nulo => write!(f, "nulo"),
        }
    }
}

impl Valor {
    /// Convierte a booleano para condiciones
    pub fn es_verdadero(&self) -> bool {
        match self {
            Valor::Booleano(b) => *b,
            Valor::Nulo => false,
            Valor::Numero(n) => *n != 0,
            Valor::Decimal(d) => *d != 0.0,
            Valor::Texto(s) => !s.is_empty(),
        }
    }
    
    /// Convierte a número si es posible
    pub fn a_numero(&self) -> Result<i64, String> {
        match self {
            Valor::Numero(n) => Ok(*n),
            Valor::Decimal(d) => Ok(*d as i64),
            _ => Err(format!("No se puede convertir {} a número", self)),
        }
    }
    
    /// Convierte a decimal si es posible
    pub fn a_decimal(&self) -> Result<f64, String> {
        match self {
            Valor::Numero(n) => Ok(*n as f64),
            Valor::Decimal(d) => Ok(*d),
            _ => Err(format!("No se puede convertir {} a decimal", self)),
        }
    }
}

/// Entorno de ejecución (scope)
#[derive(Debug, Clone)]
pub struct Entorno {
    variables: HashMap<String, Valor>,
    padre: Option<Box<Entorno>>,
}

impl Entorno {
    pub fn new() -> Self {
        Entorno {
            variables: HashMap::new(),
            padre: None,
        }
    }
    
    pub fn con_padre(padre: Entorno) -> Self {
        Entorno {
            variables: HashMap::new(),
            padre: Some(Box::new(padre)),
        }
    }
    
    pub fn definir(&mut self, nombre: String, valor: Valor) {
        self.variables.insert(nombre, valor);
    }
    
    pub fn obtener(&self, nombre: &str) -> Option<Valor> {
        if let Some(valor) = self.variables.get(nombre) {
            Some(valor.clone())
        } else if let Some(padre) = &self.padre {
            padre.obtener(nombre)
        } else {
            None
        }
    }
    
    pub fn asignar(&mut self, nombre: &str, valor: Valor) -> Result<(), String> {
        if self.variables.contains_key(nombre) {
            self.variables.insert(nombre.to_string(), valor);
            Ok(())
        } else if let Some(padre) = &mut self.padre {
            padre.asignar(nombre, valor)
        } else {
            Err(format!("Variable '{}' no definida", nombre))
        }
    }
}

/// Definición de función en tiempo de ejecución
#[derive(Debug, Clone)]
pub struct FuncionDefinida {
    pub parametros: Vec<Parametro>,
    pub cuerpo: Vec<Sentencia>,
}

/// Ejecutor
pub struct Ejecutor {
    entorno: Entorno,
    funciones: HashMap<String, FuncionDefinida>,
    salida: Vec<String>,
}

impl Ejecutor {
    pub fn new() -> Self {
        Ejecutor {
            entorno: Entorno::new(),
            funciones: HashMap::new(),
            salida: Vec::new(),
        }
    }
    
    /// Ejecuta un programa
    pub fn ejecutar(&mut self, programa: &Programa) -> Result<(), String> {
        // Primera pasada: registrar funciones
        for declaracion in &programa.declaraciones {
            if let Declaracion::Funcion(funcion) = declaracion {
                self.funciones.insert(
                    funcion.nombre.clone(),
                    FuncionDefinida {
                        parametros: funcion.parametros.clone(),
                        cuerpo: funcion.cuerpo.clone(),
                    },
                );
            }
        }
        
        // Segunda pasada: ejecutar escenas
        for declaracion in &programa.declaraciones {
            if let Declaracion::Escena(escena) = declaracion {
                self.ejecutar_escena(escena)?;
            }
        }
        
        Ok(())
    }
    
    /// Ejecuta una escena
    fn ejecutar_escena(&mut self, escena: &Escena) -> Result<(), String> {
        for sentencia in &escena.cuerpo {
            self.ejecutar_sentencia(sentencia)?;
        }
        Ok(())
    }
    
    /// Ejecuta una sentencia
    fn ejecutar_sentencia(&mut self, sentencia: &Sentencia) -> Result<Option<Valor>, String> {
        match sentencia {
            Sentencia::Mostrar(expr, pos) => {
                let valor = self.evaluar_expresion(expr)?;
                let texto = format!("📺 {}", valor);
                self.salida.push(texto.clone());
                println!("{}", texto);
                Ok(None)
            }
            
            Sentencia::Var(nombre, expr, pos) => {
                let valor = self.evaluar_expresion(expr)?;
                self.entorno.definir(nombre.clone(), valor);
                Ok(None)
            }
            
            Sentencia::Asignacion(nombre, expr, pos) => {
                let valor = self.evaluar_expresion(expr)?;
                self.entorno.asignar(nombre, valor).map_err(|e| {
                    format!("{} en {}", e, pos)
                })?;
                Ok(None)
            }
            
            Sentencia::Si(condicion, entonces, sino, pos) => {
                let valor_condicion = self.evaluar_expresion(condicion)?;
                
                if valor_condicion.es_verdadero() {
                    for sentencia in entonces {
                        if let Some(retorno) = self.ejecutar_sentencia(sentencia)? {
                            return Ok(Some(retorno));
                        }
                    }
                } else if let Some(bloque_sino) = sino {
                    for sentencia in bloque_sino {
                        if let Some(retorno) = self.ejecutar_sentencia(sentencia)? {
                            return Ok(Some(retorno));
                        }
                    }
                }
                
                Ok(None)
            }
            
            Sentencia::Mientras(condicion, cuerpo, pos) => {
                while self.evaluar_expresion(condicion)?.es_verdadero() {
                    for sentencia in cuerpo {
                        if let Some(retorno) = self.ejecutar_sentencia(sentencia)? {
                            return Ok(Some(retorno));
                        }
                    }
                }
                Ok(None)
            }
            
            Sentencia::Para(variable, inicio, fin, cuerpo, pos) => {
                let valor_inicio = self.evaluar_expresion(inicio)?.a_numero().map_err(|e| {
                    format!("Inicio de 'para' debe ser número: {} en {}", e, pos)
                })?;
                
                let valor_fin = self.evaluar_expresion(fin)?.a_numero().map_err(|e| {
                    format!("Fin de 'para' debe ser número: {} en {}", e, pos)
                })?;
                
                for i in valor_inicio..valor_fin {
                    self.entorno.definir(variable.clone(), Valor::Numero(i));
                    
                    for sentencia in cuerpo {
                        if let Some(retorno) = self.ejecutar_sentencia(sentencia)? {
                            return Ok(Some(retorno));
                        }
                    }
                }
                
                Ok(None)
            }
            
            Sentencia::Retornar(valor_opt, pos) => {
                let valor = if let Some(expr) = valor_opt {
                    self.evaluar_expresion(expr)?
                } else {
                    Valor::Nulo
                };
                Ok(Some(valor))
            }
            
            Sentencia::Expresion(expr) => {
                self.evaluar_expresion(expr)?;
                Ok(None)
            }
        }
    }
    
    /// Evalúa una expresión
    fn evaluar_expresion(&mut self, expr: &Expresion) -> Result<Valor, String> {
        match expr {
            Expresion::Numero(n, _) => Ok(Valor::Numero(*n)),
            
            Expresion::Decimal(d, _) => Ok(Valor::Decimal(*d)),
            
            Expresion::Texto(s, _) => Ok(Valor::Texto(s.clone())),
            
            Expresion::Booleano(b, _) => Ok(Valor::Booleano(*b)),
            
            Expresion::Variable(nombre, pos) => {
                self.entorno.obtener(nombre).ok_or_else(|| {
                    format!("Variable '{}' no definida en {}", nombre, pos)
                })
            }
            
            Expresion::Binaria(izq, op, der, pos) => {
                let valor_izq = self.evaluar_expresion(izq)?;
                let valor_der = self.evaluar_expresion(der)?;
                
                self.aplicar_operador_binario(valor_izq, op, valor_der, pos)
            }
            
            Expresion::Unaria(op, expr, pos) => {
                let valor = self.evaluar_expresion(expr)?;
                
                match op {
                    OperadorUnario::Negacion => match valor {
                        Valor::Numero(n) => Ok(Valor::Numero(-n)),
                        Valor::Decimal(d) => Ok(Valor::Decimal(-d)),
                        _ => Err(format!("Operador '-' no aplicable a {} en {}", valor, pos)),
                    },
                    OperadorUnario::No => {
                        Ok(Valor::Booleano(!valor.es_verdadero()))
                    }
                }
            }
            
            Expresion::Llamada(nombre, argumentos, pos) => {
                self.llamar_funcion(nombre, argumentos, pos)
            }
        }
    }
    
    /// Aplica un operador binario
    fn aplicar_operador_binario(
        &self,
        izq: Valor,
        op: &OperadorBinario,
        der: Valor,
        pos: &Posicion,
    ) -> Result<Valor, String> {
        match op {
            OperadorBinario::Suma => match (&izq, &der) {
                (Valor::Numero(a), Valor::Numero(b)) => Ok(Valor::Numero(a + b)),
                (Valor::Decimal(a), Valor::Decimal(b)) => Ok(Valor::Decimal(a + b)),
                (Valor::Numero(a), Valor::Decimal(b)) => Ok(Valor::Decimal(*a as f64 + b)),
                (Valor::Decimal(a), Valor::Numero(b)) => Ok(Valor::Decimal(a + *b as f64)),
                (Valor::Texto(a), Valor::Texto(b)) => Ok(Valor::Texto(format!("{}{}", a, b))),
                (Valor::Texto(a), b) => Ok(Valor::Texto(format!("{}{}", a, b))),
                (a, Valor::Texto(b)) => Ok(Valor::Texto(format!("{}{}", a, b))),
                _ => Err(format!("Operador '+' no aplicable a {} y {} en {}", izq, der, pos)),
            },
            
            OperadorBinario::Resta => match (&izq, &der) {
                (Valor::Numero(a), Valor::Numero(b)) => Ok(Valor::Numero(a - b)),
                (Valor::Decimal(a), Valor::Decimal(b)) => Ok(Valor::Decimal(a - b)),
                (Valor::Numero(a), Valor::Decimal(b)) => Ok(Valor::Decimal(*a as f64 - b)),
                (Valor::Decimal(a), Valor::Numero(b)) => Ok(Valor::Decimal(a - *b as f64)),
                _ => Err(format!("Operador '-' no aplicable a {} y {} en {}", izq, der, pos)),
            },
            
            OperadorBinario::Multiplicacion => match (&izq, &der) {
                (Valor::Numero(a), Valor::Numero(b)) => Ok(Valor::Numero(a * b)),
                (Valor::Decimal(a), Valor::Decimal(b)) => Ok(Valor::Decimal(a * b)),
                (Valor::Numero(a), Valor::Decimal(b)) => Ok(Valor::Decimal(*a as f64 * b)),
                (Valor::Decimal(a), Valor::Numero(b)) => Ok(Valor::Decimal(a * *b as f64)),
                _ => Err(format!("Operador '*' no aplicable a {} y {} en {}", izq, der, pos)),
            },
            
            OperadorBinario::Division => match (&izq, &der) {
                (Valor::Numero(a), Valor::Numero(b)) => {
                    if *b == 0 {
                        Err(format!("División por cero en {}", pos))
                    } else {
                        Ok(Valor::Numero(a / b))
                    }
                }
                (Valor::Decimal(a), Valor::Decimal(b)) => {
                    if *b == 0.0 {
                        Err(format!("División por cero en {}", pos))
                    } else {
                        Ok(Valor::Decimal(a / b))
                    }
                }
                (Valor::Numero(a), Valor::Decimal(b)) => {
                    if *b == 0.0 {
                        Err(format!("División por cero en {}", pos))
                    } else {
                        Ok(Valor::Decimal(*a as f64 / b))
                    }
                }
                (Valor::Decimal(a), Valor::Numero(b)) => {
                    if *b == 0 {
                        Err(format!("División por cero en {}", pos))
                    } else {
                        Ok(Valor::Decimal(a / *b as f64))
                    }
                }
                _ => Err(format!("Operador '/' no aplicable a {} y {} en {}", izq, der, pos)),
            },
            
            OperadorBinario::Modulo => match (&izq, &der) {
                (Valor::Numero(a), Valor::Numero(b)) => {
                    if *b == 0 {
                        Err(format!("Módulo por cero en {}", pos))
                    } else {
                        Ok(Valor::Numero(a % b))
                    }
                }
                _ => Err(format!("Operador '%' no aplicable a {} y {} en {}", izq, der, pos)),
            },
            
            OperadorBinario::Igual => Ok(Valor::Booleano(izq == der)),
            
            OperadorBinario::Diferente => Ok(Valor::Booleano(izq != der)),
            
            OperadorBinario::Menor => match (&izq, &der) {
                (Valor::Numero(a), Valor::Numero(b)) => Ok(Valor::Booleano(a < b)),
                (Valor::Decimal(a), Valor::Decimal(b)) => Ok(Valor::Booleano(a < b)),
                (Valor::Numero(a), Valor::Decimal(b)) => Ok(Valor::Booleano((*a as f64) < *b)),
                (Valor::Decimal(a), Valor::Numero(b)) => Ok(Valor::Booleano(*a < (*b as f64))),
                _ => Err(format!("Operador '<' no aplicable a {} y {} en {}", izq, der, pos)),
            },
            
            OperadorBinario::MenorIgual => match (&izq, &der) {
                (Valor::Numero(a), Valor::Numero(b)) => Ok(Valor::Booleano(a <= b)),
                (Valor::Decimal(a), Valor::Decimal(b)) => Ok(Valor::Booleano(a <= b)),
                (Valor::Numero(a), Valor::Decimal(b)) => Ok(Valor::Booleano((*a as f64) <= *b)),
                (Valor::Decimal(a), Valor::Numero(b)) => Ok(Valor::Booleano(*a <= (*b as f64))),
                _ => Err(format!("Operador '<=' no aplicable a {} y {} en {}", izq, der, pos)),
            },
            
            OperadorBinario::Mayor => match (&izq, &der) {
                (Valor::Numero(a), Valor::Numero(b)) => Ok(Valor::Booleano(a > b)),
                (Valor::Decimal(a), Valor::Decimal(b)) => Ok(Valor::Booleano(a > b)),
                (Valor::Numero(a), Valor::Decimal(b)) => Ok(Valor::Booleano((*a as f64) > *b)),
                (Valor::Decimal(a), Valor::Numero(b)) => Ok(Valor::Booleano(*a > (*b as f64))),
                _ => Err(format!("Operador '>' no aplicable a {} y {} en {}", izq, der, pos)),
            },
            
            OperadorBinario::MayorIgual => match (&izq, &der) {
                (Valor::Numero(a), Valor::Numero(b)) => Ok(Valor::Booleano(a >= b)),
                (Valor::Decimal(a), Valor::Decimal(b)) => Ok(Valor::Booleano(a >= b)),
                (Valor::Numero(a), Valor::Decimal(b)) => Ok(Valor::Booleano((*a as f64) >= *b)),
                (Valor::Decimal(a), Valor::Numero(b)) => Ok(Valor::Booleano(*a >= (*b as f64))),
                _ => Err(format!("Operador '>=' no aplicable a {} y {} en {}", izq, der, pos)),
            },
            
            OperadorBinario::Y => {
                Ok(Valor::Booleano(izq.es_verdadero() && der.es_verdadero()))
            }
            
            OperadorBinario::O => {
                Ok(Valor::Booleano(izq.es_verdadero() || der.es_verdadero()))
            }
        }
    }
    
    /// Llama a una función
    fn llamar_funcion(
        &mut self,
        nombre: &str,
        argumentos: &[Expresion],
        pos: &Posicion,
    ) -> Result<Valor, String> {
        let funcion = self.funciones.get(nombre).cloned().ok_or_else(|| {
            format!("Función '{}' no definida en {}", nombre, pos)
        })?;
        
        if argumentos.len() != funcion.parametros.len() {
            return Err(format!(
                "Función '{}' espera {} argumentos, se proporcionaron {} en {}",
                nombre,
                funcion.parametros.len(),
                argumentos.len(),
                pos
            ));
        }
        
        // Crear nuevo entorno para la función
        let entorno_anterior = self.entorno.clone();
        self.entorno = Entorno::con_padre(entorno_anterior.clone());
        
        // Evaluar y asignar argumentos
        for (i, arg) in argumentos.iter().enumerate() {
            let valor = self.evaluar_expresion(arg)?;
            self.entorno.definir(funcion.parametros[i].nombre.clone(), valor);
        }
        
        // Ejecutar cuerpo de la función
        let mut resultado = Valor::Nulo;
        for sentencia in &funcion.cuerpo {
            if let Some(retorno) = self.ejecutar_sentencia(sentencia)? {
                resultado = retorno;
                break;
            }
        }
        
        // Restaurar entorno
        self.entorno = entorno_anterior;
        
        Ok(resultado)
    }
    
    /// Obtiene la salida generada
    pub fn obtener_salida(&self) -> &[String] {
        &self.salida
    }
}
