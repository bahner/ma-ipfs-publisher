# 間 Runtime (`ma`)

A lean Tokio daemon that bridges browser-based `did:ma:` identities to Kubo
(IPFS) and manages a Wasm entity/plugin system over iroh QUIC transport.

---

## Overview

`ma` exposes two iroh QUIC services on behalf of clients that cannot reach the
Kubo RPC API directly (e.g. the `ego` browser actor):

| Service | Protocol ID | Purpose |
|---------|-------------|---------|
| IPFS publisher | `/ma/ipfs/0.0.1` | Receives signed DID-document publish requests and forwards them to Kubo |
| RPC | `/ma/rpc/0.0.1` | Entity/plugin dispatch, `#root` entity management, ping/pong |

The daemon derives its own `did:ma:` identity at startup, publishes its own DID
document to IPFS/IPNS, and then enters the main event loop.

---

## Architecture

```
ego (browser WASM) ──iroh QUIC──► /ma/rpc/0.0.1
                                       │
                                       ├─ (unfragmented) → entities/kinds/config management, ping
                                       └─ #<name>        → Wasm plugin dispatch
                   ──iroh QUIC──► /ma/ipfs/0.0.1
                                       │
                                       └─ publish DID document to Kubo/IPNS
```

Entity Wasm plugins are loaded from IPFS at startup.  State is persisted back
to IPFS on graceful shutdown.

---

## Prerequisites

