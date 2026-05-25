# ma-runtime – English
lang-name = English

own-did-published = Own DID document published to IPNS
own-did-publish-failed = Failed to publish own DID document
own-did-publish-timeout = Own DID document publish timed out after 2 minutes
started = ma runtime started
shutdown-requested = Shutdown requested
closing-endpoint = Closing iroh endpoint...
shutdown-complete = Shutdown complete
status-listening = Status server listening
rpc-message-received = Received RPC message
rpc-message-rejected = RPC message rejected
crud-message-received = Received CRUD message
crud-acl-updated = Root transport ACL updated
ipfs-message-rejected = IPFS message rejected
ctrlc-handler-failed = Ctrl-C handler failed
node-connected = Node connected to protocol
received-encrypted-ma-msg = Received encrypted ma-msg on /ma/ipfs/0.0.1
unknown-rpc-atom = Unknown RPC atom, ignoring
rpc-not-text-atom = RPC payload is not a text atom
rpc-unknown-verb = Unknown RPC verb
rpc-reply-sent = RPC reply sent
ping-received = Received :ping, sending :pong
did-publish-request-received = Received DID document publish request
document-published = Document published
did-publish-cid-reply-sent = Sent CID reply for DID publish
did-publish-resolve-failed = Could not resolve sender to deliver ipfs-publish reply
ipfs-store-request-received = Received IPFS store request
ipfs-stored = Stored content on IPFS
ipfs-store-cid-reply-sent = Sent CID reply
ipfs-store-resolve-failed = Could not resolve sender to deliver ipfs-store reply

# Entity dispatch
bootstrap-complete = Bootstrap complete
entity-loaded = Entity plugin loaded
entity-load-failed = Failed to load entity plugin
entity-not-found = Entity not found, ignoring RPC
entity-dispatched = RPC dispatched to entity
entity-replied = Entity sent RPC reply
root-create-entity = #root: create entity
root-list-entities = #root: list entities
root-delete-entity = #root: delete entity
root-entity-updated = Runtime manifest updated
entity-created = Entity created
entity-deleted = Entity deleted
entity-states-saving = Saving entity states to IPFS
entity-state-saving = Saving entity state
entity-state-saved = Entity state saved
entity-state-empty = Plugin returned empty state, skipping persist
entity-states-saved = Entity states saved
link-set = Link set
ftl-loaded = Lang messages loaded from IPFS

# First-run auto-init
no-config-found = No config found.
initialising-new-identity = Initialising new runtime identity.
generated-headless-config = Generated headless config.

# Ownership / claim
runtime-claimed = Runtime claimed.

# Protected root elements
refuse-delete-root = Steadfastly refuse to delete required root element
no-root-acl = No root ACL configured — runtime is operating without access control
acl-owners-access = Caller granted access as member of +owners
namespace-not-found = Namespace not found
no-ns-gate-acl = No gate ACL configured for this namespace
runtime-claim-persisted = Owner written to config.
runtime-already-claimed = Runtime already claimed.

# Namespace creation (:create)
namespace-created = Namespace created
namespace-already-exists = Namespace already exists
namespace-name-reserved = Reserved namespace name
namespace-create-denied = Namespace create: access denied
namespace-create-usage = Usage: :create <name>

# CRUD validation errors
blob-value-ipfs-path = blob value must be an IPFS path (/ipfs/, /ipns/, or /ipld/)
acl-value-ipfs-path = ACL value must be an IPFS path (/ipfs/, /ipns/, or /ipld/)
kind-value-ipfs-path = kind value must be an IPFS path (/ipfs/, /ipns/, or /ipld/)
cidv1-required = value must be a bare CIDv1 (starts with 'b'; CIDv0 'Qm…' not accepted)
kind-not-found = Kind not found
config-key-protected = config key '%key%' is protected
config-key-no-delete = daemon config key '%key%' cannot be deleted
config-key-not-manifest = config key '%key%' is not a known manifest config key
wrong-crud-protocol = wrong CRUD protocol: %type%
entity-name-invalid = entity name must be printable UTF-8
reserved-entity-name = entity name '%name%' is reserved
