#!/bin/bash

# ---
# Script mejorado para desbloquear KeePassXC de forma segura.
# Espera a que KeePassXC se cierre antes de eliminar el keyfile temporal.
# ---

# --- Configuración (ajusta estas rutas según tu sistema) ---
# Ruta a tu archivo clave encriptado.
ENCRYPTED_KEYFILE_PATH="$HOME/.clave_kp/keyfile.key.gpg"
# Ruta donde se guardará el archivo clave temporal.
TEMP_KEYFILE_PATH="/tmp/keyfile.key"
# Comando para ejecutar KeePassXC (puede ser "keepassxc" si está en tu PATH).
KEEPASSXC_BINARY="keepassxc"
# --- Fin de la Configuración ---

# Función de limpieza para eliminar el archivo temporal.
cleanup() {
    if [ -f "$TEMP_KEYFILE_PATH" ]; then
        echo "🧹 Eliminando archivo temporal..."
        shred -u "$TEMP_KEYFILE_PATH"
        echo "✅ Archivo temporal eliminado."
    fi
}

# Registrar la función de limpieza para que se ejecute al salir del script.
trap cleanup EXIT

# Verificar si el archivo clave encriptado existe.
if [ ! -f "$ENCRYPTED_KEYFILE_PATH" ]; then
    echo "Error: El archivo clave encriptado no se encuentra en: $ENCRYPTED_KEYFILE_PATH"
    exit 1
fi

echo "🔐 Desbloqueando archivo clave..."
# Desencriptar el archivo clave.
gpg -d "$ENCRYPTED_KEYFILE_PATH" > "$TEMP_KEYFILE_PATH"
if [ $? -ne 0 ]; then
    echo "Error: No se pudo desencriptar el archivo clave. Verifica tu configuración de GPG."
    exit 1
fi

echo "🔓 Abriendo KeePassXC... (Cierra KeePassXC para eliminar el keyfile)"
# Ejecutar KeePassXC y esperar a que el proceso termine.
"$KEEPASSXC_BINARY" --keyfile "$TEMP_KEYFILE_PATH"

echo "KeePassXC se ha cerrado."
