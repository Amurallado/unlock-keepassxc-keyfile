#!/bin/bash

# ---
# Script para encriptar un keyfile existente para KeePassXC con GPG
# ---

# --- Variables ---
KEYFILE="keyfile.key"
ENCRYPTED_KEYFILE="keyfile.key.gpg"
GPG_RECIPIENT=""

# --- Funciones ---

# Función para verificar si un comando existe
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Función para encriptar el keyfile con GPG
encrypt_keyfile() {
    read -p "Introduce tu GPG User ID o email para encriptar el keyfile: " GPG_RECIPIENT
    if [ -z "$GPG_RECIPIENT" ]; then
        echo "El GPG User ID no puede estar vacío."
        exit 1
    fi

    echo "Encriptando '$KEYFILE' para el destinatario: $GPG_RECIPIENT..."
    gpg --yes --encrypt --recipient "$GPG_RECIPIENT" --output "$ENCRYPTED_KEYFILE" "$KEYFILE"
    if [ $? -ne 0 ]; then
        echo "Error al encriptar el keyfile. Verifica que el GPG User ID sea correcto y que '$KEYFILE' exista."
        exit 1
    fi
    echo "Keyfile encriptado en: $ENCRYPTED_KEYFILE"
}

# --- Flujo Principal ---

echo "--- Asistente de Encriptación de Keyfile para KeePassXC ---"

# 1. Verificar dependencias
if ! command_exists gpg; then
    echo "Error: GPG es necesario para ejecutar este script."
    exit 1
fi

# 2. Verificar si el keyfile existe
if [ ! -f "$KEYFILE" ]; then
    echo "Error: El archivo '$KEYFILE' no se encontró."
    echo "Por favor, crea un archivo llamado '$KEYFILE' en este directorio antes de continuar."
    exit 1
fi

# 3. Encriptar el keyfile
encrypt_keyfile

# 4. Limpieza
read -p "¿Deseas eliminar el keyfile original no encriptado '$KEYFILE'? (s/n): " choice
if [[ "$choice" == "s" || "$choice" == "S" ]]; then
    rm "$KEYFILE"
    echo "Keyfile original eliminado."
fi

echo ""
echo "--- ¡Encriptación completada! ---"
echo "El archivo encriptado es '$ENCRYPTED_KEYFILE'."
echo "Ahora puedes usar este archivo en la configuración de seguridad de tu base de datos KeePassXC."