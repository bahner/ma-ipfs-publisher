# 間 Runtime (`ma`)

A lean daemon that exposes `/ma/ipfs/0.0.1` and `/ma/rpc/0.0.1` on behalf of
clients that cannot reach the Kubo RPC API directly (e.g. browser-based 間
actors). It runs on a host with a Kubo daemon, derives its own `did:ma` identity
at startup, publishes its own DID document, then handles two services over iroh
QUIC transport:

- **`/ma/ipfs/0.0.1`** — optional (enabled by `ipfs_publisher: true` in config, default `true`);
  receives signed IPFS-publish requests and publishes
  `did:ma` DID documents to IPFS/IPNS via Kubo on behalf of the caller.
- **`/ma/rpc/0.0.1`** — receives RPC messages; responds to `:ping` atoms with
  `:pong` replies using the `/ma/rpc/0.0.1` transport.

A minimal status HTTP server runs on `127.0.0.1:5003` (configurable).

## Design principles

- **No backward compatibility.** This is active development. Clean, simple code
  is preferred over compatibility shims for hypothetical users. Remove old forms
  without hesitation when a better design emerges.
- **Two services, nothing more.** Only `/ma/ipfs/0.0.1` and `/ma/rpc/0.0.1`
  are registered. No gossip, no additional RPC.
- **No local protocol code.** All publish logic, validation, secret-bundle
  handling, config, ACL, and transport are provided by the `ma-core` crate.
  Local code is nothing but glue.
- **Keys in memory only.** IPNS private key material arriving in a request is
  used once and immediately zeroized (`zeroize`). The daemon's own keys live in
  an encrypted `SecretBundle` on disk, decrypted into memory at startup and
  never written out again. The ipns key bytes are also zeroized after the own
  DID document is published.
- **Own identity published at startup.** The daemon derives its `did:ma` from
  `ipns_secret_key` via `libp2p-identity`, builds a signed `Document` with
  `did_signing_key` and `did_encryption_key`, and publishes it to Kubo before
  accepting any connections.
- **Strict input validation.** Every incoming CBOR envelope on `/ma/ipfs/0.0.1`
  is validated by `validate_ipfs_publish_request`, which parses the signed
  message, checks content-type, validates and verifies the DID document
  (including its proof signature), and asserts that the sender's IPNS identity
  matches the document's DID.
- **Replay protection.** A `ReplayGuard` (sliding 120-second window) is applied
  to `/ma/ipfs/0.0.1` messages before any processing.
- **ACL with deny-wins semantics.** An explicit `null` entry in the `AclMap`
  denies a principal and overrides any wildcard allow. Capabilities are plain
  strings in YAML sequences — `/ma/rpc/0.0.1` requires `"rpc"`,
  `/ma/ipfs/0.0.1` requires `"ipfs"`.
- **Never default or fall back to open ACLs.** An empty `AclMap` (no entries)
  denies everyone. Code must never construct or substitute an open ACL
  (`{"*": ["*"]}`) as a fallback for a missing or unreadable ACL document.
  A missing ACL is a configuration error — fail loudly rather than silently
  opening access. The `:acl:` delete verb on the root ACL is therefore a no-op;
  to change the ACL, replace it with a new published CID via `:acl: <cid>`.

## Dependencies

Only published crates — **never local paths**:

```toml
anyhow = "1"
axum = { version = "0.7", default-features = false, features = ["http1", "tokio"] }
ciborium = "0.2"
clap = { version = "4", features = ["derive"] }
directories = "5"
ma-core = { version = "0.10.15", default-features = false, features = ["config", "kubo", "iroh", "acl"] }
serde_json = "1"
serde_yaml = "0.9"
tokio = { version = "1", features = ["macros", "rt-multi-thread", "signal", "time", "sync"] }
tracing = "0.1"
zeroize = "1"
```

`ma-core 0.10.10` exposes everything this daemon uses for DID handling, so no
direct `ma-did` dependency is required.

> **Development note:** A `[patch.crates-io] ma-core = { path = "../rust-ma-core" }`
> is active in `Cargo.toml` during development. Remove the patch and update the
> version when publishing.

## Configuration

The default slug is `ma`. Config, secret bundle, and log file
follow XDG paths via ma-core:

| File | Default path |
|------|-------------|
| Config | `$XDG_CONFIG_HOME/ma/ma.yaml` |
| Secret bundle | `$XDG_CONFIG_HOME/ma/ma.bin` |
| ACL | `$XDG_CONFIG_HOME/ma/ma.acl` (optional) |
| Log | `$XDG_DATA_HOME/ma/ma.log` |

