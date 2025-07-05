# 🔐 Scripts para Desbloquear KeePassXC con Keyfile Encriptado

¡Automatiza y protege el acceso a tu bóveda KeePassXC usando un archivo clave cifrado con GPG!

## 🚀 ¿Qué hacen estos scripts?

- **Desbloquean tu archivo clave cifrado (`keyfile.key.gpg`) usando GPG.**
- **Abren KeePassXC automáticamente con el keyfile temporal.**
- **Eliminan el archivo clave temporal para mayor seguridad.**

## 📂 Archivos

- [`kp_unlock_linux.sh`](kp_unlock_linux.sh): Script para sistemas Linux.
- [`kp_unlock_windows.ps1`](kp_unlock_windows.ps1): Script para Windows PowerShell.

## 🛠️ Uso

### En Linux

```sh
bash kp_unlock_linux.sh
```

### En Windows (PowerShell)

```powershell
.\kp_unlock_windows.ps1
```

> **Nota:** Cambia las rutas de usuario si es necesario y asegúrate de tener GPG y KeePassXC instalados.

## 💡 Consejo de seguridad

Tu archivo clave nunca permanece sin cifrar en disco: se elimina automáticamente tras abrir KeePassXC.

---

¡Mantén tu bóveda segura y tu flujo de trabajo ágil! 🚦
