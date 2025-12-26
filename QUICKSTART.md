# 🚀 V-Code - Inicio Rápido

## ⚡ 5 Minutos para Ejecutar V-Code

### 1️⃣ Instalar Rust (si no lo tienes)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2️⃣ Clonar y Compilar

```bash
cd compilador
cargo build --release
```

### 3️⃣ Ejecutar Primer Programa

```bash
cargo run ../ejemplos/hola.vc
```

Salida esperada:
```
🚀 V-Code v0.1.0 - Lenguaje de Programación para VR
...
📺 V-Code ha nacido
📺 Lenguaje de programación nativo para VR
✅ Ejecución completada exitosamente
```

## 🎯 Ejemplos Incluidos

```bash
# Variables y operaciones
cargo run ../ejemplos/variables.vc

# Control de flujo
cargo run ../ejemplos/control_flujo.vc

# Funciones
cargo run ../ejemplos/funciones.vc

# Demo VR
cargo run ../ejemplos/vr_demo.vc
```

## ✍️ Tu Primer Programa

Crea `mi_app.vc`:

```vcode
escena MiApp {
    var nombre = "Paolo"
    var edad = 25
    
    mostrar "Hola, " + nombre
    
    si edad > 18 {
        mostrar "Mayor de edad"
    }
    
    para i = 0, 5 {
        mostrar "Contando: " + i
    }
}
```

Ejecuta:
```bash
cargo run mi_app.vc
```

## 📚 Siguiente Pasos

1. ✅ Lee [README.md](README.md) - Documentación completa
2. 🏗️ Lee [ARCHITECTURE.md](ARCHITECTURE.md) - Arquitectura interna
3. 🤝 Lee [CONTRIBUTING.md](CONTRIBUTING.md) - Guía de contribución
4. 🚀 ¡Empieza a construir en V-Code!

## 💡 Tips

### Modo Watch (Auto-reload)

```bash
cargo install cargo-watch
cargo watch -x 'run ../ejemplos/hola.vc'
```

### Optimizar Performance

```bash
cargo build --release
./target/release/vcode mi_app.vc
```

### Instalar Globalmente

```bash
cargo install --path .
vcode ~/proyectos/app.vc  # Desde cualquier lugar
```

## ❓ Ayuda Rápida

```bash
# Ver estructura del proyecto
tree -L 2 compilador/

# Ejecutar tests
cargo test

# Verificar código
cargo clippy

# Formatear código
cargo fmt
```

## 🐛 Problemas Comunes

### "cargo: command not found"
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### "linking with cc failed" (Linux)
```bash
sudo apt install build-essential  # Ubuntu/Debian
sudo dnf install gcc              # Fedora
```

### Compilación lenta
```bash
# Usa release solo cuando necesites velocidad
cargo build          # Debug - rápido de compilar
cargo build --release # Release - rápido de ejecutar
```

---

**¡Bienvenido a V-Code!** 🥽✨

Para más detalles, consulta la documentación completa en README.md
