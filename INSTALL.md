# Installation

## Install the binary with a dedicated user

First of all, you need to install cargo on your system.

The service will run with a dedicated account, so create it and switch to that user:

```
sudo useradd mollysocket -m -d /opt/mollysocket
sudo -su mollysocket
cd
```

Install mollysocket using cargo: `cargo install mollysocket`.

Link the executable for ease of use `ln -s /opt/mollysocket/.cargo/bin/mollysocket /opt/mollysocket/ms`.

Run `MOLLY_CONF=/opt/mollysocket/prod.toml /opt/mollysocket/ms c l` once to create the config file and remove `mollysocket.db`, it will be recreated later. This is to ensure it will have the correct permissions with the service.

Switch back to your usual account: `exit`.

## App configuration

In `/opt/mollysocket/prod.toml`, replace the `allowed_endpoints` with `allowed_endpoints = ['*']`. You will be able to set something more restrictive after your phone registration.

*If you host your own Push server*, then explicitly add it to the allowed endpoints `allowed_endpoints = ['*', 'https://push.mydomain.tld']` (remove `'*'` if you will use your push server only).

Change the path for the `db = '/opt/mollysocket/mollysocket.db'`.

## Install systemd service

Download the [systemd unit file](https://github.com/mollyim/mollysocket/raw/main/mollysocket.service) and place it in the right direction `/etc/systemd/system/`.

You should be able to see that service now `systemctl status mollysocket`.

You can enable it `systemctl enable --now mollysocket`, the service is now active (`systemctl status mollysocket`), and will be started on system boot.

## (Option A) Proxy server

You will need to proxy everything from `/` to `http://127.0.0.1:8020/` (8020 is the value define in the systemd unit file for `$ROCKET_PORT`, it can be changed if needed).

## (Option B) Air gapped mode

You will have to switch on *air gapped mode* on Molly (Android). It will have a command to copy to run on your server. You must run this command as user `mollysocket` with `MOLLY_CONF=/opt/mollysocket/prod.toml`.

For instance `sudo -su mollysocket MOLLY_CONF=/opt/mollysocket/prod.toml /opt/mollysocket/ms connection add baab32b9-d60b-4c39-9e14-15d8f6e1527e 2 thisisrandom 'https://push.mydomain.tld/upthisisrandom?up'`.

## (Optional) More restrictive configuration

Once you have registered Molly (with option A or B), and you will be the only user using this service, you can restrict `allowed_uuids = ['baab32b9-d60b-4c39-9e14-15d8f6e1527e']` and `allowed_endpoints = ['https://push.mydomain.tld/upthisisrandom?up']` in the config file.