- Rust (latest stable)
- [Kubo](https://docs.ipfs.tech/install/command-line/) running on
  `http://127.0.0.1:5001` (or set `kubo_rpc_url` in config)

---

## Building

```sh
cargo build --release
```

Binary: `target/release/ma`.

---

## First-time setup

### 1. Generate config and secret bundle

```sh
ma --gen-headless-config
```

Writes four random 32-byte keys encrypted with a random passphrase to:

| File | Default path |
|------|-------------|
| Config | `$XDG_CONFIG_HOME/ma/ma.yaml` |
| Secret bundle | `$XDG_CONFIG_HOME/ma/ma.bin` |

The passphrase is printed once — store it or set
`MA_MA_SECRET_BUNDLE_PASSPHRASE` in the environment.

### 2. Bootstrap entities and locales (optional)

Run once after Kubo is ready to set up the entity system:

```sh
ma --gen-root-cid bootstrap.example.yaml
```

This:
1. Publishes kind nodes, entity nodes, and the root `RuntimeManifest` as
   IPLD DAG-CBOR objects to Kubo.
2. Publishes all `*.ftl` files from `locales/` to IPFS.
3. Stores locale links under `RuntimeManifest.locales` (e.g. `nb` → CID).
4. Prints the generated `root_cid` and exits.

Recommended runtime model:
- Runtime head CID is read from IPNS at startup.
- `--root-cid <cid>` overrides IPNS for the current process.
- `locales_cid` is kept in config and can be updated independently.

Warning for `--root-cid`:
- It immediately resets runtime head for this run.
- If you pass the wrong CID, retrieve the previous CID from runtime logs and restart with that value.

On subsequent starts, the daemon restores runtime head from IPNS automatically.
**No locale files or entity code are compiled into the binary at runtime.**

To refresh locales without rebuilding entities:

```sh
make gen-locales-cid
```

This republishes all `locales/*.ftl` and prints a new `locales_cid`
for manual config updates.

It does **not** rewrite the runtime manifest tree or produce a new `root_cid`.

#### `bootstrap.yaml` format

```yaml
runtime:
  owner: did:ma:<your-ipns>   # DID of the operator; shown by `ma --status`

  # Kinds: protocol families → implementations → metadata
  kinds:
    stateless:
      ping:
        protocol: /ma/stateless/ping/0.0.1
        wasi: false             # Rust/native Extism plugin
        api:
          - handle_cast         # fn handle_cast() — process incoming message
        host_functions:
          - ma_send             # full-control outbound envelope
      python:
        protocol: /ma/stateless/python/0.0.1
        wasi: true              # Python/WASI plugin
        api:
          - handle_cast
        host_functions:
          - ma_reply            # reply to sender
          - ma_send
    stateful:
      python:
        protocol: /ma/stateful/python/0.0.1
        wasi: true
        api:
          - init                # called on load with persisted state bytes
          - handle_call
          - save_state          # janitor snapshot trigger
        host_functions:
          - ma_reply
          - ma_send
          - ma_set_state        # persist state to IPFS via runtime

  # Entities: named plugin instances
  entities:
    ping:
      kind: /ma/stateless/ping/0.0.1
      behavior_cid: bafkrei…   # CID of compiled .wasm — `ipfs add plugin.wasm`
      acl:
        "*": x                 # any caller may invoke
    fortune:
      kind: /ma/stateless/python/0.0.1
      behavior_cid: Qm…
      acl:
        "*": x
      # owner: did:ma:<other>  # optional; falls back to runtime.owner
```

---

## Running

```sh
ma
# or
ma --acl-file /etc/ma/acl.yaml --status-bind 0.0.0.0:5003
```

---

## CLI flags

| Flag | Default | Description |
|------|---------|-------------|
| `--gen-root-cid <YAML>` | — | Publish bootstrap manifest + locales to IPFS, print `root_cid`, then exit |
| `--root-cid <CID>` | — | WARNING: resets runtime head for this process; recover prior CID from logs if mistaken |
| `--gen-locales-cid` | `false` | Re-publish locale files only and print a new `locales_cid` |
| `--locales-cid` | `false` | Alias for `--gen-locales-cid` |
| `--locales-dir <DIR>` | `locales` | Directory containing `.ftl` locale files |
| `--acl-file <PATH>` | — | ACL YAML; open (`*`) if omitted |
| `--poll-ms <MS>` | `100` | Service poll interval |
| `--lang <LANG>` | `nb` | Log language: `nb` or `en`. Also `MA_LANG` env var. |
| `--status-bind <ADDR>` | `127.0.0.1:5003` | Status HTTP server address |
| `--gen-headless-config` | — | Generate config + bundle, then exit |

---

## Startup sequence

1. Parse CLI, load config.
2. If `--gen-headless-config`: write config + bundle, exit.
3. If `--gen-root-cid <yaml>`: wait for Kubo, publish IPLD + locales, print `root_cid`, exit.
4. If `--gen-locales-cid` (or `--locales-cid`): wait for Kubo, publish locale map only, print new `locales_cid`, exit.
5. Load ACL.
6. Load `SecretBundle`.
7. Create iroh QUIC endpoint, register services.
8. Build own DID document, spawn background publish to IPNS.
9. Wait for Kubo readiness (10 attempts).
10. Fetch locale from `RuntimeManifest.locales`; fall back to key names if unavailable.
11. Load Wasm entity plugins from IPFS manifest.
12. Start status HTTP server.
13. Main event loop: drain RPC and IPFS inboxes every `poll_ms`.
14. On `Ctrl-C`: save entity states to IPFS, update manifest CID in config, close endpoint.

---

## Entity / plugin system

### Concept

All entities are created and managed via RPC to the built-in `#root` entity.
Entity behaviour lives in Wasm blobs stored on IPFS.

### IPLD schema (DAG-CBOR on Kubo)

```
RuntimeManifest { owner, kinds: {family→implementation→CID}, entities: {name→CID→EntityNode}, locales: {lang→CID} }
KindNode        { protocol, api: [String] }
EntityNode      { name, kind, behavior: {"/":CID}, state: {"/":CID}?, owner, acl: {"/":CID}, wasi: bool }
```

### Management RPC — dot-path protocol

Send CBOR to the **bare** runtime DID (`did:ma:<runtime>`, no fragment).
Verb format: `":ns.path[:[value]]"` — trailing `:` means set/delete.

#### Entities

| Verb (CBOR atom/array) | Args | Effect | Reply |
|------------------------|------|--------|-------|
| `":entities"` | — | List all entity names | JSON `["name", …]` |
| `":entities.<name>"` | — | Get entity node | JSON `EntityNode` |
| `[":entities.<name>:", "<cid>"]` | CID of EntityNode | Create / update entity | JSON `{cid, entity_cid, status}` |
| `":entities.<name>:"` | — | Delete entity | JSON `{cid, status}` |

#### Kinds

| Verb | Args | Effect | Reply |
|------|------|--------|-------|
| `":kinds"` | — | List all kinds | JSON object |
| `":kinds.<family>"` | — | Get all implementations in family | JSON object |
| `":kinds.<family>.<impl>"` | — | Get kind ref | JSON |
| `[":kinds.<family>.<impl>:", "<cid>"]` | CID | Set kind ref | JSON `{cid, status}` |
| `":kinds.<family>.<impl>:"` | — | Delete kind ref | JSON `{cid, status}` |

#### Config

| Verb | Args | Effect | Reply |
|------|------|--------|-------|
| `":config"` | — | Get entire config map | JSON object |
| `":config.<key>"` | — | Get value for key | JSON |
| `[":config.<key>:", "<value>"]` | string or JSON | Set key | JSON `{cid, status}` |
| `":config.<key>:"` | — | Delete key | JSON `{cid, status}` |

#### Ping

| Verb | Effect |
|------|--------|
| `":ping"` | Dispatch to `ping` entity plugin if loaded; otherwise ignored |

#### From ego terminal

```
# list entities
@sky:entities

# get entity
@sky:entities.ping

# create / update entity (CID is the EntityNode dag-cbor CID)
@sky:entities.ping: bafyreiXXXXXXXX

# delete entity
@sky:entities.ping:

# set a kind
@sky:kinds.ma-entity.wasi: bafyreiYYYYYYYY

# set a config key
@sky:config.poll_interval_ms: 250
```

### Plugin ABI

Each Wasm plugin must export:

| Export | Description |
|--------|-------------|
| `init(state: Bytes)` | Called at load with persisted state (empty bytes on first load) |
| `handle_message(PluginInput JSON) → PluginOutput JSON` | Handle an incoming RPC message |
| `get_state() → Bytes` | Return current state for IPFS persistence |

**`PluginInput`:**
```json
{
  "msg": {
    "id": "…", "from": "did:ma:…", "to": "did:ma:…#name",
    "created_at": 1234567890123456789,
    "expires": 1234567890,
    "reply_to": null,
    "content_type": "application/cbor",
    "content": [/* bytes */]
  },
  "ctx": { "self": "did:ma:…#name", "owner": "did:ma:…" }
}
```

**`PluginOutput`:**
```json
{ "reply": { "content_type": "application/json", "content": [/* bytes */] } }
```
`"reply": null` for no reply.

---

## ACL

```yaml
acl:
  "*": "rwx"          # everyone: full access
  "did:ma:bob": "rx"  # read + execute, no write
  "did:ma:eve":       # null = explicit deny
```

- Deny always wins over wildcard allow. An explicit `null` entry denies.
- `/ma/ipfs/0.0.1` requires `w`; `/ma/rpc/0.0.1` requires `x`.
- Fragment stripped from DID-URLs before lookup.
- Default when no file supplied: `"*": "rwx"`.

---

## Status web server

| Endpoint | Content-Type | Description |
|----------|-------------|-------------|
| `GET /` | `text/html` | Human-friendly page |
| `GET /status.json` | `application/json` | Machine-readable status |

### CORS policy (default-deny except allowlist)

Status endpoints are served with a CORS allowlist (not `*`).

Default allowed origins:

- `http://localhost:8000`
- `http://127.0.0.1:8000`
- `https://xn--nf5a.bahner.com`
- `https://間.bahner.com`

Override in config extra as either a YAML list or comma-separated string:

```yaml
status_cors_allowed_origins:
  - http://localhost:8000
  - http://127.0.0.1:8000
  - https://xn--nf5a.bahner.com
  - https://間.bahner.com
```

### `status.json`

```json
{
  "did": "did:ma:<ipns>",
  "ipns": "/ipns/<ipns>",
  "endpoint_id": "<iroh-id>",
  "uptime_secs": 42,
  "ipfs_publisher": true,
  "ipfs_requests": 0,
  "rpc_requests": 0,
  "pings_received": 0,
  "started_at": 1234567890,
  "entity_names": ["greeter", "counter"],
  "runtime": {"/": "bafyreiXXXX"}
}
```

---

## Runtime tuning keys (config extra)

These keys are safe/anonymous operational settings and are also mirrored into
the runtime manifest `config` node.

```yaml
did_resolver_positive_ttl_secs: 60
did_resolver_negative_ttl_secs: 10

did_document_publishing_interval_secs: 300
did_document_publishing_timeout_secs: 120
did_document_publishing_lifetime_hours: 8760

ipns_publish_lifetime_hours: 8760
ipns_publish_allow_offline: true
ipns_publish_resolve: false

status_cors_allowed_origins:
  - http://localhost:8000
  - http://127.0.0.1:8000
  - https://xn--nf5a.bahner.com
  - https://間.bahner.com
```

### IPNS safety notes

- Keep `ipns_secret_key` private and encrypted in `SecretBundle`.
- Prefer `ipns_publish_resolve: false` and explicit republish intervals to
  reduce publish-time dependency on network resolution.
- Use bounded timeouts for publish operations
  (`did_document_publishing_timeout_secs`).

---

## Configuration keys (`ma.yaml`)

| Key | Description |
|-----|-------------|
| `kubo_rpc_url` | Kubo RPC URL (default `http://127.0.0.1:5001`) |
| `secret_bundle_passphrase` | Bundle passphrase. Also `MA_MA_SECRET_BUNDLE_PASSPHRASE` env. |
| `ipfs_publisher` | `true`/`false` — enable `/ma/ipfs/0.0.1` (default `true`) |
| `locales_cid` | CID of locale map (`{lang -> IPLD link}`) used for i18n |

Runtime root CID publishing uses a deterministic Kubo key alias derived from the
runtime identity in `SecretBundle` (same derivation used for own DID publish).

---

## Security notes

- **IPNS keys are zero-use.** The sender's IPNS private key carried in each
  `/ma/ipfs/0.0.1` request is used once for the Kubo publish call and
  immediately zeroized.
- **Replay protection** via a 120-second sliding window on `/ma/ipfs/0.0.1`.
- **Input validation** via `validate_ipfs_publish_request` (ma-core): checks
  content-type, DID document proof, sender identity match.
- **Wasm sandboxing** via extism/Wasmtime.
- **No auto-execution** of arbitrary CIDs — plugins load only from
  `behavior_cid` values in the signed IPLD manifest.
- Config and bundle files are created with mode `0600`.

---

## Makefile

```sh
make build    # cargo build
make release  # cargo build --release
make check    # cargo check
make run      # run the daemon
```
