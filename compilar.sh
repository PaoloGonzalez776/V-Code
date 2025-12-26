#!/bin/bash

# Script de Compilación de V-Code v0.1.0
# Genera binarios listos para distribución

set -e

echo "🚀 Compilando V-Code v0.1.0..."
echo ""

# Verificar que Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo no está instalado"
    echo "Instala Rust desde: https://rustup.rs/"
    exit 1
fi

echo "✓ Rust encontrado: $(rustc --version)"
echo ""

# Navegar al directorio del compilador
cd "$(dirname "$0")/compilador"

echo "📦 Compilando en modo release (optimizado)..."
echo "   Esto puede tardar 1-2 minutos la primera vez..."
echo ""

# Compilar en modo release
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Compilación exitosa!"
    echo ""
    echo "📍 Binario generado en:"
    echo "   $(pwd)/target/release/vcode"
    echo ""
    
    # Verificar el tamaño del binario
    SIZE=$(du -h target/release/vcode | cut -f1)
    echo "📊 Tamaño del binario: $SIZE"
    echo ""
    
    # Crear carpeta de distribución
    echo "📦 Creando paquete de distribución..."
    cd ..
    mkdir -p dist/v-code-0.1.0
    
    # Copiar binario
    cp compilador/target/release/vcode dist/v-code-0.1.0/
    
    # Copiar ejemplos
    cp -r ejemplos dist/v-code-0.1.0/
    
    # Copiar documentación esencial
    cp README.md dist/v-code-0.1.0/
    cp QUICKSTART.md dist/v-code-0.1.0/
    cp LICENSE dist/v-code-0.1.0/
    
    # Crear README para usuarios finales
    cat > dist/v-code-0.1.0/README-USUARIOS.md << 'EOF'
# V-Code v0.1.0 - Lenguaje de Programación para VR

## 🚀 Inicio Rápido (30 segundos)

### 1. Dar permisos de ejecución (solo primera vez)
```bash
chmod +x vcode
```

### 2. Ejecutar tu primer programa
```bash
./vcode ejemplos/hola.vc
```

Deberías ver:
```
🚀 V-Code v0.1.0 - Lenguaje de Programación para VR
...
📺 V-Code ha nacido
📺 Lenguaje de programación nativo para VR
✅ Ejecución completada exitosamente
```

## ✍️ Crear tu Propio Programa

Crea un archivo `mi_app.vc`:

```vcode
escena MiApp {
    var nombre = "Tu Nombre"
    mostrar "Hola, " + nombre
    
    para i = 0, 5 {
        mostrar "Contando: " + i
    }
}
```

Ejecuta:
```bash
./vcode mi_app.vc
```

## 📚 Más Ejemplos

Explora la carpeta `ejemplos/`:
- `hola.vc` - Hola mundo
- `variables.vc` - Variables y tipos
- `control_flujo.vc` - If, while, for
- `funciones.vc` - Funciones
- `vr_demo.vc` - Conceptos VR

## 🌍 Instalar Globalmente (Opcional)

Para usar `vcode` desde cualquier lugar:

```bash
sudo mv vcode /usr/local/bin/
# Ahora puedes hacer: vcode mi_programa.vc
```

## 📖 Documentación Completa

Lee `README.md` para la sintaxis completa del lenguaje.

## 🐛 Problemas?

- GitHub: https://github.com/PaoloGonzalez776/V-Code-Librerias/issues
- Documentación: Ver README.md

---

**V-Code** - Lenguaje de programación nativo para VR en español 🥽✨
EOF
    
    echo "✅ Paquete de distribución creado en: dist/v-code-0.1.0/"
    echo ""
    echo "📦 Contenido del paquete:"
    ls -lh dist/v-code-0.1.0/
    echo ""
    echo "🎯 Siguiente paso:"
    echo "   Comprime la carpeta y súbela a GitHub Releases:"
    echo "   cd dist && tar -czf v-code-0.1.0-linux-x64.tar.gz v-code-0.1.0/"
    echo ""
    echo "🚀 Los usuarios solo necesitan:"
    echo "   1. Descargar el archivo .tar.gz"
    echo "   2. Extraer: tar -xzf v-code-0.1.0-linux-x64.tar.gz"
    echo "   3. Ejecutar: cd v-code-0.1.0 && ./vcode ejemplos/hola.vc"
    echo ""
else
    echo ""
    echo "❌ Error en la compilación"
    echo "Revisa los errores arriba"
    exit 1
fi

echo "✨ ¡Listo! V-Code está compilado y empaquetado."
