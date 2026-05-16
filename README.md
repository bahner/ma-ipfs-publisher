# ma-ipfs-publisher

A lean daemon that bridges browser-based `did:ma` actors to a local
[Kubo](https://github.com/ipfs/kubo) IPFS node.  It listens for signed
publish requests over an encrypted [iroh](https://iroh.computer/) QUIC
transport and forwards them to Kubo's RPC API, so that clients that cannot
reach Kubo directly (e.g. browser tabs) can still publish their `did:ma` DID
documents to IPFS/IPNS.

Two protocols are served:

| Protocol | Purpose |
|---|---|
| `/ma/ipfs/0.0.1` | Receives signed DID-document publish requests and forwards them to Kubo |
| `/ma/rpc/0.0.1` | Ping/pong health-check; replies `:pong` to any `:ping` |

A minimal HTTP status page is also served on `127.0.0.1:5003`.

---

## Prerequisites

| Requirement | Notes |
|---|---|
| **Rust ≥ 1.95** | Install via [rustup](https://rustup.rs/) |
| **Kubo** (go-ipfs) | Must be running and reachable at `http://127.0.0.1:5001` (default) |

Make sure Kubo is started and its RPC API is up before running
`ma-ipfs-publisher`.  The daemon will refuse to start if Kubo is unreachable.

---

## Building

```sh
# Debug build
cargo build

# Optimised release build (also copies the binary to the project root)
make
# or
cargo build --release
```

---

## First-time setup

Run the generator once to create a fresh identity and configuration:

```sh
ma-ipfs-publisher --gen-headless-config
```

This will:

1. Generate four random 32-byte keys:
   `iroh_secret_key`, `ipns_secret_key`, `did_signing_key`, `did_encryption_key`.
2. Encrypt them into a `SecretBundle` with a random passphrase.
3. Write the config **and** the encrypted bundle to their XDG paths (mode `0600`).

| File | Default path |
|---|---|
| Config | `$XDG_CONFIG_HOME/ma/ma-ipfs-publisher.yaml` |
| Secret bundle | `$XDG_CONFIG_HOME/ma/ma-ipfs-publisher.bin` |
| Log | `$XDG_DATA_HOME/ma/ma-ipfs-publisher.log` |

On most Linux systems `$XDG_CONFIG_HOME` defaults to `~/.config` and
`$XDG_DATA_HOME` to `~/.local/share`.

---

## Configuration

The config file is YAML.  All fields can be overridden by environment
variables.

### Required

| Key | Environment variable | Description |
|---|---|---|
| `secret_bundle_passphrase` | `MA_MA_IPFS_PUBLISHER_SECRET_BUNDLE_PASSPHRASE` *or* `MA_SECRET_BUNDLE_PASSPHRASE` | Passphrase that decrypts the secret bundle |

After `--gen-headless-config` the passphrase is already written into the
config file.  In production you may prefer to supply it only via the
environment variable (and remove it from the YAML).

### Optional

| Key | Default | Description |
|---|---|---|
| `kubo_rpc_url` | `http://127.0.0.1:5001` | URL of the Kubo RPC API |

### Example config snippet

```yaml
secret_bundle_passphrase: "change-me"
kubo_rpc_url: "http://127.0.0.1:5001"
```

---

## Running

```sh
# Simplest — reads config from the XDG paths
ma-ipfs-publisher

# With a custom ACL file
ma-ipfs-publisher --acl-file /etc/ma/acl.yaml

# With a custom status bind address
ma-ipfs-publisher --status-bind 0.0.0.0:5003

# Norwegian log messages (default)
MA_LANG=nb ma-ipfs-publisher

# English log messages
MA_LANG=en ma-ipfs-publisher
```

On startup the daemon will:

1. Load and decrypt the secret bundle.
2. Derive its `did:ma` identity from `ipns_secret_key`.
3. Build and publish its own signed DID document to Kubo (background task,
   up to 2-minute timeout).
4. Start accepting iroh connections.
5. Start the HTTP status server.

Shut down cleanly with `Ctrl-C`.

---

## CLI flags

| Flag | Default | Description |
|---|---|---|
| `--acl-file <PATH>` | *(open — allow everyone)* | ACL YAML file |
| `--poll-ms <MS>` | `100` | Service poll interval (milliseconds) |
| `--status-bind <ADDR>` | `127.0.0.1:5003` | Status web server bind address |
| `--lang <LANG>` | `nb` | Log-message language (`nb` or `en`) |
| `--gen-headless-config` | — | Generate config + secret bundle, then exit |

---

## ACL

If no `--acl-file` is given the daemon is open to everyone (`*`).

An ACL file must contain an `acl:` sequence:

```yaml
acl:
  - "*"                        # allow everyone …
  - "!did:ma:<bad-ipns>"       # … except this identity and all its DID-URLs
  - "did:ma:<trusted-ipns>"    # (explicit allow — useful without *)
```

Rules:

- **Deny always wins.**  A `!` prefix means deny.  A deny overrides any
  allow, including `*`.
- An identity-level deny (`did:ma:<ipns>` with no fragment) automatically
  blocks every DID-URL under that identity.
- Entries are validated at load time; an invalid entry causes a hard error.
- ACL is checked on both `/ma/ipfs/0.0.1` and `/ma/rpc/0.0.1`.

---

## Status endpoints

| Endpoint | Content-Type | Description |
|---|---|---|
| `GET /` | `text/html` | Human-readable status page |
| `GET /status.json` | `application/json` | Machine-readable JSON |

Example JSON response:

```json
{
  "did": "did:ma:<ipns>",
  "endpoint_id": "<iroh-node-id>",
  "uptime_secs": 42,
  "ipfs_requests": 7,
  "rpc_requests": 3,
  "pings_received": 3,
  "started_at": 1747389600
}
```

---

## Security notes

- **Private keys never leave memory in plaintext.**  The `SecretBundle` is
  encrypted on disk.  IPNS key bytes arriving in a publish request are used
  once and immediately zeroized.  The daemon's own `ipns_secret_key` bytes
  are also zeroized right after the own DID document is published at startup.
- **Replay protection.**  A 120-second sliding-window `ReplayGuard` is applied
  to every `/ma/ipfs/0.0.1` message.
- **Strict validation.**  Every incoming publish request is verified: CBOR
  structure, content-type, DID document proof signature, and IPNS
  identity-to-DID binding.
- **The daemon cannot impersonate other identities.**  It carries only its own
  transport key (`iroh_secret_key`) and its own DID identity keys — it has no
  access to any other actor's signing or encryption keys.
- Config and bundle files are written with mode `0600`.

---

## Makefile targets

| Target | Description |
|---|---|
| `make` / `make all` | Release build, binary copied to project root |
| `make lint` | `cargo clippy`, `cargo fmt --check`, `mdl *.md` |
| `make test` | Strict clippy (pedantic + nursery) |
| `make clean` | `cargo clean` + remove local binary |
| `make publish` | `scp` the binary to the `ma-ipfs-publisher` host |
| `make distclean` | `clean` + remove `Cargo.lock` |
