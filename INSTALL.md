# Guía de Instalación de V-Code

Esta guía te ayudará a instalar y configurar V-Code en tu sistema.

## 📋 Requisitos Previos

### Sistema Operativo

V-Code funciona en:
- ✅ Linux (Ubuntu 20.04+, Fedora 35+, Arch Linux)
- ✅ macOS (11.0+)
- ✅ Windows 10/11 (con WSL2 recomendado)

### Software Requerido

1. **Rust** (versión 1.70 o superior)
2. **Git**

## 🦀 Instalar Rust

### Linux y macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Sigue las instrucciones en pantalla y luego:

```bash
source $HOME/.cargo/env
```

### Windows

Descarga e instala `rustup-init.exe` desde: https://rustup.rs/

### Verificar Instalación

```bash
rustc --version
cargo --version
```

Deberías ver algo como:
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

## 📥 Clonar el Repositorio

```bash
git clone https://github.com/PaoloGonzalez776/V-Code-Librerias.git
cd V-Code-Librerias
```

## 🔨 Compilar V-Code

### Modo Debug (Desarrollo)

```bash
cd compilador
cargo build
```

El binario se creará en: `target/debug/vcode`

### Modo Release (Producción)

```bash
cd compilador
cargo build --release
```

El binario optimizado se creará en: `target/release/vcode`

**Nota**: El modo release es significativamente más rápido (2-10x) pero tarda más en compilar.

## ✅ Verificar Instalación

### Ejecutar Tests

```bash
cd compilador
cargo test
```

Deberías ver:
```
running 3 tests
test tests::test_pipeline_basico ... ok
test tests::test_variables_y_operaciones ... ok
test tests::test_condicionales ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Ejecutar Ejemplo

```bash
cargo run ../ejemplos/hola.vc
```

Salida esperada:
```
🚀 V-Code v0.1.0 - Lenguaje de Programación para VR

📂 Cargando archivo: ../ejemplos/hola.vc
📝 Código fuente cargado (XXX bytes)

🔍 Fase 1: Análisis léxico (Lexer)
   ✓ XX tokens generados

🔧 Fase 2: Análisis sintáctico (Parser)
   ✓ AST generado correctamente
   ✓ 1 escena(s) encontrada(s)
   ✓ 0 función(es) encontrada(s)

⚡ Fase 3: Ejecución (Intérprete)
─────────────────────────────────────────
🎬 Ejecutando escena: Principal
📺 V-Code ha nacido
📺 Lenguaje de programación nativo para VR
─────────────────────────────────────────

✅ Ejecución completada exitosamente
```

## 🚀 Instalar Globalmente (Opcional)

Para usar `vcode` desde cualquier ubicación:

```bash
cd compilador
cargo install --path .
```

Ahora puedes ejecutar:

```bash
vcode ~/proyectos/mi_app.vc
```

## 🔧 Solución de Problemas

### Error: "cargo: command not found"

**Solución**: Rust no está instalado o no está en el PATH.

```bash
# Agregar Cargo al PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Para hacerlo permanente, agregar a ~/.bashrc o ~/.zshrc
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Error de Compilación en Windows

**Solución**: Instala Visual Studio Build Tools o usa WSL2.

#### Opción 1: Visual Studio Build Tools

1. Descarga desde: https://visualstudio.microsoft.com/downloads/
2. Instala "Desktop development with C++"
3. Reinicia el terminal

#### Opción 2: WSL2 (Recomendado)

```powershell
wsl --install
```

Luego sigue las instrucciones para Linux dentro de WSL2.

### Error: "linking with `cc` failed"

**Solución en Linux**: Instala herramientas de compilación.

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install build-essential
```

#### Fedora
```bash
sudo dnf install gcc
```

#### Arch Linux
```bash
sudo pacman -S base-devel
```

### Compilación Lenta

**Solución**: Usa el compilador de optimización incremental.

Crea `~/.cargo/config.toml`:

```toml
[build]
incremental = true
```

O compila solo en release cuando sea necesario:

```bash
cargo build --release
```

## 📚 Configuración de Editor

### Visual Studio Code

1. Instala las extensiones:
   - `rust-analyzer` (análisis de código Rust)
   - `CodeLLDB` (debugger)

2. Crea `.vscode/settings.json` en el proyecto:

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true
}
```

### Vim/Neovim

Instala CoC con `coc-rust-analyzer`:

```vim
:CocInstall coc-rust-analyzer
```

### Emacs

Usa `rust-mode` y `lsp-mode`:

```elisp
(use-package rust-mode)
(use-package lsp-mode
  :hook (rust-mode . lsp))
```

## 🧪 Ejecutar en Modo Desarrollo

Durante el desarrollo, usa `cargo run` directamente:

```bash
# Ejecutar con hot reload (requiere cargo-watch)
cargo install cargo-watch
cargo watch -x 'run ../ejemplos/hola.vc'
```

## 🎯 Siguiente Pasos

1. ✅ Instalación completa
2. 📖 Lee el [README.md](README.md) para aprender la sintaxis
3. 💻 Prueba los ejemplos en `ejemplos/`
4. 🚀 Empieza a programar en V-Code
5. 🤝 Contribuye al proyecto ([CONTRIBUTING.md](CONTRIBUTING.md))

## 📞 Ayuda Adicional

Si encuentras problemas:

1. **Issues**: https://github.com/PaoloGonzalez776/V-Code-Librerias/issues
2. **Discussions**: https://github.com/PaoloGonzalez776/V-Code-Librerias/discussions
3. **Documentación Rust**: https://www.rust-lang.org/learn

---

¡Bienvenido a V-Code! 🥽✨
