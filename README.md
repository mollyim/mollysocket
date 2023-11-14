# MollySocket

MollySocket allows getting signal notifications via [UnifiedPush](https://unifiedpush.org/). It works like a linked device, which doesn't have an encryption key, connected to the Signal server. Everytime it receives an encrypted event, it notifies your mobile via UnifiedPush.

## Setup

1. You need the right flavor of Molly to use UnifiedPush: <https://github.com/mollyim/mollyim-android-unifiedpush>.
2. You can install MollySocket via:
    1. Crates.io: `cargo install mollysocket`
    2. Docker/Podman: `docker pull ghcr.io/mollyim/mollysocket:latest`
    3. Direct download: <https://github.com/mollyim/mollysocket/releases>

## Configuration

MollySocket web server does not provide TLS. It should be accessible behind a reverse proxy.

It is possible to use MollySocket without the web server: see the Air Gapped mode on Android settings.
In this mode MollySocket doesn't 

### Environment variables
* `ROCKET_PORT` : port used by the webserver.
* `MOLLY_CONF` : path to the configuration file.
* `RUST_LOG` : log level.

### Configuration file

The configuration file uses the [TOML format](https://toml.io/). Below is an overview of configuration options.

| Option            | Description                                       | Examples                                                | Default              |
|-------------------|---------------------------------------------------|---------------------------------------------------------|----------------------|
| allowed_endpoints | List of UnifiedPush servers                       | `["*"]`,`["https://yourdomain.tld", "https://ntfy.sh"]` | `["http://0.0.0.0"]` |
| allowed_uuids     | UUIDs of signal accounts that may use this server | `["*"]`, `["abcdef-12345-tuxyz-67890"]`                 | `["*"]`              |
| db                | Path to the DB                                    | `"/data/ms.sqlite"`                                     | `db.sqlite`          |

#### `allowed_endpoints`

These are the UnifiedPush endpoints that MollySocket may use to push notifications with. 

**Note that endpoints on your local network must be allowed explicitly**

As [per spec](https://unifiedpush.org/spec/server/), an endpoint is an [IRI](https://en.wikipedia.org/wiki/Internationalized_Resource_Identifier).
Examples:
 - `http://localhost`
 - `https://mydomain.tld`
 - `https://mydomain.tld:443`
 - `https://ntfy.sh/mySecretSubscription`

You can thus be very open and allow everything with `["*"]` or be increasingly specific even defining which subscription should be used.
The subscription URI can be found in your distributor app.

#### `allowed_uuids`

You can allow registration for all accounts by setting `allowed_uuids` to `['*']`. Else set your account ids in the array: `['account_id1','account_id2']`.

The account IDs are showing in the Molly application under Settings > Notifications > UnifiedPush.
You may need to activate UnifiedPush first before your account ID is shown.

### Android
* If MollySocket webserver is not accessible from the Internet, you can enable the **Air Gapped** mode. You will have to register your connection manually on MollySocket.
* Every time MollySocket receives a(n encrypted) data : it notifies Molly via UnifiedPush if it hasn't notified the last 5 seconds. Then Molly open the websocket for 60secs.


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

