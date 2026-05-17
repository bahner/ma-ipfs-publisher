# AGENTS.md

## Overview

The MA runtime is a distributed actor-oriented runtime built around:

- `did:ma` identities
- signed and optionally encrypted messages
- IPLD/IPFS persistence
- transport independence (currently centered around iroh)
- portable sandboxed execution through Extism/Wasm plugins

The runtime itself is intentionally minimal.

It does **not** define:
- a MOO
- rooms
- pets
- worlds
- gameplay semantics
- chat semantics

Those are implemented through:
- kinds
- plugins/behaviors
- runtime messages
- policies

The runtime only guarantees:
- identity
- message delivery semantics
- state persistence
- capability enforcement
- deterministic execution boundaries

---

# Core Concepts

## Entity

An entity is the fundamental runtime object.

Minimal structure:

```json
{
  "id": "did:ma:k51...#fragment",
  "owner": "did:ma:k51...#agent",
  "kind": "/ma/pet/0.0.1",
  "behavior": "bafy...",
  "state": {},
  "acl": {}
}
```

Entities:
- receive messages
- maintain persistent state
- execute behavior plugins
- may send messages to other entities

The runtime itself does not care what the entity represents.

---

# Kinds

## Purpose

A `kind` defines:
- runtime expectations
- allowed host functions
- required capabilities
- message semantics
- optional schemas/contracts

A kind is effectively:
- an execution profile
- an API contract
- a capability boundary

Example kinds:

```text
/ma/root/0.0.1
/ma/generic/0.0.1
/ma/mailbox/0.0.1
/ma/pet/0.0.1
/ma/world/0.0.1
```

---

## Why Kinds Exist

Without kinds, arbitrary plugins would:
- race on state
- mutate incompatible structures
- invent incompatible semantics
- violate runtime invariants

Kinds provide structure.

Example:

A `/ma/pet/0.0.1` kind may define:

Required verbs:
- feed
- pet
- wash
- tick

Expected state:
- hunger
- mood
- aggression
- cleanliness

The runtime itself does not understand pets.
Only the kind and plugin do.

---

# Behaviors / Plugins

## Extism Plugins

Behavior is implemented as:
- Wasm
- loaded through Extism
- sandboxed
- capability-limited

Plugins are content-addressed:

```text
behavior = CID
```

Meaning:
- immutable
- cacheable
- reproducible

---

## Plugin Philosophy

Plugins should ideally:
- be deterministic
- be stateless internally
- rely on runtime-managed persistent state

Persistent state lives in the entity, not inside the plugin.

---

## One Plugin Per Entity

An entity should generally have:
- one primary behavior plugin

Reason:
- avoids concurrent state mutation races
- keeps execution deterministic
- simplifies persistence semantics

Instead of:
- multiple independent plugins mutating shared state

The preferred architecture is:
- one plugin dispatching internally

Example:

```text
on_message(msg):
    switch msg.type:
        feed()
        pet()
        wash()
```

---

# State Model

## Runtime-Owned State

Plugins do not own persistence.

The runtime owns:
- serialization
- storage
- synchronization
- encryption

Plugins receive state and return updated state.

---

## Execution Model

Conceptually:

```text
message arrives
 -> runtime loads state
 -> plugin executes
 -> plugin returns updated state
 -> runtime persists state
```

This naturally serializes execution.

---

## Queue Semantics

Entities should generally process:
- one message at a time

This avoids:
- races
- partial updates
- inconsistent state

The runtime may internally implement:
- mailboxes
- queues
- locks
- schedulers

But externally the model should appear sequential.

---

# Root Entity

## Purpose

Every runtime identity reserves:

```text
did:ma:<identity>#root
```

or implicitly:

```text
did:ma:<identity>
```

which resolves to root semantics.

---

## Root Responsibilities

Root is the administrative authority for a runtime instance.

Root handles:
- entity creation
- entity destruction
- entity updates
- behavior assignment
- ownership enforcement
- capability policy

---

## Root Required Functions

The root kind MUST support:

### Create Entity

```text
:create
```

Example payload:

```json
{
  "id": "#pet123",
  "kind": "/ma/pet/0.0.1",
  "behavior": "bafy...",
  "owner": "did:ma:..."
}
```

---

### Destroy Entity

```text
:destroy
```

---

### Update Entity

```text
:update
```

Examples:
- changing ACL
- replacing behavior
- updating metadata

---

### Get Entity

```text
:get
```

---

### List Entities

```text
:list
```

Optional but highly recommended.

---

# Required Runtime Host Functions

These functions form the minimal runtime API exposed to plugins.

---

## send()

Send a message to another entity.

Example:

```rust
send(target, content, content_type)
```

---

## reply()

Reply to the current message.

```rust
reply(content)
```

Automatically uses:
- sender
- correlation
- reply_to

---

## get_state()

Retrieve current persistent state.

---

## set_state()

Replace persistent state.

Usually atomic.

---

## self()

Returns the current entity DID.

---

## sender()

Returns sender DID of current message.

---

## message_id()

Returns current message ID.

---

## now()

Returns current runtime timestamp.

---

## random()

Optional deterministic-safe random generation.

---

## nanoid()

Generate runtime-safe identifiers.

---

# Capability Model

Plugins should never automatically receive unrestricted access.

Capabilities are granted by:
- kind
- runtime
- world policy
- ACL

Example:

```json
{
  "allowed_effects": [
    "say",
    "emote",
    "send"
  ]
}
```

---

# Message Model

Messages are immutable envelopes.

Minimal conceptual structure:

```json
{
  "id": "...",
  "from": "did:ma:...",
  "to": "did:ma:...",
  "created_at": "...",
  "ttl": 30,
  "content_type": "application/x-ma-rpc",
  "content": {}
}
```

---

# Recommended Architecture

## Runtime Responsibilities

The runtime should handle:
- identity
- routing
- persistence
- queues
- sandboxing
- ACL
- capability enforcement
- serialization
- encryption
- transport

---

## Plugin Responsibilities

Plugins should handle:
- behavior
- logic
- interpretation
- state transitions

---

# Recommended Design Principles

## Keep The Runtime Small

The runtime should avoid:
- game semantics
- domain semantics
- business logic
- application assumptions

---

## Prefer Message Passing

Avoid:
- shared mutable memory
- global state
- direct references

Prefer:
- explicit messages

---

## Prefer Immutable Plugins

Plugins are easiest to reason about when:
- immutable
- content-addressed
- deterministic

---

## Prefer Sequential Entity Execution

Concurrency between entities is fine.

Concurrency within a single entity should generally be avoided.

---

# Example

## Pet Entity

```json
{
  "id": "did:ma:k51...#fluffy",
  "kind": "/ma/pet/0.0.1",
  "behavior": "bafybeipetplugin",
  "state": {
    "hunger": 10,
    "mood": "happy"
  }
}
```

Messages:

```text
{ :feed, amount: 5 }
{ :pet }
{ :wash }
```

The plugin decides:
- how hunger changes
- mood transitions
- aggression
- emotes

The runtime only guarantees:
- serialized execution
- persistence
- messaging
- sandboxing

---

# Long-Term Goal

The long-term goal is:
- a minimal distributed actor runtime
- transport-independent
- language-independent
- plugin-driven
- browser-capable
- deterministic enough for distributed systems
- flexible enough for worlds, agents, automation, games, and other actor systems

The runtime is intentionally closer to:
- Erlang actors
- capability systems
- distributed object runtimes

than to:
- traditional game engines
- tightly coupled MMO servers
- shared-memory object systems

