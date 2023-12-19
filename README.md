# MollySocket

MollySocket allows getting signal notifications via [UnifiedPush](https://unifiedpush.org/). It works like a linked device, which doesn't have an encryption key, connected to the Signal server. Everytime it receives an encrypted event, it notifies your mobile via UnifiedPush.

## Overview

```mermaid
---
title: Message flow
---
graph TD
    
    MS[fa:fa-tablets MollySocket]
    S[fa:fa-comment Signal Server]
    P[fa:fa-server Push server]
    subgraph "fa:fa-mobile Android"
        D[fa:fa-tower-broadcast Distributor App]
        MA[fa:fa-tablets Molly Android]
    end
    MS -- 1. Persistent connection --> S
    MS -- 2. 'Notifications present' --> P
    P -- 3. 'Notications present for Molly' --> D
    D -- 4. 'Check Signal servers' --> MA
    MA -- 5. 'Got messages?' --> S
    S -- 6. Messages --> MA
```

## Setup

1. You need the right flavor of Molly to use UnifiedPush: <https://github.com/mollyim/mollyim-android-unifiedpush>.
2. You can install MollySocket via:
    1. Docker/Podman: `docker pull ghcr.io/mollyim/mollysocket:latest`
    2. Crates.io: `cargo install mollysocket` (see [INSTALL.md](INSTALL.md) for the setup) 
    3. Direct download: <https://github.com/mollyim/mollysocket/releases> (see [INSTALL.md](INSTALL.md) for the setup)
3. A [distributor app](https://unifiedpush.org/users/distributors/) (easiest is [ntfy](https://f-droid.org/en/packages/io.heckel.ntfy/))

You can optionally install your own push server like ntfy or NextPush.
For beginners, you can use a free service like ntfy.sh (do consider donating if you have the means).

## Web Server

MollySocket exposes a web server so that Molly can send the information it needs to operate. You should configure TLS with a reverse proxy in front of MollySocket.

It is possible to use MollySocket without the web server, but you will have to manually register the information MollySocket needs: see the **Air Gapped** mode on Android settings.

## Configuration

The configuration file uses the [TOML format](https://toml.io/). Below is an overview of configuration options. You can configure each parameter using either the conf file, the environment variable or the cli option (if available).

| Parameter (conf. file) | Environment variable       | Cli Option  | Description                                       | Default              | Examples                                                |
|------------------------|----------------------------|-------------|---------------------------------------------------|----------------------|---------------------------------------------------------|
|                        | RUST_LOG                \* | -v/-vv/-vvv | Verbosity                                         | error                | RUST_LOG=info, RUST_LOG=debug                           |
|                        | MOLLY_CONF                 | -c \*       | Path to the configuration file, optional          |                      | /etc/mollysocket.conf                                   |
| host                   | MOLLY_HOST              \* |             | Listening address of the web server               | 127.0.0.1            | 0.0.0.0                                                 |
| port                   | MOLLY_PORT              \* |             | Listening port of the web server                  | 8020                 | 8080                                                    |
| allowed_endpoints      | MOLLY_ALLOWED_ENDPOINTS \* |             | List of UnifiedPush servers                       | `["*"]`              | `["*"]`,`["https://yourdomain.tld","https://ntfy.sh"]`  |
| allowed_uuids          | MOLLY_ALLOWED_UUIDS     \* |             | UUIDs of signal accounts that may use this server | `["*"]`              | `["*"]`, `["abcdef-12345-tuxyz-67890"]`                 |
| db                     | MOLLY_DB                \* |             | Path to the DB                                    | `db.sqlite`          | `"/data/ms.sqlite"`                                     |

\* Takes the precedence

#### `allowed_endpoints`

These are the UnifiedPush endpoints that MollySocket may use to push notifications with. 

⚠️ **If you self-host your push server, add your push server to the `allowed_endpoints`.** ⚠️

That's because, for security reasons, endpoints on your local network must be allowed explicitly. You just have to set the scheme (https), the domain and the port if required. For instance `allowed_endpoints=['https://push.mydomain.tld']`

#### `allowed_uuids`

You can allow registration for all accounts by setting `allowed_uuids` to `['*']`. Else set your account ids in the array: `['account_id1','account_id2']`.

The account IDs are showing in the Molly application under Settings > Notifications > UnifiedPush.
You need to activate UnifiedPush first before your account ID is shown.

## About security

**Relative to Signal security**

MollySocket receives the credentials for a linked device and does not receive any encryption key. Which means:
* Someone with access to MollySocket database can't change the identity key, to impersonate users. See [setKeys](https://github.com/signalapp/Signal-Server/blob/v8.67.0/service/src/main/java/org/whispersystems/textsecuregcm/controllers/KeysController.java#L111).
* Someone with access to MollySocket database may be able to use the credentials of linked devices to spam the Signal server and hit the rate limits. I haven't checked if this would temporarily block the account or just the linked device. (Availability risk)
* Someone with access to MollySocket database may be able to change some account field in a destructive way. For instance changing the account Name to something random. The cleartext will be random since these field are encrypted and require encryption keys to be properly encrypted.

## License
AGPLv3: see [LICENSE.txt](./LICENSE.txt).

## Disclaimer
This project is NOT sponsored by or affiliated to Signal Messenger or Signal Foundation.

The software is produced independently of Signal and carries no guarantee about quality, security or anything else. Use at your own risk.