`secret_bundle_passphrase` must be set (env `MA_MA_SECRET_BUNDLE_PASSPHRASE`,
or `MA_SECRET_BUNDLE_PASSPHRASE`, or in the YAML config).

`kubo_rpc_url` defaults to `http://127.0.0.1:5001`.

### IPFS publisher toggle

Add to `~/.config/ma/ma.yaml` to disable the IPFS publisher service:

```yaml
ipfs_publisher: false
```

The key lives in `config.extra` (a `serde_yaml::Mapping`). Default is `true`
(enabled) when the key is absent.

### First-time setup

```sh
ma --gen-headless-config
```

Generates a fresh `SecretBundle` with four random 32-byte keys (`iroh_secret_key`,
`ipns_secret_key`, `did_signing_key`, `did_encryption_key`), encrypts it with a
random passphrase, and writes both config and bundle to the XDG paths with mode
`0600`.

### Runtime

```sh
ma
# or with an explicit ACL file:
ma --acl-file /etc/ma/acl.yaml
# or with a custom status bind address:
ma --status-bind 0.0.0.0:5003
```

## CLI flags

| Flag | Default | Description |
|------|---------|-------------|
| `--acl-file <PATH>` | — | ACL YAML file; defaults to open (`*`) if omitted |
| `--poll-ms <MS>` | `100` | Service poll interval |
| `--status-bind <ADDR>` | `127.0.0.1:5003` | Status web server bind address |
| `--gen-headless-config` | — | Generate config + secret bundle and exit |

## ACL format

The ACL YAML must contain an `acl:` map from principal to a YAML sequence of
capability strings. The default when no file is supplied is open
(`"*": [rpc, ipfs]`).

```yaml
acl:
  "*": [rpc, ipfs]            # everyone: RPC + IPFS publish
  "did:ma:alice": ["*"]       # alice: all capabilities
  "did:ma:bob": [rpc]         # bob: RPC only, no IPFS publish
  "did:ma:eve":               # null = explicit deny
```

Built-in capability strings:

| Capability | Required by |
|------------|-------------|
| `"rpc"` | `/ma/rpc/0.0.1` |
| `"ipfs"` | `/ma/ipfs/0.0.1` |
| `"read"` | (reserved — future read-only access) |
| `"create"` | Create namespaces/entities |
| `"update"` | Update namespaces/entities |
| `"delete"` | Delete namespaces/entities |
| `"*"` | Wildcard — grants all capabilities when used in an Allow set |

Arbitrary capability strings are valid — entity and namespace ACLs may use
verb names or sub-namespace names as capabilities.

Rules:

- **Deny always wins** over wildcard allow. An explicit `null` entry (bare key,
  `key: ~`, or `key: null` in YAML) is an explicit deny.
- Direct principal lookup wins over the `"*"` wildcard.
- Fragment stripped from DID-URLs before lookup.
- ACL is checked on every incoming message on both services.

## Status web server

Runs on `127.0.0.1:5003` (or `--status-bind`). Two endpoints:

| Endpoint | Content | Description |
|----------|---------|-------------|
| `GET /` | `text/html` | Human-friendly status page |
| `GET /status.json` | `application/json` | Compact JSON status object |

The JSON object contains:

```json
{
  "did": "did:ma:<ipns>",
  "endpoint_id": "<iroh-id>",
  "uptime_secs": 42,
  "ipfs_publisher": true,
  "ipfs_requests": 0,
  "rpc_requests": 0,
  "pings_received": 0,
  "started_at": 1234567890
}
```

## Wire format rules

**All data over iroh transport is CBOR. No JSON is sent between peers.**

- RPC requests: CBOR atom (`:verb`) or array `[":verb", arg1, arg2, …]`.
- RPC replies: CBOR atom (`:pong`, `:ok`, `:error`) or tuple `[":ok", payload]` / `[":error", reason]`.
- Entity content in replies: CBOR-encoded `EntityNode` (same structure as
  stored in IPFS DAG-CBOR), never JSON.
- Entity definitions written by users in ego use **YAML** as the human-readable
  format. YAML is stored to IPFS via `dag_put` (DAG-CBOR), and the resulting
  CID is the canonical reference.

The one exception: Kubo's HTTP RPC API (`/api/v0/…`) speaks JSON. That is an
internal implementation detail of `crate::kubo` and is invisible to peers.

