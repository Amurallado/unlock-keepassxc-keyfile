#!/bin/bash

echo "🔐 Desbloqueando archivo clave..."
gpg -d ~/.clave_kp/keyfile.key.gpg > /tmp/keyfile.key

echo "🔓 Abriendo KeePassXC..."
keepassxc --keyfile /tmp/keyfile.key &

sleep 10

echo "🧹 Eliminando archivo temporal..."
shred -u /tmp/keyfile.key

echo "✅ Bóveda lanzada y segura."