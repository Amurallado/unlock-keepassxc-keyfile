# ---
# Script mejorado para desbloquear KeePassXC de forma segura en Windows.
# Espera a que KeePassXC se cierre antes de eliminar el keyfile temporal.
# ---

# --- Configuración (ajusta estas rutas según tu sistema) ---
# Ruta a tu archivo clave encriptado. Por defecto, busca en tu perfil de usuario.
$EncryptedKeyfilePath = "$env:USERPROFILE\.clave_kp\keyfile.key.gpg"
# Ruta completa al ejecutable de KeePassXC.
$KeePassXCPath = "C:\Program Files\KeePassXC\KeePassXC.exe"
# --- Fin de la Configuración ---

# El archivo clave temporal se guardará en la carpeta temporal del usuario.
$TempKeyfilePath = "$env:TEMP\keyfile.key"

# Función para eliminar el archivo de forma segura (sobrescribe y luego elimina).
function SecureRemove-Item {
    param([string]$Path)
    if (Test-Path $Path) {
        Write-Host "🧹 Eliminando archivo temporal de forma segura..."
        # Sobrescribir el archivo con ceros para ofuscar el contenido.
        $fileStream = [System.IO.File]::Open($Path, [System.IO.FileMode]::Open, [System.IO.FileAccess]::Write)
        $zeroBuffer = [byte[]]::new($fileStream.Length)
        $fileStream.Write($zeroBuffer, 0, $zeroBuffer.Length)
        $fileStream.Close()
        # Eliminar el archivo.
        Remove-Item $Path -Force
        Write-Host "✅ Archivo temporal eliminado."
    }
}

# Verificar si el archivo clave encriptado existe.
if (-not (Test-Path $EncryptedKeyfilePath)) {
    Write-Host "Error: El archivo clave encriptado no se encuentra en: $EncryptedKeyfilePath" -ForegroundColor Red
    exit
}

# Registrar la limpieza para que se ejecute al salir.
Register-EngineEvent -SourceIdentifier PowerShell.Exiting -Action { SecureRemove-Item -Path $TempKeyfilePath } | Out-Null

Write-Host "🔐 Desbloqueando archivo clave..."
try {
    # Desencriptar el archivo clave.
    gpg -d "$EncryptedKeyfilePath" > "$TempKeyfilePath"
} catch {
    Write-Host "Error: No se pudo desencriptar el archivo clave. Verifica tu configuración de GPG." -ForegroundColor Red
    exit
}

Write-Host "🔓 Abriendo KeePassXC... (Cierra KeePassXC para eliminar el keyfile)"
# Iniciar KeePassXC y esperar a que se cierre.
$process = Start-Process -FilePath $KeePassXCPath -ArgumentList "--keyfile `"$TempKeyfilePath`"" -Wait -PassThru

Write-Host "KeePassXC se ha cerrado."

# La limpieza se ejecutará automáticamente al salir del script.
