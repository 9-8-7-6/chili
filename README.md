# chili
Software for detecting system boot events and potential unauthorized logins, currently only for **Linux**

## Feature
* Send an email notification on system boot (detected via boot ID)

## Steps

### Create .env file
Example format:
```dotenv
EMAIL_USERNAME="your_email@example.com"
EMAIL_PASSWORD="your_password"
```

### Build and Install as a Service
```bash
bash install_chili.sh
```

This will:
* Compile the binary in release mode
* Install to /opt/chili/chili
* Create a system user chili
* Install and enable a systemd service named chili.service

### Check service status
```bash
sudo systemctl status chili
```

## Follow logs
```bash
journalctl -u chili -f
```