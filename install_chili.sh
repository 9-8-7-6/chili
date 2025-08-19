#!/usr/bin/env bash
# install_chili.sh
# Build -> install to absolute path -> run as systemd service
set -euo pipefail

# ======= Configurable parameters =======
APP_NAME="chili"                 # binary & service name
INSTALL_DIR="/opt/chili"         # absolute install path for all files
SERVICE_FILE="/etc/systemd/system/${APP_NAME}.service"
RUN_USER="chili"
RUN_GROUP="chili"
ENV_FILE="${INSTALL_DIR}/.env"   # if a .env file exists, it will be used (optional)
# =======================================

# Ensure we are in the project root
if [[ ! -f "Cargo.toml" ]]; then
  echo "Please run this script from the project root (where Cargo.toml is located)." >&2
  exit 1
fi

echo "==> 1/6 Build release"
cargo build --release

echo "==> 2/6 Install binary and files to absolute path: ${INSTALL_DIR}"
sudo mkdir -p "${INSTALL_DIR}"
sudo install -m 0755 "target/release/${APP_NAME}" "${INSTALL_DIR}/${APP_NAME}"

# Copy .env file if present
if [[ -f ".env" ]]; then
  echo "    Copying .env -> ${ENV_FILE}"
  sudo cp -f ".env" "${ENV_FILE}"
fi

echo "==> 3/6 Create service user/group if missing"
if ! id -u "${RUN_USER}" >/dev/null 2>&1; then
  sudo useradd --system --create-home --home-dir "${INSTALL_DIR}" --shell /usr/sbin/nologin "${RUN_USER}"
fi
# Some systems may not create the group automatically
if ! getent group "${RUN_GROUP}" >/dev/null 2>&1; then
  sudo groupadd --system "${RUN_GROUP}" || true
fi
sudo chown -R "${RUN_USER}:${RUN_GROUP}" "${INSTALL_DIR}"

echo "==> 4/6 Write systemd unit: ${SERVICE_FILE}"
sudo tee "${SERVICE_FILE}" >/dev/null <<UNIT
[Unit]
Description=${APP_NAME} daemon
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
# Run the binary using absolute path
ExecStart=${INSTALL_DIR}/${APP_NAME}
# Run under ${RUN_USER}:${RUN_GROUP}
User=${RUN_USER}
Group=${RUN_GROUP}
# Use project folder as working directory (to read files like .env)
WorkingDirectory=${INSTALL_DIR}
# Load .env if present, ignore if missing (note the leading -)
EnvironmentFile=-${ENV_FILE}

# Restart policy
Restart=on-failure
RestartSec=2s

# Logging goes to journald
StandardOutput=journal
StandardError=journal

# Security hardening (adjust as needed)
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=full
ProtectHome=true
ReadWritePaths=${INSTALL_DIR}

[Install]
WantedBy=multi-user.target
UNIT

echo "==> 5/6 Reload systemd, enable & start"
sudo systemctl daemon-reload
sudo systemctl enable --now "${APP_NAME}.service"

echo "==> 6/6 Done"
echo "Check status:     sudo systemctl status ${APP_NAME}"
echo "Follow logs:      journalctl -u ${APP_NAME} -f"
echo "Binary location:  ${INSTALL_DIR}/${APP_NAME}"