## RPC protocol

Content types are defined in ma-spec, not ma-core — they are string literals:

| Direction | Content-Type |
|-----------|--------------|
| Request | `application/x-ma-rpc` |
| Reply | `application/x-ma-rpc-reply` |

RPC verbs are CBOR-encoded text strings beginning with `:`.

### Dot-path grammar

Unfragmented RPC messages (addressed to `did:ma:<ipns>`, no fragment) use a
dot-path grammar rooted in four namespaces:

```
:entities[.<name>][:<verb>]  — entity management
:kinds[.<family>[.<impl>]]   — kind/protocol registry (read-only)
:config[.<key>]              — runtime config
:ping                        — liveness check
```

| Pattern | Meaning |
|---------|---------|
| `:entities` | list all entity names |
| `:entities.<name>` | get EntityNode (as CBOR) |
| `:entities.<name>:` | delete entity |
| `:entities.<name>: <cid>` | upsert entity by CID (fetches DAG-CBOR from IPFS) |
| `:entities.<name>:edit` | return current EntityNode for client-side editing |
| `:ping` | reply `:pong` |

Fragment-addressed messages (`did:ma:<ipns>#<name>`) are routed directly to
the named entity plugin (Wasm `handle_cast`).

### `:edit` verb

`:entities.<name>:edit` returns the current `EntityNode` as CBOR. The **client**
(ego) is responsible for opening an editor so the user can modify it. After
editing, the client publishes the updated node to IPFS (`dag_put`), then sends
`:entities.<name>: <new-cid>` to register it. The runtime never initiates an
editor session; it only stores and retrieves by CID.

### `:ping`

Replies with `:pong` to `did:ma:<sender_ipns>#ping`. The reply sets `reply_to`
to the originating message's ID and is delivered via `endpoint.outbox()`.

## ma-core API used

| Purpose | Call |
|---------|------|
| Config + CLI | `Config::from_args(&args, MA_DEFAULT_SLUG)` |
| First-time config | `Config::gen_headless(&args, MA_DEFAULT_SLUG)` |
| Key material | `SecretBundle::load(path, passphrase)` |
| IPNS derivation | `libp2p_identity::ed25519::SecretKey::try_from_bytes` → `Keypair` → `PeerId::to_base58()` |
| Own DID document | `Document::new`, `SigningKey::from_private_key_bytes`, `EncryptionKey::from_private_key_bytes`, `VerificationMethod::new`, `document.sign`, `document.marshal` |
| iroh endpoint | `ma_core::new_ma_endpoint(iroh_secret_key)` |
| Register service | `endpoint.service("/ma/ipfs/0.0.1")` + `endpoint.service("/ma/rpc/0.0.1")` |
| Kubo publisher | `IpfsDidPublisher::new(kubo_rpc_url)` |
| Kubo readiness | `publisher.wait_until_ready(attempts)` |
| Request validation | `validate_ipfs_publish_request(message_cbor)` |
| Publish | `publisher.publish_document(did_doc_json, ipns_key_b64)` |
| Replay guard | `ReplayGuard::default()` + `check_and_insert(&headers)` |
| ACL | `AclMap` (serde-deserialised from YAML) + `check_cap(acl, caller, cap)` |
| Outbox (pong) | `endpoint.outbox(&resolver, &sender_did, "/ma/rpc/0.0.1").await` → `outbox.send(&msg)` |
| Resolver | `IpfsGatewayResolver::new(kubo_rpc_url)` |

## Security notes

- `application/x-ma-ipfs-request` payloads **must** be encrypted envelopes per
  the ma-spec (messaging-format.md §2.2.1). The iroh transport provides the
  encrypted channel; `validate_ipfs_publish_request` enforces content-type.
- The IPNS private key embedded in each `/ma/ipfs/0.0.1` request is the
  sender's full publishing authority over their DID. It is used once and
  zeroized immediately after the Kubo call.
- The daemon's own `ipns_secret_key` bytes are zeroized immediately after the
  own DID document is published at startup.
- The daemon carries no signing or encryption keys of its own beyond those
  needed for transport and its own DID identity — it cannot impersonate any
  other `did:ma` identity.
- All files written by ma-core (config, bundle) use mode `0600`.
- The `iroh_secret_key` is only for the iroh QUIC transport layer; it is
  distinct from `ipns_secret_key` which roots the `did:ma` identity.
