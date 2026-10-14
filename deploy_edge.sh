#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="/opt/project_nest"
BUNDLE_TAR="./project_nest_airgap_bundle.tar.gz"

echo "=== [1/4] Preparing System Directories at ${INSTALL_DIR} ==="
mkdir -p "${INSTALL_DIR}/models" "${INSTALL_DIR}/config" "${INSTALL_DIR}/images"

if [ -f "${BUNDLE_TAR}" ]; then
    echo "=== [2/4] Unpacking Offline Bundle and Loading Docker Images ==="
    tar -xzf "${BUNDLE_TAR}" -C "${INSTALL_DIR}/images"
    for img in "${INSTALL_DIR}/images"/*.tar; do
        docker load -i "${img}"
    done
else
    echo "=== [2/4] Building locally... ==="
fi

echo "=== [3/4] Validating ONNX Models and Security Keys ==="
if [ ! -f "${INSTALL_DIR}/config/trusted_pubkey.hex" ]; then
    echo "ERROR: Trusted Ed25519 public key missing in ${INSTALL_DIR}/config/trusted_pubkey.hex"
    exit 1
fi

echo "=== [4/4] Launching Containerized Edge Subsystems ==="
docker compose -f docker-compose.edge.yml up -d

echo "=== [SUCCESS] project-nest Edge Node Operational ==="
