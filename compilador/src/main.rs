// main.rs - Punto de entrada del compilador V-Code

mod ast;
mod lexer;
mod parser;
mod ejecutor;

use std::env;
use std::fs;
use std::process;

use lexer::Lexer;
use parser::Parser;
use ejecutor::Ejecutor;

fn main() {
    // Banner
    println!("🚀 V-Code v0.1.0 - Lenguaje de Programación para VR");
    println!("   Desarrollado por Paolo Gonzalez");
    println!("   https://github.com/PaoloGonzalez776/V-Code");
    println!();
    
    // Obtener argumentos
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("❌ Error: No se especificó archivo de entrada");
        eprintln!();
        eprintln!("Uso: {} <archivo.vc>", args[0]);
        eprintln!();
        eprintln!("Ejemplo:");
        eprintln!("  {} ejemplos/hola.vc", args[0]);
        process::exit(1);
    }
    
    let archivo = &args[1];
    
    // Leer archivo
    println!("📂 Leyendo archivo: {}", archivo);
    let codigo = match fs::read_to_string(archivo) {
        Ok(contenido) => contenido,
        Err(e) => {
            eprintln!("❌ Error al leer archivo '{}': {}", archivo, e);
            process::exit(1);
        }
    };
    
    // Ejecutar
    if let Err(e) = ejecutar(&codigo) {
        eprintln!();
        eprintln!("❌ Error de ejecución:");
        eprintln!("   {}", e);
        process::exit(1);
    }
    
    println!();
    println!("✅ Ejecución completada exitosamente");
}

fn ejecutar(codigo: &str) -> Result<(), String> {
    // Fase 1: Análisis léxico
    println!("🔍 Fase 1: Análisis léxico...");
    let mut lexer = Lexer::new(codigo);
    let tokens = lexer.tokenizar().map_err(|e| {
        format!("Error léxico: {}", e)
    })?;
    
    println!("   ✓ {} tokens generados", tokens.len());
    
    // Fase 2: Análisis sintáctico
    println!("🔍 Fase 2: Análisis sintáctico...");
    let mut parser = Parser::new(tokens);
    let programa = parser.parsear().map_err(|e| {
        format!("Error sintáctico: {}", e)
    })?;
    
    println!("   ✓ AST generado correctamente");
    
    // Fase 3: Ejecución
    println!("🔍 Fase 3: Ejecución...");
    println!();
    println!("─────────────────────────────────");
    println!("📺 Salida del programa:");
    println!("─────────────────────────────────");
    println!();
    
    let mut ejecutor = Ejecutor::new();
    ejecutor.ejecutar(&programa).map_err(|e| {
        format!("Error de ejecución: {}", e)
    })?;
    
    println!();
    println!("─────────────────────────────────");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hola_mundo() {
        let codigo = r#"
            escena Principal {
                mostrar "Hola, V-Code"
            }
        "#;
        
        assert!(ejecutar(codigo).is_ok());
    }
    
    #[test]
    fn test_variables() {
        let codigo = r#"
            escena Principal {
                var x = 10
                var y = 20
                var suma = x + y
                mostrar suma
            }
        "#;
        
        assert!(ejecutar(codigo).is_ok());
    }
    
    #[test]
    fn test_funciones() {
        let codigo = r#"
            funcion sumar(a: numero, b: numero): numero {
                retornar a + b
            }
            
            escena Principal {
                var resultado = sumar(5, 3)
                mostrar resultado
            }
        "#;
        
        assert!(ejecutar(codigo).is_ok());
    }
    
    #[test]
    fn test_control_flujo() {
        let codigo = r#"
            escena Principal {
                var edad = 25
                
                si edad > 18 {
                    mostrar "Mayor de edad"
                } sino {
                    mostrar "Menor de edad"
                }
            }
        "#;
        
        assert!(ejecutar(codigo).is_ok());
    }
    
    #[test]
    fn test_bucle_para() {
        let codigo = r#"
            escena Principal {
                para i = 0, 5 {
                    mostrar i
                }
            }
        "#;
        
        assert!(ejecutar(codigo).is_ok());
    }
}
