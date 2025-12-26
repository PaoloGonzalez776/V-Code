# 📦 Instrucciones de Compilación - V-Code v0.1.0

## 🎯 Objetivo

Compilar V-Code una sola vez para generar el binario que cualquier programador puede usar **sin instalar Rust**.

---

## ⚡ Opción Rápida: Script Automático

### En Linux/macOS:

```bash
# 1. Dale permisos al script (solo primera vez)
chmod +x compilar.sh

# 2. Ejecuta el script
./compilar.sh
```

### En Windows:

```bash
# Doble click en:
compilar.bat

# O desde PowerShell:
.\compilar.bat
```

**El script hace TODO automáticamente:**
- ✅ Compila V-Code en modo optimizado
- ✅ Crea carpeta `dist/v-code-0.1.0/`
- ✅ Copia el binario, ejemplos y documentación
- ✅ Deja todo listo para distribuir

---

## 🔧 Opción Manual: Paso a Paso

Si prefieres hacerlo manualmente o el script falla:

### 1. Instalar Rust (si no lo tienes)

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Windows
# Descarga y ejecuta: https://rustup.rs/
```

### 2. Compilar V-Code

```bash
# Navegar al compilador
cd compilador

# Compilar (tarda 1-2 minutos la primera vez)
cargo build --release

# El binario está en:
# Linux/Mac: target/release/vcode
# Windows: target\release\vcode.exe
```

### 3. Probar que funciona

```bash
# Linux/Mac
./target/release/vcode ../ejemplos/hola.vc

# Windows
.\target\release\vcode.exe ..\ejemplos\hola.vc
```

Deberías ver:
```
🚀 V-Code v0.1.0 - Lenguaje de Programación para VR
...
📺 V-Code ha nacido
✅ Ejecución completada exitosamente
```

### 4. Crear Paquete de Distribución

```bash
# Desde la raíz del proyecto
mkdir -p dist/v-code-0.1.0

# Copiar binario
cp compilador/target/release/vcode dist/v-code-0.1.0/

# Copiar ejemplos
cp -r ejemplos dist/v-code-0.1.0/

# Copiar docs esenciales
cp README.md QUICKSTART.md LICENSE dist/v-code-0.1.0/
```

---

## 📦 Crear Archivo para GitHub Releases

### Linux/macOS:

```bash
cd dist
tar -czf v-code-0.1.0-linux-x64.tar.gz v-code-0.1.0/
```

### Windows:

```powershell
# Click derecho en la carpeta v-code-0.1.0
# "Enviar a" → "Carpeta comprimida"
# Renombrar a: v-code-0.1.0-windows-x64.zip
```

---

## 🚀 Subir a GitHub

### 1. Crear un Release en GitHub

```bash
# Ve a tu repositorio:
https://github.com/PaoloGonzalez776/V-Code-Librerias

# Click en "Releases" (lado derecho)
# Click en "Create a new release"
```

### 2. Configurar el Release

- **Tag**: `v0.1.0`
- **Title**: `V-Code v0.1.0 - Primer Release Oficial`
- **Description**:

```markdown
# 🚀 V-Code v0.1.0

Primer release oficial del lenguaje de programación nativo para VR en español.

## 🎯 Descarga para tu Sistema

- **Linux x64**: [v-code-0.1.0-linux-x64.tar.gz](link)
- **Windows x64**: [v-code-0.1.0-windows-x64.zip](link)
- **macOS**: [v-code-0.1.0-macos-arm64.tar.gz](link)

## ⚡ Inicio Rápido

### Linux/macOS
```bash
tar -xzf v-code-0.1.0-linux-x64.tar.gz
cd v-code-0.1.0
chmod +x vcode
./vcode ejemplos/hola.vc
```

### Windows
```bash
# Extraer el .zip
# Ejecutar:
vcode.exe ejemplos\hola.vc
```

## ✨ Características

