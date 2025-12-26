# 🚀 Compilar V-Code SIN Instalar Rust

## ⚡ OPCIÓN AUTOMÁTICA: GitHub Actions

GitHub compila V-Code automáticamente por ti en la nube. **TÚ NO INSTALAS NADA.**

---

## 📝 Pasos (5 minutos)

### 1. Subir el Código a GitHub

```bash
# En tu terminal (en la carpeta del proyecto)
git add .
git commit -m "V-Code v0.1.0 inicial"
git push origin main
```

### 2. Crear un Tag de Versión

```bash
# Crear tag v0.1.0
git tag v0.1.0

# Subir el tag (esto activa la compilación automática)
git push origin v0.1.0
```

### 3. Esperar (2-3 minutos)

GitHub Actions automáticamente:
- ✅ Instala Rust en la nube
- ✅ Compila V-Code para Linux, Windows y macOS
- ✅ Crea el Release
- ✅ Sube los binarios

Ve a tu repositorio:
```
https://github.com/PaoloGonzalez776/V-Code-Librerias/actions
```

Verás el workflow ejecutándose. Cuando termine (✓ verde):

### 4. Ver el Release

```
https://github.com/PaoloGonzalez776/V-Code-Librerias/releases
```

**¡Ya están los binarios listos para descargar!**

```
v-code-0.1.0-linux-x64.tar.gz
v-code-0.1.0-windows-x64.zip
v-code-0.1.0-macos-arm64.tar.gz
```

---

## 🎯 Resultado Final

Los programadores van a:

```
https://github.com/PaoloGonzalez776/V-Code-Librerias/releases
```

Descargan el archivo para su sistema.

Y ejecutan:
```bash
./vcode ejemplos/hola.vc
```

**SIN instalar Rust. SIN compilar nada. TODO automático.** ✨

---

## 🔄 Para Futuras Versiones

Cada vez que quieras publicar una nueva versión:

```bash
# Hacer cambios en el código
git add .
git commit -m "Nuevas features"
git push

# Crear nuevo tag
git tag v0.2.0
git push origin v0.2.0

# GitHub compila automáticamente y crea el release
```

---

## 🐛 Si Algo Falla

1. Ve a `Actions` en tu repo de GitHub
2. Click en el workflow que falló
3. Verás los logs exactos del error
4. Copia el error y me lo pasas

---

## ✅ Ventajas de Esta Opción

- ✅ **No instalas nada** en tu máquina
- ✅ **Compila para 3 sistemas** (Linux, Windows, Mac)
- ✅ **Totalmente automático**
- ✅ **Gratis** (GitHub Actions es gratis para repos públicos)
- ✅ **Profesional** (así lo hacen proyectos grandes)

---

## 📋 Checklist

- [ ] Subir código a GitHub: `git push`
- [ ] Crear tag: `git tag v0.1.0`
- [ ] Push del tag: `git push origin v0.1.0`
- [ ] Esperar 2-3 minutos
- [ ] Verificar en: `github.com/tu-usuario/V-Code-Librerias/releases`
- [ ] ¡Descargar y probar!

---

**¡Eso es todo! GitHub hace la magia por ti.** 🪄
