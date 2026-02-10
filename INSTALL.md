# Installation

This file shows how to install and configure mollysocket **on your system using a systemd service**.

**This should be relevant if you use docker**

## Install the binary with a dedicated user

First of all, you need to install mollysocket on your system.

#### Create a dedicated account

The service will run with a dedicated account, so create it and switch to that user:

```console
# useradd mollysocket -M
```

#### Install the binary

You have 2 solutions to install the binary.

1. Use an already compiled binary: <https://github.com/mollyim/mollysocket/releases/>. Download it to `/usr/local/bin/` and link the executable: `ln -s /usr/local/bin/{REPLACE_WITH_DOWNLOADED_MS} /usr/local/bin/ms`

2. Use cargo. This method allows you to use cargo to maintain mollysocket up to date. First of all, you need to [install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (you need at least version 1.59). Then, install mollysocket using cargo: `cargo install mollysocket`. *You probably need to install some system packages, like libssl-dev libsqlite3-dev*. Then copy the compile binary to your system: `cp ~/.cargo/bin/mollysocket /usr/local/bin/ms`.

## Install systemd services

Download the 2 systemd unit files [mollysocket.service](https://github.com/mollyim/mollysocket/raw/main/mollysocket.service) and [mollysocket-vapid.service](https://github.com/mollyim/mollysocket/raw/main/mollysocket-vapid.service) and place them in the right direction `/etc/systemd/system/`.

### Start the service

You should be able to see that service now `systemctl status mollysocket`.

You can enable it `systemctl enable --now mollysocket`, the service is now active (`systemctl status mollysocket`), and will be started on system boot.

## App configuration

*If you host your own Push server*, then explicitly add it to the allowed endpoints. In `/etc/mollysocket/conf.toml`, edit `allowed_endpoints = ['*', 'https://push.mydomain.tld']` (remove `'*'` if you will use your push server only). Then restart the service `systemctl restart mollysocket`.


## (Option A) Proxy server

You will need to proxy everything from `/` to `http://127.0.0.1:8020/` (8020 is the value define in the systemd unit file for `$ROCKET_PORT`, it can be changed if needed).

You also need to forward the `Host` header.

If you proxy from another path like `/molly/` instead of `/`, you also need to pass the original URL.

For Nginx, it looks like:

```
    location / {
        proxy_pass http://127.0.0.1:8020/;
        proxy_set_header            Host $host;
        proxy_set_header X-Original-URL $uri;
    }
```

## (Option B) Air gapped mode

To find the MollySocket QR code:

- If you can use port-forwarding through SSH to your server, then run the following command: `ssh -L 8020:localhost:8020 your_server`, then open http://localhost:8020 on your machine. You can ignore alerts if there are any. Then click on _airgapped mode_.

- If you can't use port-forwarding, change `webserver` to `false` in your config file (_/etc/mollysocket/conf.toml_) and restart your service:

```console
# systemctl restart mollysocket
# journalctl -u mollysocket
# # This should show a QR code
```

After scanning the QR code, you will have a command to copy to run on your server. You must run this command as user `mollysocket` with `MOLLY_CONF=/etc/mollysocket/conf.toml`.

For instance `sudo -su mollysocket MOLLY_CONF=/etc/mollysocket/conf.toml /usr/local/bin/ms connection add baab32b9-d60b-4c39-9e14-15d8f6e1527e 2 thisisrandom 'https://push.mydomain.tld/upthisisrandom?up'`.

## (Optional) More restrictive configuration

Once you have registered Molly (with option A or B), and you will be the only user using this service, you can restrict `allowed_uuids = ['baab32b9-d60b-4c39-9e14-15d8f6e1527e']` and `allowed_endpoints = ['https://push.mydomain.tld/upthisisrandom?up']` in the config file.

## Backup the VAPID privkey

If you wish to backup your VAPID privkey, you can run the following:

```console
# systemd-creds decrypt /etc/mollysocket/vapid.key
```
