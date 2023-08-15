# MollySocket

This software is still in alpha.

MollySocket allows getting signal notifications via [UnifiedPush](https://unifiedpush.org/). It works like a linked device, which doesn't have encryption key, connected to the Signal server. Everytime it receives an encrypted event, it notifies your mobile via UnifiedPush.

## Status
This is still in alpha.

Review of the server code is welcomed.

The associated pull request for Molly (android) can be found here: <https://github.com/mollyim/mollyim-android/pull/152>.

We are currently testing it and the efficiency of the different strategy. Your feedback is welcome here: <https://github.com/MollySocket/mollysocket/issues/1>.

## Configuration
* MollySocket web server does not provide TLS. It should be accessible behind a reverse proxy. It is possible to use MollySocket without the web server: see the Air Gaped mode on Android settings.

### Environment
* Use the environment variable `ROCKET_PORT` to change the port used by the webserver.
* Use the environment variable `MOLLY_CONF` to change the path to the configuration file.
* Use the environment variable `RUST_LOG` to change the log level.

### Configuration file
* You can allow registration for all accounts by setting `allowed_uuids` to `['*']`. Else set your account ids in the array: `['account_id1','account_id2']`.
* You can allow all endpoints by adding `*` to `allowed_endpoints` (for instance `['*']`). Else you can add the allowed endpoints in the array: `['https://dom1.tld','https//dom2.tld:4443]`. **Note that endpoints on your local network must be allowed explicitly**
* You can specify the db path in the `db` setting.

### Android
* If MollySocket webserver is not accessible from the Internet, you can enable the **Air Gaped** mode. You will have to register your connection manually on MollySocket.
* There are 2 different fetch strategies:
  * REST: Every time MollySocket receives a(n encrypted) data : it notifies Molly via UnifiedPush and Molly fetch using the rest strategy (that's a built-in strategy)
  * Websocket: Every time MollySocket receives a(n encrypted) data : it notifies Molly via UnifiedPush if it hasn't notified the last 5 seconds. Then Molly open the websocket for 20secs. This strategy avoid to reach some rate limit for some public provider such as https://ntfy.sh but may increase a little bit the battery drain.


## About security

**Relative to Signal security**

MollySocket receives the credentials for a linked device and does not receive any encryption key. Which means:
* Someone with access to MollySocket database can't change the identity key, to impersonate users. See [setKeys](https://github.com/signalapp/Signal-Server/blob/v8.67.0/service/src/main/java/org/whispersystems/textsecuregcm/controllers/KeysController.java#L111).
* Someone with access to MollySocket database may be able to use the credentials of linked devices to spam the Signal server and hit the rate limits. I haven't checked if this would temporarily block the account or just the linked device. (Availability risk)
* Someone with access to MollySocket database may be able to change some account field in a destructive way. For instance changing the account Name to something random. The cleartext will be random since these field are encrypted and require encryption keys to be properly encrypted.

## License
AGPLv3: see [LICENSE.txt](./LICENSE.txt).

## Disclaimer
This project is NOT sponsored by or affiliated to Signal Messenger, Signal Foundation or the Molly project (*).

The software is produced independently of Signal and carries no guarantee about quality, security or anything else. Use at your own risk.

\* But they are ok with name "MollySocket"
