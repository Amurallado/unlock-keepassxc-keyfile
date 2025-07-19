# 🔐 Scripts para Desbloquear KeePassXC con Keyfile Encriptado

¡Automatiza y protege el acceso a tu bóveda KeePassXC usando un archivo clave cifrado con GPG!

## 🚀 ¿Qué hacen estos scripts?

- **Desbloquean tu archivo clave cifrado** (`keyfile.key.gpg`) usando GPG.
- **Abren KeePassXC** con el keyfile temporal.
- **Esperan a que KeePassXC se cierre y eliminan de forma segura** el keyfile temporal, para que nunca quede desprotegido en el disco.

## 📂 Archivos

- [`setup_keyfile.sh`](setup_keyfile.sh): Asistente para encriptar tu `keyfile.key`.
- [`kp_unlock_linux.sh`](kp_unlock_linux.sh): Script mejorado para sistemas Linux.
- [`kp_unlock_windows.ps1`](kp_unlock_windows.ps1): Script mejorado para Windows PowerShell.

## 🛠️ Cómo Empezar

### Paso 1: Crea y Encripta tu Keyfile

1.  **Crea un keyfile:** Puedes usar KeePassXC para generar un nuevo archivo (`keyfile.key`) o usar uno que ya tengas.
2.  **Encripta el keyfile:** Coloca `keyfile.key` en el directorio y ejecuta el asistente:

    ```sh
    bash setup_keyfile.sh
    ```
    El script te pedirá tu GPG User ID y creará `keyfile.key.gpg`.

### Paso 2: Configura el Script de Desbloqueo

Antes de usar los scripts, ábrelos y ajusta las rutas en la sección de **Configuración** para que coincidan con tu sistema.

**En `kp_unlock_linux.sh`:**
```bash
ENCRYPTED_KEYFILE_PATH="$HOME/.clave_kp/keyfile.key.gpg"
KEEPASSXC_BINARY="keepassxc"
```

**En `kp_unlock_windows.ps1`:**
```powershell
$EncryptedKeyfilePath = "$env:USERPROFILE\.clave_kp\keyfile.key.gpg"
$KeePassXCPath = "C:\Program Files\KeePassXC\KeePassXC.exe"
```

### Paso 3: Ejecuta el Script

Una vez configurado, ya puedes usarlo.

**En Linux:**
```sh
bash kp_unlock_linux.sh
```

**En Windows (PowerShell):**
```powershell
.\kp_unlock_windows.ps1
```

## 💡 Consejo de Seguridad

Gracias a las mejoras, el keyfile temporal solo existe mientras KeePassXC está en ejecución. En cuanto cierras el programa, el script lo detecta y **elimina el archivo de forma segura y automática**.

---

¡Mantén tu bóveda segura y tu flujo de trabajo ágil! 🚦

