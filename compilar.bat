@echo off
REM Script de Compilación de V-Code v0.1.0 para Windows
REM Genera binarios listos para distribución

echo Compilando V-Code v0.1.0...
echo.

REM Verificar que Rust está instalado
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Cargo no esta instalado
    echo Instala Rust desde: https://rustup.rs/
    pause
    exit /b 1
)

echo Rust encontrado
cargo --version
echo.

REM Navegar al directorio del compilador
cd compilador

echo Compilando en modo release (optimizado)...
echo Esto puede tardar 1-2 minutos la primera vez...
echo.

REM Compilar en modo release
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo Compilacion exitosa!
    echo.
    echo Binario generado en:
    echo %CD%\target\release\vcode.exe
    echo.
    
    REM Crear carpeta de distribución
    cd ..
    if not exist "dist\v-code-0.1.0" mkdir dist\v-code-0.1.0
    
    REM Copiar binario
    copy compilador\target\release\vcode.exe dist\v-code-0.1.0\
    
    REM Copiar ejemplos
    xcopy ejemplos dist\v-code-0.1.0\ejemplos\ /E /I /Y
    
    REM Copiar documentación
    copy README.md dist\v-code-0.1.0\
    copy QUICKSTART.md dist\v-code-0.1.0\
    copy LICENSE dist\v-code-0.1.0\
    
    echo.
    echo Paquete de distribucion creado en: dist\v-code-0.1.0\
    echo.
    echo Siguiente paso:
    echo Comprime la carpeta y subela a GitHub Releases
    echo.
    echo Los usuarios solo necesitan:
    echo 1. Descargar el archivo .zip
    echo 2. Extraer
    echo 3. Ejecutar: vcode.exe ejemplos\hola.vc
    echo.
) else (
    echo.
    echo Error en la compilacion
    echo Revisa los errores arriba
    pause
    exit /b 1
)

echo Listo! V-Code esta compilado y empaquetado.
pause
