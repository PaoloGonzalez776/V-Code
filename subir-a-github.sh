#!/bin/bash
# Script para subir V-Code a GitHub
# Ejecuta estos comandos uno por uno

echo "🚀 Subiendo V-Code a GitHub..."
echo ""

# Navegar al directorio
cd V-Code-Librerias

# Limpiar repo si existe
rm -rf .git

# Inicializar git
git init
git config user.email "paolo@visor-os.dev"
git config user.name "Paolo Gonzalez"

# Agregar todos los archivos
git add -A

# Commit inicial
git commit -m "V-Code v0.1.0 - Compilador completo con GitHub Actions"

# Conectar con GitHub (usando tu token)
git remote add origin https://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@github.com/PaoloGonzalez776/V-Code-Librerias.git

# Cambiar a rama main
git branch -M main

# Push a GitHub
git push -u origin main --force

echo ""
echo "✅ Código subido!"
echo ""
echo "Ahora crea el tag para activar la compilación:"
echo "git tag v0.1.0"
echo "git push origin v0.1.0"
echo ""
echo "GitHub Actions compilará automáticamente los binarios."
echo "Ve a: https://github.com/PaoloGonzalez776/V-Code-Librerias/actions"
