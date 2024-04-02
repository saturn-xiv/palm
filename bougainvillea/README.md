# Bougainvillea - An online journal analysis system

## Server setup

```bash
sudo apt install systemd-journal-remote
sudo systemctl enable systemd-journal-remote

# Recommenced not less than 253
sudo sudo dpkg -l systemd-journal-remote

# create storage dir
sudo mkdir -p /var/log/journal/remote
sudo chown -R systemd-journal-remote:systemd-journal-remote /var/log/journal/remote

# firewall
sudo ufw allow in 19532/tcp
sudo ufw allow in 80/tcp


```

- /lib/systemd/system/systemd-journal-remote.service

```ini
ExecStart=/lib/systemd/systemd-journal-remote --listen-http=-3 --output=/var/log/journal/remote/all.journal
```

- [/etc/systemd/journal-remote.conf](https://www.freedesktop.org/software/systemd/man/latest/journal-remote.conf.html)

```ini
[Remote]
Seal=false
SplitMode=none
Compress=yes
SystemKeepFree=5%
SystemMaxFileSize=1G
```

## Client setup

```bash
sudo apt install systemd-journal-upload
sudo systemctl enable systemd-journal-upload
sudo systemctl start systemd-journal-upload
```

- /etc/systemd/journal-upload.conf

```ini
[Upload]
URL=http://SERVER-IP:19532
```

- Testing

```bash
# run on server
sudo ls -lah /var/log/journal/remote/
sudo journalctl -f --file=/var/log/journal/remote/all.journal

# run on client
logger -p syslog.debug "### TEST MESSAGE from client ###"
```