- ✅ Sintaxis en español nativa
- ✅ Variables y funciones
- ✅ Control de flujo (si, mientras, para)
- ✅ Preparado para VR (escenas, frames)
- ✅ Sin dependencias - solo descarga y usa

## 📚 Documentación

- [README.md](https://github.com/PaoloGonzalez776/V-Code-Librerias/blob/main/README.md)
- [QUICKSTART.md](https://github.com/PaoloGonzalez776/V-Code-Librerias/blob/main/QUICKSTART.md)
- [Ejemplos](https://github.com/PaoloGonzalez776/V-Code-Librerias/tree/main/ejemplos)

## 🐛 Reportar Problemas

[Issues](https://github.com/PaoloGonzalez776/V-Code-Librerias/issues)

---

**V-Code** - El lenguaje del futuro de la VR 🥽✨
```

### 3. Subir los Archivos

Arrastra y suelta:
- `v-code-0.1.0-linux-x64.tar.gz`
- `v-code-0.1.0-windows-x64.zip`
- (Cualquier otra plataforma que hayas compilado)

### 4. Publicar

Click en **"Publish release"**

---

## 🌍 Compilar para Múltiples Plataformas

### Desde Linux, compilar para Windows:

```bash
# Instalar target
rustup target add x86_64-pc-windows-gnu

# Instalar mingw
sudo apt install mingw-w64

# Compilar
cargo build --release --target x86_64-pc-windows-gnu

# Binario en:
target/x86_64-pc-windows-gnu/release/vcode.exe
```

### Desde macOS, compilar para Linux:

```bash
# Instalar cross (herramienta de cross-compilation)
cargo install cross

# Compilar
cross build --release --target x86_64-unknown-linux-gnu

# Binario en:
target/x86_64-unknown-linux-gnu/release/vcode
```

---

## 🎓 Lo que Obtienes

Después de compilar, tendrás:

```
dist/v-code-0.1.0/
├── vcode              ← Ejecutable (Linux/Mac)
├── vcode.exe          ← Ejecutable (Windows)
├── ejemplos/
│   ├── hola.vc
│   ├── variables.vc
│   ├── control_flujo.vc
│   ├── funciones.vc
│   └── vr_demo.vc
├── README.md
├── QUICKSTART.md
└── LICENSE
```

**Este paquete es lo que los programadores descargan.**

No necesitan Rust, Cargo, ni nada. Solo:
```bash
./vcode programa.vc
```

---

## ✅ Checklist Final

Antes de publicar, verifica:

- [ ] El binario compila sin errores
- [ ] `./vcode ejemplos/hola.vc` funciona
- [ ] Todos los ejemplos se ejecutan correctamente
- [ ] README.md está actualizado
- [ ] LICENSE está incluido
- [ ] Tag de git creado: `git tag v0.1.0`
- [ ] Release creado en GitHub
- [ ] Binarios subidos al release

---

## 🎯 Resultado Final

Los programadores harán:

```bash
# 1. Descargar
wget https://github.com/PaoloGonzalez776/V-Code-Librerias/releases/download/v0.1.0/v-code-0.1.0-linux-x64.tar.gz

# 2. Extraer
tar -xzf v-code-0.1.0-linux-x64.tar.gz

# 3. Usar
cd v-code-0.1.0
./vcode ejemplos/hola.vc
```

**¡Y ya están programando en V-Code!** Sin instalar nada más. 🚀

---

## 🆘 Solución de Problemas

### "cargo: command not found"

Rust no está instalado. Instala desde https://rustup.rs/

### "error: linker `cc` not found"

```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora
sudo dnf install gcc

# Arch
sudo pacman -S base-devel
```

### Compilación muy lenta

Es normal la primera vez (1-2 minutos). Las siguientes son más rápidas.

### Error de permisos en Linux/Mac

```bash
chmod +x vcode
```

---

**¿Dudas?** Abre un issue en GitHub o consulta la documentación completa.
