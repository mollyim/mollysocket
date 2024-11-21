# Installation

This file shows how to install and configure mollysocket **on your system using a systemd service**.

**This should be relevant if you use docker**

## Install the binary with a dedicated user

First of all, you need to install mollysocket on your system.

#### Create a dedicated account

The service will run with a dedicated account, so create it and switch to that user:

```
sudo useradd mollysocket -m -d /opt/mollysocket
sudo -su mollysocket
cd
```

#### Install the binary

You have 2 solutions to install the binary.

1. Use an already compiled binary: <https://github.com/mollyim/mollysocket/releases/>. To follow the systemd service, and for ease of use, link the executable (replace with the right version of the binary): `ln -s /opt/mollysocket/mollysocket-amd64-1.2.0 /opt/mollysocket/ms`

2. Use cargo. This method allows you to use cargo to maintain mollysocket up to date. First of all, you need to [install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (you need at least version 1.59). Then, install mollysocket using cargo: `cargo install mollysocket`. *You probably need to install some system packages, like libssl-dev libsqlite3-dev*. To follow the systemd service, and for ease of use, link the executable: `ln -s /opt/mollysocket/.cargo/bin/mollysocket /opt/mollysocket/ms`.

#### Prepare the config file

Download a sample of the config file: `wget -O /opt/mollysocket/prod.toml https://github.com/mollyim/mollysocket/raw/main/config-sample.toml`.

#### Done

Switch back to your usual account: `exit`.

## App configuration

*If you host your own Push server*, then explicitly add it to the allowed endpoints. In `/opt/mollysocket/prod.toml`, edit `allowed_endpoints = ['*', 'https://push.mydomain.tld']` (remove `'*'` if you will use your push server only).

## Install systemd service

Download the [systemd unit file](https://github.com/mollyim/mollysocket/raw/main/mollysocket.service) and place it in the right direction `/etc/systemd/system/`.

### Add a VAPID key

#### Option 1. With systemd-creds (Recommended)

You can use [systemd-creds](https://systemd.io/CREDENTIALS/) to encrypt the vapid key. Run the following command as _root_ to get the systemd-creds parameters:

```console
# sudo -u mollysocket mollysocket vapid gen | systemd-creds encrypt --name=ms_vapid -p - -
SetCredentialEncrypted=ms_vapid: \
        k6iUCUh0RJCQyvL8k8q1UyAAAAABAAAADAAAABAAAAC1lFmbWAqWZ8dCCQkAAAAAgAAAA \
        AAAAAALACMA0AAAACAAAAAAfgAg9uNpGmj8LL2nHE0ixcycvM3XkpOCaf+9rwGscwmqRJ \
        cAEO24kB08FMtd/hfkZBX8PqoHd/yPTzRxJQBoBsvo9VqolKdy9Wkvih0HQnQ6NkTKEdP \
        HQ08+x8sv5sr+Mkv4ubp3YT1Jvv7CIPCbNhFtag1n5y9J7bTOKt2SQwBOAAgACwAAABIA \
        ID8H3RbsT7rIBH02CIgm/Gv1ukSXO3DMHmVQkDG0wEciABAAII6LvrmL60uEZcp5qnEkx \
        SuhUjsDoXrJs0rfSWX4QAx5PwfdFuxPusgE==
```

This will output `SetCredentialEncrypted` you can use in your systemd unit file:

```ini
[Service]
SetCredentialEncrypted=ms_vapid: \
        k6iUCUh0RJCQyvL8k8q1UyAAAAABAAAADAAAABAAAAC1lFmbWAqWZ8dCCQkAAAAAgAAAA \
        AAAAAALACMA0AAAACAAAAAAfgAg9uNpGmj8LL2nHE0ixcycvM3XkpOCaf+9rwGscwmqRJ \
        cAEO24kB08FMtd/hfkZBX8PqoHd/yPTzRxJQBoBsvo9VqolKdy9Wkvih0HQnQ6NkTKEdP \
        HQ08+x8sv5sr+Mkv4ubp3YT1Jvv7CIPCbNhFtag1n5y9J7bTOKt2SQwBOAAgACwAAABIA \
        ID8H3RbsT7rIBH02CIgm/Gv1ukSXO3DMHmVQkDG0wEciABAAII6LvrmL60uEZcp5qnEkx \
        SuhUjsDoXrJs0rfSWX4QAx5PwfdFuxPusgE==
Environment=MOLLY_VAPID_KEY_FILE=%d/ms_vapid
```

#### Option 2. Plaintext

It is also possible to pass the value of the vapid key in plaintext to an environment variable in your unit file. Run the following command as _mollysocket_ user:

```console
$ mollysocket vapid gen
DSqYuWchrB6yIMYJtidvqANeRQic4uWy34afzZRsZnI
```

And use the output of the command in your systemd unit file:

```ini
[Service]
Environment=MOLLY_VAPID_PRIVKEY=DSqYuWchrB6yIMYJtidvqANeRQic4uWy34afzZRsZnI
```

### Start the service

You should be able to see that service now `systemctl status mollysocket`.

You can enable it `systemctl enable --now mollysocket`, the service is now active (`systemctl status mollysocket`), and will be started on system boot.

## (Option A) Proxy server

You will need to proxy everything from `/` to `http://127.0.0.1:8020/` (8020 is the value define in the systemd unit file for `$ROCKET_PORT`, it can be changed if needed).

## (Option B) Air gapped mode

You will have to switch on *air gapped mode* on Molly (Android). It will have a command to copy to run on your server. You must run this command as user `mollysocket` with `MOLLY_CONF=/opt/mollysocket/prod.toml`.

For instance `sudo -su mollysocket MOLLY_CONF=/opt/mollysocket/prod.toml /opt/mollysocket/ms connection add baab32b9-d60b-4c39-9e14-15d8f6e1527e 2 thisisrandom 'https://push.mydomain.tld/upthisisrandom?up'`.

## (Optional) More restrictive configuration

Once you have registered Molly (with option A or B), and you will be the only user using this service, you can restrict `allowed_uuids = ['baab32b9-d60b-4c39-9e14-15d8f6e1527e']` and `allowed_endpoints = ['https://push.mydomain.tld/upthisisrandom?up']` in the config file.
