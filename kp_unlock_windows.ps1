Write-Host "🔐 Desbloqueando archivo clave..."
gpg -d C:\Users\TU_USUARIO\.clave_kp\keyfile.key.gpg > $env:TEMP\keyfile.key

Start-Process "C:\Program Files\KeePassXC\KeePassXC.exe" -ArgumentList "--keyfile `"$env:TEMP\keyfile.key`""

Start-Sleep -Seconds 10

Remove-Item $env:TEMP\keyfile.key -Force
Write-Host "✅ Bóveda lanzada y archivo clave eliminado."